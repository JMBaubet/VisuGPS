use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures::{StreamExt, SinkExt};
use log::{info, error, debug};
use local_ip_address::local_ip;
use tokio::io::AsyncWriteExt;
use tauri::{command, AppHandle, Manager, Emitter};

#[command]
pub fn update_visualize_view_state(app_handle: AppHandle, state: VisualizeViewState) {
    send_visualize_view_state_update(&app_handle, state);
}
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::oneshot;
use serde::{Serialize, Deserialize};

// Embed the client files
const REMOTE_CLIENT_INDEX_HTML: &str = include_str!("../../src/remote_client/index.html");
const REMOTE_CLIENT_STYLE_CSS: &str = include_str!("../../src/remote_client/style.css");
const REMOTE_CLIENT_MAIN_JS_TEMPLATE: &str = include_str!("../../src/remote_client/main.js");
const REMOTE_CLIENT_UTILS_JS: &str = include_str!("../../src/remote_client/remote-utils.js");
const REMOTE_CLIENT_UI_JS: &str = include_str!("../../src/remote_client/remote-ui.js");
const REMOTE_CLIENT_WEBSOCKET_JS: &str = include_str!("../../src/remote_client/remote-websocket.js");

use crate::remote_clients;
use crate::AppState;

use crate::remote_blacklist;

use lazy_static::lazy_static;

// Fonction pour générer le JavaScript avec l'IP dynamique
fn generate_main_js_with_ip(server_ip: &str, server_port: u16) -> String {
    REMOTE_CLIENT_MAIN_JS_TEMPLATE
        .replace("const WS_SERVER_IP = \"192.168.1.65\";", &format!("const WS_SERVER_IP = \"{}\";", server_ip))
        .replace("const WS_SERVER_PORT = 9001;", &format!("const WS_SERVER_PORT = {};", server_port))
}

// Shared state for active WebSocket connections (senders)
pub type ClientSender = tokio_tungstenite::tungstenite::protocol::WebSocket<tokio::net::TcpStream>;
pub type ClientSenders = Arc<Mutex<HashMap<String, futures::channel::mpsc::Sender<Message>>>>;

// Shared state for pending pairing requests
pub type PendingPairingRequests = Arc<Mutex<HashMap<String, (String, oneshot::Sender<bool>)>>>;

