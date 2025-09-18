use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use chrono::{DateTime, Utc};
use geo::{LineString as GeoLineString, Point};
use geo::prelude::*;

// Struct to hold calculation results
struct TrackStats {
    total_distance_km: f64,
    positive_elevation_m: i32,
    summit_altitude_m: i32,
    summit_distance_km: f64,
}

#[derive(Serialize, Debug)]
struct LineString {
    #[serde(rename = "type")]
    type_field: String,
    coordinates: Vec<Vec<f64>>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitsFile {
    pub version: String,
    pub description: String,
    pub auteur: String,
    pub commentaires: String,
    pub villes: Vec<serde_json::Value>,
    pub traceurs: Vec<super::Traceur>,
    pub editeurs: Vec<Editor>,
    #[serde(rename = "indexCircuits")]
    pub index_circuits: u32,
    pub circuits: Vec<Circuit>,
}

struct GpxMetadata {
    name: Option<String>,
    creator: Option<String>,
    time: Option<DateTime<Utc>>,
    first_point_lat: Option<f64>,
    first_point_lon: Option<f64>,
}

pub fn process_gpx_file(app_handle: &AppHandle, filename: &str, traceur_id: String) -> Result<String, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    
    let settings_path = app_state.app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let gpx_dir_path = get_gpx_directory(&settings)?;
    let file_path = gpx_dir_path.join(filename);

    if !file_path.exists() {
        return Err(format!("Le fichier GPX '{}' n\'a pas été trouvé.", filename));
    }

    let (metadata, track_points) = extract_gpx_data(&file_path)?;

    let rounded_track_points: Vec<Vec<f64>> = track_points.iter().map(|point| {
        vec![
            (point[0] * 100_000.0).round() / 100_000.0,
            (point[1] * 100_000.0).round() / 100_000.0, 
            (point[2] * 10.0).round() / 10.0,         
        ]
    }).collect();

    let editor_name = identify_editor_from_creator(&metadata.creator.unwrap_or_default());
    add_editor_if_not_exists(app_handle, &editor_name)?;

    let mut circuits_file = read_circuits_file(app_handle)?;
    let new_circuit_id = format!("circ-{:04}", circuits_file.index_circuits + 1);

    let lon_depart = metadata.first_point_lon.unwrap_or_default();
    let lat_depart = metadata.first_point_lat.unwrap_or_default();

    let smoothing_distance = super::get_setting_value(&settings, "data.groupes.Importation.parametres.denivele_lissage_distance")
        .and_then(|v| v.as_i64())
        .unwrap_or(10) as f64;

    let stats = calculate_track_stats(&rounded_track_points, smoothing_distance);

    let new_circuit = Circuit {
        circuit_id: new_circuit_id.clone(),
        nom: metadata.name.unwrap_or_else(|| filename.to_string()),
        ville_depart_id: String::new(),
        traceur_id: traceur_id, // Utiliser le traceur_id passé en paramètre
        editeur_id: circuits_file.editeurs.iter().find(|e| e.nom == editor_name).map(|e| e.id.clone()).unwrap_or_default(),
        url: String::new(),
        distance_km: stats.total_distance_km,
        denivele_m: stats.positive_elevation_m,
        depart: CircuitDepart {
            lon: (lon_depart * 100_000.0).round() / 100_000.0,
            lat: (lat_depart * 100_000.0).round() / 100_000.0,
        },
        sommet: CircuitSommet {
            altitude_m: stats.summit_altitude_m,
            km: stats.summit_distance_km,
        },
        iso_date_time: metadata.time.unwrap_or_else(Utc::now),
        distance_verifiee_km: 0.0,
        evt: CircuitEvt {
            compteurs: CircuitCompteurs { zoom: 0, pause: 0, info: 0 },
            affichage: CircuitAffichage { depart: true, arrivee: true, marqueurs_10km: true },
        },
    };

    circuits_file.circuits.push(new_circuit);
    circuits_file.index_circuits += 1;
    write_circuits_file(app_handle, &circuits_file)?;

    create_line_string_file(app_handle, &new_circuit_id, &rounded_track_points)?;

    Ok(format!("Fichier importé. Editeur: {}. Circuit: {}.", editor_name, new_circuit_id))
}

