use crate::AppState;
use crate::gpx_processor::Circuit;
use crate::get_setting_value;
use crate::read_circuits_file;
use crate::write_circuits_file;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::State;
use serde::{Serialize, Deserialize};
use zip::write::FileOptions;

#[derive(Serialize, Deserialize)]
struct ExportMetadata {
    circuit: Circuit,
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
        circuit: circuit.clone(),
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

#[derive(Serialize, Deserialize)]
struct ImportResult {
    success: bool,
    message: String,
    circuit_name: Option<String>,
}

#[tauri::command]
pub async fn import_circuit(
    _app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    file_path: String,
) -> Result<String, String> { // Returning String message for simplicity, or complex object?
    // Using simple string here as requested by plan, UI handles snackbar
    let state_guard = state.lock().unwrap();
    let app_env_path = state_guard.app_env_path.clone(); // Clone path
    drop(state_guard); // Release lock as we might need it for read_circuits_file maybe? (It uses file read, so ok)
    // Actually read_circuits_file helper is standalone. But write_circuits_file is too.

    let zip_path = Path::new(&file_path);
    if !zip_path.exists() {
        return Err(format!("Le fichier n'existe pas : {}", file_path));
    }

    let file = fs::File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    // 1. Read Metadata
    let metadata: ExportMetadata = {
        let mut metadata_file = archive.by_name("metadata.json").map_err(|_| "Archive invalide : metadata.json manquant".to_string())?;
        let mut metadata_content = String::new();
        metadata_file.read_to_string(&mut metadata_content).map_err(|e| e.to_string())?;
        serde_json::from_str(&metadata_content).map_err(|e| format!("Erreur lecture métadonnées : {}", e))?
    };

    // 2. Check for Duplicates (Circuit ID)
    let mut circuits_file = read_circuits_file(&app_env_path)?;
    if circuits_file.circuits.iter().any(|c| c.circuit_id == metadata.circuit_id) {
        return Err(format!("Le circuit '{}' (ID: {}) existe déjà.", metadata.circuit_name, metadata.circuit_id));
    }

    // 3. Handle Ville (City) Conflict/Creation
    let mut final_ville_id = metadata.ville_id.clone();
    
    if let Some(existing_ville) = circuits_file.villes.iter().find(|v| v.nom.eq_ignore_ascii_case(&metadata.ville_name)) {
        final_ville_id = existing_ville.id.clone();
    } else {
        if circuits_file.villes.iter().any(|v| v.id == final_ville_id) {
             final_ville_id = uuid::Uuid::new_v4().to_string();
        }
        
        circuits_file.villes.push(crate::Ville {
            id: final_ville_id.clone(),
            nom: metadata.ville_name.clone(),
        });
    }

    // 4. Handle Traceur Conflict/Creation
    let mut final_traceur_id = metadata.traceur_id.clone();
    
    if let Some(existing_traceur) = circuits_file.traceurs.iter().find(|t| t.nom.eq_ignore_ascii_case(&metadata.traceur_name)) {
        final_traceur_id = existing_traceur.id.clone();
    } else {
        if circuits_file.traceurs.iter().any(|t| t.id == final_traceur_id) {
             final_traceur_id = uuid::Uuid::new_v4().to_string();
        }
        circuits_file.traceurs.push(crate::Traceur {
            id: final_traceur_id.clone(),
            nom: metadata.traceur_name.clone(),
        });
    }
    


    // 5. Extract Files
    let circuit_dir = app_env_path.join("data").join(&metadata.circuit_id);
    fs::create_dir_all(&circuit_dir).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let name = file.name().to_string();
        
        if name.starts_with("circuit/") {
            // It's a circuit file
            let file_name = Path::new(&name).file_name().unwrap(); // extract filename from circuit/xyz.json
            let dest_path = circuit_dir.join(file_name);
            let mut dest_file = fs::File::create(&dest_path).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut dest_file).map_err(|e| e.to_string())?;
        }
    }

    // 6. Update Circuits List
    let mut imported_circuit = metadata.circuit.clone();
    imported_circuit.ville_depart_id = final_ville_id;
    imported_circuit.traceur_id = final_traceur_id;

    circuits_file.circuits.push(imported_circuit);
    write_circuits_file(&app_env_path, &circuits_file).map_err(|e| e.to_string())?;

    Ok(format!("Circuit '{}' importé avec succès !", metadata.circuit_name))
}
