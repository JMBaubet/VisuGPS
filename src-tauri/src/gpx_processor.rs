use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use chrono::{DateTime, Utc};

// Structures for circuits.json
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Editor {
    id: String,
    nom: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitDepart {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitSommet {
    #[serde(rename = "altitudeM")]
    altitude_m: i32,
    km: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitCompteurs {
    zoom: i32,
    pause: i32,
    info: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitAffichage {
    depart: bool,
    arrivee: bool,
    #[serde(rename = "marqueurs10km")]
    marqueurs_10km: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitEvt {
    compteurs: CircuitCompteurs,
    affichage: CircuitAffichage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Circuit {
    #[serde(rename = "circuitId")]
    circuit_id: String,
    nom: String,
    #[serde(rename = "villeDepartId")]
    ville_depart_id: String,
    #[serde(rename = "traceurId")]
    traceur_id: String,
    #[serde(rename = "editeurId")]
    editeur_id: String,
    url: String,
    #[serde(rename = "distanceKm")]
    distance_km: f64,
    #[serde(rename = "deniveleM")]
    denivele_m: i32,
    depart: CircuitDepart,
    sommet: CircuitSommet,
    #[serde(rename = "isoDateTime")]
    iso_date_time: DateTime<Utc>,
    #[serde(rename = "distanceVerifieeKm")]
    distance_verifiee_km: f64,
    evt: CircuitEvt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CircuitsFile {
    version: String,
    description: String,
    // auteur: String,
    // commentaires: String,
    // villes: Vec<serde_json::Value>,
    // traceurs: Vec<serde_json::Value>,
    editeurs: Vec<Editor>,
    #[serde(rename = "indexCircuits")]
    index_circuits: i32,
    circuits: Vec<Circuit>,
}

// Data extracted from GPX for partial circuit creation
struct GpxMetadata {
    name: Option<String>,
    creator: Option<String>,
    time: Option<DateTime<Utc>>,
    first_point_lat: Option<f64>,
    first_point_lon: Option<f64>,
}

// Main public function to be called by the Tauri command
pub fn process_gpx_file(app_handle: &AppHandle, filename: &str) -> Result<String, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let gpx_dir_path = get_gpx_directory(app_handle)?;
    let file_path = gpx_dir_path.join(filename);

    if !file_path.exists() {
        return Err(format!("Le fichier GPX '{}' n\'a pas été trouvé.", filename));
    }

    // 1. Extract metadata and track points from GPX
    let (metadata, track_points) = extract_gpx_data(&file_path)?;

    // 2. Identify editor
    let editor_name = identify_editor_from_creator(&metadata.creator.unwrap_or_default());
    add_editor_if_not_exists(app_handle, &editor_name)?;

    // 3. Update circuits.json
    let mut circuits_file = read_circuits_file(app_handle)?;
    let new_circuit_id = format!("circ-{:04}", circuits_file.index_circuits + 1);

    let new_circuit = Circuit {
        circuit_id: new_circuit_id.clone(),
        nom: metadata.name.unwrap_or_else(|| filename.to_string()),
        ville_depart_id: String::new(), // To be filled later
        traceur_id: String::new(), // To be filled later
        editeur_id: circuits_file.editeurs.iter().find(|e| e.nom == editor_name).map(|e| e.id.clone()).unwrap_or_default(),
        url: String::new(), // To be filled later
        distance_km: 0.0, // To be calculated later
        denivele_m: 0, // To be calculated later
        depart: CircuitDepart {
            lon: metadata.first_point_lon.unwrap_or_default(),
            lat: metadata.first_point_lat.unwrap_or_default(),
        },
        sommet: CircuitSommet {
            altitude_m: 0,
            km: 0.0,
        },
        iso_date_time: metadata.time.unwrap_or_else(Utc::now),
        distance_verifiee_km: 0.0, // To be calculated later
        evt: CircuitEvt {
            compteurs: CircuitCompteurs { zoom: 0, pause: 0, info: 0 },
            affichage: CircuitAffichage { depart: true, arrivee: true, marqueurs_10km: true },
        },
    };

    circuits_file.circuits.push(new_circuit);
    circuits_file.index_circuits += 1;
    write_circuits_file(app_handle, &circuits_file)?;

    // 4. Create lineString.json
    create_line_string_file(app_handle, &new_circuit_id, &track_points)?;

    Ok(format!("Fichier importé. Editeur: {}. Circuit: {}.", editor_name, new_circuit_id))
}

// Helper to get the GPX directory from settings
fn get_gpx_directory(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let settings_path = app_state.app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let gpx_dir_setting = super::get_setting_value(&settings, "data.groupes.Importation.parametres.GPXFile")
        .ok_or_else(|| "Configuration du dossier GPX introuvable.".to_string())?;

    if gpx_dir_setting == "DEFAULT_DOWNLOADS" {
        dirs::download_dir().ok_or_else(|| "Impossible de trouver le dossier de téléchargement.".to_string())
    } else {
        Ok(PathBuf::from(gpx_dir_setting))
    }
}

// Helper to parse XML and extract metadata and track points
fn extract_gpx_data(file_path: &Path) -> Result<(GpxMetadata, Vec<Vec<f64>>), String> {
    let xml = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut metadata = GpxMetadata {
        name: None,
        creator: None,
        time: None,
        first_point_lat: None,
        first_point_lon: None,
    };
    let mut track_points: Vec<Vec<f64>> = Vec::new(); // [lon, lat, ele]

    let mut in_trkpt = false;
    let mut current_lat: Option<f64> = None;
    let mut current_lon: Option<f64> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"gpx" => {
                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| e.to_string())?;
                            if attr.key.as_ref() == b"creator" {
                                metadata.creator = Some(attr.decode_and_unescape_value(reader.decoder()).map_err(|e| e.to_string())?.into_owned());
                            }
                        }
                    },
                    b"name" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            metadata.name = Some(t.unescape().map_err(|e|e.to_string())?.to_string());
                        }
                    },
                    b"time" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                             let time_str = t.unescape().map_err(|e|e.to_string())?.to_string();
                            if let Ok(dt) = time_str.parse::<DateTime<Utc>>() {
                                metadata.time = Some(dt);
                            }
                        }
                    },
                    b"trkpt" => {
                        in_trkpt = true;
                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| e.to_string())?;
                            match attr.key.as_ref() {
                                b"lat" => current_lat = Some(attr.decode_and_unescape_value(reader.decoder()).map_err(|e| e.to_string())?.parse().map_err(|_| "Invalid lat".to_string())?),
                                b"lon" => current_lon = Some(attr.decode_and_unescape_value(reader.decoder()).map_err(|e| e.to_string())?.parse().map_err(|_| "Invalid lon".to_string())?),
                                _ => {},
                            }
                        }
                        if metadata.first_point_lat.is_none() && current_lat.is_some() {
                            metadata.first_point_lat = current_lat;
                            metadata.first_point_lon = current_lon;
                        }
                    },
                    b"ele" => {
                        if in_trkpt {
                            if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                                let ele: f64 = t.unescape().map_err(|e| e.to_string())?.parse().map_err(|_| "Invalid ele".to_string())?;
                                if let (Some(lat), Some(lon)) = (current_lat, current_lon) {
                                    track_points.push(vec![lon, lat, ele]);
                                    current_lat = None;
                                    current_lon = None;
                                }
                            }
                        }
                    },
                    _ => {},
                }
            },
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"trkpt" {
                    in_trkpt = false;
                }
            },
            Ok(Event::Eof) => break, // End of file
            Err(e) => return Err(format!("Erreur de parsing XML: {}", e)),
            _ => {},
        }
        buf.clear();
    }
    Ok((metadata, track_points))
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

