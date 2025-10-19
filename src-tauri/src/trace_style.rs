use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use tauri::State;

#[derive(Deserialize, Debug)]
struct TrackingPoint {
    altitude: f64,
}

fn get_slope_color(slope: f64, slope_colors: &HashMap<String, String>) -> String {
    let color_key = if slope <= 0.0 {
        "TrancheNegative"
    } else if slope < 3.0 {
        "Tranche1"
    } else if slope < 6.0 {
        "Tranche2"
    } else if slope < 9.0 {
        "Tranche3"
    } else if slope < 12.0 {
        "Tranche4"
    } else {
        "Tranche5"
    };
    slope_colors
        .get(color_key)
        .cloned()
        .unwrap_or_else(|| "#FFFFFF".to_string())
}

#[tauri::command]
pub async fn get_slope_color_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().unwrap();
    let data_dir = &app_state.app_env_path;

    let tracking_path = data_dir
        .join("data")
        .join(&circuit_id)
        .join("tracking.json");
    if !tracking_path.exists() {
        return Err(format!(
            "tracking.json not found for circuit {}",
            circuit_id
        ));
    }

    let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<TrackingPoint> =
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

    if tracking_points.len() < 2 {
        return Ok(serde_json::json!(slope_colors
            .get("Tranche1")
            .cloned()
            .unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    let mut slopes = Vec::new();
    for i in 1..tracking_points.len() {
        let p1 = &tracking_points[i - 1];
        let p2 = &tracking_points[i];
        let altitude_change = p2.altitude - p1.altitude;
        let slope = if segment_length > 0.0 {
            (altitude_change / segment_length) * 100.0
        } else {
            0.0
        };
        slopes.push(slope);
    }

    let colors: Vec<String> = slopes
        .iter()
        .map(|&s| get_slope_color(s, &slope_colors))
        .collect();

    let total_distance = (tracking_points.len() - 1) as f64 * segment_length;
    if total_distance <= 0.0 {
        return Ok(serde_json::json!(colors
            .first()
            .cloned()
            .unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    let mut expression: Vec<serde_json::Value> = vec![
        "interpolate".into(),
        vec!["linear"].into(),
        vec!["line-progress"].into(),
    ];

    let transition_length = 25.0; // 25m

    // Start with color of first segment
    expression.push(0.0.into());
    expression.push(colors[0].clone().into());

    for i in 1..colors.len() {
        let junction_dist = i as f64 * segment_length;
        let color_before = &colors[i - 1];
        let color_after = &colors[i];

        if color_before != color_after {
            let transition_start_dist = junction_dist - transition_length;
            let transition_end_dist = junction_dist + transition_length;

            if transition_start_dist > 0.0 {
                expression.push((transition_start_dist / total_distance).into());
                expression.push(color_before.clone().into());
            }

            if transition_end_dist < total_distance {
                expression.push((transition_end_dist / total_distance).into());
                expression.push(color_after.clone().into());
            }
        }
    }

    // Ensure the line is fully colored to the end
    expression.push(1.0.into());
    expression.push(colors.last().unwrap().clone().into());

    Ok(serde_json::to_value(expression).unwrap())
}