lazy_static! {
    pub static ref CLIENT_SENDERS: ClientSenders = Arc::new(Mutex::new(HashMap::new()));
    pub static ref PENDING_PAIRING_REQUESTS: PendingPairingRequests = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ACTIVE_CLIENT_ID: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    pub static ref CLIENT_ID_TO_ADDR: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
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
#[allow(non_snake_case)]
pub struct PairingResponse {
    pub r#type: String,
    pub status: String,
    pub reason: Option<String>,
    pub appState: Option<String>, // Current app view (MainView, VisualizeView, etc.)
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
#[serde(rename_all = "camelCase")]
pub struct VisualizeViewState {
    pub is_back_button_visible: bool,
    pub is_controls_card_visible: bool,
    pub is_altitude_visible: bool,
    pub is_commune_widget_visible: bool,
    pub is_distance_display_visible: bool,
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
        let my_local_ip_clone = my_local_ip.clone();
        let port_clone = port;

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

                // Reading loop and cleanup logic are now self-contained in this task
                let read_task = tokio::spawn(async move {
                    let mut client_id_for_this_connection: Option<String> = None;

                    while let Some(message) = read.next().await {
                        match message {
                            Ok(msg) => {
                                if msg.is_text() {
                                    let msg_text = msg.to_text().unwrap_or("");
                                    if msg_text.is_empty() { continue; }
                                    
                                    let (app_env_path, current_app_view) = {
                                        let app_state_mutex = app_handle_clone.state::<Mutex<AppState>>();
                                        let app_state = app_state_mutex.lock().unwrap();
                                        (app_state.app_env_path.clone(), app_state.current_view.clone())
                                    };

                                    if let Ok(pairing_req) = serde_json::from_str::<PairingRequest>(msg_text) {
                                        if remote_blacklist::is_client_blacklisted(&app_env_path, &pairing_req.client_id).unwrap_or(false) {
                                            log::warn!("Requête de couplage refusée pour le client blacklisté : {}", pairing_req.client_id);
                                            let response = PairingResponse {
                                                r#type: "pairing_response".to_string(),
                                                status: "refused".to_string(),
                                                reason: Some("Cet appareil a été bloqué.".to_string()),
                                                appState: Some(current_app_view.clone()),
                                            };
                                            if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                    log::error!("Erreur lors de l'envoi de la réponse de blacklist au client {}: {}", peer_addr, e);
                                                }
                                            }
                                            continue; // Continuer d'ignorer la logique de couplage après avoir répondu
                                        }

                                        let is_other_client_active = {
                                            let active_client_id = ACTIVE_CLIENT_ID.lock().unwrap();
                                            active_client_id.is_some() && active_client_id.as_deref() != Some(&pairing_req.client_id)
                                        };

                                        if is_other_client_active {
                                            info!("Couplage refusé pour {}: un autre client est déjà actif.", pairing_req.client_id);
                                            let response = PairingResponse {
                                                r#type: "pairing_response".to_string(),
                                                status: "refused".to_string(),
                                                reason: Some("Une autre télécommande est déjà connectée.".to_string()),
                                                appState: Some(current_app_view.clone()),
                                            };
                                            if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                    error!("Erreur lors de l'envoi de la réponse de refus au client {}: {}", peer_addr, e);
                                                }
                                            }
                                            continue;
                                        }

                                                                                        if let Ok(is_authorized) = remote_clients::is_client_authorized(&app_env_path, &pairing_req.client_id) {
                                                                                    if is_authorized {
                                    
                                                                                        {
                                                                                            let mut active_client_id = ACTIVE_CLIENT_ID.lock().unwrap();
                                                                                            *active_client_id = Some(pairing_req.client_id.clone());
                                                                                            CLIENT_ID_TO_ADDR.lock().unwrap().insert(pairing_req.client_id.clone(), peer_addr.to_string());
                                                                                        }
                                                                                        client_id_for_this_connection = Some(pairing_req.client_id.clone());
                                                                                        app_handle_clone.emit("remote_control_status_changed", "connected").unwrap();
                                        
                                                                                        let response = PairingResponse {
                                                                                            r#type: "pairing_response".to_string(),
                                                                                            status: "accepted".to_string(),
                                                                                            reason: None,
                                                                                            appState: Some(current_app_view.clone()),
                                                                                        };
                                                                                        if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                                                            if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                                                                error!("Erreur lors de l'envoi de la réponse de couplage au client {}: {}", peer_addr, e);
                                                                                            }
                                                                                        }
                                                                                    } else {
                                    
                                                                                        
                                                                                        if current_app_view != "Main" && current_app_view != "Settings" {
                                
                                                                                            let response = PairingResponse {
                                                                                                r#type: "pairing_response".to_string(),
                                                                                                status: "refused".to_string(),
                                                                                                reason: Some("Le couplage est uniquement autorisé depuis l'accueil ou les paramètres.".to_string()),
                                                                                                appState: Some(current_app_view.clone()),
                                                                                            };
                                                                                            if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                                                                if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                                                                    error!("Erreur lors de l'envoi de la réponse de refus (vue) au client {}: {}", peer_addr, e);
                                                                                                }
                                                                                            }
                                                                                            continue;
                                                                                        }
                                        
                                                                                        let (tx_decision, rx_decision) = oneshot::channel::<bool>();
                                                                                        pending_pairing_requests_clone.lock().unwrap().insert(pairing_req.client_id.clone(), (pairing_req.pairing_code.clone(), tx_decision));
                                        
                                                                                        app_handle_clone.emit("ask_pairing_approval", &pairing_req).expect("Failed to emit pairing approval event");
                                        
                                                                                        match rx_decision.await {
                                                                                            Ok(true) => {
                                        
                                                                                                {
                                                                                                    let mut active_client_id = ACTIVE_CLIENT_ID.lock().unwrap();
                                                                                                    *active_client_id = Some(pairing_req.client_id.clone());
                                                                                                    CLIENT_ID_TO_ADDR.lock().unwrap().insert(pairing_req.client_id.clone(), peer_addr.to_string());
                                                                                                }
                                                                                                client_id_for_this_connection = Some(pairing_req.client_id.clone());
                                                                                                app_handle_clone.emit("remote_control_status_changed", "connected").unwrap();
                                        
                                                                                                remote_clients::add_authorized_client(&app_env_path, pairing_req.client_id.clone(), format!("Mobile Client ({})", pairing_req.pairing_code))
                                                                                                    .expect("Failed to add authorized client");
                                                                                                let response = PairingResponse {
                                                                                                    r#type: "pairing_response".to_string(),
                                                                                                    status: "accepted".to_string(),
                                                                                                    reason: None,
                                                                                                    appState: Some(current_app_view.clone()),
                                                                                                };
                                                                                                if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
                                                                                                    if let Err(e) = sender.try_send(Message::Text(serde_json::to_string(&response).unwrap())) {
                                                                                                        error!("Erreur lors de l'envoi de la réponse de couplage au client {}: {}", peer_addr, e);
                                                                                                    }
                                                                                                }
                                                                                            },
                                                                                                                                                Ok(false) => {
                                                                                                                                                    let reason = "Refusé par l'utilisateur.".to_string();
                                                                                                                                                    if let Err(e) = remote_blacklist::add_to_blacklist(&app_env_path, pairing_req.client_id.clone(), reason.clone()) {
                                                                                                                                                        log::error!("Impossible d'ajouter le client à la blacklist : {}", e);
                                                                                                                                                    }
                                                                                            
                                                                                                                                                    let response = PairingResponse {
                                                                                                                                                        r#type: "pairing_response".to_string(),
                                                                                                                                                        status: "refused".to_string(),
                                                                                                                                                        reason: Some(reason),
                                                                                                                                                        appState: Some(current_app_view.clone()),
                                                                                                                                                    };                                                                                                if let Some(sender) = CLIENT_SENDERS.lock().unwrap().get_mut(&peer_addr.to_string()) {
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
                                                                            break;
                                                                        }
                                                                    },
                                                                    Err(e) => {
                                                                        error!("Erreur de lecture du message du client {}: {}", peer_addr, e);
                                                                        break;
                                                                    }
                                                                }
                                                            }
                                        
                                                            // Cleanup on disconnect
                                                            if let Some(id) = &client_id_for_this_connection {
                                                                let mut active_client = ACTIVE_CLIENT_ID.lock().unwrap();
                                                                if active_client.as_ref() == Some(id) {
                                                                    *active_client = None;
                                                                    app_handle_clone.emit("remote_control_status_changed", "disconnected").unwrap();
                                                                }
                                                                CLIENT_ID_TO_ADDR.lock().unwrap().remove(id);
                                                            }
                                                        });
                let _ = tokio::join!(write_task, read_task);

                CLIENT_SENDERS.lock().unwrap().remove(&peer_addr.to_string());
                info!("Client {} déconnecté.", peer_addr);
            } else if header.starts_with("GET /remote") {
                // Handle HTTP GET request for static files
                let path = header.split_whitespace().nth(1).unwrap_or("/");
                debug!("Chemin de la requête HTTP: {}", path);

                match path {
                    "/remote" | "/remote/" | "/remote/index.html" => {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                            REMOTE_CLIENT_INDEX_HTML.len(),
                            REMOTE_CLIENT_INDEX_HTML
                        );
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            error!("Erreur lors de l'envoi de la réponse HTTP à {}: {}", peer_addr, e);
                        }
                    },
                    "/remote/style.css" => {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nContent-Length: {}\r\n\r\n{}",
                            REMOTE_CLIENT_STYLE_CSS.len(),
                            REMOTE_CLIENT_STYLE_CSS
                        );
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            error!("Erreur lors de l'envoi de la réponse HTTP à {}: {}", peer_addr, e);
                        }
                    },
                    "/remote/remote-utils.js" => {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/javascript\r\nContent-Length: {}\r\n\r\n{}",
                            REMOTE_CLIENT_UTILS_JS.len(),
                            REMOTE_CLIENT_UTILS_JS
                        );
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            error!("Erreur lors de l'envoi de la réponse HTTP à {}: {}", peer_addr, e);
                        }
                    },
                    "/remote/remote-ui.js" => {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/javascript\r\nContent-Length: {}\r\n\r\n{}",
                            REMOTE_CLIENT_UI_JS.len(),
                            REMOTE_CLIENT_UI_JS
                        );
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            error!("Erreur lors de l'envoi de la réponse HTTP à {}: {}", peer_addr, e);
                        }
                    },
                    "/remote/remote-websocket.js" => {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/javascript\r\nContent-Length: {}\r\n\r\n{}",
                            REMOTE_CLIENT_WEBSOCKET_JS.len(),
                            REMOTE_CLIENT_WEBSOCKET_JS
                        );
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            error!("Erreur lors de l'envoi de la réponse HTTP à {}: {}", peer_addr, e);
                        }
                    },
                    "/remote/main.js" => {
                        let dynamic_js = generate_main_js_with_ip(&my_local_ip_clone, port_clone);
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: text/javascript\r\nContent-Length: {}\r\n\r\n{}",
                            dynamic_js.len(),
                            dynamic_js
                        );
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            error!("Erreur lors de l'envoi de la réponse HTTP à {}: {}", peer_addr, e);
                        }
                    },
                    _ => {
                        let _ = stream.write_all(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").await;
                        return;
                    }
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
    let active_client_id_lock = ACTIVE_CLIENT_ID.lock().unwrap();
    if let Some(active_id) = active_client_id_lock.as_ref() {
        let client_id_to_addr_lock = CLIENT_ID_TO_ADDR.lock().unwrap();
        if let Some(addr) = client_id_to_addr_lock.get(active_id) {
            let mut senders_lock = CLIENT_SENDERS.lock().unwrap();
            if let Some(sender) = senders_lock.get_mut(addr) {
                let message = serde_json::json!({
                    "type": "app_state_update",
                    "appState": new_state,
                    "clientId": active_id
                });
                let message_text = serde_json::to_string(&message).unwrap();
                if let Err(e) = sender.try_send(Message::Text(message_text)) {
                    log::error!("Erreur lors de l'envoi de la mise à jour d'état au client {}: {}", addr, e);
                }
            }
        }
    }
}

