use std::fs;
use std::sync::Mutex;

use geo::prelude::*;
use geo::{Coord, LineString as GeoLineString, Point};
use polyline::encode_coordinates;
use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::{get_setting_value, AppState};
use crate::colors::convert_vuetify_color_to_hex;
use crate::gpx_processor;

#[tauri::command]
#[allow(clippy::too_many_lines)]
pub async fn generate_gpx_thumbnail(
    app_handle: AppHandle,
    circuit_id: String,
    line_string_path: String,
    settings: Value, // Les paramètres de la vignette
) -> Result<String, String> {
    let (mapbox_token, app_env_path) = {
        let state_mutex = app_handle.state::<Mutex<AppState>>();
        let app_state = state_mutex.lock().unwrap();
        (app_state.mapbox_token.clone(), app_state.app_env_path.clone())
    };

    // 1. Lire le lineString.json
    let line_string_content = fs::read_to_string(&line_string_path)
        .map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let line_string_data: Value = serde_json::from_str(&line_string_content)
        .map_err(|e| format!("Failed to parse lineString.json: {}", e))?;

    let coordinates = line_string_data["coordinates"]
        .as_array()
        .ok_or("Coordinates not found in lineString.json")?;

    if coordinates.is_empty() {
        return Err("No coordinates found in lineString.json".to_string());
    }

    // Extraire les coordonnées de départ et d'arrivée
    let start_coord = coordinates[0].as_array().ok_or("Invalid start coordinate format")?;
    let end_coord = coordinates[coordinates.len() - 1].as_array().ok_or("Invalid end coordinate format")?;

    let start_lon = start_coord[0].as_f64().ok_or("Invalid start longitude")?;
    let start_lat = start_coord[1].as_f64().ok_or("Invalid start latitude")?;
    let end_lon = end_coord[0].as_f64().ok_or("Invalid end longitude")?;
    let end_lat = end_coord[1].as_f64().ok_or("Invalid end latitude")?;




    // 2. Extraire les paramètres de la vignette
    let style_vignette = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.parametres.styleVignette")
        .and_then(|v| v.as_str()).unwrap_or("mapbox://styles/mapbox/streets-v12");
    let color_gpx_vignette = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.Trace.parametres.colorGPXVignette")
        .and_then(|v| v.as_str()).unwrap_or("orange-darken-4");
    let largeur = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.Dimentions.parametres.largeur")
        .and_then(|v| v.as_u64()).unwrap_or(400) as u32;
    let format_str = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.Dimentions.parametres.format")
        .and_then(|v| v.as_str()).unwrap_or("1/1");
    let presence_distance = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.MarqueurDistance.parametres.presenceDistance")
        .and_then(|v| v.as_bool()).unwrap_or(true);
    let distance_interval = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.MarqueurDistance.parametres.Distance")
        .and_then(|v| v.as_u64()).unwrap_or(10) as f64; // en km
    let show_markers = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.DepartArrivee.parametres.Vignettes")
        .and_then(|v| v.as_bool()).unwrap_or(true);
    let couleur_depart = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.DepartArrivee.parametres.couleurDépart")
        .and_then(|v| v.as_str()).unwrap_or("green-darken-2");
    let couleur_arrivee = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.DepartArrivee.parametres.couleurArrivée")
        .and_then(|v| v.as_str()).unwrap_or("red-darken-2");
    let couleur_depart_arrivee = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.DepartArrivee.parametres.couleurDépartArrivée")
        .and_then(|v| v.as_str()).unwrap_or("blue-darken-2");
    let distance_max_close = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.DepartArrivee.parametres.distanceMax")
        .and_then(|v| v.as_u64()).unwrap_or(250) as f64; // en mètres

    // Convertir la couleur Vuetify en hex
    let gpx_color_hex = convert_vuetify_color_to_hex(color_gpx_vignette);

    // Calculer la hauteur en fonction du format
    let hauteur = match format_str {
        "1/1" => largeur,
        "4/3" => (largeur as f32 * 0.75) as u32,
        "16/9" => (largeur as f32 * 0.5625) as u32,
        _ => largeur, // Par défaut 1/1
    };

    // Construire la chaîne de coordonnées pour le chemin Mapbox
    // Convertir les coordonnées en GeoLineString pour la simplification
    let geo_points: Vec<Point<f64>> = coordinates.iter().map(|c| {
        let lon = c[0].as_f64().unwrap_or_default();
        let lat = c[1].as_f64().unwrap_or_default();
        Point::new(lon, lat)
    }).collect();
    let geo_line = GeoLineString::from(geo_points);

    let nb_pts = coordinates.len();
    let simplified_line = if nb_pts > 2000 { // Plus de 2000 points
        geo_line.simplify(&0.0005)
    } else if nb_pts > 500 { // Entre 500 et 2000 points
        geo_line.simplify(&0.0001)
    } else if nb_pts > 100 { // Entre 100 et 500 points
        geo_line.simplify(&0.00005)
    } else { // Moins de 100 points, ou très petite trace
        geo_line.simplify(&0.00001) // Appliquer une petite simplification par défaut
    };

    let nb_pts_simplified = simplified_line.points().count();

    let encoded_polyline = encode_coordinates(simplified_line.points().map(|p| Coord { x: p.x(), y: p.y() }), 5)
        .map_err(|e| format!("Failed to encode polyline: {:?}", e))?;
    // Remplacer uniquement les barres obliques inverses, comme requis par l'API Mapbox et la documentation du crate polyline.
    // Un encodage excessif peut entraîner des erreurs.
    let path_string = encoded_polyline.replace('\\', "%5C").replace('?', "%3F").replace('@', "%40").replace('[', "%5B").replace(']', "%5D");

    let largeur_trace = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.Trace.parametres.largeurTrace")
        .and_then(|v| v.as_u64()).unwrap_or(3) as u32;

    // Construire l'overlay pour la trace
    let mut overlay_parts = vec![];
    overlay_parts.push(format!("path-{}+{}({})", largeur_trace, gpx_color_hex, path_string)); // Largeur variable, couleur hex

    // Ajouter les marqueurs si activés
    if show_markers {
        // Calculer la distance entre le départ et l'arrivée pour déterminer si elles sont proches
        let dist_meters = gpx_processor::haversine_distance(start_lat, start_lon, end_lat, end_lon);

        if dist_meters <= distance_max_close {
            // Départ et arrivée proches, utiliser un seul marqueur
            let color_hex = convert_vuetify_color_to_hex(couleur_depart_arrivee);
            overlay_parts.push(format!("pin-s-circle+{}({},{})", color_hex, start_lon, start_lat));
        } else {
            // Départ et arrivée distincts
            let color_start_hex = convert_vuetify_color_to_hex(couleur_depart);
            let color_end_hex = convert_vuetify_color_to_hex(couleur_arrivee);
            overlay_parts.push(format!("pin-s+{}({:.4},{:.4})", color_start_hex, start_lon, start_lat));
            overlay_parts.push(format!("pin-s+{}({:.4},{:.4})", color_end_hex, end_lon, end_lat));
        }
    }

    // Calculer les distances cumulées et ajouter les marqueurs de distance/direction
    let mut cumulative_distance_km = 0.0;
    let mut last_marker_distance_km = 0.0;
    let mut distance_marker_count = 0;

    for i in 0..coordinates.len() {
        let current_coord = coordinates[i].as_array().ok_or("Invalid coordinate format")?;
        let current_lon = current_coord[0].as_f64().ok_or("Invalid longitude")?;
        let current_lat = current_coord[1].as_f64().ok_or("Invalid latitude")?;

        if i > 0 {
            let prev_coord = coordinates[i-1].as_array().ok_or("Invalid coordinate format")?;
            let prev_lon = prev_coord[0].as_f64().ok_or("Invalid longitude")?;
            let prev_lat = prev_coord[1].as_f64().ok_or("Invalid latitude")?;
            cumulative_distance_km += gpx_processor::haversine_distance(prev_lat, prev_lon, current_lat, current_lon) / 1000.0;
        }

        if presence_distance && cumulative_distance_km >= (last_marker_distance_km + distance_interval) {
            distance_marker_count += 1;
            
            // Ajouter un marqueur de distance
            let base_color = get_setting_value(&settings, "data.groupes.Importation.groupes.Vignette.groupes.MarqueurDistance.parametres.couleurPinDistance")
                .and_then(|v| v.as_str()).unwrap_or("red");

            let decade = distance_marker_count / 10;
            let intensity_suffix = match decade {
                0 => "lighten-3", // 1-9
                1 => "lighten-2", // 10-19
                2 => "lighten-1", // 20-29
                3 => "darken-1",  // 30-39
                4 => "darken-2",  // 40-49
                5 => "darken-3",  // 50-59
                _ => "" // Default to no suffix for 60+
            };

            let marker_color_hex = if !intensity_suffix.is_empty() {
                let full_color_name = format!("{}-{}", base_color, intensity_suffix);
                convert_vuetify_color_to_hex(&full_color_name)
            } else {
                "000000".to_string() // Default to black for pins >= 60
            };

            let pin_label = distance_marker_count % 10;
            overlay_parts.push(format!("pin-s-{}+{}({},{})", pin_label, &marker_color_hex, current_lon, current_lat));

            last_marker_distance_km = cumulative_distance_km;
        }
    }

    let overlay_string = overlay_parts.join(",");

    // 3. Construire l'URL de l'API Mapbox Static Images
    let mapbox_url = format!(
        "https://api.mapbox.com/styles/v1/{}/static/{}/auto/{}x{}{}{}",
        style_vignette.replace("mapbox://styles/", ""), // Supprimer le préfixe
        overlay_string,
        largeur,
        hauteur,
        "@2x", // Pour une meilleure qualité
        format!("?access_token={}", mapbox_token)
    );
    // 4. Effectuer la requête HTTP
    let client = reqwest::Client::new();
    let response = client.get(&mapbox_url).send().await
        .map_err(|e| format!("Failed to fetch Mapbox static image: {}", e))?;

    let status = response.status(); // Store the status before consuming response
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Mapbox API returned an error: {} - {}", status, error_text));
    }

    // 5. Sauvegarder l'image
    let image_bytes = response.bytes().await
        .map_err(|e| format!("Failed to read image bytes: {}", e))?;

    let data_dir = app_env_path.join("data");
    fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data directory: {}", e))?;

    let circuit_data_dir = data_dir.join(&circuit_id);
    fs::create_dir_all(&circuit_data_dir)
        .map_err(|e| format!("Failed to create circuit data directory: {}", e))?;

    let thumbnail_path = circuit_data_dir.join("vignette.png");
    fs::write(&thumbnail_path, image_bytes)
        .map_err(|e| format!("Failed to write thumbnail file: {}", e))?;

    Ok(thumbnail_path.to_string_lossy().into_owned())
}