// Helper to read circuits.json
fn read_circuits_file(app_handle: &AppHandle) -> Result<CircuitsFile, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let circuits_path = app_state.app_env_path.join("circuits.json");
    let content = fs::read_to_string(&circuits_path).map_err(|e| e.to_string())?;
    serde_json::from_str::<CircuitsFile>(&content).map_err(|e| e.to_string())
}

// Helper to write circuits.json
fn write_circuits_file(app_handle: &AppHandle, circuits_file: &CircuitsFile) -> Result<(), String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let circuits_path = app_state.app_env_path.join("circuits.json");
    let updated_content = serde_json::to_string_pretty(circuits_file).map_err(|e| e.to_string())?;
    fs::write(&circuits_path, updated_content).map_err(|e| e.to_string())
}

// Helper to update circuits.json
fn add_editor_if_not_exists(app_handle: &AppHandle, editor_name: &str) -> Result<(), String> {
    if editor_name == "Inconnu" {
        return Ok(())
    }

    let mut circuits_file = read_circuits_file(app_handle)?;

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
        write_circuits_file(app_handle, &circuits_file)?;
    }

    Ok(())
}

// Helper to create lineString.json
fn create_line_string_file(app_handle: &AppHandle, circuit_id: &str, track_points: &Vec<Vec<f64>>) -> Result<(), String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let data_dir = app_state.app_env_path.join("data");
    let circuit_data_dir = data_dir.join(circuit_id);

    fs::create_dir_all(&circuit_data_dir).map_err(|e| format!("Impossible de créer le dossier de données du circuit: {}", e))?;

    let linestring_path = circuit_data_dir.join("lineString.json");
    let linestring_content = serde_json::to_string_pretty(track_points).map_err(|e| e.to_string())?;
    fs::write(&linestring_path, linestring_content).map_err(|e| e.to_string())
}