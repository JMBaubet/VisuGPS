use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::{StreamExt, SinkExt};
use log::{info, error, debug};
use local_ip_address::local_ip;
use tokio::io::AsyncWriteExt;
use tauri::{AppHandle, Manager, Emitter};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::oneshot;
use serde::{Serialize, Deserialize};

// Embed the client files
const REMOTE_CLIENT_INDEX_HTML: &str = include_str!("../../src/remote_client/index.html");
const REMOTE_CLIENT_STYLE_CSS: &str = include_str!("../../src/remote_client/style.css");
const REMOTE_CLIENT_MAIN_JS: &str = include_str!("../../src/remote_client/main.js");

use crate::remote_clients;
use crate::AppState;

use lazy_static::lazy_static;

// Shared state for active WebSocket connections (senders)
pub type ClientSender = tokio_tungstenite::tungstenite::protocol::WebSocket<tokio::net::TcpStream>;
pub type ClientSenders = Arc<Mutex<HashMap<String, futures::channel::mpsc::Sender<Message>>>>;

// Shared state for pending pairing requests
pub type PendingPairingRequests = Arc<Mutex<HashMap<String, (String, oneshot::Sender<bool>)>>>;

lazy_static! {
    pub static ref CLIENT_SENDERS: ClientSenders = Arc::new(Mutex::new(HashMap::new()));
    pub static ref PENDING_PAIRING_REQUESTS: PendingPairingRequests = Arc::new(Mutex::new(HashMap::new()));
}

