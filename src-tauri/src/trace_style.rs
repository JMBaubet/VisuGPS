use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use tauri::State;
use geo::algorithm::haversine_distance::HaversineDistance;
use geo::Point;

#[derive(Deserialize, Debug)]
struct TrackingPoint {
    altitude: f64,
    coordonnee: [f64; 2],
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
        return get_slope_color_expression(state, circuit_id.clone(), slope_colors, segment_length).await;
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
        return get_slope_color_expression(state, circuit_id, slope_colors, segment_length).await;
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
        return get_slope_color_expression(state, circuit_id, slope_colors, segment_length).await;
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
        return get_slope_color_expression(state, circuit_id, slope_colors, segment_length).await;
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

/// Génère une FeatureCollection de segments individuels avec propriétés typées
/// Utilise la géométrie haute résolution de lineString.json découpée selon les segments tracking.json
#[tauri::command]
pub async fn get_colored_segments_geojson(
    state: State<'_, Mutex<crate::AppState>>,
    circuit_id: String,
    slope_colors: HashMap<String, String>,
    segment_length: f64,
) -> Result<serde_json::Value, String> {
    let (metadata_path, tracking_path, linestring_path) = {
        let app_state = state.lock().unwrap();
        let data_dir = app_state.app_env_path.clone();
        let metadata_path = data_dir.join("data").join(&circuit_id).join("segments_metadata.json");
        let tracking_path = data_dir.join("data").join(&circuit_id).join("tracking.json");
        let linestring_path = data_dir.join("data").join(&circuit_id).join("lineString.json");
        (metadata_path, tracking_path, linestring_path)
    };

    if !tracking_path.exists() || !linestring_path.exists() {
        return Err("Tracking or LineString file not found".to_string());
    }

    // Charger tracking (pour infos segments et pentes)
    let tracking_content = fs::read_to_string(tracking_path).map_err(|e| e.to_string())?;
    let tracking_points: Vec<TrackingPoint> =
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
        // On utilise i * segment_length comme approximation fiable de la progression
        let start_dist_m = i as f64 * segment_length;
        let end_dist_m = (i + 1) as f64 * segment_length;

        // Calcul pente (inchangé)
        let altitude_change = p2_track.altitude - p1_track.altitude;
        let slope = if segment_length > 0.0 {
            (altitude_change / segment_length) * 100.0
        } else {
            0.0
        };
        let color_raw = get_slope_color(slope, &slope_colors);

        // Déterminer info segment
        let segment_km = start_dist_m / 1000.0;
        let mut segment_type = "normal";
        let mut zone_id = -1;
        if let Some(ref meta) = metadata {
            for (idx, zone) in meta.overlapping_zones.iter().enumerate() {
                let mid_km = segment_km + (segment_length / 2000.0); // Check au milieu
                let is_aller = mid_km >= zone.aller_start_km && mid_km <= zone.aller_end_km;
                let is_retour = mid_km >= zone.retour_start_km && mid_km <= zone.retour_end_km;
                if is_aller { segment_type = "aller_overlap"; zone_id = idx as i32; break; }
                if is_retour { segment_type = "retour_overlap"; zone_id = idx as i32; break; }
            }
        }

        // Construire la géométrie détaillée pour ce segment
        let mut segment_coords = Vec::new();
        
        // Ajouter le point de départ théorique (ou le dernier point réel proche)
        // Pour éviter les trous, on commence par le point tracking exact ? 
        // Non, on utilise les points High Res inclus dans l'intervalle.
        
        // On inclut les points de lineString dont la distance cumulée est dans ]start, end]
        // On ajoute aussi p1_track au début si nécessaire pour coller ?
        // Mieux : On prend coord[current_coord_idx] comme départ si dispo.
        
        // Start Point (Approximation High Res)
        // On cherche le premier point LineString >= start_dist si on a perdu le fil
        while current_coord_idx < high_res_dists.len() && high_res_dists[current_coord_idx] < start_dist_m {
            current_coord_idx += 1;
        }
        
        // Ajouter le point précédent (interpolé) ou le point tracking pour pas avoir de trou au début
        segment_coords.push(p1_track.coordonnee);

        // Ajouter tous les points intermédiaires
        let mut temp_idx = current_coord_idx;
        while temp_idx < high_res_dists.len() && high_res_dists[temp_idx] <= end_dist_m {
             let c = &raw_coords[temp_idx];
             segment_coords.push([c[0].as_f64().unwrap_or(0.0), c[1].as_f64().unwrap_or(0.0)]);
             temp_idx += 1;
        }
        // Ne pas avancer current_coord_idx trop vite pour le prochain segment ?
        // Si on a consommé les points, le prochain segment commencera après.
        // Mais attention si segment_length est très grand et qu'on saute des points ? Non.
        
        // Ajouter le point de fin tracking pour fermer le segment proprement
        segment_coords.push(p2_track.coordonnee);

        let feature = serde_json::json!({
            "type": "Feature",
            "geometry": {
                "type": "LineString",
                "coordinates": segment_coords
            },
            "properties": {
                "color_raw": color_raw,
                "segment_type": segment_type,
                "zone_id": zone_id,
                "index": i
            }
        });
        features.push(feature);
    }

    Ok(serde_json::json!({
        "type": "FeatureCollection",
        "features": features
    }))
}