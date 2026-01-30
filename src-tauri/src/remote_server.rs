use axum::{
    extract::{Json, State, FromRef},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use log::{debug, info, warn, error};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tauri::{AppHandle, Manager, Emitter};

use crate::remote_sse::{sse_handler, healthcheck_handler, SseMessage, SseState};
use crate::remote_clients;
use crate::remote_blacklist;
use crate::AppState;
use crate::get_setting_value;

/// État partagé du serveur remote control
#[derive(Clone)]
pub struct RemoteServerState {
    pub sse_state: Arc<SseState>,
    pub app_handle: AppHandle,
    pub settings: serde_json::Value,
}

impl FromRef<RemoteServerState> for Arc<SseState> {
    fn from_ref(state: &RemoteServerState) -> Self {
        state.sse_state.clone()
    }
}

/// Structure pour la requête de pairing
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairingRequest {
    pub client_id: String,
    pub pairing_code: String,
}

/// Structure pour la réponse de pairing
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct PairingResponse {
    pub status: String,
    pub reason: Option<String>,
    pub appState: Option<String>,
    pub settings: Option<RemoteSettings>,
    pub session_token: Option<String>,
    pub debug_info: Option<String>,
}

/// Paramètres envoyés au client remote
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteSettings {
    pub speed_min_value: f32,
    pub speed_max_value: f32,
    pub speed_default_value: f32,
}

/// Structure pour les commandes envoyées par le client
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteCommandRequest {
    pub command: String,
    pub payload: Option<serde_json::Value>,
}

/// Réponse standardisée pour les commandes
#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub status: String,
    pub message: String,
}

