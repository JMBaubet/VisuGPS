
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use tauri::State;
use geo::algorithm::haversine_distance::HaversineDistance;
use geo::Point;
use crate::variant_processor::{Archive, VariantModification};

#[derive(serde::Deserialize, Debug)]

pub struct TrackingPoint {
    pub altitude: f64,
    pub coordonnee: [f64; 2],
}

fn get_slope_color(slope: f64, slope_colors: &HashMap<String, String>) -> String {
    let color_key = if slope < -12.0 {
        "TrancheNeg6"
    } else if slope < -9.0 {
        "TrancheNeg5"
    } else if slope < -6.0 {
        "TrancheNeg4"
    } else if slope < -3.0 {
        "TrancheNeg3"
    } else if slope < -1.0 {
        "TrancheNeg2"
    } else if slope < 1.0 {
        "TrancheNeg1"
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
    _segment_length: f64,
    filename: Option<String>,
    tracking_data: Option<Vec<TrackingPoint>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.lock().unwrap();
    let data_dir = &app_state.app_env_path;

    let tracking_points = if let Some(data) = tracking_data {
        data
    } else {
        let target_filename = filename.unwrap_or_else(|| "tracking.json".to_string());
        let tracking_path = data_dir
            .join("data")
            .join(&circuit_id)
            .join(&target_filename); // Warning: check if target_filename has .json

        if !tracking_path.exists() {
            return Err(format!(
                "Tracking file not found at {:?}",
                tracking_path
            ));
        }

        let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?
    };

    if tracking_points.len() < 2 {
        return Ok(serde_json::json!(slope_colors
            .get("Tranche1")
            .cloned()
            .unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    let mut segments = Vec::new();
    let mut cumulative_dist = 0.0;

    for i in 1..tracking_points.len() {
        let p1 = &tracking_points[i - 1];
        let p2 = &tracking_points[i];
        
        let point1 = Point::new(p1.coordonnee[0], p1.coordonnee[1]);
        let point2 = Point::new(p2.coordonnee[0], p2.coordonnee[1]);
        let dist = point1.haversine_distance(&point2);

        // Ignore tiny segments to verify monotonicity/avoid div/0
        if dist > 0.01 { 
            let altitude_change = p2.altitude - p1.altitude;
            let slope = (altitude_change / dist) * 100.0;
            
            cumulative_dist += dist;
            segments.push((cumulative_dist, get_slope_color(slope, &slope_colors)));
        }
    }

    let total_distance = cumulative_dist;
    
    if total_distance <= 0.0 {
        return Ok(serde_json::json!(slope_colors
            .get("Tranche1")
            .cloned()
            .unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    let mut expression: Vec<serde_json::Value> = vec![
        "interpolate".into(),
        vec!["linear"].into(),
        vec!["line-progress"].into(),
    ];

    // Build gradient with 25m soft transitions
    let transition_length = 12.5; // 25m each side
    let mut last_val = 0.0;

    // Start
    expression.push(0.0.into());
    let start_color = if !segments.is_empty() { segments[0].1.clone() } else { "#FFFFFF".to_string() };
    expression.push(start_color.into());

    for i in 0..segments.len() - 1 {
        let (end_dist, current_color) = &segments[i];
        let next_color = &segments[i+1].1;
        
        // Only insert transition if color changes
        if current_color != next_color {
            let transition_start_dist = end_dist - transition_length;
            let transition_end_dist = end_dist + transition_length;

            // Stop for current color (start of transition)
            let stop1 = (transition_start_dist / total_distance).max(last_val + 0.000001).min(1.0);
            expression.push(stop1.into());
            expression.push(current_color.clone().into());
            last_val = stop1;

            // Stop for next color (end of transition)
            let stop2 = (transition_end_dist / total_distance).max(last_val + 0.000001).min(1.0);
            if stop2 < 1.0 {
                expression.push(stop2.into());
                expression.push(next_color.clone().into());
                last_val = stop2;
            } else {
                break;
            }
        }
    }

    // End
    expression.push(1.0.into());
    let end_color = if !segments.is_empty() { segments.last().unwrap().1.clone() } else { "#FFFFFF".to_string() };
    expression.push(end_color.into());

    Ok(serde_json::to_value(expression).unwrap())
}

/// Génère une expression de couleur filtrée pour n'afficher que les segments actifs
/// dans les zones de superposition
#[tauri::command]
pub async fn get_filtered_slope_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
    zone_id: Option<usize>,
    show_direction: Option<String>, // "aller" ou "retour"
) -> Result<serde_json::Value, String> {
    // Si pas de zone active, retourner l'expression normale
    if zone_id.is_none() || show_direction.is_none() {
        return get_slope_color_expression(state, circuit_id.clone(), slope_colors, segment_length, None, None).await;
    }

    let (_data_dir, metadata_path, tracking_path) = {
        let app_state = state.lock().unwrap();
        let data_dir = app_state.app_env_path.clone();
        let metadata_path = data_dir
            .join("data")
            .join(&circuit_id)
            .join("segments_metadata.json");
        let tracking_path = data_dir
            .join("data")
            .join(&circuit_id)
            .join("tracking.json");
        (data_dir, metadata_path, tracking_path)
    }; // app_state est libéré ici
    
    if !metadata_path.exists() {
        // Pas de métadonnées, retourner l'expression normale
        return get_slope_color_expression(state, circuit_id, slope_colors, segment_length, None, None).await;
    }

    let metadata_content = fs::read_to_string(metadata_path).map_err(|e| e.to_string())?;
    let metadata: crate::segment_analyzer::SegmentMetadata =
        serde_json::from_str(&metadata_content).map_err(|e| e.to_string())?;

    // Trouver la zone active
    let active_zone = metadata
        .overlapping_zones
        .iter()
        .find(|z| z.zone_id == zone_id.unwrap());

    if active_zone.is_none() {
        return get_slope_color_expression(state, circuit_id, slope_colors, segment_length, None, None).await;
    }

    let zone = active_zone.unwrap();
    let direction = show_direction.unwrap();

    // Charger les points de tracking
    
    let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<TrackingPoint> =
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

    if tracking_points.len() < 2 {
        return Ok(serde_json::json!(slope_colors
            .get("Tranche1")
            .cloned()
            .unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    // Calculer les pentes
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

    // Convertir en couleurs, mais rendre transparent UNIQUEMENT les segments
    // de la zone de superposition qui ne correspondent pas à la direction active
    let total_distance = (tracking_points.len() - 1) as f64 * segment_length;
    let colors: Vec<String> = slopes
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let segment_km = (i as f64 * segment_length) / 1000.0;
            
            // Vérifier si ce segment est dans la zone de superposition (aller OU retour)
            let is_in_overlap_zone = 
                (segment_km >= zone.aller_start_km && segment_km <= zone.aller_end_km) ||
                (segment_km >= zone.retour_start_km && segment_km <= zone.retour_end_km);
            
            if !is_in_overlap_zone {
                // Segment hors zone de superposition : toujours afficher normalement
                get_slope_color(s, &slope_colors)
            } else {
                // Segment dans la zone de superposition : vérifier la direction
                let is_active_direction = if direction == "aller" {
                    segment_km >= zone.aller_start_km && segment_km <= zone.aller_end_km
                } else {
                    segment_km >= zone.retour_start_km && segment_km <= zone.retour_end_km
                };
                
                if is_active_direction {
                    // Direction active : afficher normalement
                    get_slope_color(s, &slope_colors)
                } else {
                    // Direction inactive dans la zone : rendre transparent
                    "rgba(0, 0, 0, 0)".to_string()
                }
            }
        })
        .collect();

    if total_distance <= 0.0 {
        return Ok(serde_json::json!(colors
            .first()
            .cloned()
            .unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    // Construire l'expression Mapbox
    let mut expression: Vec<serde_json::Value> = vec![
        "interpolate".into(),
        vec!["linear"].into(),
        vec!["line-progress"].into(),
    ];

    let transition_length = 25.0; // 25m

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

    expression.push(1.0.into());
    expression.push(colors.last().unwrap().clone().into());

    Ok(serde_json::to_value(expression).unwrap())
}

/// Génère le gradient pour les segments HORS zones de superposition (trace-main)
#[tauri::command]
pub async fn get_main_segments_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    // Charger les métadonnées pour identifier les zones
    let (metadata_path, tracking_path) = {
        let app_state = state.lock().unwrap();
        let data_dir = app_state.app_env_path.clone();
        let metadata_path = data_dir.join("data").join(&circuit_id).join("segments_metadata.json");
        let tracking_path = data_dir.join("data").join(&circuit_id).join("tracking.json");
        (metadata_path, tracking_path)
    };

    // Si pas de métadonnées, retourner gradient complet
    if !metadata_path.exists() {
        return get_slope_color_expression(state, circuit_id, slope_colors, segment_length, None, None).await;
    }

    let metadata_content = fs::read_to_string(metadata_path).map_err(|e| e.to_string())?;
    let metadata: crate::segment_analyzer::SegmentMetadata =
        serde_json::from_str(&metadata_content).map_err(|e| e.to_string())?;

    // Charger tracking et calculer pentes
    let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<TrackingPoint> =
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

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

    // Rendre transparent les segments DANS les zones
    let total_distance = (tracking_points.len() - 1) as f64 * segment_length;
    let colors: Vec<String> = slopes
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let segment_km = (i as f64 * segment_length) / 1000.0;
            
            // Vérifier si dans une zone de superposition
            let in_overlap = metadata.overlapping_zones.iter().any(|zone| {
                (segment_km >= zone.aller_start_km && segment_km <= zone.aller_end_km) ||
                (segment_km >= zone.retour_start_km && segment_km <= zone.retour_end_km)
            });

            if in_overlap {
                "#808080".to_string() // Gris neutre dans zones
            } else {
                get_slope_color(s, &slope_colors) // Normal hors zones
            }
        })
        .collect();

    build_gradient_expression(colors, segment_length, total_distance)
}

/// Génère le gradient pour les segments ALLER des zones (trace-aller)
#[tauri::command]
pub async fn get_aller_segments_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    get_direction_segments_expression(state, circuit_id, slope_colors, segment_length, "aller").await
}

/// Génère le gradient pour les segments RETOUR des zones (trace-retour)
#[tauri::command]
pub async fn get_retour_segments_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    get_direction_segments_expression(state, circuit_id, slope_colors, segment_length, "retour").await
}

/// Génère le gradient pour les zones en gris neutre (trace-overlap-neutral)
#[tauri::command]
pub async fn get_neutral_overlap_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    let neutral_color = "#808080".to_string(); // Gris neutre
    
    let (metadata_path, tracking_path) = {
        let app_state = state.lock().unwrap();
        let data_dir = app_state.app_env_path.clone();
        let metadata_path = data_dir.join("data").join(&circuit_id).join("segments_metadata.json");
        let tracking_path = data_dir.join("data").join(&circuit_id).join("tracking.json");
        (metadata_path, tracking_path)
    };

    if !metadata_path.exists() {
        return Ok(serde_json::json!(neutral_color));
    }

    let metadata_content = fs::read_to_string(metadata_path).map_err(|e| e.to_string())?;
    let metadata: crate::segment_analyzer::SegmentMetadata =
        serde_json::from_str(&metadata_content).map_err(|e| e.to_string())?;

    let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<TrackingPoint> =
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

    let total_distance = (tracking_points.len() - 1) as f64 * segment_length;
    let colors: Vec<String> = (0..tracking_points.len() - 1)
        .map(|i| {
            let segment_km = (i as f64 * segment_length) / 1000.0;
            
            let in_overlap = metadata.overlapping_zones.iter().any(|zone| {
                (segment_km >= zone.aller_start_km && segment_km <= zone.aller_end_km) ||
                (segment_km >= zone.retour_start_km && segment_km <= zone.retour_end_km)
            });

            if in_overlap {
                neutral_color.clone() // Gris dans zones
            } else {
                "rgba(0, 0, 0, 0)".to_string() // Transparent hors zones
            }
        })
        .collect();

    build_gradient_expression(colors, segment_length, total_distance)
}

// Helper function pour générer gradient aller ou retour
async fn get_direction_segments_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
    direction: &str,
) -> Result<serde_json::Value, String> {
    let (metadata_path, tracking_path) = {
        let app_state = state.lock().unwrap();
        let data_dir = app_state.app_env_path.clone();
        let metadata_path = data_dir.join("data").join(&circuit_id).join("segments_metadata.json");
        let tracking_path = data_dir.join("data").join(&circuit_id).join("tracking.json");
        (metadata_path, tracking_path)
    };

    if !metadata_path.exists() {
        return Ok(serde_json::json!("rgba(0, 0, 0, 0)"));
    }

    let metadata_content = fs::read_to_string(metadata_path).map_err(|e| e.to_string())?;
    let metadata: crate::segment_analyzer::SegmentMetadata =
        serde_json::from_str(&metadata_content).map_err(|e| e.to_string())?;

    let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<TrackingPoint> =
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

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

    let total_distance = (tracking_points.len() - 1) as f64 * segment_length;
    let colors: Vec<String> = slopes
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let segment_km = (i as f64 * segment_length) / 1000.0;
            
            let in_direction_segment = metadata.overlapping_zones.iter().any(|zone| {
                if direction == "aller" {
                    segment_km >= zone.aller_start_km && segment_km <= zone.aller_end_km
                } else {
                    segment_km >= zone.retour_start_km && segment_km <= zone.retour_end_km
                }
            });

            if in_direction_segment {
                get_slope_color(s, &slope_colors)
            } else {
                // Vérifier si on est dans une zone d'overlap (quelconque)
                let in_any_overlap = metadata.overlapping_zones.iter().any(|zone| {
                    (segment_km >= zone.aller_start_km && segment_km <= zone.aller_end_km) ||
                    (segment_km >= zone.retour_start_km && segment_km <= zone.retour_end_km)
                });

                if in_any_overlap {
                    // Si on est dans un overlap mais pas le bon sens -> Gris (Blend avec Trace Complète)
                    "#808080".to_string()
                } else {
                    // Hors overlap -> Couleur normale
                    get_slope_color(s, &slope_colors)
                }
            }
        })
        .collect();

    build_gradient_expression(colors, segment_length, total_distance)
}

// Helper pour construire l'expression Mapbox
fn build_gradient_expression(
    colors: Vec<String>,
    segment_length: f64,
    total_distance: f64,
) -> Result<serde_json::Value, String> {
    if total_distance <= 0.0 {
        return Ok(serde_json::json!(colors.first().cloned().unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    let mut expression: Vec<serde_json::Value> = vec![
        "interpolate".into(),
        vec!["linear"].into(),
        vec!["line-progress"].into(),
    ];

    let transition_length = 25.0;

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

    expression.push(1.0.into());
    expression.push(colors.last().unwrap().clone().into());

    Ok(serde_json::to_value(expression).unwrap())
}

/// Helper pour construire l'expression Mapbox avec transitions différenciées (POC)
fn build_poc_gradient_expression(
    colors: Vec<String>,
    segment_length: f64,
    total_distance: f64,
) -> Result<serde_json::Value, String> {
    if total_distance <= 0.0 {
        return Ok(serde_json::json!(colors.first().cloned().unwrap_or_else(|| "#FFFFFF".to_string())));
    }

    let mut expression: Vec<serde_json::Value> = vec![
        "interpolate".into(),
        vec!["linear"].into(),
        vec!["line-progress"].into(),
    ];

    let slope_transition = 12.5; // 25m total pour les pentes
    let transparency_transition = 5.0; // 10m total pour la transparence
    let mut last_val = 0.0;

    expression.push(0.0.into());
    expression.push(colors[0].clone().into());

    for i in 1..colors.len() {
        let junction_dist = i as f64 * segment_length;
        let color_before = &colors[i - 1];
        let color_after = &colors[i];

        if color_before != color_after {
            // Déterminer la longueur de transition selon le type de changement
            let is_transparency_change = color_before == "rgba(0, 0, 0, 0)" || color_after == "rgba(0, 0, 0, 0)";
            let t_len = if is_transparency_change { transparency_transition } else { slope_transition };

            let transition_start_dist = junction_dist - t_len;
            let transition_end_dist = junction_dist + t_len;

            // Stop début transition
            let stop1 = (transition_start_dist / total_distance).max(last_val + 0.000001).min(1.0);
            expression.push(stop1.into());
            expression.push(color_before.clone().into());
            last_val = stop1;

            // Stop fin transition
            let stop2 = (transition_end_dist / total_distance).max(last_val + 0.000001).min(1.0);
            if stop2 < 1.0 {
                expression.push(stop2.into());
                expression.push(color_after.clone().into());
                last_val = stop2;
            } else {
                break;
            }
        }
    }

    expression.push(1.0.into());
    expression.push(colors.last().unwrap().clone().into());

    Ok(serde_json::to_value(expression).unwrap())
}

#[tauri::command]
pub async fn get_debug_full_aller_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    get_debug_direction_expression(state, circuit_id, slope_colors, segment_length, "aller").await
}

#[tauri::command]
pub async fn get_debug_full_retour_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    get_debug_direction_expression(state, circuit_id, slope_colors, segment_length, "retour").await
}

async fn get_debug_direction_expression(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
    direction: &str,
) -> Result<serde_json::Value, String> {
    let (metadata_path, tracking_path) = {
        let app_state = state.lock().unwrap();
        let data_dir = app_state.app_env_path.clone();
        let metadata_path = data_dir.join("data").join(&circuit_id).join("segments_metadata.json");
        let tracking_path = data_dir.join("data").join(&circuit_id).join("tracking.json");
        (metadata_path, tracking_path)
    };

    if !metadata_path.exists() {
        return Ok(serde_json::json!("rgba(0, 0, 0, 0)"));
    }

    let metadata_content = fs::read_to_string(metadata_path).map_err(|e| e.to_string())?;
    let metadata: crate::segment_analyzer::SegmentMetadata =
        serde_json::from_str(&metadata_content).map_err(|e| e.to_string())?;

    let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<TrackingPoint> =
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

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

    let total_distance = (tracking_points.len() - 1) as f64 * segment_length;
    let colors: Vec<String> = slopes
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let segment_km = (i as f64 * segment_length) / 1000.0;
            
            // Nouvelle logique simplifiée :
            // On ne masque QUE la portion opposée au sein d'une zone d'overlap.
            // Tout le reste (hors overlap ou portion spécifique de la direction) est affiché.
            let should_hide = metadata.overlapping_zones.iter().any(|zone| {
                if direction == "aller" {
                    // Dans le calque ALLER, on masque la portion RETOUR de l'overlap
                    segment_km >= zone.retour_start_km && segment_km <= zone.retour_end_km
                } else {
                    // Dans le calque RETOUR, on masque la portion ALLER de l'overlap
                    segment_km >= zone.aller_start_km && segment_km <= zone.aller_end_km
                }
            });

            if should_hide {
                "rgba(0, 0, 0, 0)".to_string()
            } else {
                get_slope_color(s, &slope_colors)
            }
        })
        .collect();

    build_poc_gradient_expression(colors, segment_length, total_distance)
}

