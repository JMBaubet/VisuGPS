use serde::{Deserialize, Serialize};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use chrono::{DateTime, Utc};
use geo::{LineString as GeoLineString, Point};
use geo::prelude::*;
use uuid::Uuid;

// Struct to hold calculation results
struct TrackStats {
    total_distance_km: f64,
    positive_elevation_m: i32,
    summit_altitude_m: i32,
    summit_distance_km: f64,
}

// Structures for Geocoding API responses
#[derive(Deserialize, Debug)]
struct Commune {
    nom: String,
}

#[derive(Deserialize, Debug)]
struct MapboxReverseGeocodeResponse {
    features: Vec<MapboxFeature>,
}

#[derive(Deserialize, Debug)]
struct MapboxFeature {
    text: String,
    place_type: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ville {
    id: String,
    nom: String,
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
    pub circuit_id: String, // Rendre public
    pub nom: String,
    #[serde(rename = "villeDepartId")]
    pub ville_depart_id: String,
    #[serde(rename = "traceurId")]
    pub traceur_id: String, // Rendre public
    #[serde(rename = "editeurId")]
    pub editeur_id: String,
    pub url: String,
    #[serde(rename = "distanceKm")]
    pub distance_km: f64,
    #[serde(rename = "deniveleM")]
    pub denivele_m: i32,
    pub depart: CircuitDepart,
    pub sommet: CircuitSommet,
    #[serde(rename = "isoDateTime")]
    pub iso_date_time: DateTime<Utc>,
    #[serde(rename = "distanceVerifieeKm")]
    pub distance_verifiee_km: f64,
    pub evt: CircuitEvt,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitsFile {
    pub version: String,
    pub description: String,
    pub auteur: String,
    pub commentaires: String,
    pub villes: Vec<Ville>,
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
    link: Option<String>,
}

pub async fn process_gpx_file(app_handle: &AppHandle, filename: &str) -> Result<String, String> {
    let app_state: tauri::State<super::AppState> = app_handle.state();
    
    let settings_path = app_state.app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let mapbox_token = super::get_setting_value(&settings, "data.groupes.Système.groupes.Tokens.parametres.mapbox")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Token Mapbox non trouvé".to_string())?;

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

    let editor_name = identify_editor_from_creator(metadata.creator.as_deref().unwrap_or_default());
    add_editor_if_not_exists(app_handle, &editor_name)?;

    let mut url = get_url_from_metadata(&metadata, &editor_name);
    if editor_name == "Garmin Connect" {
        let re = Regex::new(r"COURSE_(\d+)").unwrap();
        if let Some(caps) = re.captures(filename) {
            if let Some(course_id) = caps.get(1) {
                url = format!("https://connect.garmin.com/modern/course/{}", course_id.as_str());
            }
        }
    }

    let mut circuits_file = read_circuits_file(app_handle)?;
    let new_circuit_id = format!("circ-{:04}", circuits_file.index_circuits + 1);

    let lon_depart = metadata.first_point_lon.unwrap_or_default();
    let lat_depart = metadata.first_point_lat.unwrap_or_default();

    let ville_id = get_ville_depart_id(&mut circuits_file, lon_depart, lat_depart, &mapbox_token).await.unwrap_or_default();

    let smoothing_distance = super::get_setting_value(&settings, "data.groupes.Importation.parametres.denivele_lissage_distance")
        .and_then(|v| v.as_i64())
        .unwrap_or(10) as f64;

    let stats = calculate_track_stats(&rounded_track_points, smoothing_distance);

    let new_circuit = Circuit {
        circuit_id: new_circuit_id.clone(),
        nom: metadata.name.unwrap_or_else(|| filename.to_string()),
        ville_depart_id: ville_id, // Utilisation de l'ID de la ville
        traceur_id: String::new(), // Initialiser avec une chaîne vide
        editeur_id: circuits_file.editeurs.iter().find(|e| e.nom == editor_name).map(|e| e.id.clone()).unwrap_or_default(),
        url,
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

    Ok(new_circuit_id) // Retourner le circuitId
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
    let xml_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let re = Regex::new(r#"\s*xmlns(:\w+)?=\"[^\"]*\""#).unwrap();
    let xml = re.replace_all(&xml_content, "").to_string();

    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();

    let mut metadata = GpxMetadata {
        name: None,
        creator: None,
        time: None,
        first_point_lat: None,
        first_point_lon: None,
        link: None,
    };
    let mut track_points: Vec<Vec<f64>> = Vec::new();

    let mut link_from_trk: Option<String> = None;
    let mut link_from_meta: Option<String> = None;
    let mut link_from_license: Option<String> = None;

    let mut current_lat: Option<f64> = None;
    let mut current_lon: Option<f64> = None;
    let mut path: Vec<Vec<u8>> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                path.push(e.name().as_ref().to_vec());
                let current_path_str = path.iter().map(|p| std::str::from_utf8(p).unwrap_or("")).collect::<Vec<&str>>().join("/");

                match current_path_str.as_str() {
                    "gpx" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"creator" {
                                    if let Ok(val) = attr.decode_and_unescape_value(reader.decoder()) {
                                        metadata.creator = Some(val.into_owned());
                                    }
                                }
                            }
                        }
                    },
                    "gpx/metadata/name" | "gpx/trk/name" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(name) = t.unescape() {
                                if metadata.name.is_none() {
                                    metadata.name = Some(name.to_string());
                                }
                            }
                        }
                    },
                    "gpx/metadata/copyright/license" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(license) = t.unescape() {
                                link_from_license = Some(license.to_string());
                            }
                        }
                    },
                    "gpx/metadata/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) = attr.decode_and_unescape_value(reader.decoder()) {
                                        link_from_meta = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    "gpx/trk/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) = attr.decode_and_unescape_value(reader.decoder()) {
                                        link_from_trk = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    "gpx/metadata/time" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(time_str) = t.unescape() {
                                if let Ok(dt) = time_str.parse::<DateTime<Utc>>() {
                                    metadata.time = Some(dt);
                                }
                            }
                        }
                    },
                    "gpx/trk/trkseg/trkpt" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                match attr.key.as_ref() {
                                    b"lat" => current_lat = attr.decode_and_unescape_value(reader.decoder()).ok().and_then(|v| v.parse().ok()),
                                    b"lon" => current_lon = attr.decode_and_unescape_value(reader.decoder()).ok().and_then(|v| v.parse().ok()),
                                    _ => {},
                                }
                            }
                        }
                        if metadata.first_point_lat.is_none() && current_lat.is_some() {
                            metadata.first_point_lat = current_lat;
                            metadata.first_point_lon = current_lon;
                        }
                    },
                    "gpx/trk/trkseg/trkpt/ele" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            if let (Some(lat), Some(lon)) = (current_lat, current_lon) {
                                if let Ok(ele_str) = t.unescape() {
                                    if let Ok(ele) = ele_str.parse::<f64>() {
                                        track_points.push(vec![lon, lat, ele]);
                                        current_lat = None;
                                        current_lon = None;
                                    }
                                }
                            }
                        }
                    },
                    _ => {},
                }
            },
            Ok(Event::Empty(e)) => {
                path.push(e.name().as_ref().to_vec());
                let current_path_str = path.iter().map(|p| std::str::from_utf8(p).unwrap_or("")).collect::<Vec<&str>>().join("/");
                match current_path_str.as_str() {
                    "gpx/metadata/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) = attr.decode_and_unescape_value(reader.decoder()) {
                                        link_from_meta = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    "gpx/trk/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) = attr.decode_and_unescape_value(reader.decoder()) {
                                        link_from_trk = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    },
                     "gpx/trk/trkseg/trkpt" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                match attr.key.as_ref() {
                                    b"lat" => current_lat = attr.decode_and_unescape_value(reader.decoder()).ok().and_then(|v| v.parse().ok()),
                                    b"lon" => current_lon = attr.decode_and_unescape_value(reader.decoder()).ok().and_then(|v| v.parse().ok()),
                                    _ => {},
                                }
                            }
                        }
                        if metadata.first_point_lat.is_none() && current_lat.is_some() {
                            metadata.first_point_lat = current_lat;
                            metadata.first_point_lon = current_lon;
                        }
                        // This is a self-closing trkpt, it won't have an ele child, so we might need to handle that if ele is an attribute.
                        // Assuming ele is always a separate tag for now.
                    },
                    _ => {},
                }
                path.pop(); // Pop immediately for empty tags
            },
            Ok(Event::End(e)) => {
                if !path.is_empty() && path.last().unwrap() == e.name().as_ref() {
                    path.pop();
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Erreur de parsing XML: {}", e)),
            _ => {},
        }
        buf.clear();
    }

    metadata.link = link_from_trk.or(link_from_meta).or(link_from_license);

    Ok((metadata, track_points))
}

