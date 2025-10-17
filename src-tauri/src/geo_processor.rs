use geo::prelude::{HaversineDistance, EuclideanDistance};
use geo::{Point, LineString};
use geojson::{GeoJson};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TrackingPointJs {
    pub increment: u32,
    pub point_de_control: bool,
    pub nbr_segment: u32,
    pub coordonnee: Vec<f64>,
    pub altitude: f64,
    pub commune: Option<String>,
    pub cap: f64,
    pub zoom: f64,
    pub pitch: f64,
    pub coordonnee_camera: Vec<f64>,
    pub altitude_camera: f64,
    pub edited_zoom: Option<f64>,
    pub edited_pitch: Option<f64>,
    pub edited_cap: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedTrackingPoint {
    #[serde(flatten)]
    pub tracking_point: TrackingPointJs,
    pub distance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedTrackingDataResult {
    pub processed_points: Vec<ProcessedTrackingPoint>,
    pub total_distance_km: f64,
}

// Helper function to calculate distance along a LineString to a projected point
pub fn distance_along_line_to_point(line: &LineString<f64>, point: &Point<f64>) -> f64 {
    let mut total_distance = 0.0;
    let mut min_dist_to_segment = f64::MAX;
    let mut closest_point_on_segment = Point::new(0.0, 0.0);
    let mut closest_segment_index = 0;

    let points: Vec<Point<f64>> = line.points().map(|p| p.clone()).collect(); // Collect owned points

    // Find the segment closest to the point
    for i in 0..points.len() - 1 {
        let p1 = &points[i];
        let p2 = &points[i + 1];

        let segment_start = Point::new(p1.x(), p1.y());
        let segment_end = Point::new(p2.x(), p2.y());

        // Project point onto segment
        let segment_vec = Point::new(segment_end.x() - segment_start.x(), segment_end.y() - segment_start.y());
        let point_vec = Point::new(point.x() - segment_start.x(), point.y() - segment_start.y());

        let dot_product = point_vec.x() * segment_vec.x() + point_vec.y() * segment_vec.y();
        let segment_length_sq = segment_vec.x().powi(2) + segment_vec.y().powi(2);

        let t = if segment_length_sq == 0.0 { // Handle zero-length segment
            0.0
        } else {
            (dot_product / segment_length_sq).max(0.0).min(1.0)
        };

        let projected_x = segment_start.x() + t * segment_vec.x();
        let projected_y = segment_start.y() + t * segment_vec.y();
        let projected_point = Point::new(projected_x, projected_y);

        let dist = point.euclidean_distance(&projected_point);

        if dist < min_dist_to_segment {
            min_dist_to_segment = dist;
            closest_point_on_segment = projected_point;
            closest_segment_index = i;
        }
    }

    // Calculate distance up to the start of the closest segment
    for i in 0..closest_segment_index {
        total_distance += points[i].haversine_distance(&points[i + 1]);
    }

    // Add distance from start of closest segment to the projected point
    total_distance += points[closest_segment_index].haversine_distance(&closest_point_on_segment);

    total_distance
}

#[tauri::command]
pub fn process_tracking_data(
    _line_string_geojson: GeoJson, // This is kept for compatibility but not used
    tracking_points_js: Vec<TrackingPointJs>,
) -> Result<ProcessedTrackingDataResult, String> {
    let mut processed_points = Vec::new();
    let segment_length_m = 100.0; // Based on user confirmation that each increment is 100m

    for tp_js in tracking_points_js {
        let distance_m = tp_js.increment as f64 * segment_length_m;

        processed_points.push(ProcessedTrackingPoint {
            tracking_point: tp_js.clone(),
            distance: distance_m / 1000.0, // distance in km
        });
    }

    let total_distance_km = if let Some(last_point) = processed_points.last() {
        (last_point.distance * 100.0).round() / 100.0
    } else {
        0.0
    };

    Ok(ProcessedTrackingDataResult {
        processed_points,
        total_distance_km,
    })
}