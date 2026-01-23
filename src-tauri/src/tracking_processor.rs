use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::Manager;
use geo::{
    Point,
    prelude::*,
    algorithm::line_interpolate_point::LineInterpolatePoint,
    algorithm::haversine_bearing::HaversineBearing
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct TrackingPoint {
    increment: u32,
    point_de_control: bool,
    nbr_segment: u32,
    coordonnee: [f64; 2],
    altitude: f64,
    commune: Option<String>,
    cap: f64,
    zoom: f64,
    pitch: f64,
    coordonnee_camera: Vec<f64>,
    altitude_camera: f64,
    edited_zoom: Option<f64>,
    edited_pitch: Option<f64>,
    edited_cap: Option<f64>,
    
    // 🔴 Distance cumulée en km (nécessaire pour detect_overlapping_segments)
    distance: f64,
    
    // 🔴 NOUVEAU: Métadonnées pour segments irréguliers et points d'ancrage
    #[serde(skip_serializing_if = "Option::is_none")]
    actual_segment_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_regular_segment: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anchor_point: Option<bool>,
}

pub fn generate_tracking_file(
    app_env_path: &Path,
    circuit_id: &str,
    track_points: &Vec<Vec<f64>>,
    settings: &serde_json::Value,
    output_filename: Option<&str>,
    override_first: Option<serde_json::Value>,
    override_last: Option<serde_json::Value>,
) -> Result<usize, String> {
    let segment_length = super::get_setting_value(settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);
    let bearing_smoothing = super::get_setting_value(settings, "data.groupes.Importation.groupes.Tracking.parametres.LissageCap")
        .and_then(|v| v.as_u64())
        .unwrap_or(15) as usize;
    let default_zoom = super::get_setting_value(settings, "data.groupes.Importation.groupes.Camera.parametres.Zoom")
        .and_then(|v| v.as_u64().map(|i| i as u32))
        .unwrap_or(16);
    let default_pitch = super::get_setting_value(settings, "data.groupes.Importation.groupes.Camera.parametres.Pitch")
        .and_then(|v| v.as_u64().map(|i| i as u32))
        .unwrap_or(60);

    let geo_points: Vec<Point<f64>> = track_points
        .iter()
        .map(|p| Point::new(p[0], p[1]))
        .collect();

    // Store (Point, Altitude)
    let mut calculated_points: Vec<(Point<f64>, f64)> = Vec::new();
    
    let mut distance_needed = 0.0;
    let mut distance_traversed = 0.0;

    // Add first point
    if let (Some(first_pt), Some(first_orig)) = (geo_points.first(), track_points.first()) {
        calculated_points.push((*first_pt, first_orig[2]));
        distance_needed += segment_length;
    }

    // Iterate segments by index to preserve topology and altitude context
    for i in 0..geo_points.len() - 1 {
        let p1 = geo_points[i];
        let p2 = geo_points[i+1];
        let alt1 = track_points[i][2];
        let alt2 = track_points[i+1][2];

        let segment_len = p1.haversine_distance(&p2);

        while distance_traversed + segment_len >= distance_needed {
            let dist_into_segment = distance_needed - distance_traversed;
            let fraction = if segment_len > 0.0 { dist_into_segment / segment_len } else { 0.0 };
            
            // Interpolate Position
            let new_point = geo::Line::new(p1, p2).line_interpolate_point(fraction).unwrap();
            
            // Interpolate Altitude directly from the current segment
            // This guarantees we don't jump to a nearby overlapping segment
            let new_alt = alt1 + (alt2 - alt1) * fraction;
            
            calculated_points.push((new_point, new_alt));
            distance_needed += segment_length;
        }
        distance_traversed += segment_len;
    }

    // Add the very last point if needed
    if let (Some(last_pt), Some(last_orig)) = (geo_points.last(), track_points.last()) {
        if let Some((last_calc_pt, _)) = calculated_points.last() {
            if last_calc_pt.haversine_distance(last_pt) > 1.0 {
                 calculated_points.push((*last_pt, last_orig[2]));
            }
        }
    }

    let points_only: Vec<Point<f64>> = calculated_points.iter().map(|(p, _)| *p).collect();
    let mut tracking_points: Vec<TrackingPoint> = Vec::new();

    for (i, (point, altitude)) in calculated_points.iter().enumerate() {
        let cap = calculate_smoothed_bearing(i, &points_only, bearing_smoothing);

        // 🔴 CORRECTION: actualSegmentLength = longueur NOMINALE du segment (ex: 100m)
        // Exception: dernier segment peut être plus court
        // Les points d'ancrage seront marqués plus tard dans variant_processor.rs
        let actual_seg_length: Option<f64> = if i < calculated_points.len() - 1 {
            // Par défaut, tous les segments font segment_length (100m)
            // Sauf le tout dernier segment qui peut être plus court
            if i == calculated_points.len() - 2 {
                // Avant-dernier point : calculer la vraie distance vers le dernier
                let next_point = &calculated_points[i + 1].0;
                Some(point.haversine_distance(next_point))
            } else {
                // Tous les autres segments : longueur nominale
                Some(segment_length)
            }
        } else {
            None // Dernier point, pas de segment suivant
        };

        // 🔴 CORRECTION: Segment régulier = longueur proche de segment_length (±5%)
        let is_regular = actual_seg_length.map(|len| {
            let tolerance = segment_length * 0.05;
            (len - segment_length).abs() <= tolerance
        });

        // 🔴 NOTE: isAnchorPoint sera défini plus tard dans variant_processor.rs
        // Pour les traces principales, pas de points d'ancrage
        let is_anchor = None;

        // Calculer la distance cumulée en km
        let cumulative_distance_km = (i as f64 * segment_length) / 1000.0;
        
        let mut tp = TrackingPoint {
            increment: i as u32,
            point_de_control: i == 0 || i == calculated_points.len() - 1,
            nbr_segment: 0,
            coordonnee: [(point.x() * 100000.0).round() / 100000.0, (point.y() * 100000.0).round() / 100000.0],
            altitude: (altitude * 10.0).round() / 10.0,
            commune: None,
            cap: (cap * 10.0).round() / 10.0,
            zoom: default_zoom as f64,
            pitch: default_pitch as f64,
            coordonnee_camera: vec![],
            altitude_camera: 0.0,
            edited_zoom: None,
            edited_pitch: None,
            edited_cap: None,
            
            // 🔴 Distance cumulée
            distance: cumulative_distance_km,
            
            // 🔴 NOUVEAU: Métadonnées
            actual_segment_length: actual_seg_length,
            is_regular_segment: is_regular,
            is_anchor_point: is_anchor,
        };

        if i == 0 {
            if let Some(ref ovr) = override_first {
                apply_override_to_tp(&mut tp, ovr);
            }
        }
        if i == calculated_points.len() - 1 {
            if let Some(ref ovr) = override_last {
                apply_override_to_tp(&mut tp, ovr);
            }
        }

        tracking_points.push(tp);
    }

    let data_dir = app_env_path.join("data");
    let circuit_data_dir = data_dir.join(circuit_id);
    let tracking_filename = output_filename.unwrap_or("tracking.json");
    let tracking_path = circuit_data_dir.join(tracking_filename);

    let tracking_content = serde_json::to_string_pretty(&tracking_points).map_err(|e| e.to_string())?;
    fs::write(&tracking_path, tracking_content).map_err(|e| e.to_string())?; 

    Ok(tracking_points.len())
}


#[tauri::command]
pub fn read_tracking_file(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    filename: Option<String>,
) -> Result<serde_json::Value, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<super::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let data_dir = app_env_path.join("data");
    let circuit_data_dir = data_dir.join(circuit_id);
    let target_filename = filename.unwrap_or_else(|| "tracking.json".to_string());
    let tracking_path = circuit_data_dir.join(target_filename);

    if !tracking_path.exists() {
        return Err("Tracking file not found".to_string());
    }

    let file_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let json_content: serde_json::Value = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;
    Ok(json_content)
}

