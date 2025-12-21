use crate::AppState;
use crate::gpx_processor::Circuit;
use crate::get_setting_value;
use crate::read_circuits_file;
use crate::write_circuits_file;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{State, Manager};
use serde::{Serialize, Deserialize};
use zip::write::FileOptions;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImportDirEntry {
    pub name: String,
    #[serde(rename = "isDir")]
    pub is_dir: bool,
    pub path: String,
    pub size: u64,
    pub modified: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImportListResult {
    pub path: String,
    pub entries: Vec<ImportDirEntry>,
}

#[derive(Serialize, Deserialize)]
struct ExportMetadata {
    circuit: Circuit,
    circuit_name: String,
    circuit_id: String,
    ville_name: String,
    ville_id: String,
    traceur_name: String,
    traceur_id: String,
    editeur_name: Option<String>,
    editeur_id: Option<String>,
    version_export: String,
    messages: Vec<serde_json::Value>, // Events
    message_library: Vec<serde_json::Value>, // Message Definitions
}



#[tauri::command]
pub fn list_import_files(
    state: State<Mutex<AppState>>,
    path: Option<String>,
    extensions: Vec<String>,
) -> Result<ImportListResult, String> {
    let state = state.lock().unwrap();
    let settings_path = state.app_env_path.join("settings.json");
    let file_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;

    // Determine the base path
    let target_path = if let Some(p) = path {
        if p == "DEFAULT_IMPORT" {
            // Use the setting value
             let import_dir_setting = get_setting_value(&settings, "data.groupes.Importation.parametres.ImportDir")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| "ImportDir setting not found".to_string())?;

            if import_dir_setting == "DEFAULT_DOWNLOADS" {
                dirs::download_dir().ok_or_else(|| "Could not find download directory".to_string())?
            } else {
                PathBuf::from(import_dir_setting)
            }
        } else {
             PathBuf::from(p)
        }
    } else {
         // Default if path is None (should behave like DEFAULT_IMPORT)
         let import_dir_setting = get_setting_value(&settings, "data.groupes.Importation.parametres.ImportDir")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "ImportDir setting not found".to_string())?;

        if import_dir_setting == "DEFAULT_DOWNLOADS" {
            dirs::download_dir().ok_or_else(|| "Could not find download directory".to_string())?
        } else {
            PathBuf::from(import_dir_setting)
        }
    };

    if !target_path.exists() || !target_path.is_dir() {
        return Err(format!("Directory not found: {}", target_path.display()));
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&target_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let entry_path = entry.path();
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().into_owned();

         // Filter hidden files
        if name.starts_with('.') {
            continue;
        }

        let is_dir = metadata.is_dir();
        
        let should_include = if is_dir {
            true
        } else {
             if extensions.is_empty() {
                 true
             } else {
                 if let Some(ext) = entry_path.extension().and_then(|s| s.to_str()) {
                     extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
                 } else {
                     false
                 }
             }
        };

        if should_include {
            let modified = metadata.modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            entries.push(ImportDirEntry {
                name,
                is_dir,
                path: entry_path.to_string_lossy().into_owned(),
                size: metadata.len(),
                modified,
            });
        }
    }

    // Sort: Dirs first, then files, alphabetical
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Dirs (true) before files (false)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    Ok(ImportListResult {
        path: target_path.to_string_lossy().into_owned(),
        entries,
    })
}

