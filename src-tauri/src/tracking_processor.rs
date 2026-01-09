use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
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
    cap: u32,
    zoom: u32,
    pitch: u32,
    coordonnee_camera: Vec<f64>,
    altitude_camera: u32,
}

pub fn generate_tracking_file(
    app_env_path: &Path,
    circuit_id: &str,
    track_points: &Vec<Vec<f64>>,
    settings: &serde_json::Value,
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

        let tracking_point = TrackingPoint {
            increment: i as u32,
            point_de_control: i == 0,
            nbr_segment: 0,
            coordonnee: [(point.x() * 100000.0).round() / 100000.0, (point.y() * 100000.0).round() / 100000.0],
            altitude: (altitude * 10.0).round() / 10.0,
            commune: None,
            cap: cap.round() as u32,
            zoom: default_zoom,
            pitch: default_pitch,
            coordonnee_camera: vec![],
            altitude_camera: 0,
        };
        tracking_points.push(tracking_point);
    }

    let data_dir = app_env_path.join("data");
    let circuit_data_dir = data_dir.join(circuit_id);
    let tracking_path = circuit_data_dir.join("tracking.json");

    let tracking_content = serde_json::to_string_pretty(&tracking_points).map_err(|e| e.to_string())?;
    fs::write(&tracking_path, tracking_content).map_err(|e| e.to_string())?; 

    Ok(tracking_points.len())
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
