use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

// Structures for circuits.json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Editor {
    id: String,
    nom: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CircuitsFile {
    version: String,
    description: String,
    auteur: String,
    commentaires: String,
    villes: Vec<serde_json::Value>,
    traceurs: Vec<serde_json::Value>,
    editeurs: Vec<Editor>,
    #[serde(rename = "indexCircuits")]
    index_circuits: i32,
    circuits: Vec<serde_json::Value>,
}

// Main public function to be called by the Tauri command
pub fn process_gpx_file(app_handle: &AppHandle, filename: &str) -> Result<String, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let gpx_dir_path = get_gpx_directory(app_handle)?;
    let file_path = gpx_dir_path.join(filename);

    if !file_path.exists() {
        return Err(format!("Le fichier GPX '{}' n\'a pas été trouvé.", filename));
    }

    let creator = get_gpx_creator(&file_path)?;
    let editor_name = identify_editor_from_creator(&creator);

    add_editor_if_not_exists(app_handle, &editor_name)?;

    Ok(format!("Fichier importé. Editeur reconnu : {}", editor_name))
}

// Helper to get the GPX directory from settings
fn get_gpx_directory(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let settings_path = app_state.app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let gpx_dir_setting = super::get_setting_value(&settings, "data.groupes.Importation.parametres.GPXFile")
        .ok_or_else(|| "Configuration du dossier GPX introuvable.".to_string())?;

    if gpx_dir_setting == "DEFAULT_DOWNLOADS" {
        dirs::download_dir().ok_or_else(|| "Impossible de trouver le dossier de téléchargement.".to_string())
    } else {
        Ok(std::path::PathBuf::from(gpx_dir_setting))
    }
}

// Helper to parse XML and get the creator
fn get_gpx_creator(file_path: &Path) -> Result<String, String> {
    let xml = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                if e.name().as_ref() == b"gpx" {
                    for attr in e.attributes() {
                        let attr = attr.map_err(|e| e.to_string())?;
                        if attr.key.as_ref() == b"creator" {
                                                        let value = attr.decode_and_unescape_value(reader.decoder()).map_err(|e| e.to_string())?;
                            return Ok(value.to_string());
                        }
                    }
                    return Ok("Inconnu".to_string()); // No creator attribute found
                }
            }
            Ok(Event::Eof) => break, // End of file
            Err(e) => return Err(format!("Erreur de parsing XML: {}", e)),
            _ => {},
        }
        buf.clear();
    }
    Ok("Inconnu".to_string()) // No <gpx> tag found
}

// Helper to identify editor from creator string
fn identify_editor_from_creator(creator: &str) -> String {
    let lower_creator = creator.to_lowercase();
    if lower_creator.contains("strava") {
        "Strava".to_string()
    } else if lower_creator.contains("garmin") {
        "Garmin Connect".to_string()
    } else if lower_creator.contains("openrunner") {
        "OpenRunner".to_string()
    } else if lower_creator.contains("ridewithgps") {
        "RideWithGPS".to_string()
    } else if creator.is_empty() || creator == "Inconnu" {
        "Inconnu".to_string()
    } else {
        creator.to_string() // Return the creator string itself if not recognized
    }
}

// Helper to update circuits.json
fn add_editor_if_not_exists(app_handle: &AppHandle, editor_name: &str) -> Result<(), String> {
    if editor_name == "Inconnu" {
        return Ok(()); // Do not add "Inconnu" to the list
    }

    let app_state: tauri::State<super::AppState> = app_handle.state();
    let circuits_path = app_state.app_env_path.join("circuits.json");

    let mut circuits_file: CircuitsFile = {
        let content = fs::read_to_string(&circuits_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())?
    };

    let editor_exists = circuits_file.editeurs.iter().any(|e| e.nom.eq_ignore_ascii_case(editor_name));

    if !editor_exists {
        let max_id = circuits_file.editeurs.iter()
            .filter_map(|e| e.id.strip_prefix("ed-").and_then(|s| s.parse::<i32>().ok()))
            .max()
            .unwrap_or(0);
        
        let new_id = format!("ed-{:04}", max_id + 1);
        
        let new_editor = Editor {
            id: new_id,
            nom: editor_name.to_string(),
        };

        circuits_file.editeurs.push(new_editor);

        let updated_content = serde_json::to_string_pretty(&circuits_file).map_err(|e| e.to_string())?;
        fs::write(&circuits_path, updated_content).map_err(|e| e.to_string())?;
    }

    Ok(())
}