/// Handler pour POST /api/pair
async fn pair_handler(
    State(state): State<RemoteServerState>,
    Json(request): Json<PairingRequest>,
) -> impl IntoResponse {
    debug!("Requête de pairing reçue pour client: {}", request.client_id);

    // Vérifier si la limite de télécommandes est atteinte
    let hb_state = state.app_handle.state::<crate::HeartbeatState>();
    let now = chrono::Utc::now().timestamp();
    let active_clients = hb_state.active_clients.lock().unwrap();

    // Compter les clients réellement actifs (heartbeat < 10s)
    let active_count = active_clients.iter()
        .filter(|(id, &t)| id.as_str() != request.client_id && (now - t) < 10)
        .count();

    if active_count >= 2 {
        warn!("Tentative de pairing rejetée: limite de 2 appareils connectés atteinte");
        return (StatusCode::CONFLICT, Json(PairingResponse {
            status: "busy".to_string(),
            reason: Some("La limite de 2 télécommandes connectées est atteinte.".to_string()),
            appState: None,
            settings: None,
            session_token: None,
            debug_info: None,
        })).into_response();
    }
    drop(active_clients); // Libérer le lock avant la suite

    let app_env_path = {
        let app_state = state.app_handle.state::<Mutex<AppState>>();
        let app_state_lock = app_state.lock().unwrap();
        app_state_lock.app_env_path.clone()
    };

    let current_app_view = {
        let app_state = state.app_handle.state::<Mutex<AppState>>();
        let app_state_lock = app_state.lock().unwrap();
        app_state_lock.current_view.clone()
    };

    // Vérifier si le client est blacklisté
    if remote_blacklist::is_client_blacklisted(&app_env_path, &request.client_id).unwrap_or(false) {
        debug!("Nouveau client mis en attente: {}", request.client_id);
        return (StatusCode::FORBIDDEN, Json(PairingResponse {
            status: "refused".to_string(),
            reason: Some("Cet appareil a été bloqué.".to_string()),
            appState: Some(current_app_view),
            settings: None,
            session_token: None,
            debug_info: None,
        })).into_response();
    }

    // Vérifier si le client est déjà autorisé
    let is_authorized = remote_clients::is_client_authorized(&app_env_path, &request.client_id)
        .unwrap_or(false);

    if is_authorized {
        debug!("Client déjà autorisé: {}", request.client_id);
        
        // Générer un token de session
        let session_token = uuid::Uuid::new_v4().to_string(); 

        // Lire les settings à chaud pour avoir les valeurs à jour
        let settings_path = app_env_path.join("settings.json");
        debug!("Reading settings from: {:?}", settings_path);
        
        let current_settings = if settings_path.exists() {
             match std::fs::read_to_string(&settings_path) {
                Ok(content) => {
                    debug!("Settings file read successfully ({} bytes)", content.len());
                    serde_json::from_str(&content).unwrap_or_else(|e| {
                        error!("Failed to parse settings.json: {}", e);
                        state.settings.clone()
                    })
                },
                Err(e) => {
                    error!("Failed to read settings.json: {}", e);
                    state.settings.clone()
                },
            }
        } else {
            warn!("Settings file not found at {:?}", settings_path);
            state.settings.clone()
        };

        // Helper to parse setting value as f32 (handles Number and String)
        let parse_setting_f32 = |val: Option<&serde_json::Value>, default: f32| -> f32 {
            val.and_then(|v| {
                if let Some(f) = v.as_f64() {
                    Some(f as f32)
                } else if let Some(s) = v.as_str() {
                    s.parse::<f32>().ok()
                } else {
                    None
                }
            }).unwrap_or(default)
        };

        let speed_min = parse_setting_f32(
            get_setting_value(&current_settings, "data.groupes.Visualisation.groupes.Lecture.groupes.Vitesse.parametres.min_value"),
            0.1
        );
        let speed_max = parse_setting_f32(
            get_setting_value(&current_settings, "data.groupes.Visualisation.groupes.Lecture.groupes.Vitesse.parametres.max_value"),
            20.0
        );
        let speed_default = parse_setting_f32(
            get_setting_value(&current_settings, "data.groupes.Visualisation.groupes.Lecture.groupes.Vitesse.parametres.default_value"),
            1.0
        );
        
        debug!("Extracted Speed Settings: Min={}, Max={}, Default={}", speed_min, speed_max, speed_default);

        let remote_settings = RemoteSettings {
            speed_min_value: speed_min,
            speed_max_value: speed_max,
            speed_default_value: speed_default,
        };

        // Émettre l'événement de connexion
        let _ = state.app_handle.emit("remote_control_status_changed", "connected");

        return (StatusCode::OK, Json(PairingResponse {
            status: "accepted".to_string(),
            reason: None,
            appState: Some(current_app_view),
            settings: Some(remote_settings),
            session_token: Some(session_token),
            debug_info: None,
        })).into_response();
    }

    // Vérifier si le pairing est autorisé depuis la vue actuelle
    if current_app_view != "Main" && current_app_view != "Settings" && current_app_view != "Visualize" {
        debug!("Pairing refusé: vue non autorisée ({})", current_app_view);
        return (StatusCode::FORBIDDEN, Json(PairingResponse {
            status: "refused".to_string(),
            reason: Some("Le couplage est uniquement autorisé depuis l'accueil, les paramètres ou en visualisation.".to_string()),
            appState: Some(current_app_view),
            settings: None,
            session_token: None,
            debug_info: None,
        })).into_response();
    }

    // --- PROPER PAIRING LOGIC ---
    debug!("Demande de couplage reçue pour le client: {}", request.client_id);
    
    {
        let app_state = state.app_handle.state::<Mutex<AppState>>();
        let app_state_lock = app_state.lock().unwrap();
        let mut pending = app_state_lock.pending_clients.lock().unwrap();
        pending.insert(request.client_id.clone(), request.pairing_code.clone());
    }

    // Notifier le Desktop qu'un client attend
    let _ = state.app_handle.emit("remote_pairing_request", serde_json::json!({
        "clientId": request.client_id,
        "pairingCode": request.pairing_code
    }));

    return (StatusCode::OK, Json(PairingResponse {
        status: "pending".to_string(),
        reason: None,
        appState: Some(current_app_view),
        settings: None,
        session_token: None,
        debug_info: None,
    })).into_response();
}

/// Handler pour POST /api/command
async fn command_handler(
    State(state): State<RemoteServerState>,
    Json(request): Json<RemoteCommandRequest>,
) -> impl IntoResponse {
    debug!("Commande reçue: {}", request.command);

    // Todo: Vérifier le token de session (à implémenter)

    // Émettre l'événement vers l'application
    let event_name = format!("remote_command::{}", request.command);
    match state.app_handle.emit(&event_name, request.payload) {
        Ok(_) => (StatusCode::OK, Json(CommandResponse {
            status: "success".to_string(),
            message: "Command executed".to_string(),
        })),
        Err(e) => {
            error!("Erreur lors de l'émission de la commande: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CommandResponse {
                status: "error".to_string(),
                message: format!("Failed to execute command: {}", e),
            }))
        }
    }
}