// Structs for client-server communication
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairingRequest {
    pub r#type: String,
    pub client_id: String,
    pub pairing_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairingResponse {
    pub r#type: String,
    pub status: String,
    pub reason: Option<String>,
    pub app_state: Option<String>, // Current app view (MainView, VisualizeView, etc.)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteCommand {
    pub r#type: String,
    pub client_id: String,
    pub command: String,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteCommandResponse {
    pub r#type: String,
    pub status: String,
    pub message: String,
    pub app_state: Option<String>,
}

pub async fn start_remote_server(app_handle: AppHandle, port: u16) {
    let my_local_ip = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(e) => {
            error!("Impossible d'obtenir l'adresse IP locale: {}", e);
            return;
        }
    };

    let addr = format!("0.0.0.0:{}", port);
    info!("Serveur de télécommande WebSocket et HTTP en écoute sur ws://{}:{}/ et http://{}:{}/remote", my_local_ip, port, my_local_ip, port);

    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("Impossible de lier le serveur WebSocket/HTTP à {}: {}", addr, e);
            return;
        }
    };

    loop {
        let (mut stream, _) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => {
                error!("Erreur lors de l'acceptation de la connexion: {}", e);
                continue;
            }
        };
        let peer_addr = stream.peer_addr().expect("connected streams should have a peer address");
        debug!("Nouvelle connexion entrante de: {}", peer_addr);

        let app_handle_clone = app_handle.clone();
        // Use the global static variables
        // let client_senders_clone = CLIENT_SENDERS.clone(); // Removed
        let pending_pairing_requests_clone = PENDING_PAIRING_REQUESTS.clone(); // Re-added

                tokio::spawn(async move {
                    let mut buffer = [0; 1024];
                    let n = match stream.peek(&mut buffer).await {
                        Ok(n) => n,
                        Err(e) => {
                            error!("Erreur lors du peek sur le stream: {}", e);
                            return;
                        }
                    };
        
                    let header = String::from_utf8_lossy(&buffer[..n]);
        
                    if header.contains("Upgrade: websocket") {
                        // Handle WebSocket connection
                        info!("Tentative de connexion WebSocket de: {}", peer_addr);
                        let ws_stream = match accept_async(stream).await {
                            Ok(s) => s,
                            Err(e) => {
                                error!("Erreur lors de l'établissement de la connexion WebSocket avec {}: {}", peer_addr, e);
                                return;
                            }
                        };
        
                        let (mut write, mut read) = ws_stream.split();
        
                        // Create a channel for sending messages to this client
                        let (tx_mpsc, mut rx_mpsc) = futures::channel::mpsc::channel::<Message>(10);
                        // Store the sender in the global map
                        CLIENT_SENDERS.lock().unwrap().insert(peer_addr.to_string(), tx_mpsc);
        
                        // Task to send messages to the client
                        let write_task = tokio::spawn(async move {
                            while let Some(message) = rx_mpsc.next().await {
                                if let Err(e) = write.send(message).await {
                                    error!("Erreur lors de l'envoi du message au client {}: {}", peer_addr, e);
                                    break;
                                }
                            }
                        });
        
                        // Reading loop
                        let read_task = tokio::spawn(async move {
                            while let Some(message) = read.next().await {
                                match message {
                                    Ok(msg) => {
                                        if msg.is_text() {
                                            let msg_text = msg.to_text().unwrap_or("");
                                            if msg_text.is_empty() { continue; }
                                            info!("Reçu du client {}: {}", peer_addr, msg_text);
        
                                            let (app_env_path, current_app_view) = {
                                                let app_state_mutex = app_handle_clone.state::<Mutex<AppState>>();
                                                let app_state = app_state_mutex.lock().unwrap();
                                                (app_state.app_env_path.clone(), app_state.current_view.clone())
                                            };
        
                                            if let Ok(pairing_req) = serde_json::from_str::<PairingRequest>(msg_text) {
                                                if let Ok(is_authorized) = remote_clients::is_client_authorized(&app_env_path, &pairing_req.client_id) {
                                                    if is_authorized {
                                                        info!("Client {} déjà autorisé.", pairing_req.client_id);
                                                        let response = PairingResponse {
                                                            r#type: "pairing_response".to_string(),
                                                            status: "accepted".to_string(),
                                                            reason: None,
                                                            app_state: Some(current_app_view.clone()),
                                                        };
                                                        if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                            if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                                error!("Erreur lors de l'envoi de la réponse de couplage au client {}: {}", peer_addr, e);
                                                            }
                                                        }
                                                    } else {
                                                        info!("Client {} non autorisé, demande de couplage.", pairing_req.client_id);
                                                        let (tx_decision, rx_decision) = oneshot::channel::<bool>();
                                                        pending_pairing_requests_clone.lock().unwrap().insert(pairing_req.client_id.clone(), (pairing_req.pairing_code.clone(), tx_decision));
        
                                                        app_handle_clone.emit("ask_pairing_approval", &pairing_req).expect("Failed to emit pairing approval event");
        
                                                        match rx_decision.await {
                                                            Ok(true) => {
                                                                info!("Couplage accepté par le frontend pour {}", pairing_req.client_id);
                                                                remote_clients::add_authorized_client(&app_env_path, pairing_req.client_id.clone(), format!("Mobile Client ({})", pairing_req.pairing_code))
                                                                    .expect("Failed to add authorized client");
                                                                let response = PairingResponse {
                                                                    r#type: "pairing_response".to_string(),
                                                                    status: "accepted".to_string(),
                                                                    reason: None,
                                                                    app_state: Some(current_app_view.clone()),
                                                                };
                                                                if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                                    if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                                        error!("Erreur lors de l'envoi de la réponse de couplage au client {}: {}", peer_addr, e);
                                                                    }
                                                                }
                                                            },
                                                            Ok(false) => {
                                                                info!("Couplage refusé par le frontend pour {}", pairing_req.client_id);
                                                                let response = PairingResponse {
                                                                    r#type: "pairing_response".to_string(),
                                                                    status: "refused".to_string(),
                                                                    reason: Some("Refusé par l'utilisateur.".to_string()),
                                                                    app_state: Some(current_app_view.clone()),
                                                                };
                                                                if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                                    if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                                        error!("Erreur lors de l'envoi de la réponse de couplage au client {}: {}", peer_addr, e);
                                                                    }
                                                                }
                                                            },
                                                            Err(e) => {
                                                                error!("Erreur lors de l'attente de la décision du frontend pour {}: {}", pairing_req.client_id, e);
                                                            }
                                                        }
                                                        pending_pairing_requests_clone.lock().unwrap().remove(&pairing_req.client_id);
                                                    }
                                                } else {
                                                    error!("Erreur lors de la vérification de l'autorisation du client {}", pairing_req.client_id);
                                                }
                                            } else if let Ok(remote_command) = serde_json::from_str::<RemoteCommand>(msg_text) {
                                                let is_authorized = remote_clients::is_client_authorized(&app_env_path, &remote_command.client_id).unwrap_or(false);
                                                if !is_authorized {
                                                    let response = RemoteCommandResponse {
                                                        r#type: "command_response".to_string(),
                                                        status: "unauthorized".to_string(),
                                                        message: "Client non autorisé.".to_string(),
                                                        app_state: Some(current_app_view.clone()),
                                                    };
                                                    if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                        if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                            error!("Erreur lors de l'envoi de la réponse de commande au client {}: {}", peer_addr, e);
                                                        }
                                                    }
                                                    continue;
                                                }
        
                                                                                    info!("Commande reçue du client {}: {}", remote_command.client_id, remote_command.command);
                                                
                                                                                    // --- DIAGNOSTIC HACK ---
                                                                                    println!("[HACK] Emitting test-event from remote_control.rs");
                                                                                    if let Err(e) = app_handle_clone.emit("test-event", "Hello from remote_control!") {
                                                                                        eprintln!("Hack emit failed: {}", e);
                                                                                    }
                                                                                    // --- END HACK ---
                                                
                                                                                    app_handle_clone.emit(&format!("remote_command::{}", remote_command.command), remote_command.payload)
                                                                                        .expect("Failed to emit remote command event");        
                                                let response = RemoteCommandResponse {
                                                    r#type: "command_response".to_string(),
                                                    status: "success".to_string(),
                                                    message: format!("Commande {} reçue.", remote_command.command),
                                                    app_state: Some(current_app_view.clone()),
                                                };
                                                if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                    if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                        error!("Erreur lors de l'envoi de la réponse de commande au client {}: {}", peer_addr, e);
                                                    }
                                                }
                                            } else {
                                                info!("Message texte inconnu du client {}: {}", peer_addr, msg_text);
                                            }
                                        } else if msg.is_binary() {
                                            debug!("Reçu des données binaires du client {}", peer_addr);
                                        } else if msg.is_ping() {
                                            debug!("Reçu un ping du client {}", peer_addr);
                                        } else if msg.is_close() {
                                            info!("Client {} a envoyé une trame de fermeture.", peer_addr);
                                            break;
                                        }
                                    },
                                    Err(e) => {
                                        error!("Erreur de lecture du message du client {}: {}", peer_addr, e);
                                        break;
                                    }
                                }
                            }
                        });
        
                        // Wait for both tasks to complete
                        let _ = tokio::join!(write_task, read_task);
        
                        CLIENT_SENDERS.lock().unwrap().remove(&peer_addr.to_string());
                        info!("Client {} déconnecté.", peer_addr);
                    }
                    else if header.starts_with("GET /remote") {
                        // Handle HTTP GET request for static files
                        info!("Requête HTTP GET pour le client distant de: {}", peer_addr);
                        let path = header.split_whitespace().nth(1).unwrap_or("/");
                        debug!("Chemin de la requête HTTP: {}", path);
        
                        let (content, content_type) = match path {
                            "/remote" | "/remote/" | "/remote/index.html" => (REMOTE_CLIENT_INDEX_HTML, "text/html"),
                            "/remote/style.css" => (REMOTE_CLIENT_STYLE_CSS, "text/css"),
                            "/remote/main.js" => (REMOTE_CLIENT_MAIN_JS, "text/javascript"),
                            _ => {
                                let _ = stream.write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").await;
                                return;
                            }
                        };
        
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: {0}\r\nContent-Length: {1}\r\n\r\n{2}",
                            content_type,
                            content.len(),
                            content
                        );
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            error!("Erreur lors de l'envoi de la réponse HTTP à {}: {}", peer_addr, e);
                        }
                        if let Err(e) = stream.flush().await {
                            error!("Erreur lors du flush du stream HTTP à {}: {}", peer_addr, e);
                        }
                        debug!("Réponse HTTP envoyée à {}.", peer_addr);
                    } else {
                        // Unknown request type
                        debug!("Requête inconnue de {}:\n{}", peer_addr, header);
                        let _ = stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").await;
                    } 
                    
                });    }
}

// Fonction pour notifier toutes les télécommandes connectées d'un changement d'état
pub fn send_app_state_update(_app_handle: &AppHandle, new_state: &str) {
    let mut client_senders = CLIENT_SENDERS.lock().unwrap();
    let message = serde_json::json!({
        "type": "app_state_update",
        "appState": new_state
    });
    
    let message_text = serde_json::to_string(&message).unwrap();
    let message_ws = Message::Text(message_text);
    
    info!("Envoi de la mise à jour d'état '{}' à {} télécommandes connectées", new_state, client_senders.len());
    
    for (client_addr, sender) in client_senders.iter_mut() {
        if let Err(e) = sender.try_send(message_ws.clone()) {
            error!("Erreur lors de l'envoi de la mise à jour d'état au client {}: {}", client_addr, e);
        }
    }
}
