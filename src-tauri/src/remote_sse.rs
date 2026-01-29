use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    response::IntoResponse,
    http::StatusCode,
};
use tokio_stream::{Stream, StreamExt};
use log::{debug, error};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Structure pour gérer les événements SSE
#[derive(Clone, Debug, serde::Serialize)]
pub struct SseMessage {
    pub event_type: String,
    pub data: serde_json::Value,
}

/// État partagé pour le serveur SSE
#[derive(Clone)]
pub struct SseState {
    pub tx: broadcast::Sender<SseMessage>,
}

impl SseState {
    /// Créer un nouveau SseState avec un canal de broadcast existant
    pub fn new(tx: broadcast::Sender<SseMessage>) -> Self {
        Self { tx }
    }

    /// Envoyer un message SSE à tous les clients connectés
    pub fn send(&self, message: SseMessage) {
        let _ = self.tx.send(message);
    }

    /// Envoyer une mise à jour d'état de l'application
    pub fn send_app_state_update(&self, app_state: &str) {
        self.send(SseMessage {
            event_type: "app_state_update".to_string(),
            data: serde_json::json!({
                "appState": app_state
            }),
        });
    }

    /// Envoyer une mise à jour d'état de visualisation
    pub fn send_visualize_view_state_update(&self, state: &crate::remote_control::VisualizeViewState) {
        self.send(SseMessage {
            event_type: "visualize_view_state_update".to_string(),
            data: serde_json::to_value(state).unwrap_or_default(),
        });
    }

    /// Envoyer une mise à jour de vitesse d'animation
    pub fn send_animation_speed_update(&self, speed: f32) {
        self.send(SseMessage {
            event_type: "animation_speed_update".to_string(),
            data: serde_json::json!({
                "speed": speed
            }),
        });
    }

    /// Envoyer une mise à jour d'état de pause
    pub fn send_pause_state_update(&self, paused: bool) {
        self.send(SseMessage {
            event_type: "pause_state_update".to_string(),
            data: serde_json::json!({
                "paused": paused
            }),
        });
    }

    /// Envoyer une mise à jour d'état d'animation
    pub fn send_animation_state_update(&self, animation_state: &str) {
        self.send(SseMessage {
            event_type: "animation_state_update".to_string(),
            data: serde_json::json!({
                "animationState": animation_state
            }),
        });
    }

    /// Envoyer une mise à jour d'avancement de l'animation
    pub fn send_animation_progress_update(&self, current_distance: f64, current_segment_index: Option<usize>) {
        self.send(SseMessage {
            event_type: "animation_progress_update".to_string(),
            data: serde_json::json!({
                "currentDistance": current_distance,
                "currentSegmentIndex": current_segment_index
            }),
        });
    }
}

/// Handler pour la route SSE /api/events
pub async fn sse_handler(
    State(state): State<Arc<SseState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    debug!("Nouvelle connexion SSE établie");
    
    let rx = state.tx.subscribe();
    
    // Convertir le broadcast receiver en stream manuellement
    let stream = async_stream::stream! {
        let mut rx = rx;
        loop {
            match rx.recv().await {
                Ok(msg) => yield Ok::<SseMessage, Infallible>(msg),
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    error!("Client SSE a manqué {} messages (buffer saturé)", n);
                    continue;
                }
                Err(_) => break,
            }
        }
    };

    let event_stream = stream.filter_map(|msg| {
        match msg {
            Ok(sse_msg) => {
                debug!("Envoi événement SSE: {}", sse_msg.event_type);
                
                // Créer un événement SSE avec type et données
                let event = Event::default()
                    .event(&sse_msg.event_type)
                    .json_data(sse_msg.data)
                    .ok();
                
                event.map(Ok)
            }
            Err(_) => None,
        }
    });

    Sse::new(event_stream).keep_alive(KeepAlive::default())
}

/// Handler pour un simple ping (healthcheck)
pub async fn healthcheck_handler() -> impl IntoResponse {
    (StatusCode::OK, "Remote Control Server OK")
}