fn get_gpx_directory(settings: &serde_json::Value) -> Result<PathBuf, String> {
    let gpx_dir_setting = super::get_setting_value(settings, "data.groupes.Importation.parametres.GPXFile")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Configuration du dossier GPX introuvable.".to_string())?;

    if gpx_dir_setting == "DEFAULT_DOWNLOADS" {
        dirs::download_dir().ok_or_else(|| "Impossible de trouver le dossier de téléchargement.".to_string())
    } else {
        Ok(PathBuf::from(gpx_dir_setting))
    }
}

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
    let mut track_points: Vec<Vec<f64>> = Vec::new();

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
                                _ => {{}},
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
                    _ => {{}},
                }
            },
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"trkpt" {
                    in_trkpt = false;
                }
            },
            Ok(Event::Eof) => break, 
            Err(e) => return Err(format!("Erreur de parsing XML: {}", e)),
            _ => {{}},
        }
        buf.clear();
    }
    Ok((metadata, track_points))
}

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
        creator.to_string()
    }
}

fn read_circuits_file(app_handle: &AppHandle) -> Result<CircuitsFile, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let circuits_path = app_state.app_env_path.join("circuits.json");
    let content = fs::read_to_string(&circuits_path).map_err(|e| e.to_string())?;
    serde_json::from_str::<CircuitsFile>(&content).map_err(|e| e.to_string())
}

fn write_circuits_file(app_handle: &AppHandle, circuits_file: &CircuitsFile) -> Result<(), String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let circuits_path = app_state.app_env_path.join("circuits.json");
    let updated_content = serde_json::to_string_pretty(circuits_file).map_err(|e| e.to_string())?;
    fs::write(&circuits_path, updated_content).map_err(|e| e.to_string())
}

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

fn create_line_string_file(app_handle: &AppHandle, circuit_id: &str, track_points: &Vec<Vec<f64>>) -> Result<(), String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    let data_dir = app_state.app_env_path.join("data");
    let circuit_data_dir = data_dir.join(circuit_id);

    fs::create_dir_all(&circuit_data_dir).map_err(|e| format!("Impossible de créer le dossier de données du circuit: {}", e))?;

    let linestring_path = circuit_data_dir.join("lineString.json");

    let linestring_data = LineString {
        type_field: "LineString".to_string(),
        coordinates: track_points.clone(),
    };

    let linestring_content = serde_json::to_string_pretty(&linestring_data).map_err(|e| e.to_string())?;
    fs::write(&linestring_path, linestring_content).map_err(|e| e.to_string())
}

fn calculate_track_stats(track_points: &Vec<Vec<f64>>, smoothing_distance_m: f64) -> TrackStats {
    if track_points.len() < 2 {
        return TrackStats {
            total_distance_km: 0.0,
            positive_elevation_m: 0,
            summit_altitude_m: track_points.get(0).map_or(0, |p| p[2] as i32),
            summit_distance_km: 0.0,
        };
    }

    let geo_points: Vec<Point<f64>> = track_points
        .iter()
        .map(|p| Point::new(p[0], p[1]))
        .collect();
    let line = GeoLineString::from(geo_points);
    let total_distance_m = line.haversine_length();

    let mut positive_elevation_m = 0.0;
    let mut summit_altitude_m = 0.0;
    let mut distance_at_summit_m = 0.0;
    let mut cumulative_distance_m = 0.0;

    let mut last_elevation_point = track_points[0].as_slice();
    let mut distance_since_last_elevation_point = 0.0;

    if let Some(first_point) = track_points.get(0) {
        summit_altitude_m = first_point[2];
    }

    for i in 1..track_points.len() {
        let p1 = track_points[i-1].as_slice();
        let p2 = track_points[i].as_slice();

        let point1_geo = Point::new(p1[0], p1[1]);
        let point2_geo = Point::new(p2[0], p2[1]);
        let segment_distance_m = point1_geo.haversine_distance(&point2_geo);
        cumulative_distance_m += segment_distance_m;
        distance_since_last_elevation_point += segment_distance_m;

        if distance_since_last_elevation_point >= smoothing_distance_m {
            let ele_diff = p2[2] - last_elevation_point[2];
            if ele_diff > 0.0 {
                positive_elevation_m += ele_diff;
            }
            last_elevation_point = p2;
            distance_since_last_elevation_point = 0.0;
        }

        if p2[2] > summit_altitude_m {
            summit_altitude_m = p2[2];
            distance_at_summit_m = cumulative_distance_m;
        }
    }

    TrackStats {
        total_distance_km: (total_distance_m / 1000.0 * 10.0).round() / 10.0, // Round to 1 decimal
        positive_elevation_m: positive_elevation_m.round() as i32,
        summit_altitude_m: summit_altitude_m.round() as i32,
        summit_distance_km: (distance_at_summit_m / 1000.0 * 10.0).round() / 10.0, // Round to 1 decimal
    }
}