pub fn send_visualize_view_state_update(_app_handle: &AppHandle, state: VisualizeViewState) {
    let active_client_id_lock = ACTIVE_CLIENT_ID.lock().unwrap();
    if let Some(active_id) = active_client_id_lock.as_ref() {
        let client_id_to_addr_lock = CLIENT_ID_TO_ADDR.lock().unwrap();
        if let Some(addr) = client_id_to_addr_lock.get(active_id) {
            let mut senders_lock = CLIENT_SENDERS.lock().unwrap();
            if let Some(sender) = senders_lock.get_mut(addr) {
                let message = serde_json::json!({
                    "type": "visualize_view_state_update",
                    "state": state
                });
                let message_text = serde_json::to_string(&message).unwrap();
                if let Err(e) = sender.try_send(Message::Text(message_text)) {
                    log::error!("Error sending visualize view state update to client {}: {}", addr, e);
                }
            }
        }
    }
}

// Send animation state updates to the remote control
pub fn send_animation_state_update(_app_handle: &AppHandle, new_state: &str) {
    let active_client_id_lock = ACTIVE_CLIENT_ID.lock().unwrap();
    if let Some(active_id) = active_client_id_lock.as_ref() {
        let client_id_to_addr_lock = CLIENT_ID_TO_ADDR.lock().unwrap();
        if let Some(addr) = client_id_to_addr_lock.get(active_id) {
            let mut senders_lock = CLIENT_SENDERS.lock().unwrap();
            if let Some(sender) = senders_lock.get_mut(addr) {
                let message = serde_json::json!({
                    "type": "animation_state_update",
                    "animationState": new_state
                });
                let message_text = serde_json::to_string(&message).unwrap();
                if let Err(e) = sender.try_send(Message::Text(message_text)) {
                    log::error!("Erreur lors de l'envoi de la mise à jour de l'état d'animation au client {}: {}", addr, e);
                }
            }
        }
    }
}

