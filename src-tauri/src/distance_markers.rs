use crate::event::{EventsFile, RangeEventData};
use crate::AppState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

// --- Data Structures ---

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DistanceMarkersConfig {
    pub afficher: bool,
    pub intervalle: u32,
    pub pre_affichage: u32,
    pub post_affichage: u32,
    pub couleur: String,
    pub orientation: String,
}

// --- Utility Functions ---

/// Checks if a message_id represents a distance marker
pub fn is_distance_marker(message_id: &str) -> bool {
    message_id.starts_with("km_")
}

/// Creates a message ID for a given distance
pub fn create_distance_message_id(distance_km: u32) -> String {
    format!("km_{}", distance_km)
}

/// Calculates the positions (increments) where distance markers should be placed
/// Returns a vector of (distance_km, increment, coordinates)
pub fn calculate_marker_positions(
    tracking_data: &serde_json::Value,
    intervalle_km: u32,
    total_distance_km: f64,
) -> Result<Vec<(u32, u32, Vec<f64>)>, String> {
    let tracking_array = tracking_data
        .as_array()
        .ok_or("Tracking data is not an array")?;

    if tracking_array.is_empty() {
        return Ok(Vec::new());
    }

    let mut markers = Vec::new();
    let mut next_marker_km = intervalle_km;

    // Iterate through tracking points to find the closest increment for each marker
    for (_index, point) in tracking_array.iter().enumerate() {
        // Calculate distance from increment (each increment = 100m = 0.1km)
        let increment = point["increment"]
            .as_u64()
            .ok_or("Missing increment field")? as u32;

        let distance_km = increment as f64 * 0.1;

        // Check if we've reached or passed the next marker distance
        while next_marker_km as f64 <= distance_km && (next_marker_km as f64) <= total_distance_km {
            // Get coordinates from the tracking point
            let coordonnee = point["coordonnee"]
                .as_array()
                .ok_or("Missing coordonnee field")?;

            let coords = vec![
                coordonnee[0].as_f64().ok_or("Invalid longitude")?,
                coordonnee[1].as_f64().ok_or("Invalid latitude")?,
            ];

            markers.push((next_marker_km, increment, coords));
            next_marker_km += intervalle_km;
        }

        if next_marker_km as f64 > total_distance_km {
            break;
        }
    }

    Ok(markers)
}

/// Removes all distance markers from the events file
pub fn remove_distance_markers_from_events(events_file: &mut EventsFile) {
    events_file
        .range_events
        .retain(|event| !is_distance_marker(&event.message_id));
}

/// Generates distance marker events based on configuration
pub fn generate_distance_marker_events(
    app_handle: &AppHandle,
    circuit_id: &str,
    config: &DistanceMarkersConfig,
    total_distance_km: f64,
) -> Result<Vec<RangeEventData>, String> {
    if !config.afficher {
        return Ok(Vec::new());
    }

    // Read tracking data
    let state_mutex = app_handle.state::<Mutex<AppState>>();
    let app_state = state_mutex.lock().unwrap();
    let tracking_path = app_state
        .app_env_path
        .join("data")
        .join(circuit_id)
        .join("tracking.json");

    let tracking_content = fs::read_to_string(&tracking_path)
        .map_err(|e| format!("Failed to read tracking.json: {}", e))?;
    let tracking_data: serde_json::Value = serde_json::from_str(&tracking_content)
        .map_err(|e| format!("Failed to parse tracking.json: {}", e))?;

    // Calculate marker positions
    let marker_positions =
        calculate_marker_positions(&tracking_data, config.intervalle, total_distance_km)?;

    // Generate range events for each marker
    let mut events = Vec::new();
    for (distance_km, anchor_increment, coords) in marker_positions {
        let start_increment = anchor_increment.saturating_sub(config.pre_affichage);
        let end_increment = anchor_increment.saturating_add(config.post_affichage);

        // Create metadata with color
        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "color".to_string(),
            serde_json::Value::String(config.couleur.clone()),
        );

        let event = RangeEventData {
            event_id: Uuid::new_v4().to_string(),
            message_id: create_distance_message_id(distance_km),
            anchor_increment,
            start_increment,
            end_increment,
            coord: coords,
            orientation: config.orientation.clone(),
            metadata: Some(serde_json::Value::Object(metadata)),
        };

        events.push(event);
    }

    Ok(events)
}

// --- Tauri Commands ---

#[tauri::command]
pub fn generate_distance_markers(
    app_handle: AppHandle,
    circuit_id: String,
    config: DistanceMarkersConfig,
    total_distance_km: f64,
) -> Result<crate::event::HydratedEventsFile, String> {
    // Read current events
    let mut events_file = crate::event::read_events(&app_handle, &circuit_id)?;

    // Remove existing distance markers
    remove_distance_markers_from_events(&mut events_file);

    // Generate new distance markers
    let new_markers =
        generate_distance_marker_events(&app_handle, &circuit_id, &config, total_distance_km)?;

    // Add new markers to events
    events_file.range_events.extend(new_markers);

    // Write events file
    crate::event::write_events(&app_handle, &circuit_id, &events_file)?;

    // Return hydrated events
    crate::event::hydrate_events(&app_handle, events_file, &circuit_id)
}

#[tauri::command]
pub fn remove_distance_markers(
    app_handle: AppHandle,
    circuit_id: String,
) -> Result<crate::event::HydratedEventsFile, String> {
    // Read current events
    let mut events_file = crate::event::read_events(&app_handle, &circuit_id)?;

    // Remove all distance markers
    remove_distance_markers_from_events(&mut events_file);

    // Write events file
    crate::event::write_events(&app_handle, &circuit_id, &events_file)?;

    // Return hydrated events
    crate::event::hydrate_events(&app_handle, events_file, &circuit_id)
}
