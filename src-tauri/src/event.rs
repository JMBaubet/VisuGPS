use crate::{AppState, Message};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::colors;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FlytoEventContent {
    pub cap: u32,
    pub coord: Vec<f64>,
    pub duree: u32,
    pub pitch: u32,
    pub zoom: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum PointEvent {
    Pause,
    Flyto(FlytoEventContent),
}

// Struct for the event data as stored in evt.json
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RangeEventData {
    pub event_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    pub anchor_increment: u32,
    pub start_increment: u32,
    pub end_increment: u32,
    pub coord: Vec<f64>,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

// Main structure for the evt.json file (updated)
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventsFile {
    pub schema_version: String,
    pub point_events: BTreeMap<u32, Vec<PointEvent>>,
    pub range_events: Vec<RangeEventData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texts: Option<Vec<String>>, // Re-introduced to handle old schema gracefully
}

// --- Frontend-facing Hydrated Structures ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HydratedRangeEvent {
    pub event_id: String,
    pub message_id: String,
    pub anchor_increment: u32,
    pub start_increment: u32,
    pub end_increment: u32,
    pub coord: Vec<f64>,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HydratedEventsFile {
    pub schema_version: String,
    pub point_events: BTreeMap<u32, Vec<PointEvent>>,
    pub range_events: Vec<HydratedRangeEvent>,
    pub missing_message_errors: Vec<MissingMessageError>, // Nouvelle ligne
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MissingMessageError {
    pub event_id: String,
    pub message_id: String,
    pub anchor_increment: u32,
    pub circuit_id: String, // Ajouté pour le contexte dans le frontend
    pub description: String,
}

// --- Frontend Payload ---

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewMessagePayload {
    pub message_id: String,
    pub pre_affichage: u32,
    pub post_affichage: u32,
    pub coord: Vec<f64>,
    pub anchor_increment: u32,
    #[serde(default = "default_orientation")]
    pub orientation: String,
}

// --- Default value function ---

fn default_orientation() -> String {
    "Droite".to_string()
}

// --- File I/O & Logic ---

fn get_events_path(app_handle: &AppHandle, circuit_id: &str) -> Result<PathBuf, String> {
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let app_state = state_mutex.lock().unwrap();
    let data_dir = app_state.app_env_path.join("data").join(circuit_id);
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory for {}: {}", circuit_id, e))?;
    Ok(data_dir.join("evt.json"))
}

pub fn read_events(app_handle: &AppHandle, circuit_id: &str) -> Result<EventsFile, String> {
    let path = get_events_path(app_handle, circuit_id)?;
    if !path.exists()
        || fs::read_to_string(&path)
            .map_err(|e| e.to_string())?
            .trim()
            .is_empty()
    {
        let default_events = EventsFile {
            schema_version: "3.0".to_string(), // New version for new structure
            point_events: BTreeMap::new(),
            range_events: Vec::new(),
            texts: None,
        };
        write_events(app_handle, circuit_id, &default_events)?;
        return Ok(default_events);
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse evt.json: {}. Content: {}", e, content))
}

pub fn write_events(
    app_handle: &AppHandle,
    circuit_id: &str,
    events_file: &EventsFile,
) -> Result<(), String> {
    let path = get_events_path(app_handle, circuit_id)?;
    let content = serde_json::to_string_pretty(events_file).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

pub fn hydrate_events(
    app_handle: &AppHandle,
    events_file: EventsFile,
    circuit_id: &str,
) -> Result<HydratedEventsFile, String> {
    let state: State<Mutex<AppState>> = app_handle.state();
    let library = crate::get_message_library(app_handle.clone(), state)?;
    let message_map: BTreeMap<String, Message> =
        library.into_iter().map(|m| (m.id.clone(), m)).collect();

    let mut hydrated_range_events = Vec::new();
    let mut missing_message_errors = Vec::new(); // Nouvelle liste d'erreurs

    for event_data in events_file.range_events {
        let mut final_message: Option<Message> = None;
        let mut final_message_id: Option<String> = None;

        if let Some(embedded_message) = event_data.message {
            // Case 1: Message is already embedded (new format for KM markers)
            final_message = Some(embedded_message.clone());
            final_message_id = Some(embedded_message.id);
        } else if let Some(ref message_id) = event_data.message_id {
             final_message_id = Some(message_id.clone());
            // Case 2: message_id is present, check if it's a legacy distance marker or a library message
            if crate::distance_markers::is_distance_marker(message_id) {
                // Legacy distance marker handling (for backward compatibility)
                let distance_text = message_id.strip_prefix("km_").unwrap_or("0");
                let color = event_data.metadata.as_ref()
                    .and_then(|m| m.get("color")).and_then(|c| c.as_str()).unwrap_or("red");
                let bg_hex = colors::convert_vuetify_color_to_hex(color);
                let text_color = colors::get_contrasting_text_color(&bg_hex);

                final_message = Some(Message {
                    id: message_id.clone(),
                    text: format!("{} km", distance_text),
                    style: crate::MessageStyle {
                        background_color: format!("#{}", bg_hex),
                        text_color,
                    },
                    source: Some("distance_markers_legacy".to_string()),
                });

            } else if let Some(message_from_lib) = message_map.get(message_id) {
                // Standard library message
                let mut hydrated_message = message_from_lib.clone();
                if !hydrated_message.style.background_color.starts_with('#') {
                    hydrated_message.style.background_color = format!(
                        "#{}",
                        colors::convert_vuetify_color_to_hex(&hydrated_message.style.background_color)
                    );
                }
                final_message = Some(hydrated_message);
            }
        }

        if let Some(message) = final_message {
             let message_id_for_event = final_message_id.unwrap_or_else(|| event_data.event_id.clone());
             hydrated_range_events.push(HydratedRangeEvent {
                event_id: event_data.event_id,
                message_id: message_id_for_event,
                anchor_increment: event_data.anchor_increment,
                start_increment: event_data.start_increment,
                end_increment: event_data.end_increment,
                coord: event_data.coord,
                orientation: event_data.orientation,
                message,
            });
        } else {
             if let Some(message_id) = event_data.message_id {
                missing_message_errors.push(MissingMessageError {
                    event_id: event_data.event_id,
                    message_id: message_id.clone(),
                    anchor_increment: event_data.anchor_increment,
                    circuit_id: circuit_id.to_string(),
                    description: format!("Le message '{}' n'a pas été trouvé.", message_id),
                });
            }
        }
    }

    Ok(HydratedEventsFile {
        schema_version: events_file.schema_version,
        point_events: events_file.point_events,
        range_events: hydrated_range_events,
        missing_message_errors, // Retourner les erreurs
    })
}

// --- Tauri Commands ---

#[tauri::command]
pub fn get_events(app_handle: AppHandle, circuit_id: String) -> Result<HydratedEventsFile, String> {
    let events_file = read_events(&app_handle, &circuit_id)?;
    hydrate_events(&app_handle, events_file, &circuit_id) // Passer circuit_id
}

#[tauri::command]
pub fn add_message_event(
    app_handle: AppHandle,
    circuit_id: String,
    payload: NewMessagePayload,
) -> Result<HydratedEventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;

    let start_increment = payload
        .anchor_increment
        .saturating_sub(payload.pre_affichage);
    let end_increment = payload
        .anchor_increment
        .saturating_add(payload.post_affichage);

    let new_event = RangeEventData {
        event_id: Uuid::new_v4().to_string(),
        message_id: Some(payload.message_id),
        message: None,
        anchor_increment: payload.anchor_increment,
        start_increment,
        end_increment,
        coord: payload.coord,
        orientation: payload.orientation,
        metadata: None,
    };

    events_file.range_events.push(new_event);
    write_events(&app_handle, &circuit_id, &events_file)?;
    hydrate_events(&app_handle, events_file, &circuit_id) // Passer circuit_id
}

#[tauri::command]
pub fn delete_message_event(
    app_handle: AppHandle,
    circuit_id: String,
    event_id: String,
) -> Result<HydratedEventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    events_file.range_events.retain(|e| e.event_id != event_id);
    write_events(&app_handle, &circuit_id, &events_file)?;
    hydrate_events(&app_handle, events_file, &circuit_id) // Passer circuit_id
}

#[tauri::command]
pub fn add_pause_event(
    app_handle: AppHandle,
    circuit_id: String,
    increment: u32,
    override_existing: bool,
) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    let events_at_increment = events_file
        .point_events
        .entry(increment)
        .or_insert_with(Vec::new);

    if events_at_increment
        .iter()
        .any(|e| matches!(e, PointEvent::Flyto(_)))
    {
        if !override_existing {
            return Err(format!(
                "A Flyto event already exists at increment {}",
                increment
            ));
        }
        events_at_increment.retain(|e| !matches!(e, PointEvent::Flyto(_)));
    }

    if !events_at_increment
        .iter()
        .any(|e| matches!(e, PointEvent::Pause))
    {
        events_at_increment.push(PointEvent::Pause);
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}

#[tauri::command]
pub fn delete_pause_event(
    app_handle: AppHandle,
    circuit_id: String,
    increment: u32,
) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    if let Some(events) = events_file.point_events.get_mut(&increment) {
        events.retain(|e| !matches!(e, PointEvent::Pause));
        if events.is_empty() {
            events_file.point_events.remove(&increment);
        }
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}

#[tauri::command]
pub fn add_flyto_event(
    app_handle: AppHandle,
    circuit_id: String,
    increment: u32,
    mut flyto_content: FlytoEventContent,
    override_existing: bool,
) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    let events_at_increment = events_file
        .point_events
        .entry(increment)
        .or_insert_with(Vec::new);

    if events_at_increment
        .iter()
        .any(|e| matches!(e, PointEvent::Pause))
    {
        if !override_existing {
            return Err(format!(
                "A Pause event already exists at increment {}",
                increment
            ));
        }
        events_at_increment.retain(|e| !matches!(e, PointEvent::Pause));
    }

    flyto_content.coord = flyto_content
        .coord
        .iter()
        .map(|&v| (v * 100000.0).round() / 100000.0)
        .collect();

    events_at_increment.retain(|e| !matches!(e, PointEvent::Flyto(_)));
    events_at_increment.push(PointEvent::Flyto(flyto_content));
    write_events(&app_handle, &circuit_id, &events_file)?;
    Ok(events_file)
}

#[tauri::command]
pub fn delete_flyto_event(
    app_handle: AppHandle,
    circuit_id: String,
    increment: u32,
) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    if let Some(events) = events_file.point_events.get_mut(&increment) {
        events.retain(|e| !matches!(e, PointEvent::Flyto(_)));
        if events.is_empty() {
            events_file.point_events.remove(&increment);
        }
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}