fn calculate_smoothed_bearing(current_index: usize, points: &Vec<Point<f64>>, window_size: usize) -> f64 {
    if points.len() < 2 || current_index >= points.len() - 1 {
        // Fallback for the very end of the track
        if current_index > 0 && current_index < points.len() {
            return points[current_index - 1].haversine_bearing(points[current_index]);
        }
        return 0.0;
    }

    let current_point = points[current_index];
    let end_index = (current_index + 1 + window_size).min(points.len());
    
    let mut bearing_distance_pairs = Vec::new();

    // Step 1: Calculate bearing and distance to each point in the look-ahead window
    for i in (current_index + 1)..end_index {
        let target_point = points[i];
        if current_point != target_point {
            let bearing = current_point.haversine_bearing(target_point);
            let distance = current_point.haversine_distance(&target_point);
            bearing_distance_pairs.push((bearing, distance));
        }
    }

    if bearing_distance_pairs.is_empty() {
        // Fallback if there are no valid look-ahead points
        if current_index + 1 < points.len() {
             return points[current_index].haversine_bearing(points[current_index + 1]);
        }
       return 0.0;
    }

    // Step 2: Find the pairs corresponding to min and max bearing
    let (min_bearing, dist_for_min) = *bearing_distance_pairs.iter().min_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap();
    let (max_bearing, dist_for_max) = *bearing_distance_pairs.iter().max_by(|a, b| a.0.partial_cmp(&b.0).unwrap()).unwrap();

    // Step 3, 4, 5: Weighted circular average using vectors
    let rad_min = min_bearing.to_radians();
    let rad_max = max_bearing.to_radians();

    let sum_x = dist_for_min * rad_min.cos() + dist_for_max * rad_max.cos();
    let sum_y = dist_for_min * rad_min.sin() + dist_for_max * rad_max.sin();

    // Step 6: Calculate final angle
    let avg_rad = sum_y.atan2(sum_x);
    let mut avg_bearing = avg_rad.to_degrees();

    // Normalize to [0, 360)
    if avg_bearing < 0.0 {
        avg_bearing += 360.0;
    }
    
    avg_bearing
}