fn get_url_from_metadata(metadata: &GpxMetadata, editor_name: &str) -> String {
    if let Some(link) = &metadata.link {
        return link.clone();
    }

    if let Some(name) = &metadata.name {
        match editor_name {
            "OpenRunner" => {
                if let Some(number) = name.split(|c: char| !c.is_numeric()).last() {
                    if !number.is_empty() {
                        return format!("https://www.openrunner.com/route-details/{}", number);
                    }
                }
            },
            _ => {}
        }
    }

    String::new()
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

async fn get_ville_depart_id(
    circuits_file: &mut CircuitsFile,
    lon: f64,
    lat: f64,
    mapbox_token: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut city_name: Option<String> = None;

    // 1. Try geo.api.gouv.fr API
    let url = format!("https://geo.api.gouv.fr/communes?lat={}&lon={}&fields=nom", lat, lon);
    if let Ok(response) = client.get(&url).send().await {
        if response.status().is_success() {
            if let Ok(data) = response.json::<Vec<Commune>>().await {
                if let Some(commune) = data.first() {
                    city_name = Some(commune.nom.clone());
                }
            }
        }
    }

    // 2. Fallback to Mapbox API
    if city_name.is_none() {
        let url = format!("https://api.mapbox.com/geocoding/v5/mapbox.places/{},{}.json?types=place&access_token={}", lon, lat, mapbox_token);
        if let Ok(response) = client.get(&url).send().await {
            if response.status().is_success() {
                if let Ok(data) = response.json::<MapboxReverseGeocodeResponse>().await {
                    if let Some(feature) = data.features.first() {
                        city_name = Some(feature.text.clone());
                    }
                }
            }
        }
    }

    // 3. Update circuits_file struct and get city ID
    if let Some(name) = city_name {
        if let Some(existing_ville) = circuits_file.villes.iter().find(|v| v.nom == name) {
            return Ok(existing_ville.id.clone());
        } else {
            let new_ville = Ville {
                id: Uuid::new_v4().to_string(),
                nom: name,
            };
            circuits_file.villes.push(new_ville.clone());
            return Ok(new_ville.id);
        }
    }

    Err("Impossible de déterminer la ville de départ".to_string())
}
