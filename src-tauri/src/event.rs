use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use crate::AppState;
use std::sync::Mutex;

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

// Payload received from the frontend to create a new message event
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NewMessagePayload {
    pub text: String,
    pub pre_affichage: u32,
    pub post_affichage: u32,
    pub coord: Vec<f64>,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<u32>,
    pub border_radius: Option<u32>,
}

// Enum for events that occur at a single point in time
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum PointEvent {
    Pause,
    Flyto(FlytoEventContent),
}

// Struct for events that span a range of time (i.e., Messages)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RangeEvent {
    pub id: String,
    pub anchor_increment: u32,
    pub start_increment: u32,
    pub end_increment: u32,
    pub text: String,
    pub coord: Vec<f64>,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub border_width: Option<u32>,
    pub border_radius: Option<u32>,
}

// Main structure for the evt.json file
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventsFile {
    pub schema_version: String,
    pub texts: Option<Vec<String>>,
    pub point_events: BTreeMap<u32, Vec<PointEvent>>,
    pub range_events: Vec<RangeEvent>,
}


// --- File I/O Functions ---

fn get_events_path(app_handle: &AppHandle, circuit_id: &str) -> Result<PathBuf, String> {
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let app_state = state_mutex.lock().unwrap();
    let app_env_path = app_state.app_env_path.clone();

    let data_dir = app_env_path.join("data").join(circuit_id);
    fs::create_dir_all(&data_dir).map_err(|e| format!("Failed to create data directory for circuit {}: {}", circuit_id, e))?;
    Ok(data_dir.join("evt.json"))
}

fn read_events(app_handle: &AppHandle, circuit_id: &str) -> Result<EventsFile, String> {
    let path = get_events_path(app_handle, circuit_id)?;
    if !path.exists() || fs::read_to_string(&path).map_err(|e| format!("Failed to read evt.json: {}", e))?.trim().is_empty() {
        let default_events = EventsFile {
            schema_version: "2.0".to_string(),
            texts: Some(Vec::new()),
            point_events: BTreeMap::new(),
            range_events: Vec::new(),
        };
        write_events(app_handle, circuit_id, &default_events)?;
        return Ok(default_events);
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read evt.json: {}", e))?;

    // TODO: Add migration logic if schema_version is not "2.0"
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse evt.json: {}", e))
}

fn write_events(app_handle: &AppHandle, circuit_id: &str, events_file: &EventsFile) -> Result<(), String> {
    let path = get_events_path(app_handle, circuit_id)?;
    let content = serde_json::to_string_pretty(events_file)
        .map_err(|e| format!("Failed to serialize events: {}", e))?;
    fs::write(&path, content)
        .map_err(|e| format!("Failed to write evt.json: {}", e))
}

// --- Tauri Commands ---

#[tauri::command]
pub fn get_events(app_handle: AppHandle, circuit_id: String) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;

    for event in &mut events_file.range_events {
        if let Some(color_name) = &event.background_color {
            if !color_name.starts_with('#') {
                event.background_color = Some(format!("#{}", colors::convert_vuetify_color_to_hex(color_name)));
            }
        }
        if let Some(color_name) = &event.border_color {
            if !color_name.starts_with('#') {
                event.border_color = Some(format!("#{}", colors::convert_vuetify_color_to_hex(color_name)));
            }
        }
    }

    Ok(events_file)
}

#[tauri::command]
pub fn add_pause_event(app_handle: AppHandle, circuit_id: String, increment: u32, override_existing: bool) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    
    let events_at_increment = events_file.point_events.entry(increment).or_insert_with(Vec::new);
    
    if events_at_increment.iter().any(|e| matches!(e, PointEvent::Flyto(_))) {
        if !override_existing {
            return Err(format!("Cannot add Pause event: A Flyto event already exists at increment {}", increment));
        } else {
            events_at_increment.retain(|e| !matches!(e, PointEvent::Flyto(_)));
        }
    }

    if !events_at_increment.iter().any(|e| matches!(e, PointEvent::Pause)) {
        events_at_increment.push(PointEvent::Pause);
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}

