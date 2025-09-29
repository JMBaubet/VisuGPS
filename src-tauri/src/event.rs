use serde::{Deserialize, Serialize};
use std::collections::BTreeMap; // Changed from HashMap
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use crate::AppState;
use std::sync::Mutex;

// Contenu de l'événement Flyto
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FlytoEventContent {
    pub cap: u32,
    pub coord: Vec<f64>, // [longitude, latitude]
    pub duree: u32,
    pub pitch: u32,
    pub zoom: u32,
}

// Contenu de l'événement Message
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MessageEventContent {
    pub text: String,
    pub duration_increments: u32,
    pub coord: Vec<f64>, // [longitude, latitude]
    pub color: String,
}

// Énumération pour les différents types d'événements
// Utilise un tag externe "type" et un champ "data" pour le contenu
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    Pause, // Se sérialisera en {"type": "Pause"}
    Flyto(FlytoEventContent), // Se sérialisera en {"type": "Flyto", "data": {...}}
    Message(MessageEventContent), // Se sérialisera en {"type": "Message", "data": {...}}
}

// Structure principale pour le fichier evt.json
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EventsFile {
    // La clé est l'incrément (u32), la valeur est un vecteur d'événements
    // Cela gère le cas où plusieurs événements peuvent se produire au même incrément.
    pub events: BTreeMap<u32, Vec<Event>>, // Changed from HashMap to BTreeMap
}

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
    if !path.exists() {
        let default_events = EventsFile::default();
        write_events(app_handle, circuit_id, &default_events)?;
        return Ok(default_events);
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read evt.json: {}", e))?;
    
    if content.trim().is_empty() {
        let default_events = EventsFile::default();
        write_events(app_handle, circuit_id, &default_events)?;
        return Ok(default_events);
    }

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

#[tauri::command]
pub fn get_events(app_handle: AppHandle, circuit_id: String) -> Result<EventsFile, String> {
    read_events(&app_handle, &circuit_id)
}

#[tauri::command]
pub fn add_pause_event(app_handle: AppHandle, circuit_id: String, increment: u32) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    
    let events_at_increment = events_file.events.entry(increment).or_insert_with(Vec::new);
    
    // Validation: Check if a Flyto event already exists at this increment
    if events_at_increment.iter().any(|e| matches!(e, Event::Flyto(_))) {
        return Err(format!("Cannot add Pause event: A Flyto event already exists at increment {}", increment));
    }

    // Add Pause event if not already present
    if !events_at_increment.iter().any(|e| matches!(e, Event::Pause)) {
        events_at_increment.push(Event::Pause);
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}

#[tauri::command]
pub fn delete_pause_event(app_handle: AppHandle, circuit_id: String, increment: u32) -> Result<EventsFile, String> {
    let mut events_file = read_events(&app_handle, &circuit_id)?;
    
    if let Some(events_at_increment) = events_file.events.get_mut(&increment) {
        // Remove all Pause events at this increment
        events_at_increment.retain(|e| !matches!(e, Event::Pause));
        
        // If the event vector is empty after removal, remove the entry from the HashMap
        if events_at_increment.is_empty() {
            events_file.events.remove(&increment);
        }
        write_events(&app_handle, &circuit_id, &events_file)?;
    }
    Ok(events_file)
}
