use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use geo::{
    LineString as GeoLineString,
    Point,
    prelude::*,
    algorithm::line_interpolate_point::LineInterpolatePoint,
    algorithm::line_locate_point::LineLocatePoint,
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
    commune: String,
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
) -> Result<(), String> {
    let segment_length = super::get_setting_value(settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);
    let bearing_smoothing = super::get_setting_value(settings, "data.groupes.Importation.groupes.Tracking.parametres.LissageCap")
        .and_then(|v| v.as_u64())
        .unwrap_or(15) as usize;
    let default_zoom = super::get_setting_value(settings, "data.groupes.Importation.groupes.Caméra.parametres.Zoom")
        .and_then(|v| v.as_u64().map(|i| i as u32))
        .unwrap_or(16);
    let default_pitch = super::get_setting_value(settings, "data.groupes.Importation.groupes.Caméra.parametres.Pitch")
        .and_then(|v| v.as_u64().map(|i| i as u32))
        .unwrap_or(60);

    let geo_points: Vec<Point<f64>> = track_points
        .iter()
        .map(|p| Point::new(p[0], p[1]))
        .collect();
    let line = GeoLineString::from(geo_points.clone());

    let mut tracking_points: Vec<TrackingPoint> = Vec::new();
    let mut increment = 0;

    let mut calculated_points = Vec::new();
    let mut distance_needed = 0.0;
    let mut distance_traversed = 0.0;

    if let Some(first_point) = line.points().next() {
        calculated_points.push(first_point);
        distance_needed += segment_length;
    }

    for segment in line.lines() {
        let p1 = segment.start;
        let p2 = segment.end;
        let segment_len = Point::from(p1).haversine_distance(&Point::from(p2));

        while distance_traversed + segment_len >= distance_needed {
            let dist_into_segment = distance_needed - distance_traversed;
            let fraction = if segment_len > 0.0 { dist_into_segment / segment_len } else { 0.0 };
            let new_point = geo::Line::new(p1, p2).line_interpolate_point(fraction).unwrap();
            calculated_points.push(new_point);
            distance_needed += segment_length;
        }
        distance_traversed += segment_len;
    }

    // Add the very last point if it's not close to the last calculated one
    if let Some(last_original_point) = line.points().last() {
        if let Some(last_calculated_point) = calculated_points.last() {
            if last_calculated_point.haversine_distance(&last_original_point) > 1.0 { // If more than 1m away
                 calculated_points.push(last_original_point);
            }
        }
    }

    for (i, point) in calculated_points.iter().enumerate() {
        let altitude = interpolate_altitude(&point, &track_points);
        let cap = calculate_smoothed_bearing(i, &calculated_points, bearing_smoothing);

        let tracking_point = TrackingPoint {
            increment,
            point_de_control: false,
            nbr_segment: 0,
            coordonnee: [(point.x() * 100000.0).round() / 100000.0, (point.y() * 100000.0).round() / 100000.0],
            altitude: (altitude * 10.0).round() / 10.0,
            commune: "".to_string(),
            cap: cap.round() as u32,
            zoom: default_zoom,
            pitch: default_pitch,
            coordonnee_camera: vec![],
            altitude_camera: 0,
        };
        tracking_points.push(tracking_point);
        increment += 1;
    }

    let data_dir = app_env_path.join("data");
    let circuit_data_dir = data_dir.join(circuit_id);
    let tracking_path = circuit_data_dir.join("tracking.json");

    let tracking_content = serde_json::to_string_pretty(&tracking_points).map_err(|e| e.to_string())?;
    fs::write(&tracking_path, tracking_content).map_err(|e| e.to_string())?; 

    Ok(())
}

fn interpolate_altitude(point: &Point<f64>, track_points: &Vec<Vec<f64>>) -> f64 {
    if track_points.len() < 2 {
        return track_points.get(0).map_or(0.0, |p| p[2]);
    }

    let geo_points: Vec<Point<f64>> = track_points.iter().map(|p| Point::new(p[0], p[1])).collect();
    let line = GeoLineString::from(geo_points);

    if let Some(fraction) = line.line_locate_point(point) {
        let total_len = line.haversine_length();
        let dist_on_line = total_len * fraction;

        let mut traversed_dist = 0.0;
        for i in 0..track_points.len() - 1 {
            let p1_vec = &track_points[i];
            let p2_vec = &track_points[i + 1];
            let p1 = Point::new(p1_vec[0], p1_vec[1]);
            let p2 = Point::new(p2_vec[0], p2_vec[1]);
            let segment_len = p1.haversine_distance(&p2);

            if traversed_dist + segment_len >= dist_on_line {
                let dist_into_segment = dist_on_line - traversed_dist;
                let segment_fraction = if segment_len > 0.0 { dist_into_segment / segment_len } else { 0.0 };
                let alt1 = p1_vec[2];
                let alt2 = p2_vec[2];
                return alt1 + (alt2 - alt1) * segment_fraction;
            }
            traversed_dist += segment_len;
        }
    }
    // Fallback to the last point's altitude
    track_points.last().map_or(0.0, |p| p[2])
}

fn calculate_smoothed_bearing(current_index: usize, points: &Vec<Point<f64>>, window_size: usize) -> f64 {
    if points.len() < 2 || current_index >= points.len() - 1 {
        return 0.0; // Not enough points or at the end
    }

    let end_index = (current_index + window_size).min(points.len());
    let mut bearings = Vec::new();

    for i in current_index..end_index - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        if p1 != p2 {
            bearings.push(p1.haversine_bearing(p2));
        }
    }

    if bearings.is_empty() {
        // Fallback for the very last segments if they are duplicates
        if current_index > 0 {
            let p1 = points[current_index - 1];
            let p2 = points[current_index];
            if p1 != p2 { return p1.haversine_bearing(p2); }
        }
        return 0.0;
    }

    // Vectorial mean
    let (sum_sin, sum_cos) = bearings.iter().fold((0.0, 0.0), |(ss, sc), &b| {
        let rad = b.to_radians();
        (ss + rad.sin(), sc + rad.cos())
    });

    let avg_rad = sum_sin.atan2(sum_cos);
    let avg_bearing = avg_rad.to_degrees();

    if avg_bearing < 0.0 {
        avg_bearing + 360.0
    } else {
        avg_bearing
    }
}