/// Handler pour GET /api/heartbeat
async fn heartbeat_handler(
    State(state): State<RemoteServerState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let client_id = params.get("clientId").cloned();
    let hb_state = state.app_handle.state::<crate::HeartbeatState>();
    let now = chrono::Utc::now().timestamp();
    
    let mut active_clients = hb_state.active_clients.lock().unwrap();

    if let Some(id) = client_id {
        // Compter les clients actifs (en excluant celui-ci s'il existe déjà)
        let active_count = active_clients.iter()
            .filter(|(&ref cid, &t)| cid != &id && (now - t) < 10)
            .count();

        if active_count < 2 || active_clients.contains_key(&id) {
            if !active_clients.contains_key(&id) {
                info!("Nouvelle télécommande connectée: {}", id);
                let _ = state.app_handle.emit("remote_control_status_changed", "connected");
            }
            active_clients.insert(id, now);
            return StatusCode::OK.into_response();
        } else {
            return (StatusCode::CONFLICT, "Remote connection limit (2) reached").into_response();
        }
    }
    
    StatusCode::BAD_REQUEST.into_response()
}

/// Handler pour GET /api/state (fallback si SSE ne fonctionne pas)
async fn state_handler(
    State(state): State<RemoteServerState>,
) -> impl IntoResponse {
    let app_state = state.app_handle.state::<Mutex<AppState>>();
    let app_state_lock = app_state.lock().unwrap();

    let response = serde_json::json!({
        "visualize_view": app_state_lock.visualize_view_state.lock().unwrap().clone(),
        "animation_state": app_state_lock.animation_state.lock().unwrap().clone(),
        "animation_speed": *app_state_lock.animation_speed.lock().unwrap(),
    });

    (StatusCode::OK, Json(response))
}

use tower_http::services::ServeDir;
use std::path::PathBuf;

// ... imports existants ...

/// Créer le routeur Axum avec toutes les routes
pub fn create_router(state: RemoteServerState, static_path: PathBuf) -> Router {
    Router::new()
        // Routes API
        .route("/api/health", get(healthcheck_handler))
        .route("/api/events", get(sse_handler))
        .route("/api/heartbeat", get(heartbeat_handler))
        .route("/api/pair", post(pair_handler))
        .route("/api/command", post(command_handler))
        .route("/api/state", get(state_handler))
        // Servir les fichiers statiques (fallback)
        .nest_service("/", ServeDir::new(static_path)) 
        // État partagé unique
        .with_state(state)
}

/// Démarrer le serveur Axum sur le port spécifié
pub async fn start_axum_server(
    app_handle: AppHandle,
    port: u16,
    settings: serde_json::Value,
    sse_sender: broadcast::Sender<SseMessage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sse_state = Arc::new(SseState { tx: sse_sender });
    
    let server_state = RemoteServerState {
        sse_state,
        app_handle: app_handle.clone(),
        settings,
    };

    // Déterminer le chemin des fichiers statiques
    // En dev : ../src/remote_client
    // En prod : resource_dir/remote_client
    let static_path = if cfg!(debug_assertions) {
        // Mode DEV: on suppose qu'on est dans src-tauri, donc on remonte
        let mut path = std::env::current_dir()?;
        // Si on est dans src-tauri, on remonte d'un cran pour accéder à src/remote_client
        // Attention: current_dir() dépend d'où on lance le binaire.
        // Avec cargo tauri dev, c'est souvent la racine du projet ou src-tauri.
        // On va essayer de trouver le dossier de manière robuste.
        if path.ends_with("src-tauri") {
            path.pop();
        }
        path.join("src").join("remote_client")
    } else {
        // Mode PROD
        app_handle.path().resource_dir()?.join("remote_client")
    };
    
    info!("Serving remote client files from: {:?}", static_path);

    let app = create_router(server_state, static_path);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    debug!("Démarrage du serveur Remote Control Axum sur {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