#[tauri::command]
pub fn delete_pause_event(app_handle: AppHandle, circuit_id: String, increment: u32) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    
    if let Some(events_at_increment) = events_file.point_events.get_mut(&increment) {
        events_at_increment.retain(|e| !matches!(e, PointEvent::Pause));
        
        if events_at_increment.is_empty() {
            events_file.point_events.remove(&increment);
        }
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}

#[tauri::command]
pub fn add_flyto_event(app_handle: AppHandle, circuit_id: String, increment: u32, mut flyto_content: FlytoEventContent, override_existing: bool) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;

    let events_at_increment = events_file.point_events.entry(increment).or_insert_with(Vec::new);

    if events_at_increment.iter().any(|e| matches!(e, PointEvent::Pause)) {
        if !override_existing {
            return Err(format!("Cannot add Flyto event: A Pause event already exists at increment {}", increment));
        } else {
            events_at_increment.retain(|e| !matches!(e, PointEvent::Pause));
        }
    }

    // Round coordinates to 5 decimal places
    let rounded_coords: Vec<f64> = flyto_content.coord.iter().map(|&val| (val * 100000.0).round() / 100000.0).collect();
    flyto_content.coord = rounded_coords;

    if !events_at_increment.iter().any(|e| matches!(e, PointEvent::Flyto(_))) {
        events_at_increment.push(PointEvent::Flyto(flyto_content));
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}

#[tauri::command]
pub fn delete_flyto_event(app_handle: AppHandle, circuit_id: String, increment: u32) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;

    if let Some(events_at_increment) = events_file.point_events.get_mut(&increment) {
        events_at_increment.retain(|e| !matches!(e, PointEvent::Flyto(_)));

        if events_at_increment.is_empty() {
            events_file.point_events.remove(&increment);
        }
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}

#[tauri::command]
pub fn add_message_event(
    app_handle: AppHandle,
    circuit_id: String,
    increment: u32,
    mut message_payload: NewMessagePayload,
) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;

    // Round coordinates to 5 decimal places
    let rounded_coords: Vec<f64> = message_payload.coord.iter().map(|&val| (val * 100000.0).round() / 100000.0).collect();
    message_payload.coord = rounded_coords;

    let start_increment = increment.saturating_sub(message_payload.pre_affichage);
    let end_increment = increment.saturating_add(message_payload.post_affichage);

    let new_event = RangeEvent {
        id: Uuid::new_v4().to_string(),
        anchor_increment: increment,
        start_increment,
        end_increment,
        text: message_payload.text.clone(),
        coord: message_payload.coord,
        background_color: message_payload.background_color,
        border_color: message_payload.border_color,
        border_width: message_payload.border_width,
        border_radius: message_payload.border_radius,
    };

    events_file.range_events.push(new_event);

    if let Some(ref mut texts) = events_file.texts {
        if !texts.contains(&message_payload.text) {
            texts.push(message_payload.text.clone());
            texts.sort();
        }
    } else {
        events_file.texts = Some(vec![message_payload.text.clone()]);
    }

    write_events(&app_handle, &circuit_id, &events_file)?;
    Ok(events_file)
}

#[tauri::command]
pub fn delete_message_event(
    app_handle: AppHandle,
    circuit_id: String,
    event_id: String,
) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;

    events_file.range_events.retain(|e| e.id != event_id);

    write_events(&app_handle, &circuit_id, &events_file)?;
    
    Ok(events_file)
}

#[tauri::command]
pub fn get_known_message_texts(app_handle: AppHandle, circuit_id: String) -> Result<Vec<String>, String> {
    let events_file = read_events(&app_handle, &circuit_id)?;
    Ok(events_file.texts.unwrap_or_default())
}