#[tauri::command]
pub async fn export_circuit(
    _app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    circuit_id: String,
) -> Result<String, String> {
    let state_guard = state.lock().unwrap();
    let app_env_path = state_guard.app_env_path.clone();
    
    // ... (Settings loading matching original) ...
    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let export_dir_setting = get_setting_value(&settings, "data.groupes.Système.groupes.Export Import.parametres.Dossier")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let version_export = get_setting_value(&settings, "data.groupes.Système.groupes.Export Import.parametres.Version Export")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "1.0".to_string());

    let export_path = match export_dir_setting {
        Some(dir) if dir != "DEFAULT_DOWNLOADS" && !dir.is_empty() => PathBuf::from(dir),
        _ => dirs::download_dir().ok_or_else(|| "Could not find download directory".to_string())?,
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

    let editeur_name = circuits_file.editeurs.iter().find(|e| e.id == circuit.editeur_id)
        .map(|e| e.nom.clone())
        .unwrap_or_else(|| "Inconnu".to_string());

    // 3. Load Messages Events and Definitions
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let evt_path = circuit_data_dir.join("evt.json");
    let mut messages = Vec::new();
    let mut message_library = Vec::new();

    if evt_path.exists() {
        let evt_content = fs::read_to_string(&evt_path).map_err(|e| e.to_string())?;
        let evt_json: serde_json::Value = serde_json::from_str(&evt_content).map_err(|e| e.to_string())?;
        
        if let Some(range_events) = evt_json.get("rangeEvents").and_then(|v| v.as_array()) {
            messages = range_events.clone();

            // Find used message IDs
            let used_ids: Vec<String> = range_events.iter()
                .filter_map(|evt| evt.get("messageId").and_then(|v| v.as_str()).map(|s| s.to_string()))
                .collect();

            // Load User Library
            let user_lib_path = app_env_path.join("messages_user.json");
            if user_lib_path.exists() {
                let lib_content = fs::read_to_string(&user_lib_path).unwrap_or_else(|_| "[]".to_string());
                let user_lib: Vec<serde_json::Value> = serde_json::from_str(&lib_content).unwrap_or_default();
                
                // Collect definitions for used IDs
                for msg_def in user_lib {
                    if let Some(id) = msg_def.get("id").and_then(|v| v.as_str()) {
                        if used_ids.contains(&id.to_string()) {
                            message_library.push(msg_def);
                        }
                    }
                }
            }
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
        editeur_name: Some(editeur_name),
        editeur_id: Some(circuit.editeur_id.clone()),
        version_export,
        messages,
        message_library,
    };

    // 4. Create Zip
    let zip_filename = format!("{}.vgps", circuit.nom.replace(" ", "_")); // Santize filename better?
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
             // ID conflict, generate new ID
             let max_id = circuits_file
                .villes
                .iter()
                .filter_map(|v| v.id.strip_prefix("vi-").and_then(|s| s.parse::<i32>().ok()))
                .max()
                .unwrap_or(0);
             final_ville_id = format!("vi-{:04}", max_id + 1);
        }
        
        // Safety check if still exists (unlikely)
         if circuits_file.villes.iter().any(|v| v.id == final_ville_id) {
             // Fallback or panic? max+1 is safe.
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

    // 5. Handle Editeur Conflict/Creation
    let mut final_editeur_id = metadata.editeur_id.clone().unwrap_or_else(|| "inconnu".to_string());
    let editeur_name = metadata.editeur_name.clone().unwrap_or_else(|| "Inconnu".to_string());
    
    // If id is "inconnu" or name is "Inconnu", we might want to try to find "Inconnu" editor or create it?
    // Or just proceed with standard logic:
    
    if let Some(existing_editeur) = circuits_file.editeurs.iter().find(|e| e.nom.eq_ignore_ascii_case(&editeur_name)) {
        final_editeur_id = existing_editeur.id.clone();
    } else {
        if circuits_file.editeurs.iter().any(|e| e.id == final_editeur_id) || final_editeur_id == "inconnu" {
             // Generate new ID with ed-XXXX format logic
             if editeur_name == "Inconnu" {
                 final_editeur_id = "ed-0000".to_string();
             } else {
                 let max_id = circuits_file
                    .editeurs
                    .iter()
                    .filter_map(|e| e.id.strip_prefix("ed-").and_then(|s| s.parse::<i32>().ok()))
                    .max()
                    .unwrap_or(0);
                 final_editeur_id = format!("ed-{:04}", max_id + 1);
             }
        }
        
        // Check again if we just generated an ID that already exists (unlikely given max+1 but compliant with safety)
        if circuits_file.editeurs.iter().any(|e| e.id == final_editeur_id) && final_editeur_id != "ed-0000" {
             // Fallback if somehow logic failed, though max+1 should cover it. 
             // Just keeping max+1 logic is sufficient.
        }

        circuits_file.editeurs.push(crate::Editor {
            id: final_editeur_id.clone(),
            nom: editeur_name,
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

    // 5b. Restore Messages (Events)
    // We ensure messages are properly written to evt.json, using the metadata's messages as the source of truth
    // to guarantee they are restored even if evt.json was not perfectly copied or was empty.
    if !metadata.messages.is_empty() {
        let evt_path = circuit_dir.join("evt.json");
        let mut evt_data: serde_json::Value = if evt_path.exists() {
             let content = fs::read_to_string(&evt_path).unwrap_or_else(|_| "{}".to_string());
             serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
        } else {
             serde_json::json!({})
        };

        // Update/Set rangeEvents
        evt_data["rangeEvents"] = serde_json::Value::Array(metadata.messages.clone());
        
        // Write back
        let new_content = serde_json::to_string_pretty(&evt_data).map_err(|e| e.to_string())?;
        fs::write(&evt_path, new_content).map_err(|e| e.to_string())?;
    }

    // 5c. Import Message Library
    if !metadata.message_library.is_empty() {
        // 1. Load User Library
        let user_lib_path = app_env_path.join("messages_user.json");
        let mut user_lib: Vec<serde_json::Value> = if user_lib_path.exists() {
             let content = fs::read_to_string(&user_lib_path).unwrap_or_else(|_| "[]".to_string());
             serde_json::from_str(&content).unwrap_or_default()
        } else {
             Vec::new()
        };

        // 2. Load Default Library to avoid duplicates
        let default_lib: Vec<serde_json::Value> = {
            #[cfg(debug_assertions)]
            {
                // In DEV, read from filesystem
                // We need to resolve path relative to manifest
                 if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
                    let mut path = std::path::PathBuf::from(manifest_dir);
                    path.push("../public/messages_default.json");
                    if let Ok(content) = fs::read_to_string(path) {
                        serde_json::from_str(&content).unwrap_or_default()
                    } else {
                        Vec::new()
                    }
                 } else {
                     Vec::new()
                 }
            }
            #[cfg(not(debug_assertions))]
            {
                // In PROD, use embedded
                const EMBEDDED_DEFAULT_MESSAGES: &str = include_str!("../../public/messages_default.json");
                serde_json::from_str(EMBEDDED_DEFAULT_MESSAGES).unwrap_or_default()
            }
        };

        let mut modified = false;
        for msg_def in metadata.message_library {
             if let Some(import_id) = msg_def.get("id").and_then(|v| v.as_str()) {
                 // Check if ID exists in Default Library
                 let exists_in_default = default_lib.iter().any(|def| def.get("id").and_then(|v| v.as_str()) == Some(import_id));
                 
                 // Check if ID exists in User Library
                 let exists_in_user = user_lib.iter().any(|existing| existing.get("id").and_then(|v| v.as_str()) == Some(import_id));

                 // Add only if NOT in default AND NOT in user
                 if !exists_in_default && !exists_in_user {
                     user_lib.push(msg_def);
                     modified = true;
                 }
             }
        }

        if modified {
             let new_content = serde_json::to_string_pretty(&user_lib).map_err(|e| e.to_string())?;
             fs::write(&user_lib_path, new_content).map_err(|e| e.to_string())?;
        }
    }

    // 6. Update Circuits List
    let mut imported_circuit = metadata.circuit.clone();
    imported_circuit.ville_depart_id = final_ville_id;
    imported_circuit.traceur_id = final_traceur_id;
    imported_circuit.editeur_id = final_editeur_id;

    circuits_file.circuits.push(imported_circuit);
    write_circuits_file(&app_env_path, &circuits_file).map_err(|e| e.to_string())?;

    Ok(format!("Circuit '{}' importé avec succès !", metadata.circuit_name))
}

fn zip_directory(dir: &Path, zip: &mut zip::ZipWriter<fs::File>, options: FileOptions<()>, prefix: &Path) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = path.strip_prefix(prefix).unwrap().to_str().unwrap().replace("\\", "/");

        if path.is_file() {
            if name.ends_with(".DS_Store") { continue; }
            zip.start_file(name, options).map_err(|e| e.to_string())?;
            let mut f = fs::File::open(&path).map_err(|e| e.to_string())?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
            zip.write_all(&buffer).map_err(|e| e.to_string())?;
        } else if path.is_dir() {
            zip.add_directory(name, options).map_err(|e| e.to_string())?;
            zip_directory(&path, zip, options, prefix)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn export_context(
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    mode_name: String,
) -> Result<String, String> {
    let state_guard = state.lock().unwrap();
    let app_env_path = state_guard.app_env_path.clone(); // Currently active env path
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let visugps_dir = app_data_dir.join("VisuGPS");
    let context_path = visugps_dir.join(&mode_name);

    if !context_path.exists() {
        return Err(format!("Le contexte '{}' n'existe pas.", mode_name));
    }

    // Load settings from ACTIVE context to get export dir
    // (Assuming we want to use the active configuration to decide where to put the export)
    // Or should we load settings from the context being exported?
    // Usually "Settings" (Export path) are global or system-wide preferences, but stored in settings.json.
    // Let's use the active environment's settings for the export destination.
    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let export_dir_setting = get_setting_value(&settings, "data.groupes.Système.groupes.Export Import.parametres.Dossier")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let export_path = match export_dir_setting {
         Some(dir) if dir != "DEFAULT_DOWNLOADS" && !dir.is_empty() => PathBuf::from(dir),
         _ => dirs::download_dir().ok_or_else(|| "Could not find download directory".to_string())?,
    };

    if !export_path.exists() {
        return Err(format!("Export directory does not exist: {}", export_path.display()));
    }

    let zip_filename = format!("Context_{}.vctx", mode_name);
    let zip_path = export_path.join(&zip_filename);
    let file = fs::File::create(&zip_path).map_err(|e| e.to_string())?;
    let mut zip = zip::ZipWriter::new(file);

    let options: FileOptions<()> = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip_directory(&context_path, &mut zip, options, &context_path)?;

    zip.finish().map_err(|e| e.to_string())?;

    Ok(format!("Contexte exporté vers {}", zip_path.display()))
}

#[tauri::command]
pub async fn import_context(
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    mode_name: String,
    file_path: String,
) -> Result<String, String> {
    let state_guard = state.lock().unwrap();
    let current_mode = state_guard.app_env.clone();
    drop(state_guard);

    if mode_name == current_mode {
        return Err("Impossible d'importer dans le mode actif.".to_string());
    }

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let visugps_dir = app_data_dir.join("VisuGPS");
    let context_path = visugps_dir.join(&mode_name);

    if !context_path.exists() {
         return Err(format!("Le contexte '{}' n'existe pas.", mode_name));
    }

    let zip_path = Path::new(&file_path);
    if !zip_path.exists() {
        return Err("Le fichier d'import n'existe pas.".to_string());
    }

    // Safety: Wipe directory logic
    // We should be careful. removing dir requires it to end with mode_name.
    if !context_path.ends_with(&mode_name) {
         return Err("Erreur de chemin de sécurité.".to_string());
    }
    
    // Check zip validity first
    let file = fs::File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    // Wipe
    fs::remove_dir_all(&context_path).map_err(|e| e.to_string())?;
    fs::create_dir_all(&context_path).map_err(|e| e.to_string())?;

    // Unzip
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => context_path.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    Ok(format!("Contexte '{}' importé avec succès !", mode_name))
}
