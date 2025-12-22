use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use geo::{prelude::*, LineString as GeoLineString, Point};
use image::ImageFormat;
use local_ip_address::local_ip;
use qrcode::QrCode;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};


use super::{get_setting_value, AppState, CircuitsFile, Editor, Ville};
use crate::distance_markers;
use crate::event;
use crate::tracking_processor;

#[tauri::command]
pub fn generate_qrcode_base64(url: String) -> Result<String, String> {
    let code = QrCode::new(url.as_bytes())
        .map_err(|e| format!("Erreur lors de la création du QR code: {}", e))?;
    let image = code.render::<image::Luma<u8>>().build();

    let mut buf = Cursor::new(Vec::new());
    image
        .write_to(&mut buf, ImageFormat::Png)
        .map_err(|e| format!("Erreur lors de l'écriture du PNG: {}", e))?;
    let png_data = buf.into_inner();

    let base64_string = general_purpose::STANDARD.encode(&png_data);
    Ok(format!("data:image/png;base64,{}", base64_string))
}

#[tauri::command]
pub fn get_remote_control_url(app_handle: AppHandle) -> Result<String, String> {
    let my_local_ip = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(e) => return Err(format!("Impossible d'obtenir l'adresse IP locale: {}", e)),
    };

    let app_env_path = {
        let app_state = app_handle.state::<Mutex<AppState>>();
        let app_state_lock = app_state.lock().unwrap();
        app_state_lock.app_env_path.clone()
    };

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value =
        serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let remote_port = get_setting_value(
        &settings,
        "data.groupes.Système.groupes.Télécommande.parametres.Port",
    )
    .and_then(|v| v.as_i64())
    .map(|p| p as u16)
    .unwrap_or(9001);

    Ok(format!("http://{}:{}/remote", my_local_ip, remote_port))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DraftCircuit {
    gpx_filename: String,
    nom: String,
    depart: CircuitDepart,
    distance_km: f64,
    denivele_m: i32,
    sommet: CircuitSommet,
    iso_date_time: DateTime<Utc>,
    url: String,
    editor_name: String,
    ville_nom: String,
    track_points: Vec<Vec<f64>>,
}

// Struct to hold calculation results
struct TrackStats {
    total_distance_km: f64,
    positive_elevation_m: i32,
    summit_altitude_m: i32,
    summit_distance_km: f64,
}

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
}