#[tauri::command]
pub fn notify_pause_state_changed(paused: bool) {
    let active_client_id_lock = ACTIVE_CLIENT_ID.lock().unwrap();
    if let Some(active_id) = active_client_id_lock.as_ref() {
        let client_id_to_addr_lock = CLIENT_ID_TO_ADDR.lock().unwrap();
        if let Some(addr) = client_id_to_addr_lock.get(active_id) {
            let mut senders_lock = CLIENT_SENDERS.lock().unwrap();
            if let Some(sender) = senders_lock.get_mut(addr) {
                let message = serde_json::json!({
                    "type": "pause_state_update",
                    "payload": paused
                });
                let message_text = serde_json::to_string(&message).unwrap();
                if let Err(e) = sender.try_send(Message::Text(message_text)) {
                    log::error!("Erreur lors de l'envoi de la mise à jour de l'état de pause au client {}: {}", addr, e);
                }
            }
        }
    }
}

#[tauri::command]
pub fn disconnect_active_remote_client(app_handle: AppHandle) -> Result<(), String> {
    let active_client_id_lock = ACTIVE_CLIENT_ID.lock().unwrap();
    if let Some(client_id) = active_client_id_lock.as_ref() {
        
        // Remove client from the authorized list
        let app_env_path = {
            let state = app_handle.state::<Mutex<AppState>>();
            let state_lock = state.lock().unwrap();
            state_lock.app_env_path.clone()
        };
        remote_clients::remove_authorized_client(&app_env_path, client_id)?;

        // Send shutdown message to the client
        let client_id_to_addr_lock = CLIENT_ID_TO_ADDR.lock().unwrap();
        if let Some(addr) = client_id_to_addr_lock.get(client_id) {
            let mut senders_lock = CLIENT_SENDERS.lock().unwrap();
            if let Some(sender) = senders_lock.get_mut(addr) {
                let shutdown_msg = serde_json::json!({
                    "type": "server_shutdown",
                    "reason": "Autorisation révoquée par l'hôte."
                });
                if sender.try_send(Message::Text(shutdown_msg.to_string())).is_err() {
                    log::warn!("Impossible d'envoyer le message de déconnexion au client, mais son autorisation a bien été révoquée.");
                }
            }
        }
    } else {
        return Err("Aucun client actif à déconnecter.".to_string());
    }
    Ok(())
}