fn apply_override_to_tp(tp: &mut TrackingPoint, ovr: &serde_json::Value) {
    if let Some(alt) = ovr.get("altitude").and_then(|v| v.as_f64()) {
        tp.altitude = alt;
    }
    if let Some(pdc) = ovr.get("pointDeControl").and_then(|v| v.as_bool()) {
        tp.point_de_control = pdc;
    }
    if let Some(nbr) = ovr.get("nbrSegment").and_then(|v| v.as_u64()) {
        tp.nbr_segment = nbr as u32;
    }
    if let Some(commune) = ovr.get("commune").and_then(|v| v.as_str()) {
        tp.commune = Some(commune.to_string());
    }
    if let Some(cap) = ovr.get("cap").and_then(|v| v.as_f64()) {
        tp.cap = cap;
    }
    if let Some(zoom) = ovr.get("zoom").and_then(|v| v.as_f64()) {
        tp.zoom = zoom;
    }
    if let Some(pitch) = ovr.get("pitch").and_then(|v| v.as_f64()) {
        tp.pitch = pitch;
    }
    if let Some(coords_cam) = ovr.get("coordonneeCamera").and_then(|v| v.as_array()) {
        tp.coordonnee_camera = coords_cam.iter().filter_map(|v| v.as_f64()).collect();
    }
    if let Some(alt_cam) = ovr.get("altitudeCamera").and_then(|v| v.as_f64()) {
        tp.altitude_camera = alt_cam;
    }
    if let Some(zoom) = ovr.get("editedZoom").and_then(|v| v.as_f64()) {
        tp.edited_zoom = Some(zoom);
    }
    if let Some(pitch) = ovr.get("editedPitch").and_then(|v| v.as_f64()) {
        tp.edited_pitch = Some(pitch);
    }
    if let Some(cap) = ovr.get("editedCap").and_then(|v| v.as_f64()) {
        tp.edited_cap = Some(cap);
    }
}