/// Génère une FeatureCollection de segments individuels avec propriétés typées
/// Utilise la géométrie haute résolution de lineString.json découpée selon les segments tracking.json
#[tauri::command]
pub async fn get_colored_segments_geojson(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
    variant_id: Option<String>,
) -> Result<serde_json::Value, String> {
    let (metadata_path, tracking_path, linestring_path) = {
        let app_state = state.lock().unwrap();
        let data_dir = app_state.app_env_path.clone();
        
        let (meta_file, track_file, line_file) = if let Some(ref vid) = variant_id {
            (
                format!("segments_metadata_{}.json", vid),
                format!("tracking_{}_FULL.json", vid),
                format!("lineString_{}_FULL.json", vid)
            )
        } else {
            (
                "segments_metadata.json".to_string(),
                "tracking.json".to_string(),
                "lineString.json".to_string()
            )
        };

        let metadata_path = data_dir.join("data").join(&circuit_id).join(meta_file);
        let tracking_path = data_dir.join("data").join(&circuit_id).join(track_file);
        let linestring_path = data_dir.join("data").join(&circuit_id).join(line_file);
        (metadata_path, tracking_path, linestring_path)
    };

    if !tracking_path.exists() || !linestring_path.exists() {
        return Err(format!("Tracking or LineString file not found at {:?}", if !tracking_path.exists() { tracking_path } else { linestring_path }));
    }

    // Charger tracking (pour infos segments et pentes)
    let tracking_content = fs::read_to_string(&tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<crate::geo_processor::TrackingPointJs> =
        serde_json::from_str(&tracking_content).map_err(|e| e.to_string())?;

    // Charger LineString (pour géométrie haute résolution)
    let linestring_content = fs::read_to_string(linestring_path).map_err(|e| e.to_string())?;
    let linestring_json: serde_json::Value = 
        serde_json::from_str(&linestring_content).map_err(|e| e.to_string())?;
    
    // Extraire coordonnes brutes
    let raw_coords = match linestring_json.get("coordinates") {
        Some(c) => c.as_array().ok_or("Invalid coordinates array")?,
        None => match linestring_json.get("geometry").and_then(|g| g.get("coordinates")) {
            Some(c) => c.as_array().ok_or("Invalid coordinates in geometry")?,
            None => return Err("Could not find coordinates in lineString.json".to_string()),
        }
    };

    // Charger métadonnées si elles existent
    let metadata: Option<crate::segment_analyzer::SegmentMetadata> = if metadata_path.exists() {
        let content = fs::read_to_string(metadata_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).ok()
    } else {
        None
    };

    // Pré-calculer les distances cumulées pour la géométrie haute résolution
    let mut high_res_dists = Vec::with_capacity(raw_coords.len());
    high_res_dists.push(0.0);
    let mut total_high_res_dist = 0.0;
    
    for i in 0..raw_coords.len() - 1 {
        let p1_val = &raw_coords[i];
        let p2_val = &raw_coords[i+1];
        let p1 = Point::new(p1_val[0].as_f64().unwrap_or(0.0), p1_val[1].as_f64().unwrap_or(0.0));
        let p2 = Point::new(p2_val[0].as_f64().unwrap_or(0.0), p2_val[1].as_f64().unwrap_or(0.0));
        
        // Distance en mètres
        let dist = p1.haversine_distance(&p2);
        total_high_res_dist += dist;
        high_res_dists.push(total_high_res_dist);
    }

    let mut features = Vec::new();
    let mut current_coord_idx = 0;

    // Itérer sur les segments LOGIQUES (tracking)
    for i in 0..tracking_points.len() - 1 {
        let p1_track = &tracking_points[i];
        let p2_track = &tracking_points[i + 1];

        // Définir la fenêtre de distance pour ce segment
        let start_dist_m = i as f64 * segment_length;
        let end_dist_m = (i + 1) as f64 * segment_length;

        // Calcul pente
        let altitude_change = p2_track.altitude - p1_track.altitude;
        let slope = if segment_length > 0.0 {
            (altitude_change / segment_length) * 100.0
        } else {
            0.0
        };
        let color_raw = get_slope_color(slope, &slope_colors);

        // Déterminer le statut pour les variants (NEW, COMMON)
        let mut status = "COMMON";
        if let Some(ref t_type) = p1_track.type_troncon {
            if t_type == "Segment" || t_type == "Depart" || t_type == "Arrivee" || t_type == "Départ" || t_type == "Arrivée" {
                status = "NEW";
            }
        }

        // Déterminer le type de segment (Normal, Aller Overlap, Retour Overlap)
        let mut segment_type = "normal";
        if let Some(ref meta) = metadata {
            let mid_km = (start_dist_m + end_dist_m) / 2000.0;
            for zone in &meta.overlapping_zones {
                if mid_km >= zone.aller_start_km && mid_km <= zone.aller_end_km {
                    segment_type = "aller_overlap";
                    break;
                }
                if mid_km >= zone.retour_start_km && mid_km <= zone.retour_end_km {
                    segment_type = "retour_overlap";
                    break;
                }
            }
        }

        // Construire la géométrie détaillée
        let mut segment_coords = Vec::new();
        while current_coord_idx < high_res_dists.len() && high_res_dists[current_coord_idx] < start_dist_m {
            current_coord_idx += 1;
        }
        
        segment_coords.push(vec![p1_track.coordonnee[0], p1_track.coordonnee[1]]);
        let mut temp_idx = current_coord_idx;
        while temp_idx < high_res_dists.len() && high_res_dists[temp_idx] <= end_dist_m {
             let c = &raw_coords[temp_idx];
             segment_coords.push(vec![c[0].as_f64().unwrap_or(0.0), c[1].as_f64().unwrap_or(0.0)]);
             temp_idx += 1;
        }
        segment_coords.push(vec![p2_track.coordonnee[0], p2_track.coordonnee[1]]);

        let mut props = serde_json::json!({
            "color_raw": color_raw,
            "segment_type": segment_type,
            "index": i
        });

        if variant_id.is_some() {
            if let Some(obj) = props.as_object_mut() {
                obj.insert("status".to_string(), serde_json::json!(status));
            }
        }

        features.push(serde_json::json!({
            "type": "Feature",
            "geometry": { "type": "LineString", "coordinates": segment_coords },
            "properties": props
        }));
    }

    // --- LOGIQUE ABANDONED (Si variante active) ---
    if let Some(ref vid) = variant_id {
        let data_dir = state.lock().unwrap().app_env_path.clone().join("data").join(&circuit_id);
        let archive_path = data_dir.join(format!("variant_{}.json", vid));
        
        if archive_path.exists() {
            if let Ok(archive_content) = fs::read_to_string(&archive_path) {
                if let Ok(archive) = serde_json::from_str::<Archive>(&archive_content) {
                    // Charger Master Tracking et LineString
                    let master_track_path = data_dir.join("tracking.json");
                    let master_ls_path = data_dir.join("lineString.json");

                    if master_track_path.exists() && master_ls_path.exists() {
                        if let (Ok(track_c), Ok(ls_c)) = (fs::read_to_string(master_track_path), fs::read_to_string(master_ls_path)) {
                            if let (Ok(master_tracking), Ok(master_ls_json)) = (
                                serde_json::from_str::<Vec<crate::geo_processor::TrackingPointJs>>(&track_c),
                                serde_json::from_str::<serde_json::Value>(&ls_c)
                            ) {
                                // Coordonnées Master
                                let master_raw_coords = master_ls_json.get("coordinates")
                                    .or_else(|| master_ls_json.get("geometry").and_then(|g| g.get("coordinates")))
                                    .and_then(|c| c.as_array())
                                    .ok_or("Invalid master coordinates")?;

                                // Distances Master
                                let mut master_high_res_dists = Vec::with_capacity(master_raw_coords.len());
                                master_high_res_dists.push(0.0);
                                let mut total_m_dist = 0.0;
                                for i in 0..master_raw_coords.len() - 1 {
                                    let p1 = Point::new(master_raw_coords[i][0].as_f64().unwrap_or(0.0), master_raw_coords[i][1].as_f64().unwrap_or(0.0));
                                    let p2 = Point::new(master_raw_coords[i+1][0].as_f64().unwrap_or(0.0), master_raw_coords[i+1][1].as_f64().unwrap_or(0.0));
                                    total_m_dist += p1.haversine_distance(&p2);
                                    master_high_res_dists.push(total_m_dist);
                                }

                                // Identifier les plages abandonnées
                                for modif in archive.modifications {
                                    let (start_idx, end_idx) = match modif {
                                        VariantModification::DepartDeporte { anchor_index_on_master, .. } => (0, anchor_index_on_master),
                                        VariantModification::ArriveeReportee { anchor_index_on_master, .. } => (anchor_index_on_master, master_tracking.len() - 1),
                                        VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } => (anchor_start.index, anchor_end.index),
                                    };

                                    if start_idx >= end_idx || end_idx >= master_tracking.len() { continue; }

                                    let mut m_current_coord_idx = 0;
                                    for i in start_idx..end_idx {
                                        let p1 = &master_tracking[i];
                                        let p2 = &master_tracking[i+1];
                                        let s_dist = i as f64 * segment_length;
                                        let e_dist = (i + 1) as f64 * segment_length;

                                        let slope = if segment_length > 0.0 { ((p2.altitude - p1.altitude) / segment_length) * 100.0 } else { 0.0 };
                                        let color = get_slope_color(slope, &slope_colors);

                                        let mut s_coords = Vec::new();
                                        while m_current_coord_idx < master_high_res_dists.len() && master_high_res_dists[m_current_coord_idx] < s_dist {
                                            m_current_coord_idx += 1;
                                        }

                                        // Déterminer le type de segment master pour ABANDONED (optionnel mais propre)
                                        let m_segment_type = "normal";
                                        // On pourrait charger les métadonnées du maître ici aussi si besoin
                                        // Mais restons sur "normal" pour ABANDONED pour l'instant.

                                        s_coords.push(vec![p1.coordonnee[0], p1.coordonnee[1]]);
                                        let mut t_idx = m_current_coord_idx;
                                        while t_idx < master_high_res_dists.len() && master_high_res_dists[t_idx] <= e_dist {
                                            let c = &master_raw_coords[t_idx];
                                            s_coords.push(vec![c[0].as_f64().unwrap_or(0.0), c[1].as_f64().unwrap_or(0.0)]);
                                            t_idx += 1;
                                        }
                                        s_coords.push(vec![p2.coordonnee[0], p2.coordonnee[1]]);

                                        features.push(serde_json::json!({
                                            "type": "Feature",
                                            "geometry": { "type": "LineString", "coordinates": s_coords },
                                            "properties": {
                                                "color_raw": color,
                                                "segment_type": m_segment_type,
                                                "status": "ABANDONED",
                                                "index": i
                                            }
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(serde_json::json!({
        "type": "FeatureCollection",
        "features": features
    }))
}