#[derive(Serialize, Debug)]
struct LineString {
    #[serde(rename = "type")]
    type_field: String,
    coordinates: Vec<Vec<f64>>,
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
pub struct CircuitZoomParam {
    pub enabled: bool,
    pub valeur: f64,
    pub distance: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitZoom {
    pub depart: CircuitZoomParam,
    pub arrivee: CircuitZoomParam,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CircuitEvt {
    pub compteurs: CircuitCompteurs,
    pub affichage: CircuitAffichage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CircuitDistanceMarkersConfig {
    pub intervalle: u32,
    pub pre_affichage: u32,
    pub post_affichage: u32,
    pub orientation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub couleur: Option<String>,
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
    #[serde(rename = "trackingKm")]
    pub tracking_km: f64,
    #[serde(rename = "nomCommunes")]
    pub nom_communes: bool,
    #[serde(rename = "avancementCommunes")]
    pub avancement_communes: i32,
    pub evt: CircuitEvt,
    pub zoom: CircuitZoom,
    #[serde(
        rename = "distanceMarkersConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub distance_markers_config: Option<CircuitDistanceMarkersConfig>,
}

struct GpxMetadata {
    name: Option<String>,
    creator: Option<String>,
    time: Option<DateTime<Utc>>,
    first_point_lat: Option<f64>,
    first_point_lon: Option<f64>,
    link: Option<String>,
}

pub async fn analyze_gpx_file(
    app_handle: &AppHandle,
    filename: &str,
) -> Result<DraftCircuit, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<Mutex<AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value =
        serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let mapbox_token = super::get_setting_value(
        &settings,
        "data.groupes.Système.groupes.Tokens.parametres.mapbox",
    )
    .and_then(|v| v.as_str())
    .map(|s| s.to_string())
    .ok_or_else(|| "Token Mapbox non trouvé".to_string())?;

    let gpx_dir_path = get_gpx_directory(&settings)?;
    let file_path = gpx_dir_path.join(filename);

    if !file_path.exists() {
        return Err(format!(
            "Le fichier GPX '{}' n\'a pas été trouvé.",
            filename
        ));
    }

    let (metadata, track_points) = extract_gpx_data(&file_path)?;

    // --- Start of new code for duplicate check ---
    let circuits_file = super::read_circuits_file(&app_env_path)?;
    let gpx_name = metadata
        .name
        .clone()
        .unwrap_or_else(|| filename.to_string());

    if circuits_file.circuits.iter().any(|c| c.nom == gpx_name) {
        return Err(format!(
            "Un circuit avec le nom '{}' existe déjà.",
            gpx_name
        ));
    }
    // --- End of new code for duplicate check ---

    let rounded_track_points: Vec<Vec<f64>> = track_points
        .iter()
        .map(|point| {
            vec![
                (point[0] * 100_000.0).round() / 100_000.0,
                (point[1] * 100_000.0).round() / 100_000.0,
                (point[2] * 10.0).round() / 10.0,
            ]
        })
        .collect();

    let editor_name = identify_editor_from_creator(metadata.creator.as_deref().unwrap_or_default());

    let mut url = get_url_from_metadata(&metadata, &editor_name);
    if editor_name == "Garmin Connect" {
        let re = Regex::new(r"COURSE_(\d+)").unwrap();
        if let Some(caps) = re.captures(filename) {
            if let Some(course_id) = caps.get(1) {
                url = format!(
                    "https://connect.garmin.com/modern/course/{}",
                    course_id.as_str()
                );
            }
        }
    }

    let lon_depart = metadata.first_point_lon.unwrap_or_default();
    let lat_depart = metadata.first_point_lat.unwrap_or_default();

    let ville_nom = get_city_name_from_coords(lon_depart, lat_depart, &mapbox_token)
        .await
        .unwrap_or_else(|_| "Inconnue".to_string());

    let smoothing_distance = super::get_setting_value(
        &settings,
        "data.groupes.Importation.parametres.denivele_lissage_distance",
    )
    .and_then(|v| v.as_i64())
    .unwrap_or(10) as f64;

    let stats = calculate_track_stats(&rounded_track_points, smoothing_distance);

    Ok(DraftCircuit {
        gpx_filename: filename.to_string(),
        nom: metadata.name.unwrap_or_else(|| filename.to_string()),
        depart: CircuitDepart {
            lon: (lon_depart * 100_000.0).round() / 100_000.0,
            lat: (lat_depart * 100_000.0).round() / 100_000.0,
        },
        distance_km: stats.total_distance_km,
        denivele_m: stats.positive_elevation_m,
        sommet: CircuitSommet {
            altitude_m: stats.summit_altitude_m,
            km: stats.summit_distance_km,
        },
        iso_date_time: metadata.time.unwrap_or_else(Utc::now),
        url,
        editor_name,
        ville_nom,
        track_points: rounded_track_points,
    })
}

pub fn commit_new_circuit(
    app_handle: &AppHandle,
    draft: DraftCircuit,
    traceur_id: String,
) -> Result<String, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let mut circuits_file = super::read_circuits_file(&app_env_path)?;

    let editeur_id = resolve_editor_id(&mut circuits_file, &draft.editor_name)?;
    let ville_id = resolve_ville_id(&mut circuits_file, &draft.ville_nom)?;

    let new_circuit_id = format!("circ-{:04}", circuits_file.index_circuits + 1);
    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value =
        serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let zoom_depart_enabled = super::get_setting_value(
        &settings,
        "data.groupes.Edition.groupes.Camera.groupes.ZoomDepart.parametres.zoomDepart",
    )
    .and_then(|v| v.as_bool())
    .unwrap_or(true);
    let zoom_depart_valeur = super::get_setting_value(
        &settings,
        "data.groupes.Edition.groupes.Camera.groupes.ZoomDepart.parametres.zoomDepartValeur",
    )
    .and_then(|v| v.as_f64())
    .unwrap_or(18.0);
    let zoom_depart_distance = super::get_setting_value(
        &settings,
        "data.groupes.Edition.groupes.Camera.groupes.ZoomDepart.parametres.zoomDepartDistance",
    )
    .and_then(|v| v.as_i64())
    .unwrap_or(20) as i32;

    let zoom_arrivee_enabled = super::get_setting_value(
        &settings,
        "data.groupes.Edition.groupes.Camera.groupes.ZoomArrivee.parametres.zoomArrivee",
    )
    .and_then(|v| v.as_bool())
    .unwrap_or(true);
    let zoom_arrivee_valeur = super::get_setting_value(
        &settings,
        "data.groupes.Edition.groupes.Camera.groupes.ZoomArrivee.parametres.zoomArriveeValeur",
    )
    .and_then(|v| v.as_f64())
    .unwrap_or(18.0);
    let zoom_arrivee_distance = super::get_setting_value(
        &settings,
        "data.groupes.Edition.groupes.Camera.groupes.ZoomArrivee.parametres.distanceZoomArrivee",
    )
    .and_then(|v| v.as_i64())
    .unwrap_or(20) as i32;

    let new_circuit = Circuit {
        circuit_id: new_circuit_id.clone(),
        nom: draft.nom,
        ville_depart_id: ville_id,
        traceur_id,
        editeur_id,
        url: draft.url,
        distance_km: draft.distance_km,
        denivele_m: draft.denivele_m,
        depart: draft.depart,
        sommet: draft.sommet,
        iso_date_time: draft.iso_date_time,
        tracking_km: 0.0,    // Initialized to 0.0
        nom_communes: false, // Initialized to false
        avancement_communes: 0,
        evt: CircuitEvt {
            compteurs: CircuitCompteurs {
                zoom: 0,
                pause: 0,
                info: 0,
            },
            affichage: CircuitAffichage {
                depart: true,
                arrivee: true,
                marqueurs_10km: true,
            },
        },
        zoom: CircuitZoom {
            depart: CircuitZoomParam {
                enabled: zoom_depart_enabled,
                valeur: zoom_depart_valeur,
                distance: zoom_depart_distance,
            },
            arrivee: CircuitZoomParam {
                enabled: zoom_arrivee_enabled,
                valeur: zoom_arrivee_valeur,
                distance: zoom_arrivee_distance,
            },
        },
        distance_markers_config: None,
    };

    circuits_file.circuits.push(new_circuit.clone()); // Clone new_circuit here
    circuits_file.index_circuits += 1;

    super::write_circuits_file(&app_env_path, &circuits_file)?;

    // --- Start of new code for QR code generation ---
    if !new_circuit.url.is_empty() {
        let settings_path = app_env_path.join("settings.json");
        let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
        let settings: serde_json::Value =
            serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

        let qrcode_size = super::get_setting_value(
            &settings,
            "data.groupes.Importation.groupes.QRCode.parametres.taille",
        )
        .and_then(|v| v.as_u64())
        .unwrap_or(512) as u32; // Default to 512 if not found or invalid

        let code = QrCode::new(&new_circuit.url)
            .map_err(|e| format!("Failed to create QR code: {}", e))?;
        let luma_buf = code
            .render::<image::Luma<u8>>()
            .module_dimensions(
                qrcode_size / code.width() as u32,
                qrcode_size / code.width() as u32,
            ) // Calculate module size
            .build();

        let image = image::DynamicImage::ImageLuma8(luma_buf).to_rgba8();

        let circuit_data_dir = app_env_path.join("data").join(&new_circuit_id);
        fs::create_dir_all(&circuit_data_dir)
            .map_err(|e| format!("Failed to create circuit data directory: {}", e))?;
        let qrcode_path = circuit_data_dir.join("urlQrcode.png");
        image
            .save(&qrcode_path)
            .map_err(|e| format!("Failed to save QR code image: {}", e))?;
    }
    // --- End of new code for QR code generation ---

    create_line_string_file(&app_env_path, &new_circuit_id, &draft.track_points)?;

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value =
        serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let total_tracking_points = tracking_processor::generate_tracking_file(
        &app_env_path,
        &new_circuit_id,
        &draft.track_points,
        &settings,
    )?;

    // Auto-add distance markers if setting is true
    let auto_add_distance_markers = super::get_setting_value(
        &settings,
        "data.groupes.Edition.groupes.Messages.groupes.Distance.parametres.ajouter",
    )
    .and_then(|v| v.as_bool())
    .unwrap_or(false); // Default to false if setting is missing

    if auto_add_distance_markers {
        let dm_defaults = distance_markers::get_distance_markers_defaults(app_handle.clone())?;

        // Create DistanceMarkersConfig for event generation
        let dm_config_for_events = distance_markers::DistanceMarkersConfig {
            intervalle: dm_defaults.intervalle,
            pre_affichage: dm_defaults.pre_affichage,
            post_affichage: dm_defaults.post_affichage,
            orientation: dm_defaults.orientation.clone(),
            couleur: dm_defaults.couleur.clone(),
        };

        // Get couleur from settings (same logic as in generate_distance_markers command)
        let dm_couleur = super::get_setting_value(
            &settings,
            "data.groupes.Edition.groupes.Messages.groupes.Distance.parametres.couleur",
        )
        .and_then(|v| v.as_str())
        .unwrap_or("red")
        .to_string();

        // Generate events
        let generated_dm_events = distance_markers::generate_distance_marker_events(
            app_handle,
            &new_circuit_id,
            &dm_config_for_events,
            new_circuit.distance_km, // total_distance_km
            &dm_couleur,
        )?;

        // Update circuit's distance_markers_config in circuits_file (which is mutable)
        if let Some(circuit) = circuits_file
            .circuits
            .iter_mut()
            .find(|c| c.circuit_id == new_circuit_id)
        {
            circuit.distance_markers_config =
                Some(crate::gpx_processor::CircuitDistanceMarkersConfig {
                    intervalle: dm_config_for_events.intervalle,
                    pre_affichage: dm_config_for_events.pre_affichage,
                    post_affichage: dm_config_for_events.post_affichage,
                    orientation: dm_config_for_events.orientation,
                    couleur: Some(dm_couleur),
                });
        }
        // Write circuits_file again to persist the distance_markers_config
        super::write_circuits_file(&app_env_path, &circuits_file)?;

        // Read current events file, extend with generated events, and write back
        let mut events_file = event::read_events(app_handle, &new_circuit_id)?;
        events_file.range_events.extend(generated_dm_events);
        event::write_events(app_handle, &new_circuit_id, &events_file)?;
    }

    // --- Start of new code for auto-add Start/Arrival messages ---
    let afficher_depart = super::get_setting_value(
        &settings,
        "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.afficherDepart",
    )
    .and_then(|v| v.as_bool())
    .unwrap_or(true);

    let afficher_arrivee = super::get_setting_value(
        &settings,
        "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.afficherArrivee",
    )
    .and_then(|v| v.as_bool())
    .unwrap_or(true);

    if afficher_depart || afficher_arrivee {
        let mut events_file = event::read_events(app_handle, &new_circuit_id)?;

        if afficher_depart {
             let message_depart = super::get_setting_value(
                &settings,
                "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.messageDepart",
            )
            .and_then(|v| v.as_str())
            .unwrap_or("_Départ_green")
            .to_string();

            let orientation_depart_droite = super::get_setting_value(
                &settings,
                "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.orientationDepartDroite",
            )
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

             let post_affichage_depart = super::get_setting_value(
                &settings,
                "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.postAffichageDepart",
            )
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;

            if let Some(first_point) = draft.track_points.first() {
                 let start_event = event::RangeEventData {
                    event_id: uuid::Uuid::new_v4().to_string(),
                    message_id: Some(message_depart),
                    message: None,
                    anchor_increment: 0,
                    start_increment: 0,
                    end_increment: post_affichage_depart, // 0 + post
                    coord: first_point.clone(),
                    orientation: if orientation_depart_droite { "Droite".to_string() } else { "Gauche".to_string() },
                    metadata: None,
                };
                events_file.range_events.push(start_event);
            }
        }

        if afficher_arrivee {
            let message_arrivee = super::get_setting_value(
                &settings,
                "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.messageArrivee",
            )
            .and_then(|v| v.as_str())
            .unwrap_or("_Arrivée_red")
            .to_string();

            let orientation_arrivee_droite = super::get_setting_value(
                &settings,
                "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.orientationArriveeDroite",
            )
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

             let pre_affichage_arrivee = super::get_setting_value(
                &settings,
                "data.groupes.Importation.groupes.Label Départ Arrivée.parametres.preAffichageArrivee",
            )
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as u32;

            if let Some(last_point) = draft.track_points.last() {
                let last_idx = (total_tracking_points - 1) as u32;
                let start_idx = last_idx.saturating_sub(pre_affichage_arrivee);

                 let end_event = event::RangeEventData {
                    event_id: uuid::Uuid::new_v4().to_string(),
                    message_id: Some(message_arrivee),
                    message: None,
                    anchor_increment: last_idx,
                    start_increment: start_idx,
                    end_increment: last_idx,
                    coord: last_point.clone(),
                    orientation: if orientation_arrivee_droite { "Droite".to_string() } else { "Gauche".to_string() },
                    metadata: None,
                };
                 events_file.range_events.push(end_event);
            }
        }

        event::write_events(app_handle, &new_circuit_id, &events_file)?;
    }
    // --- End of new code for auto-add Start/Arrival messages ---

    // --- Start of new code for auto-deleting GPX file ---
    let auto_delete = get_setting_value(
        &settings,
        "data.groupes.Importation.parametres.autoDelete",
    )
    .and_then(|v| v.as_bool())
    .unwrap_or(false);

    if auto_delete {
        let gpx_dir_path = get_gpx_directory(&settings)?;
        let file_path = gpx_dir_path.join(&draft.gpx_filename);
        if file_path.exists() {
            if let Err(e) = fs::remove_file(&file_path) {
                // Log the error but don't fail the whole operation
                // as the main import was successful.
                eprintln!("Failed to delete GPX file '{}': {}", file_path.display(), e);
            }
        }
    }
    // --- End of new code for auto-deleting GPX file ---

    Ok(new_circuit_id)
}

fn resolve_editor_id(
    circuits_file: &mut CircuitsFile,
    editor_name: &str,
) -> Result<String, String> {
    if let Some(editor) = circuits_file
        .editeurs
        .iter()
        .find(|e| e.nom.eq_ignore_ascii_case(editor_name))
    {
        Ok(editor.id.clone())
    } else {
        if editor_name.is_empty() || editor_name == "Inconnu" {
            return Ok("ed-0000".to_string()); // Specific ID for "Inconnu"
        }
        let max_id = circuits_file
            .editeurs
            .iter()
            .filter_map(|e| e.id.strip_prefix("ed-").and_then(|s| s.parse::<i32>().ok()))
            .max()
            .unwrap_or(0);

        let new_id = format!("ed-{:04}", max_id + 1);

        let new_editor = Editor {
            id: new_id.clone(),
            nom: editor_name.to_string(),
        };

        circuits_file.editeurs.push(new_editor);
        Ok(new_id)
    }
}

fn resolve_ville_id(circuits_file: &mut CircuitsFile, ville_nom: &str) -> Result<String, String> {
    if let Some(ville) = circuits_file.villes.iter().find(|v| v.nom == ville_nom) {
        Ok(ville.id.clone())
    } else {
        let max_id = circuits_file
            .villes
            .iter()
            .filter_map(|v| v.id.strip_prefix("vi-").and_then(|s| s.parse::<i32>().ok()))
            .max()
            .unwrap_or(0);
        
        let new_id = format!("vi-{:04}", max_id + 1);

        let new_ville = Ville {
            id: new_id.clone(),
            nom: ville_nom.to_string(),
        };
        circuits_file.villes.push(new_ville.clone());
        Ok(new_ville.id)

    }
}

fn get_gpx_directory(settings: &serde_json::Value) -> Result<PathBuf, String> {
    let gpx_dir_setting =
        super::get_setting_value(settings, "data.groupes.Importation.parametres.ImportDir")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "Configuration du dossier d'import introuvable.".to_string())?;

