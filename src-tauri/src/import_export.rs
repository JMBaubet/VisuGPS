use crate::AppState;
use crate::get_setting_value;
use crate::read_circuits_file;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use serde::{Serialize, Deserialize};
use zip::write::FileOptions;

#[derive(Serialize, Deserialize)]
struct ExportMetadata {
    circuit_name: String,
    circuit_id: String,
    ville_name: String,
    ville_id: String,
    traceur_name: String,
    traceur_id: String,
    version_export: String,
    messages: Vec<serde_json::Value>, // Using Value for flexibility
}

#[tauri::command]
pub async fn export_circuit(
    _app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    circuit_id: String,
) -> Result<String, String> {
    let state_guard = state.lock().unwrap();
    let app_env_path = state_guard.app_env_path.clone(); // Clone path to release lock if needed, though we hold it
    
    // 1. Get Settings for Export Path and Version
    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let export_dir_setting = get_setting_value(&settings, "data.groupes.Système.groupes.Export Import.parametres.Dossier")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Export Directory setting not found".to_string())?;

    let version_export = get_setting_value(&settings, "data.groupes.Système.groupes.Export Import.parametres.Version Export")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "1.0".to_string());

    let export_path = if export_dir_setting == "DEFAULT_DOWNLOADS" {
        dirs::download_dir().ok_or_else(|| "Could not find download directory".to_string())?
    } else {
        PathBuf::from(export_dir_setting)
    };

    if !export_path.exists() {
        return Err(format!("Export directory does not exist: {}", export_path.display()));
    }

    // 2. Load Circuit Info
    let circuits_file = read_circuits_file(&app_env_path)?;
    let circuit = circuits_file.circuits.iter().find(|c| c.circuit_id == circuit_id)
        .ok_or_else(|| format!("Circuit {} not found", circuit_id))?;

    let ville_name = circuits_file.villes.iter().find(|v| v.id == circuit.ville_depart_id)
        .map(|v| v.nom.clone())
        .unwrap_or_else(|| "Inconnue".to_string());

    let traceur_name = circuits_file.traceurs.iter().find(|t| t.id == circuit.traceur_id)
        .map(|t| t.nom.clone())
        .unwrap_or_else(|| "Inconnu".to_string());

    // 3. Load Messages
    // Load evt.json. Assuming it is in data/[circuit_id]/evt.json based on common patterns, 
    // OR it handles events globally?
    // Checking lib.rs, event module commands don't seem to take circuit_id for *loading* initially?
    // Wait, event::get_events takes circuitId in frontend usually if it's per circuit.
    // Let's assume data/[circuit_id]/evt.json logic if that's where events are stored.
    // Actually, based on AddMessageEvent in EditView.vue -> invoke('add_message_event', { circuitId ... })
    // It implies events are stored relative to the circuit.
    
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let evt_path = circuit_data_dir.join("evt.json");
    let mut messages = Vec::new();
    if evt_path.exists() {
        let evt_content = fs::read_to_string(&evt_path).map_err(|e| e.to_string())?;
        let evt_json: serde_json::Value = serde_json::from_str(&evt_content).map_err(|e| e.to_string())?;
        if let Some(range_events) = evt_json.get("rangeEvents").and_then(|v| v.as_array()) {
            messages = range_events.clone();
        }
    }

    // Prepare Metadata
    let metadata = ExportMetadata {
        circuit_name: circuit.nom.clone(),
        circuit_id: circuit.circuit_id.clone(),
        ville_name,
        ville_id: circuit.ville_depart_id.clone(),
        traceur_name,
        traceur_id: circuit.traceur_id.clone(),
        version_export,
        messages,
    };

    // 4. Create Zip
    let zip_filename = format!("_{}_.vgps", circuit.nom.replace(" ", "_")); // Santize filename better?
    let zip_path = export_path.join(&zip_filename);
    let file = fs::File::create(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);

    let options: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Add metadata.json
    zip.start_file("metadata.json", options).map_err(|e| e.to_string())?;
    let metadata_json = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    zip.write_all(metadata_json.as_bytes()).map_err(|e| e.to_string())?;

    // Add circuit folder contents
    // We want to preserve the structure inside the zip? 
    // Suggested structure: just the files? or a folder?
    // "Y inclut le dossier du circuit (/data/circuits/<id>)" -> implies folder structure or just contents.
    // If we just dump contents, we might collide with metadata.json if circuit has one.
    // Let's put circuit files in a "circuit" subfolder in the zip.
    
    // let circuit_files = ["lineString.json", "tracking.json", "evt.json", "errors.json"]; 
    
    // Also add images/thumbnails if they exist?
    // Usually in data/[circuit_id]/...
    
    if circuit_data_dir.exists() {
        for entry in fs::read_dir(&circuit_data_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy();
                // Avoid system files if any
                if file_name == ".DS_Store" { continue; }
                
                zip.start_file(format!("circuit/{}", file_name), options).map_err(|e| e.to_string())?;
                let mut f = fs::File::open(&path).map_err(|e| e.to_string())?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
                zip.write_all(&buffer).map_err(|e| e.to_string())?;
            }
        }
    }

    zip.finish().map_err(|e| e.to_string())?;

    Ok(format!("Circuit exporté avec succès vers {}", zip_path.display()))
}