    if gpx_dir_setting == "DEFAULT_DOWNLOADS" {
        dirs::download_dir()
            .ok_or_else(|| "Impossible de trouver le dossier de téléchargement.".to_string())
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
                let current_path_str = path
                    .iter()
                    .map(|p| std::str::from_utf8(p).unwrap_or(""))
                    .collect::<Vec<&str>>()
                    .join("/");

                match current_path_str.as_str() {
                    "gpx" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"creator" {
                                    if let Ok(val) =
                                        attr.decode_and_unescape_value(reader.decoder())
                                    {
                                        metadata.creator = Some(val.into_owned());
                                    }
                                }
                            }
                        }
                    }
                    "gpx/metadata/name" | "gpx/trk/name" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(name) = t.unescape() {
                                if metadata.name.is_none() {
                                    metadata.name = Some(name.to_string());
                                }
                            }
                        }
                    }
                    "gpx/metadata/copyright/license" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(license) = t.unescape() {
                                link_from_license = Some(license.to_string());
                            }
                        }
                    }
                    "gpx/metadata/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) =
                                        attr.decode_and_unescape_value(reader.decoder())
                                    {
                                        link_from_meta = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    "gpx/trk/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) =
                                        attr.decode_and_unescape_value(reader.decoder())
                                    {
                                        link_from_trk = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    "gpx/metadata/time" => {
                        if let Ok(Event::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(time_str) = t.unescape() {
                                if let Ok(dt) = time_str.parse::<DateTime<Utc>>() {
                                    metadata.time = Some(dt);
                                }
                            }
                        }
                    }
                    "gpx/trk/trkseg/trkpt" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                match attr.key.as_ref() {
                                    b"lat" => {
                                        current_lat = attr
                                            .decode_and_unescape_value(reader.decoder())
                                            .ok()
                                            .and_then(|v| v.parse().ok())
                                    }
                                    b"lon" => {
                                        current_lon = attr
                                            .decode_and_unescape_value(reader.decoder())
                                            .ok()
                                            .and_then(|v| v.parse().ok())
                                    }
                                    _ => {}
                                }
                            }
                        }
                        if metadata.first_point_lat.is_none() && current_lat.is_some() {
                            metadata.first_point_lat = current_lat;
                            metadata.first_point_lon = current_lon;
                        }
                    }
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
                    }
                    _ => {}
                }
            }
            Ok(Event::Empty(e)) => {
                path.push(e.name().as_ref().to_vec());
                let current_path_str = path
                    .iter()
                    .map(|p| std::str::from_utf8(p).unwrap_or(""))
                    .collect::<Vec<&str>>()
                    .join("/");
                match current_path_str.as_str() {
                    "gpx/metadata/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) =
                                        attr.decode_and_unescape_value(reader.decoder())
                                    {
                                        link_from_meta = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    "gpx/trk/link" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"href" {
                                    if let Ok(link) =
                                        attr.decode_and_unescape_value(reader.decoder())
                                    {
                                        link_from_trk = Some(link.into_owned());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    "gpx/trk/trkseg/trkpt" => {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                match attr.key.as_ref() {
                                    b"lat" => {
                                        current_lat = attr
                                            .decode_and_unescape_value(reader.decoder())
                                            .ok()
                                            .and_then(|v| v.parse().ok())
                                    }
                                    b"lon" => {
                                        current_lon = attr
                                            .decode_and_unescape_value(reader.decoder())
                                            .ok()
                                            .and_then(|v| v.parse().ok())
                                    }
                                    _ => {}
                                }
                            }
                        }
                        if metadata.first_point_lat.is_none() && current_lat.is_some() {
                            metadata.first_point_lat = current_lat;
                            metadata.first_point_lon = current_lon;
                        }
                        // This is a self-closing trkpt, it won't have an ele child, so we might need to handle that if ele is an attribute.
                        // Assuming ele is always a separate tag for now.
                    }
                    _ => {}
                }
                path.pop(); // Pop immediately for empty tags
            }
            Ok(Event::End(e)) => {
                if !path.is_empty() && path.last().unwrap() == e.name().as_ref() {
                    path.pop();
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("Erreur de parsing XML: {}", e)),
            _ => {}
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
            }
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

fn create_line_string_file(
    app_env_path: &Path,
    circuit_id: &str,
    track_points: &Vec<Vec<f64>>,
) -> Result<(), String> {
    let data_dir = app_env_path.join("data");
    let circuit_data_dir = data_dir.join(circuit_id);

    fs::create_dir_all(&circuit_data_dir).map_err(|e| {
        format!(
            "Impossible de créer le dossier de données du circuit: {}",
            e
        )
    })?;

    let linestring_path = circuit_data_dir.join("lineString.json");

    let linestring_data = LineString {
        type_field: "LineString".to_string(),
        coordinates: track_points.clone(),
    };

    let linestring_content =
        serde_json::to_string_pretty(&linestring_data).map_err(|e| e.to_string())?;
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
        let p1 = track_points[i - 1].as_slice();
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

async fn get_city_name_from_coords(
    lon: f64,
    lat: f64,
    mapbox_token: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();
    let mut city_name: Option<String> = None;

    // 1. Try geo.api.gouv.fr API
    let url = format!(
        "https://geo.api.gouv.fr/communes?lat={}&lon={}&fields=nom",
        lat, lon
    );
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

    city_name.ok_or_else(|| "Impossible de déterminer la ville de départ".to_string())
}

pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371000.0; // Rayon de la Terre en mètres
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin()
        + lat1.to_radians().cos()
            * lat2.to_radians().cos()
            * (d_lon / 2.0).sin()
            * (d_lon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}
