use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantPoint {
    pub lat: f64,
    pub lon: f64,
    pub alt: Option<f64>,
    #[serde(rename = "type")]
    pub point_type: Option<String>, // "start_point", "end_point", "waypoint"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum VariantModification {
    #[serde(rename = "DEPART_DEPORTE", rename_all = "camelCase")]
    DepartDeporte {
        #[serde(alias = "anchor_index_on_master")]
        anchor_index_on_master: usize,
        points: Vec<VariantPoint>, // Raw editing points
        #[serde(alias = "full_geometry", skip_serializing_if = "Option::is_none")]
        full_geometry: Option<Vec<VariantPoint>>, // Detailed route
        longueur: f64,
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_service: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_profile: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_status: Option<String>,
    },
    #[serde(rename = "ARRIVEE_REPORTEE", rename_all = "camelCase")]
    ArriveeReportee {
        #[serde(alias = "anchor_index_on_master")]
        anchor_index_on_master: usize,
        points: Vec<VariantPoint>, // Raw editing points
        #[serde(alias = "full_geometry", skip_serializing_if = "Option::is_none")]
        full_geometry: Option<Vec<VariantPoint>>, // Detailed route
        longueur: f64,
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_service: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_profile: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_status: Option<String>,
    },
    #[serde(rename = "SEGMENT_DEVIATION", rename_all = "camelCase")]
    SegmentDeviation {
        #[serde(alias = "anchor_start")]
        anchor_start: AnchorPoint,
        #[serde(alias = "anchor_end")]
        anchor_end: AnchorPoint,
        waypoints: Vec<VariantPoint>, // Intermediate editing points
        #[serde(alias = "full_geometry", skip_serializing_if = "Option::is_none")]
        full_geometry: Option<Vec<VariantPoint>>, // Detailed route
        longueur: f64,
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_service: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_profile: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        routing_status: Option<String>,
    },
}

impl VariantModification {
    pub fn get_start_anchor_index(&self) -> usize {
        match self {
            VariantModification::DepartDeporte { anchor_index_on_master: _, .. } => 0, // Virtual index 0 for start
            VariantModification::ArriveeReportee { anchor_index_on_master, .. } => *anchor_index_on_master,
            VariantModification::SegmentDeviation { anchor_start, .. } => anchor_start.index,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnchorPoint {
    pub index: usize,
    pub coords: [f64; 2],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantStats {
    pub total_distance: f64,
    pub total_ascent: f64,
    #[serde(alias = "master_distance", default)]
    pub master_distance: f64,
    #[serde(alias = "master_ascent", default)]
    pub master_ascent: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(alias = "creation_date")]
    pub creation_date: String,
    pub color: String,
    pub stats: VariantStats,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Archive {
    pub metadata: VariantMetadata,
    pub modifications: Vec<VariantModification>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateVariantRequest {
    circuit_id: String,
    metadata: VariantMetadata,
    modifications: Vec<VariantModification>,
}

// Elevation fetching is now handled by crate::elevation_provider

fn repair_variant_segment_altitude(
    points: &mut Vec<Vec<f64>>, 
    start_alt: f64, 
    end_alt: Option<f64>
) {
    // On ne répare que si TOUS les points sont à 0.0 (défaillance service)
    let all_zero = points.iter().all(|p| p[2] < 1.0);
    if !all_zero {
        return;
    }

    match end_alt {
        None => {
            // DEPART ou ARRIVEE : on plaque tout à l'altitude de l'ancre
            for p in points.iter_mut() {
                p[2] = start_alt;
            }
        },
        Some(target_alt) => {
            // SEGMENT : interpolation linéaire basée sur la distance cumulée
            let mut cumulative_dist = 0.0;
            let mut dists = Vec::with_capacity(points.len());
            dists.push(0.0);
            
            for i in 1..points.len() {
                let d = crate::gpx_processor::haversine_distance(
                    points[i-1][1], points[i-1][0],
                    points[i][1], points[i][0]
                );
                cumulative_dist += d;
                dists.push(cumulative_dist);
            }

            if cumulative_dist > 0.0 {
                let alt_diff = target_alt - start_alt;
                for (i, p) in points.iter_mut().enumerate() {
                    let ratio = dists[i] / cumulative_dist;
                    p[2] = start_alt + (alt_diff * ratio);
                }
            } else {
                for p in points.iter_mut() {
                    p[2] = start_alt;
                }
            }
        }
    }
}
async fn prepare_points_3d(points_raw: &Vec<VariantPoint>, segment_name: &str) -> Result<(Vec<Vec<f64>>, Option<String>), String> {
    let mut final_3d = Vec::new();
    let mut missing_alt_indices = Vec::new();
    let mut coords_for_fetch = Vec::new();

    for (i, p) in points_raw.iter().enumerate() {
        // Consider None or 0.0 as a candidate for fetching (if we want to be sure)
        // Note: 0.0 is technically valid (sea level), but often means "missing" in router outputs.
        if let Some(alt) = p.alt {
            if alt != 0.0 {
                final_3d.push(vec![p.lon, p.lat, alt]);
                continue;
            }
        }
        
        final_3d.push(vec![p.lon, p.lat, 0.0]); // Placeholder
        missing_alt_indices.push(i);
        coords_for_fetch.push([p.lon, p.lat]);
    }

    let mut warning = None;

    if !coords_for_fetch.is_empty() {
        match crate::elevation_provider::fetch_altitudes(&coords_for_fetch).await {
            Ok(fetched_alts) => {
                for (i, alt) in missing_alt_indices.iter().zip(fetched_alts.iter()) {
                    final_3d[*i][2] = *alt;
                }
            },
            Err(_) => {
                // FALLBACK: Si le service d'élévation est en panne, on continue avec 0.0 altitude.
                let msg = format!("Récupération des altitudes pour le segment <b>{}</b>, en échec !", segment_name);
                println!("{}", msg);
                warning = Some(msg);
                // On ne retourne pas d'erreur, on garde juste les points à 0.0 (qui seront réparés par la suite).
            }
        }
    }
    Ok((final_3d, warning))
}

async fn enhance_geojson_altitudes(geojson_str: String) -> (String, Option<String>) {
    let mut geojson: serde_json::Value = match serde_json::from_str(&geojson_str) {
        Ok(v) => v,
        Err(_) => return (geojson_str, None),
    };
    let mut warning = None;

    if let Some(coords) = geojson.get_mut("coordinates").and_then(|c| c.as_array_mut()) {
        let mut missing_indices = Vec::new();
        let mut coords_to_fetch = Vec::new();

        for (i, coord) in coords.iter().enumerate() {
            if let Some(c_arr) = coord.as_array() {
                let alt = if c_arr.len() >= 3 {
                    c_arr[2].as_f64().unwrap_or(0.0)
                } else {
                    0.0
                };

                if alt < 1.0 {
                    missing_indices.push(i);
                    coords_to_fetch.push([c_arr[0].as_f64().unwrap_or(0.0), c_arr[1].as_f64().unwrap_or(0.0)]);
                }
            }
        }

        if !coords_to_fetch.is_empty() {
            match crate::elevation_provider::fetch_altitudes(&coords_to_fetch).await {
                Ok(alts) => {
                    for (idx, alt) in missing_indices.iter().zip(alts.iter()) {
                        if let Some(c_arr) = coords[*idx].as_array_mut() {
                            if c_arr.len() < 3 {
                                c_arr.push(serde_json::json!(*alt));
                            } else {
                                c_arr[2] = serde_json::json!(*alt);
                            }
                        }
                    }
                }
                Err(_) => {
                    warning = Some("ALTITUDE_FETCH_ERROR".to_string());
                }
            }
        }

        // Réparation des trous par interpolation (utilisation simplifiée pour prévisu)
        let mut pts: Vec<Vec<f64>> = coords
            .iter()
            .map(|c| {
                let a = c.as_array().unwrap();
                vec![
                    a[0].as_f64().unwrap_or(0.0),
                    a[1].as_f64().unwrap_or(0.0),
                    if a.len() >= 3 { a[2].as_f64().unwrap_or(0.0) } else { 0.0 },
                ]
            })
            .collect();

        // Note: pour la prévisu, on ne dispose pas forcément des ancres de la trace maître ici
        // on fait donc une réparation basique au mieux.
        let mut i = 0;
        let total_len = pts.len();
        while i < total_len {
            if pts[i][2] < 1.0 {
                let mut j = i + 1;
                while j < total_len && pts[j][2] < 1.0 { j += 1; }
                let start_alt = if i > 0 { pts[i-1][2] } else if j < total_len { pts[j][2] } else { 0.0 };
                let end_alt_val = if j < total_len { pts[j][2] } else { start_alt };
                let gap = (j - i) as f64;
                let step = (end_alt_val - start_alt) / (gap + 1.0);
                for k in 0..(j - i) {
                    pts[i + k][2] = start_alt + step * (k as f64 + 1.0);
                }
                i = j;
            } else { i += 1; }
        }

        // Réinjection
        for (i, p) in pts.into_iter().enumerate() {
            coords[i] = serde_json::json!(p);
        }
    }

    (serde_json::to_string(&geojson).unwrap(), warning)
}

#[tauri::command]
pub async fn create_variant_files(
    app_handle: tauri::AppHandle,
    request: CreateVariantRequest,
) -> Result<Option<String>, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let circuit_data_dir = app_env_path.join("data").join(&request.circuit_id);
    if !circuit_data_dir.exists() {
        return Err("Circuit directory not found".to_string());
    }

    // Load smoothing settings
    let median_window = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.altitude_smoothing_median_window").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let avg_window = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.altitude_smoothing_avg_window").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let max_gradient = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.max_gradient_percent").and_then(|v| v.as_f64()).unwrap_or(40.0);
    let default_zoom = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Camera.parametres.Zoom").and_then(|v| v.as_f64()).unwrap_or(16.0);
    let default_pitch = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Camera.parametres.Pitch").and_then(|v| v.as_f64()).unwrap_or(60.0);

    // Load master tracking.json to copy anchor point data
    let tracking_master_path = circuit_data_dir.join("tracking.json");
    let tracking_master_content = fs::read_to_string(&tracking_master_path).map_err(|e| format!("Failed to read master tracking: {}", e))?;
    let tracking_master: Vec<serde_json::Value> = serde_json::from_str(&tracking_master_content).map_err(|e| format!("Failed to parse master tracking: {}", e))?;

    let mut warnings: Vec<String> = Vec::new();

    // Map: modification_index_original -> points_3d (déjà lissés et réparés)
    let mut geometry_cache: HashMap<usize, Vec<Vec<f64>>> = HashMap::new();

    // Process each modification for individual files
    for (index, modification) in request.modifications.iter().enumerate() {
        // Validation: Backwards segments are not allowed
        if let VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } = modification {
            if anchor_end.index <= anchor_start.index {
                return Err(format!("Le segment {} est invalide : l'ancre de fin (index {}) doit être après l'ancre de départ (index {})", 
                    index + 1, anchor_end.index, anchor_start.index));
            }
        }

        let (suffix, points_raw_opt, seg_name) = match modification {
            VariantModification::DepartDeporte { full_geometry, name, .. } => ("DEPART", full_geometry, name.clone().unwrap_or("Départ".to_string())),
            VariantModification::ArriveeReportee { full_geometry, name, .. } => ("ARRIVEE", full_geometry, name.clone().unwrap_or("Arrivée".to_string())),
            VariantModification::SegmentDeviation { full_geometry, name, .. } => ("SEGMENT", full_geometry, name.clone().unwrap_or(format!("Segment {}", index + 1)))
        };

        if let Some(points_raw) = points_raw_opt {
            if !points_raw.is_empty() {
                let suffix_full = match modification {
                    VariantModification::SegmentDeviation { .. } => format!("{}_{}", suffix, index),
                    _ => suffix.to_string() 
                };

                let (track_points_3d_raw, warning) = prepare_points_3d(points_raw, &seg_name).await?;
                
                if let Some(w) = warning {
                    warnings.push(w);
                }

                // Récupération des altitudes d'ancrage pour la réparation
                let (anchor_start_alt, anchor_end_alt) = match modification {
                    VariantModification::DepartDeporte { anchor_index_on_master, .. } => {
                        (tracking_master[*anchor_index_on_master]["altitude"].as_f64().unwrap_or(0.0), None)
                    },
                    VariantModification::ArriveeReportee { anchor_index_on_master, .. } => {
                        (tracking_master[*anchor_index_on_master]["altitude"].as_f64().unwrap_or(0.0), None)
                    },
                    VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } => {
                        (
                            tracking_master[anchor_start.index]["altitude"].as_f64().unwrap_or(0.0),
                            Some(tracking_master[anchor_end.index]["altitude"].as_f64().unwrap_or(0.0))
                        )
                    }
                };

                // 1. Appliquer le lissage géométrique (clean_altitude_data) ONLY on this new segment points
                let mut track_points_3d = crate::gpx_processor::clean_altitude_data(&track_points_3d_raw, median_window, avg_window, max_gradient);
                
                // 2. Appliquer la nouvelle logique de réparation des trous d'altitude (0.0)
                repair_variant_segment_altitude(&mut track_points_3d, anchor_start_alt, anchor_end_alt);

                // Mettre en cache pour la reconstruction FULL plus tard
                geometry_cache.insert(index, track_points_3d.clone());

                // Write LineString file
                let linestring_filename = format!("lineString_{}_{}.json", request.metadata.id, suffix_full);
                let linestring_path = circuit_data_dir.join(&linestring_filename);
                let linestring_json = serde_json::json!({
                    "type": "LineString",
                    "coordinates": track_points_3d
                });
                fs::write(&linestring_path, serde_json::to_string_pretty(&linestring_json).unwrap()).map_err(|e| e.to_string())?;

                // Generate Tracking file
                let tracking_filename = format!("tracking_{}_{}.json", request.metadata.id, suffix_full);
                
                let segment_length = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(100.0);

                // Logic for nbrSegment according to user rules
                let (override_first, override_last) = match modification {
                    VariantModification::DepartDeporte { anchor_index_on_master, .. } => {
                        // Calculate geometric length to estimate nbr segments
                        let mut length_m = 0.0;
                        for i in 0..track_points_3d.len().saturating_sub(1) {
                            length_m += crate::gpx_processor::haversine_distance(
                                track_points_3d[i][1], track_points_3d[i][0],
                                track_points_3d[i+1][1], track_points_3d[i+1][0]
                            );
                        }
                        
                        let mut segments_count = (length_m / segment_length) as u32;
                        if (length_m % segment_length) > 1.0 {
                            segments_count += 1;
                        }

                        let mut first_ovr = serde_json::json!({
                            "nbrSegment": segments_count
                        });
                        
                        // Copy camera parameters from master anchor to variant start
                        if let Some(anchor_tp) = tracking_master.get(*anchor_index_on_master) {
                            if let Some(obj) = first_ovr.as_object_mut() {
                                for field in ["zoom", "pitch", "cap", "editedZoom", "editedPitch", "editedCap"] {
                                    if let Some(val) = anchor_tp.get(field) {
                                        obj.insert(field.to_string(), val.clone());
                                    }
                                }
                            }
                        }
                        
                        (Some(first_ovr), tracking_master.get(*anchor_index_on_master).cloned())
                    },
                    VariantModification::ArriveeReportee { anchor_index_on_master, .. } => {
                        let mut first_ovr = tracking_master.get(*anchor_index_on_master).cloned().unwrap_or(serde_json::json!({}));
                        // Arrivee: nbrSegment from anchor to master end
                        let nbr = (tracking_master.len() as u32).saturating_sub(*anchor_index_on_master as u32).saturating_sub(1);
                        first_ovr["nbrSegment"] = serde_json::json!(nbr);
                        (Some(first_ovr), None)
                    },
                    VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } => {
                        let mut first_ovr = tracking_master.get(anchor_start.index).cloned().unwrap_or(serde_json::json!({}));
                        
                        // Segment: nbrSegment from anchor_start to next pointDeControl on master
                        let mut next_cp_idx = anchor_start.index + 1;
                        while next_cp_idx < tracking_master.len() {
                            if tracking_master[next_cp_idx]["pointDeControl"].as_bool().unwrap_or(false) {
                                break;
                            }
                            next_cp_idx += 1;
                        }
                        let nbr = next_cp_idx.saturating_sub(anchor_start.index) as u32;
                        first_ovr["nbrSegment"] = serde_json::json!(nbr);
                        
                        (Some(first_ovr), tracking_master.get(anchor_end.index).cloned())
                    }
                };

                let points_context: Vec<crate::tracking_processor::PointContext> = track_points_3d.iter().enumerate().map(|(i, p)| {
                    crate::tracking_processor::PointContext {
                        coords: p.clone(),
                        is_anchor: i == 0 || i == track_points_3d.len() - 1,
                        type_troncon: Some(seg_name.clone()),
                    }
                }).collect();

                crate::tracking_processor::generate_tracking_file(&app_env_path, &request.circuit_id, &points_context, &settings, Some(&tracking_filename), override_first, override_last)?;
            }
        }
    }

    // --- Statistics Calculation (Full Trace Reconstruction) ---
    let master_ls_path = circuit_data_dir.join("lineString.json");
    let master_ls_content = fs::read_to_string(&master_ls_path).map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let master_ls: serde_json::Value = serde_json::from_str(&master_ls_content).map_err(|e| format!("Failed to parse lineString.json: {}", e))?;
    let master_coords = master_ls["coordinates"].as_array().ok_or("Invalid lineString format")?;
    let master_points_high_res: Vec<Vec<f64>> = master_coords.iter().map(|c| c.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();

    let total_tracking_pts = tracking_master.len();
    let total_high_res_pts = master_points_high_res.len();

    // Topological matching
    let find_corresponding_idx = |lon: f64, lat: f64, tracking_idx: usize, start_search_from: usize| -> usize {
        let ratio = tracking_idx as f64 / total_tracking_pts as f64;
        let estimated_idx = (ratio * total_high_res_pts as f64) as usize;
        let window_size = (total_high_res_pts / 20).max(500); 
        let min_search = estimated_idx.saturating_sub(window_size).max(start_search_from);
        let max_search = (estimated_idx + window_size).min(total_high_res_pts);
        let mut min_dist = f64::MAX;
        let mut best_idx = start_search_from;
        for i in min_search..max_search {
            let mp = &master_points_high_res[i];
            let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
            if d < min_dist { min_dist = d; best_idx = i; }
        }
        best_idx
    };

    let mut final_points_ctx: Vec<crate::tracking_processor::PointContext> = Vec::new();
    let mut current_master_idx = 0;
    
    // On crée une liste triée des modifications avec leur index original pour piocher dans le cache
    let mut indexed_mods: Vec<(usize, &VariantModification)> = request.modifications.iter().enumerate().collect();
    indexed_mods.sort_by_key(|(_, m)| m.get_start_anchor_index());
    
    let mut has_arrivee_reportee = false;

    for &(original_idx, modification) in &indexed_mods {
        match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, .. } => {
                if let Some(repaired_pts) = geometry_cache.get(&original_idx) {
                    let num_pts = repaired_pts.len();
                    for (i, p) in repaired_pts.iter().enumerate() {
                        let is_last = i == num_pts - 1;
                        final_points_ctx.push(crate::tracking_processor::PointContext {
                            coords: p.clone(),
                            is_anchor: i == 0 || is_last,
                            type_troncon: Some(if is_last { "Commun".to_string() } else { "Départ".to_string() }),
                        });
                    }
                }
                let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                current_master_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, 0);
                current_master_idx += 1;
            },
            VariantModification::SegmentDeviation { anchor_start, anchor_end, .. } => {
                let start_idx = find_corresponding_idx(anchor_start.coords[0], anchor_start.coords[1], anchor_start.index, current_master_idx);
                
                if start_idx >= current_master_idx {
                    for i in current_master_idx..=start_idx {
                        if i < master_points_high_res.len() {
                            let is_anchor = i == start_idx;
                            final_points_ctx.push(crate::tracking_processor::PointContext {
                                coords: master_points_high_res[i].clone(),
                                is_anchor,
                                type_troncon: Some(if is_anchor { "Segment".to_string() } else { "Commun".to_string() }),
                            });
                        }
                    }
                }
                current_master_idx = start_idx + 1;

                if let Some(repaired_pts) = geometry_cache.get(&original_idx) {
                    let mut pts = repaired_pts.clone();
                    if !pts.is_empty() { pts.remove(0); } // Remove redundant anchor
                    let num_pts = pts.len();
                    for (i, p) in pts.into_iter().enumerate() {
                         let is_last = i == num_pts - 1;
                         final_points_ctx.push(crate::tracking_processor::PointContext {
                            coords: p,
                            is_anchor: is_last,
                            type_troncon: Some(if is_last { "Commun".to_string() } else { "Segment".to_string() }),
                        });
                    }
                }
                
                let end_idx = find_corresponding_idx(anchor_end.coords[0], anchor_end.coords[1], anchor_end.index, current_master_idx);
                current_master_idx = end_idx + 1;
            },
            VariantModification::ArriveeReportee { anchor_index_on_master, .. } => {
                has_arrivee_reportee = true;
                let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                let arrivee_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, current_master_idx);
                
                 if arrivee_idx >= current_master_idx {
                    for i in current_master_idx..=arrivee_idx {
                        if i < master_points_high_res.len() {
                            let is_anchor = i == arrivee_idx;
                            final_points_ctx.push(crate::tracking_processor::PointContext {
                                coords: master_points_high_res[i].clone(),
                                is_anchor,
                                type_troncon: Some(if is_anchor { "Arrivée".to_string() } else { "Commun".to_string() }),
                            });
                        }
                    }
                }
                
                if let Some(repaired_pts) = geometry_cache.get(&original_idx) {
                    let mut pts = repaired_pts.clone();
                    if !pts.is_empty() { pts.remove(0); }
                    for p in pts {
                        final_points_ctx.push(crate::tracking_processor::PointContext {
                            coords: p,
                            is_anchor: false, 
                            type_troncon: Some("Arrivée".to_string()),
                        });
                    }
                }
                current_master_idx = master_points_high_res.len();
            }
        }
    }

    if !has_arrivee_reportee {
        while current_master_idx < master_points_high_res.len() {
            final_points_ctx.push(crate::tracking_processor::PointContext {
                coords: master_points_high_res[current_master_idx].clone(),
                is_anchor: false,
                type_troncon: Some("Commun".to_string()),
            });
            current_master_idx += 1;
        }
    }

    // On récupère les coordonnées finales assemblées. 
    // IMPORTANT : On ne fait plus de clean_altitude_data global ici ni de réparation globale.
    // Les segments nouveaux sont déjà réparés et lissés, et les parties master sont conservées telles quelles.
    let assembled_coords: Vec<Vec<f64>> = final_points_ctx.iter().map(|ctx| ctx.coords.clone()).collect();
    
    let lissage_dist = crate::get_setting_value(&settings, "data.groupes.Importation.parametres.denivele_lissage_distance").and_then(|v| v.as_f64()).unwrap_or(10.0);
    let variant_stats = crate::gpx_processor::calculate_track_stats(&assembled_coords, lissage_dist);

    let mut metadata = request.metadata;
    metadata.stats.total_distance = variant_stats.total_distance_km;
    metadata.stats.total_ascent = variant_stats.positive_elevation_m as f64;
    
    // Compute global status for the metadata
    let mut global_status_str = "SUCCESS".to_string();
    let mut has_alt_fail = false;
    for m in &request.modifications {
        let rs = match m {
            VariantModification::DepartDeporte { routing_status, .. } => routing_status,
            VariantModification::ArriveeReportee { routing_status, .. } => routing_status,
            VariantModification::SegmentDeviation { routing_status, .. } => routing_status,
        };
        if let Some(s) = rs {
            if s == "ROUTE_FAIL" {
                global_status_str = "ROUTE_FAIL".to_string();
                break;
            } else if s == "ALT_FAIL" {
                has_alt_fail = true;
            }
        }
    }
    if global_status_str != "ROUTE_FAIL" && has_alt_fail {
        global_status_str = "ALT_FAIL".to_string();
    }
    metadata.global_status = Some(global_status_str);
    
    // Concaténer les avertissements s'il y en a
    let final_warning = if warnings.is_empty() {
        None
    } else {
        Some(warnings.join("\n"))
    };
    
    let circuits_file = crate::read_circuits_file(&app_env_path)?;
    let master_circuit = circuits_file.circuits.iter().find(|c| c.circuit_id == request.circuit_id).ok_or_else(|| format!("Master circuit {} not found", request.circuit_id))?;
    metadata.stats.master_distance = master_circuit.distance_km;
    metadata.stats.master_ascent = master_circuit.denivele_m as f64;

    // 1. Sauvegarder lineString_FULL.json
    let full_linestring_path = circuit_data_dir.join(format!("lineString_{}_FULL.json", metadata.id));
    let full_linestring = serde_json::json!({
        "type": "LineString",
        "coordinates": assembled_coords
    });
    fs::write(&full_linestring_path, serde_json::to_string_pretty(&full_linestring).unwrap())
        .map_err(|e| format!("Failed to write lineString_FULL: {}", e))?;
    
    let cleaned_coords = assembled_coords; // Pour la suite
    
    // ========== GÉNÉRATION DU TRACKING PAR ASSEMBLAGE (SANS RÉ-ÉCHANTILLONNAGE GLOBAL) ==========
    
    let full_tracking_filename = format!("tracking_{}_FULL.json", metadata.id);
    let full_tracking_path = circuit_data_dir.join(&full_tracking_filename);

    println!("[VariantGen] Start stitching for variant {}. Master points: {}", metadata.id, tracking_master.len());

    let mut full_tracking_points: Vec<serde_json::Value> = Vec::new();
    let mut current_master_tp_idx = 0;
    let segment_length = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);

    // Lissage de la jonction d'altitude sur une distance max
    let smooth_junction = |points: &mut Vec<serde_json::Value>, target_alt: f64, max_dist: f64| {
        if points.is_empty() { return; }
        // On lisse vers l'arrière : on veut que le DEBUT du segment rejoigne target_alt
        // Mais target_alt est l'altitude du point PRECEDENT le segment.
        // Donc on veut que points[0] soit proche de target_alt.
        
        let start_alt_val = points[0]["altitude"].as_f64().unwrap_or(target_alt);
        let diff = target_alt - start_alt_val; // Ce qu'il faut ajouter pour atteindre la cible
        
        if diff.abs() < 1.0 { return; } // Pas besoin de lisser si < 1m

        let mut dist_acc = 0.0;
        for i in 0..points.len() {
            if i > 0 {
                let p1 = points[i-1]["coordonnee"].as_array().unwrap();
                let p2 = points[i]["coordonnee"].as_array().unwrap();
                dist_acc += crate::gpx_processor::haversine_distance(
                    p1[1].as_f64().unwrap(), p1[0].as_f64().unwrap(),
                    p2[1].as_f64().unwrap(), p2[0].as_f64().unwrap()
                );
            }
            
            if dist_acc > max_dist { break; }
            
            let factor = 1.0 - (dist_acc / max_dist); // 1.0 au début (impact max), 0.0 à la fin
            
            if let Some(obj) = points[i].as_object_mut() {
                 if let Some(current_alt) = obj.get("altitude").and_then(|a| a.as_f64()) {
                     obj.insert("altitude".to_string(), serde_json::json!(current_alt + (diff * factor)));
                 }
            }
        }
    };

    for (mod_idx, &(_original_idx, modification)) in indexed_mods.iter().enumerate() {
        match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, full_geometry, name, .. } => {
                let seg_name = name.clone().unwrap_or("Départ".to_string());
                println!("[VariantGen] Mod {}: DEPART_DEPORTE at Master index {}", mod_idx, anchor_index_on_master);
                if let Some(geom) = full_geometry {
                    let var_tracking = generate_interpolated_tracking(&geom, segment_length, default_zoom, default_pitch);
                    println!("   -> Added {} variant tracking points", var_tracking.len());
                    
                    let mut var_tracking_transformed = var_tracking;
                    for pt in var_tracking_transformed.iter_mut() {
                        if let Some(obj) = pt.as_object_mut() {
                            obj.insert("typeTroncon".to_string(), serde_json::json!(seg_name));
                        }
                    }

                    full_tracking_points.extend(var_tracking_transformed);
                }
                // On reprend au point SUIVANT l'ancre sur le master
                current_master_tp_idx = *anchor_index_on_master + 1;
            },
            VariantModification::SegmentDeviation { anchor_start, anchor_end, full_geometry, name, .. } => {
                println!("[VariantGen] Mod {}: SEGMENT_DEVIATION from {} to {}", mod_idx, anchor_start.index, anchor_end.index);
                
                // 1. Commun : On copie de l'index actuel jusqu'à l'ancre de départ
                let mut added_common = 0;
                for i in current_master_tp_idx..=anchor_start.index {
                    if i < tracking_master.len() {
                        let mut pt = tracking_master[i].clone();
                        if i == anchor_start.index {
                            if let Some(obj) = pt.as_object_mut() {
                                obj.insert("isAnchorPoint".to_string(), serde_json::json!(true));
                                let seg_name = name.clone().unwrap_or(format!("Segment {}", mod_idx + 1));
                                obj.insert("typeTroncon".to_string(), serde_json::json!(seg_name));
                            }
                        }
                        full_tracking_points.push(pt);
                        added_common += 1;
                    }
                }
                println!("   -> Added {} common points from master", added_common);

                // 2. Déviation : Nouveaux points
                if let Some(geom) = full_geometry {
                    let mut var_tracking = generate_interpolated_tracking(&geom, segment_length, default_zoom, default_pitch);
                    println!("   -> Added {} variant deviation points", var_tracking.len());

                    let seg_name = name.clone().unwrap_or(format!("Segment {}", mod_idx + 1));

                    // LISSAGE : On récupère l'altitude du dernier point commun (l'ancre)
                    if let Some(last_common) = full_tracking_points.last() {
                         let anchor_alt = last_common["altitude"].as_f64().unwrap_or(0.0);
                         smooth_junction(&mut var_tracking, anchor_alt, 500.0); // Lissage sur 500m
                    }
                    
                    // On ignore le premier point qui est déjà présent (l'ancre Master)
                    if var_tracking.len() > 1 {
                        let mut segments_to_add = var_tracking.into_iter().skip(1).collect::<Vec<serde_json::Value>>();
                        for pt in segments_to_add.iter_mut() {
                            if let Some(obj) = pt.as_object_mut() {
                                obj.insert("typeTroncon".to_string(), serde_json::json!(seg_name));
                            }
                        }
                        full_tracking_points.extend(segments_to_add);
                    }
                }

                // Pour la prochaine section, on reprend juste APRÈS l'ancre de fin
                current_master_tp_idx = anchor_end.index + 1;
            },
            VariantModification::ArriveeReportee { anchor_index_on_master, full_geometry, name, .. } => {
                println!("[VariantGen] Mod {}: ARRIVEE_REPORTEE at Master index {}", mod_idx, anchor_index_on_master);
                
                // 1. Commun jusqu'à l'ancre
                let mut added_common = 0;
                for i in current_master_tp_idx..=*anchor_index_on_master {
                    if i < tracking_master.len() {
                        let mut pt = tracking_master[i].clone();
                        if i == *anchor_index_on_master {
                            if let Some(obj) = pt.as_object_mut() {
                                obj.insert("isAnchorPoint".to_string(), serde_json::json!(true));
                                obj.insert("typeTroncon".to_string(), serde_json::json!("Arrivée"));
                            }
                        }
                        full_tracking_points.push(pt);
                        added_common += 1;
                    }
                }
                println!("   -> Added {} common points before arrival", added_common);

                // 2. Nouvelle Arrivée
                if let Some(geom) = full_geometry {
                    let mut var_tracking = generate_interpolated_tracking(&geom, segment_length, default_zoom, default_pitch);
                    println!("   -> Added {} variant arrival points", var_tracking.len());

                    let seg_name = name.clone().unwrap_or("Arrivée".to_string());

                    // LISSAGE : On récupère l'altitude du dernier point commun (l'ancre)
                    if let Some(last_common) = full_tracking_points.last() {
                         let anchor_alt = last_common["altitude"].as_f64().unwrap_or(0.0);
                         smooth_junction(&mut var_tracking, anchor_alt, 500.0); // Lissage sur 500m
                    }

                    if var_tracking.len() > 1 {
                        let mut segments_to_add = var_tracking.into_iter().skip(1).collect::<Vec<serde_json::Value>>();
                        for pt in segments_to_add.iter_mut() {
                            if let Some(obj) = pt.as_object_mut() {
                                obj.insert("typeTroncon".to_string(), serde_json::json!(seg_name));
                            }
                        }
                        full_tracking_points.extend(segments_to_add);
                    }
                }
                // L'arrivée termine la trace
                current_master_tp_idx = tracking_master.len();
            }
        }
    }

    // On complète avec le reste du Master si on n'a pas atteint la fin
    if current_master_tp_idx < tracking_master.len() {
        let remaining = tracking_master.len() - current_master_tp_idx;
        println!("[VariantGen] Adding remaining {} points from master", remaining);
        for i in current_master_tp_idx..tracking_master.len() {
            full_tracking_points.push(tracking_master[i].clone());
        }
    }

    println!("[VariantGen] Final points count: {}. Recalculating distances...", full_tracking_points.len());

    // Finalisation : Calculer distances basées sur l'index (Règle métier : 1 point = 100m)
    // Sauf potentiellement le dernier segment qui peut être plus court, mais pour l'affichage global
    // la convention index * 100m est la plus cohérente avec la construction du fichier.
    let num_total_tp = full_tracking_points.len();
    
    let mut prev_coords: Option<Vec<f64>> = None;
    
    for (i, tp) in full_tracking_points.iter_mut().enumerate() {
        if let Some(obj) = tp.as_object_mut() {
            obj.insert("increment".to_string(), serde_json::json!(i));
            
            // Règle : Distance = Index * 100m pour tous les segments standards
            // SAUF pour le tout dernier point qui contient le reliquat (< 100m)
            let dist_km = if i == num_total_tp - 1 && i > 0 {
                // Cas spécifique du dernier point : on ajoute la distance réelle du dernier segment
                let prev_dist_km = (i - 1) as f64 * 0.1;
                
                let p_curr = obj["coordonnee"].as_array().unwrap();
                let prev_c = prev_coords.as_ref().unwrap();
                
                let lat1 = prev_c[1];
                let lon1 = prev_c[0];
                let lat2 = p_curr[1].as_f64().unwrap();
                let lon2 = p_curr[0].as_f64().unwrap();
                
                let last_segment_km = crate::gpx_processor::haversine_distance(lat1, lon1, lat2, lon2) / 1000.0;
                prev_dist_km + last_segment_km
            } else {
                // Cas standard : multiples de 100m
                i as f64 * 0.1
            };
            
            obj.insert("distance".to_string(), serde_json::json!(dist_km));
            
            // Synchronisation de l'altitude avec cleaned_coords (haute résolution)
            let current_coords = obj["coordonnee"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect::<Vec<f64>>();
            let lon = current_coords[0];
            let lat = current_coords[1];
            
            // Recherche globale brute-force mais 100% fiable
            let original_alt = obj.get("altitude").and_then(|a| a.as_f64()).unwrap_or(0.0);
            let mut best_alt = original_alt;
            let mut min_d = f64::MAX;
            
            for coords in &cleaned_coords {
                let d = crate::gpx_processor::haversine_distance(lat, lon, coords[1], coords[0]);
                if d < min_d {
                    min_d = d;
                    best_alt = coords[2];
                    if d < 1.0 { break; } // Optimisation : arrêt immédiat si très proche
                }
            }

            // On accepte l'altitude trouvée si elle est "raisonnablement" proche géographiquement (< 100m)
            // Sinon on garde l'originale (il vaut mieux 0.0 que l'altitude d'un point à 10km)
            if min_d < 100.0 {
                obj.insert("altitude".to_string(), serde_json::json!(best_alt));
            } else {
                 obj.insert("altitude".to_string(), serde_json::json!(original_alt));
            }
            prev_coords = Some(vec![lon, lat]);
        }
    }

    // 🔴 REPARATION FINALE : Interpolation des trous d'altitude (0.0)
    // Elle ne devrait plus rien avoir à faire si tout a été réparé proprement en haut, 
    // mais on la laisse en sécurité ultra-minimale (interpolation simple).
    let total_len = full_tracking_points.len();
    if total_len > 0 {
        let mut i = 0;
        while i < total_len {
            if full_tracking_points[i]["altitude"].as_f64().unwrap_or(0.0) < 1.0 {
                let mut j = i + 1;
                while j < total_len && full_tracking_points[j]["altitude"].as_f64().unwrap_or(0.0) < 1.0 { j += 1; }
                let start_alt = if i > 0 { full_tracking_points[i-1]["altitude"].as_f64().unwrap_or(0.0) } else if j < total_len { full_tracking_points[j]["altitude"].as_f64().unwrap_or(0.0) } else { 0.0 };
                let end_alt_val = if j < total_len { full_tracking_points[j]["altitude"].as_f64().unwrap_or(0.0) } else { start_alt };
                let gap = (j - i) as f64;
                let step = (end_alt_val - start_alt) / (gap + 1.0);
                for k in 0..(j - i) {
                    if let Some(obj) = full_tracking_points[i + k].as_object_mut() {
                        obj.insert("altitude".to_string(), serde_json::json!(start_alt + step * (k as f64 + 1.0)));
                    }
                }
                i = j;
            } else { i += 1; }
        }
    }

    // Sauvegarde finale du tracking_FULL enrichi
    fs::write(&full_tracking_path, serde_json::to_string_pretty(&full_tracking_points).unwrap())
        .map_err(|e| format!("Failed to write stitched tracking_FULL: {}", e))?;

    // 🔴 NOUVEAU: Recalculer nbrSegment pour tous les points de contrôle du variant
    // On considère comme point de contrôle :
    // - Les points hérités du master qui étaient pointDeControl
    // - Tous les points identifiés comme isAnchorPoint (jonctions)
    // - Le tout premier et le tout dernier point
    let num_pts = full_tracking_points.len();
    let mut control_point_indices: Vec<usize> = Vec::new();
    
    for i in 0..num_pts {
        let is_cp = full_tracking_points[i]["pointDeControl"].as_bool().unwrap_or(false);
        let is_anchor = full_tracking_points[i]["isAnchorPoint"].as_bool().unwrap_or(false);
        if i == 0 || i == num_pts - 1 || is_cp || is_anchor {
            control_point_indices.push(i);
            // On s'assure que le champ pointDeControl est bien à true pour le frontend
            if let Some(obj) = full_tracking_points[i].as_object_mut() {
                obj.insert("pointDeControl".to_string(), serde_json::json!(true));
            }
        }
    }

    // Calculer le nbrSegment entre chaque CP successif
    for k in 0..control_point_indices.len().saturating_sub(1) {
        let current_cp_idx = control_point_indices[k];
        let next_cp_idx = control_point_indices[k+1];
        let diff = (next_cp_idx - current_cp_idx) as u32;
        
        if let Some(obj) = full_tracking_points[current_cp_idx].as_object_mut() {
            obj.insert("nbrSegment".to_string(), serde_json::json!(diff));
        }
    }
    
    // Le dernier point a toujours 0 segment devant lui
    if let Some(&last_idx) = control_point_indices.last() {
        if let Some(obj) = full_tracking_points[last_idx].as_object_mut() {
            obj.insert("nbrSegment".to_string(), serde_json::json!(0));
        }
    }

    // Sauvegarder le tracking_FULL enrichi avec les données caméra master
    fs::write(&full_tracking_path, serde_json::to_string_pretty(&full_tracking_points).unwrap())
        .map_err(|e| format!("Failed to write camera-enriched tracking_FULL: {}", e))?;
    
    // 4. Générer segments_metadata (Détection des overlaps)
    // 🔴 CORRECTION: Utiliser lineString_FULL (haute résolution) au lieu de tracking_FULL
    // Cela permet d'avoir des index de métadonnées cohérents avec la géométrie brute, comme pour la trace principale.
    let distance_threshold = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.seuilDetectionSuperposition")
        .and_then(|v| v.as_f64())
        .unwrap_or(20.0); // 20 mètres par défaut

    let mut high_res_points: Vec<serde_json::Value> = Vec::new();
    let mut cumulative_dist = 0.0;
    
    // On extrait les coordonnées de full_linestring (déjà calculé plus haut)
    if let Some(coords) = full_linestring.get("coordinates").and_then(|c| c.as_array()) {
        for (i, coord) in coords.iter().enumerate() {
            let lat = coord[1].as_f64().unwrap_or(0.0);
            let lon = coord[0].as_f64().unwrap_or(0.0);
            
            if i > 0 {
                let prev = &coords[i-1];
                let dist_m = crate::segment_analyzer::haversine_distance(prev[1].as_f64().unwrap_or(0.0), prev[0].as_f64().unwrap_or(0.0), lat, lon);
                cumulative_dist += dist_m / 1000.0;
            }
            
            high_res_points.push(serde_json::json!({
                "coordonnee": [lon, lat],
                "distance": cumulative_dist
            }));
        }
    }

    let overlapping_zones = crate::segment_analyzer::detect_overlapping_segments(
        &high_res_points,
        distance_threshold
    )?;
    
    let segments_metadata = crate::segment_analyzer::SegmentMetadata {
        circuit_id: request.circuit_id.clone(),
        overlapping_zones,
        detection_threshold_meters: distance_threshold,
        total_points: high_res_points.len(),
        analysis_date: chrono::Utc::now().to_rfc3339(),
    };
    
    let metadata_path = circuit_data_dir.join(format!("segments_metadata_{}.json", metadata.id));
    fs::write(&metadata_path, serde_json::to_string_pretty(&segments_metadata).unwrap())
        .map_err(|e| format!("Failed to write segments_metadata: {}", e))?;
    
    let archive_path = circuit_data_dir.join(format!("archive_{}.json", metadata.id));
    let mut modifications = request.modifications;
    for m in modifications.iter_mut() {
        match m {
            VariantModification::DepartDeporte { full_geometry, .. } | VariantModification::ArriveeReportee { full_geometry, .. } | VariantModification::SegmentDeviation { full_geometry, .. } => *full_geometry = None,
        }
    }
    fs::write(&archive_path, serde_json::to_string_pretty(&Archive { metadata, modifications }).unwrap()).map_err(|e| e.to_string())?;

    // Update circuits.json to reset commune progress (forcing the icon to reappear)
    if let Ok(mut circuits_file) = crate::read_circuits_file(&app_env_path) {
        if let Some(circuit) = circuits_file.circuits.iter_mut().find(|c| c.circuit_id == request.circuit_id) {
            circuit.avancement_communes = 0;
            let _ = crate::write_circuits_file(&app_env_path, &circuits_file);
        }
    }

    Ok(final_warning)
}

#[tauri::command]
pub async fn get_variants(
    app_handle: tauri::AppHandle,
    circuit_id: String,
) -> Result<Vec<VariantMetadata>, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    if !circuit_data_dir.exists() {
        return Ok(Vec::new());
    }

    let mut variants = Vec::new();
    let entries = fs::read_dir(circuit_data_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if filename.starts_with("archive_var_") {
                let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                let mut archive: Archive = serde_json::from_str(&content).map_err(|e| e.to_string())?;
                
                // Compute global status on the fly if missing or just to be sure
                let mut status = "SUCCESS".to_string();
                let mut has_alt_fail = false;
                for mv in &archive.modifications {
                    let rs = match mv {
                        VariantModification::DepartDeporte { routing_status, .. } => routing_status,
                        VariantModification::ArriveeReportee { routing_status, .. } => routing_status,
                        VariantModification::SegmentDeviation { routing_status, .. } => routing_status,
                    };
                    if let Some(s) = rs {
                        if s == "ROUTE_FAIL" {
                            status = "ROUTE_FAIL".to_string();
                            break;
                        } else if s == "ALT_FAIL" {
                            has_alt_fail = true;
                        }
                    }
                }
                if status != "ROUTE_FAIL" && has_alt_fail {
                    status = "ALT_FAIL".to_string();
                }
                archive.metadata.global_status = Some(status);
                variants.push(archive.metadata);
            }
        }
    }

    // Sort by creation date DESC
    variants.sort_by(|a, b| b.creation_date.cmp(&a.creation_date));

    Ok(variants)
}

#[tauri::command]
pub async fn get_variant_details(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<Archive, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let archive_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("archive_{}.json", variant_id));

    if !archive_path.exists() {
        return Err("Archive non trouvé".to_string());
    }

    let content = fs::read_to_string(&archive_path).map_err(|e| e.to_string())?;
    let mut archive: Archive = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    // Re-hydrate full_geometry from LineString files
    for (index, modification) in archive.modifications.iter_mut().enumerate() {
        let suffix = match modification {
            VariantModification::DepartDeporte { .. } => "DEPART".to_string(),
            VariantModification::ArriveeReportee { .. } => "ARRIVEE".to_string(),
            VariantModification::SegmentDeviation { .. } => format!("SEGMENT_{}", index),
        };

        let linestring_filename = format!("lineString_{}_{}.json", archive.metadata.id, suffix);
        let linestring_path = app_env_path.join("data").join(&circuit_id).join(&linestring_filename);

        if linestring_path.exists() {
            if let Ok(ls_content) = fs::read_to_string(&linestring_path) {
                if let Ok(ls_json) = serde_json::from_str::<serde_json::Value>(&ls_content) {
                    if let Some(coords) = ls_json.get("coordinates").and_then(|c| c.as_array()) {
                        let mut full_geom = Vec::new();
                        for coord in coords {
                            if let Some(arr) = coord.as_array() {
                                if arr.len() >= 2 {
                                    full_geom.push(VariantPoint {
                                        lat: arr[1].as_f64().unwrap_or(0.0),
                                        lon: arr[0].as_f64().unwrap_or(0.0),
                                        alt: (arr.len() >= 3).then(|| arr[2].as_f64().unwrap_or(0.0)),
                                        point_type: None,
                                    });
                                }
                            }
                        }
                        match modification {
                            VariantModification::DepartDeporte { full_geometry, .. } => *full_geometry = Some(full_geom),
                            VariantModification::ArriveeReportee { full_geometry, .. } => *full_geometry = Some(full_geom),
                            VariantModification::SegmentDeviation { full_geometry, .. } => *full_geometry = Some(full_geom),
                        }
                    }
                }
            }
        }
    }

    Ok(archive)
}

#[tauri::command]
pub async fn delete_variant(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<(), String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    if !circuit_data_dir.exists() {
        return Err("Circuit directory not found".to_string());
    }

    let entries = fs::read_dir(&circuit_data_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let filename = entry.file_name().into_string().unwrap_or_default();
        
        // Delete archive, lineString and tracking files containing the variant_id
        if filename.contains(&variant_id) {
            fs::remove_file(entry.path()).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn rename_variant(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
    new_name: String,
) -> Result<(), String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let archive_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("archive_{}.json", variant_id));

    if !archive_path.exists() {
        return Err("Archive non trouvé".to_string());
    }

    let content = fs::read_to_string(&archive_path).map_err(|e| e.to_string())?;
    let mut archive: Archive = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    archive.metadata.name = new_name;
    
    let updated_content = serde_json::to_string_pretty(&archive).map_err(|e| e.to_string())?;
    fs::write(&archive_path, updated_content).map_err(|e| e.to_string())?;

    Ok(())
}

/// Charge le fichier lineString_FULL.json d'un variant
#[tauri::command]
pub async fn get_variant_full_linestring(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<serde_json::Value, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let linestring_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("lineString_{}_FULL.json", variant_id));

    if !linestring_path.exists() {
        return Err(format!("LineString FULL file not found for variant {}", variant_id));
    }

    let content = fs::read_to_string(&linestring_path).map_err(|e| e.to_string())?;
    let linestring: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(linestring)
}

/// Charge le fichier tracking_FULL.json d'un variant
#[tauri::command]
pub async fn get_variant_full_tracking(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<Vec<serde_json::Value>, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let tracking_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("tracking_{}_FULL.json", variant_id));

    if !tracking_path.exists() {
        return Err(format!("Tracking FULL file not found for variant {}", variant_id));
    }

    let content = fs::read_to_string(&tracking_path).map_err(|e| e.to_string())?;
    let tracking: Vec<serde_json::Value> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(tracking)
}

/// Charge le fichier segments_metadata.json d'un variant
#[tauri::command]
pub async fn get_variant_overlap_metadata(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<crate::segment_analyzer::SegmentMetadata, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let metadata_path = app_env_path.join("data")
        .join(&circuit_id)
        .join(format!("segments_metadata_{}.json", variant_id));

    if !metadata_path.exists() {
        return Err(format!("Segments metadata file not found for variant {}", variant_id));
    }

    let content = fs::read_to_string(&metadata_path).map_err(|e| e.to_string())?;
    let metadata: crate::segment_analyzer::SegmentMetadata = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(metadata)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteResult {
    pub geojson: String,
    pub warning: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphHopperResponse {
    paths: Vec<GraphHopperPath>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphHopperPath {
    points: GraphHopperPoints,
    distance: f64,
    ascend: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphHopperPoints {
    coordinates: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrsResponse {
    features: Vec<OrsFeature>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrsFeature {
    geometry: OrsGeometry,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrsGeometry {
    coordinates: Vec<Vec<f64>>,
}

async fn call_graphhopper(api_key: &str, profile: &str, points: &Vec<[f64; 2]>) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("Clé API GraphHopper manquante.".to_string());
    }

    let max_points_per_request = 5;
    let mut all_coordinates: Vec<Vec<f64>> = Vec::new();

    let chunks: Vec<Vec<[f64; 2]>> = if points.len() > max_points_per_request {
        let mut result = Vec::new();
        let mut start_idx = 0;
        while start_idx < points.len() - 1 {
            let end_idx = (start_idx + max_points_per_request).min(points.len());
            if end_idx - start_idx < 2 { break; }
            result.push(points[start_idx..end_idx].to_vec());
            start_idx = end_idx - 1; 
        }
        result
    } else {
        vec![points.clone()]
    };

    for (i, chunk_points) in chunks.iter().enumerate() {
        if i > 0 {
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        let client = reqwest::Client::new();
        let mut url = format!("https://graphhopper.com/api/1/route?key={}&profile={}&points_encoded=false&elevation=true", api_key, profile);
        
        for p in chunk_points {
            url.push_str(&format!("&point={},{}", p[1], p[0]));
        }

        let resp = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("GraphHopper Request failed: {}", e))?;

        if !resp.status().is_success() {
             return Err(format!("GraphHopper API Error: {}", resp.status()));
        }

        let gh_resp: GraphHopperResponse = resp.json().await.map_err(|e| format!("GH Parse error: {}", e))?;

        if let Some(path) = gh_resp.paths.first() {
            let mut coords = path.points.coordinates.clone();
            if i > 0 && !coords.is_empty() {
                coords.remove(0);
            }
            all_coordinates.extend(coords);
        } else {
             return Err("GraphHopper: Aucun chemin trouvé.".to_string());
        }
    }
    
    let geojson = serde_json::json!({
         "type": "LineString",
         "coordinates": all_coordinates
     });
     Ok(serde_json::to_string(&geojson).unwrap())
}

async fn call_openrouteservice(api_key: &str, profile: &str, points: &Vec<[f64; 2]>) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("Clé API OpenRouteService manquante.".to_string());
    }

    // Map profiles: car->driving-car, racingbike->cycling-road, bike->cycling-mountain
    let ors_profile = match profile {
        "car" => "driving-car",
        "racingbike" => "cycling-road",
        "bike" => "cycling-mountain",
        _ => "driving-car"
    };

    let client = reqwest::Client::new();
    let url = format!("https://api.openrouteservice.org/v2/directions/{}/geojson", ors_profile);
    
    let body = serde_json::json!({
        "coordinates": points
    });

    let resp = client.post(&url)
        .header("Authorization", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("ORS Request failed: {}", e))?;

     if !resp.status().is_success() {
         return Err(format!("OpenRouteService API Error: {}", resp.status()));
    }

    let ors_resp: OrsResponse = resp.json().await.map_err(|e| format!("ORS Parse error: {}", e))?;

    if let Some(feature) = ors_resp.features.first() {
         let geojson = serde_json::json!({
             "type": "LineString",
             "coordinates": feature.geometry.coordinates
         });
         Ok(serde_json::to_string(&geojson).unwrap())
    } else {
         Err("OpenRouteService: Aucun chemin trouvé.".to_string())
    }
}

#[tauri::command]
pub async fn calculate_route(
    app_handle: tauri::AppHandle,
    service: String,
    profile: String,
    points: Vec<[f64; 2]>,
) -> Result<RouteResult, String> {
    if points.len() < 2 {
        return Err("Il faut au moins 2 points pour calculer un itinéraire.".to_string());
    }

    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };
    
    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let key_gh = crate::get_setting_value(&settings, "data.groupes.Variante.groupes.Parametres.parametres.apiKeyGraphHopper").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let key_ors = crate::get_setting_value(&settings, "data.groupes.Variante.groupes.Parametres.parametres.apiKeyOpenRouteService").and_then(|v| v.as_str()).unwrap_or("").to_string();

    // Pré-check : Si les deux clés sont vides, on arrête tout de suite
    if key_gh.is_empty() && key_ors.is_empty() {
        return Err("Clés API GraphHopper et OpenRouteService manquantes dans les paramètres.".to_string());
    }

    // Define priority list based on user preference
    let mut attempt_order = Vec::new();
    if service == "GraphHopper" {
        attempt_order.push(("GraphHopper", &key_gh));
        attempt_order.push(("OpenRouteService", &key_ors));
    } else {
        attempt_order.push(("OpenRouteService", &key_ors));
        attempt_order.push(("GraphHopper", &key_gh));
    }

    let mut last_error = String::new();

    for (i, (svc_name, key)) in attempt_order.iter().enumerate() {
        let is_fallback = i > 0;
        
        // Vérification si la clé est vide
        if key.is_empty() {
            let err_msg = format!("Clé API {} manquante.", svc_name);
            if !is_fallback {
                // Si c'est le service primaire, on enregistre l'erreur et on passe DIRECTEMENT au suivant (pas de retry, pas d'arrêt)
                last_error = err_msg;
                continue;
            } else {
                // Si c'est le service de secours (et qu'on est déjà en fallback), on échoue avec le cumul
                return Err(format!("{} | {}", last_error, err_msg));
            }
        }

        // Retry logic for the current service (max 3 attempts)
        for attempt in 1..=3 {
            let result = if *svc_name == "GraphHopper" {
                call_graphhopper(key, &profile, &points).await
            } else {
                call_openrouteservice(key, &profile, &points).await
            };

            match result {
                Ok(geojson_raw) => {
                    // Enrichissement des altitudes et réparation des trous
                    let (geojson, alt_warning) = enhance_geojson_altitudes(geojson_raw).await;
                    
                    let mut warning = if is_fallback {
                        if last_error.is_empty() {
                             Some(format!("Service préférentiel indisponible. Bascule automatique sur {}.", svc_name))
                        } else {
                             Some(format!("Service préférentiel ignoré ({}). Bascule sur {}.", last_error, svc_name))
                        }
                    } else {
                        None
                    };
                    
                    if let Some(aw) = alt_warning {
                        warning = match warning {
                            Some(w) => Some(format!("{} | {}", w, aw)),
                            None => Some(aw)
                        };
                    }

                    return Ok(RouteResult { geojson, warning });
                },
                Err(e) => {
                    last_error = e.clone();
                    if attempt < 3 {
                        tokio::time::sleep(std::time::Duration::from_millis(500 * attempt)).await;
                    }
                }
            }
        }
    }

    Err(format!("Echec du routage sur tous les services. Dernière erreur: {}", last_error))
}

#[tauri::command]
pub async fn get_variant_geojson(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<String, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    // 1. Get Details (hydrated with full_geometry from lineString files)
    let archive = get_variant_details(app_handle.clone(), circuit_id.clone(), variant_id.clone()).await?;

    // 2. Load Master Trace
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let master_ls_path = circuit_data_dir.join("lineString.json");
    let master_ls_content = fs::read_to_string(&master_ls_path).map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let master_ls: serde_json::Value = serde_json::from_str(&master_ls_content).map_err(|e| format!("Failed to parse lineString.json: {}", e))?;
    let master_coords = master_ls["coordinates"].as_array().ok_or("Invalid lineString format")?;
    let master_points_high_res: Vec<Vec<f64>> = master_coords.iter().map(|c| c.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();

    // 3. Load Master Tracking (needed for indexing logic)
    let tracking_master_path = circuit_data_dir.join("tracking.json");
    let tracking_master_content = fs::read_to_string(&tracking_master_path).map_err(|e| format!("Failed to read master tracking: {}", e))?;
    let tracking_master: Vec<serde_json::Value> = serde_json::from_str(&tracking_master_content).map_err(|e| format!("Failed to parse master tracking: {}", e))?;
    let _total_tracking_pts = tracking_master.len();
    let total_high_res_pts = master_points_high_res.len();


    // 4. Stitching Logic


    // 4. Stitching Logic
     let find_corresponding_idx = |lat: f64, lon: f64, hint_idx: usize, start_search_from: usize| -> (usize, f64) {
        // hint_idx is High Resolution index (from anchors). 
        // We use it directly as the bio-center for search.
        let estimated_idx = hint_idx;
        
        let window_size = (total_high_res_pts / 20).max(500); 
        let min_search = estimated_idx.saturating_sub(window_size).max(start_search_from);
        let max_search = (estimated_idx + window_size).min(total_high_res_pts);
        
        let mut min_dist = f64::MAX;
        let mut best_idx = start_search_from;
        
        // Safety Fallback if window invalid
        if min_search >= max_search {
             let fallback_max = (start_search_from + 2000).min(total_high_res_pts);
             for i in start_search_from..fallback_max {
                let mp = &master_points_high_res[i];
                let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
                if d < min_dist { min_dist = d; best_idx = i; }
             }
             return (best_idx, min_dist);
        }

        for i in min_search..max_search {
            let mp = &master_points_high_res[i];
             let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
            if d < min_dist { min_dist = d; best_idx = i; }
        }
        
        // Fallback: Global Search if local match is poor (> 50m)
        // This handles cases where indices might be corrupted or desynchronized
        if min_dist > 50.0 {
             let mut global_min = min_dist;
             for i in start_search_from..total_high_res_pts {
                let mp = &master_points_high_res[i];
                let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
                if d < global_min { global_min = d; best_idx = i; }
                if global_min < 1.0 { break; } // Optimization: found close match (< 1m)
             }
             min_dist = global_min;
        }
        
        (best_idx, min_dist)
    };

    let mut final_points: Vec<Vec<f64>> = Vec::new();
    let mut current_master_idx = 0;
    let mut sorted_mods = archive.modifications.clone();
    sorted_mods.sort_by_key(|m| m.get_start_anchor_index());
    let mut has_arrivee_reportee = false;
    
    eprintln!("🔍 RUST: Starting variant assembly. Master has {} high-res points", total_high_res_pts);

    for modification in &sorted_mods {
         match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, full_geometry, .. } => {
                
                if let Some(geom) = full_geometry {
                     for p in geom {
                         final_points.push(vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]);
                     }
                }
                // Use master_points_high_res directly
                if *anchor_index_on_master < master_points_high_res.len() {
                    let anchor_coords = &master_points_high_res[*anchor_index_on_master];
                    // master_points_high_res: [lon, lat, alt], haversine needs (lat, lon)
                    let (idx, _dist) = find_corresponding_idx(anchor_coords[1], anchor_coords[0], *anchor_index_on_master, 0);
                    current_master_idx = idx;
                } else {
                    current_master_idx = 0; // Error fallback
                }
            },
             VariantModification::SegmentDeviation { anchor_start, anchor_end, full_geometry, .. } => {
                 // anchor_start.coords: [lon, lat], haversine needs (lat, lon)
                 let (start_idx, _start_dist) = find_corresponding_idx(anchor_start.coords[1], anchor_start.coords[0], anchor_start.index, current_master_idx);
                 
                 // Log what we actually found in master
                 if start_idx < master_points_high_res.len() {
                     let _found_point = &master_points_high_res[start_idx];
                 }
                 
                 if start_idx >= current_master_idx {
                    for i in current_master_idx..=start_idx {
                        if i < master_points_high_res.len() {
                            final_points.push(master_points_high_res[i].clone());
                        }
                    }
                }
                current_master_idx = start_idx + 1;
                
                if let Some(geom) = full_geometry {
                     for p in geom {
                         final_points.push(vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]);
                     }
                }
                
                // anchor_end.coords: [lon, lat], haversine needs (lat, lon)
                let (end_idx, _end_dist) = find_corresponding_idx(anchor_end.coords[1], anchor_end.coords[0], anchor_end.index, current_master_idx);
                
                // Log what we found
                if end_idx < master_points_high_res.len() {
                    let _found_point = &master_points_high_res[end_idx];
                }
                
                current_master_idx = end_idx;
                if current_master_idx < start_idx { 
                    current_master_idx = start_idx + 1; 
                }
             },
             VariantModification::ArriveeReportee { anchor_index_on_master, full_geometry, .. } => {
                 has_arrivee_reportee = true;
                 if *anchor_index_on_master < master_points_high_res.len() {
                    let anchor_coords = &master_points_high_res[*anchor_index_on_master];
                    // master_points_high_res: [lon, lat, alt], haversine needs (lat, lon)
                    let (arrivee_idx, _arrivee_dist) = find_corresponding_idx(anchor_coords[1], anchor_coords[0], *anchor_index_on_master, current_master_idx);
                    
                    if arrivee_idx >= current_master_idx {
                        for i in current_master_idx..=arrivee_idx {
                            if i < master_points_high_res.len() {
                                final_points.push(master_points_high_res[i].clone());
                            }
                        }
                    }
                 }
                 
                if let Some(geom) = full_geometry {
                      for p in geom {
                         final_points.push(vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]);
                     }
                }
                current_master_idx = master_points_high_res.len();
             }
         }
    }

    if !has_arrivee_reportee {
        while current_master_idx < master_points_high_res.len() {
            final_points.push(master_points_high_res[current_master_idx].clone());
            current_master_idx += 1;
        }
    }

    // Convert to GeoJSON string
      let geojson = serde_json::json!({
        "type": "Feature",
        "geometry": {
            "type": "LineString",
            "coordinates": final_points
        },
        "properties": {
            "variant_id": variant_id
        }
    });

    Ok(serde_json::to_string(&geojson).map_err(|e| e.to_string())?)
}

#[tauri::command]
pub async fn get_variant_comparison_geojson(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<String, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    // 1. Get Details
    let archive = get_variant_details(app_handle.clone(), circuit_id.clone(), variant_id.clone()).await?;

    // 2. Load Master Trace
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let master_ls_path = circuit_data_dir.join("lineString.json");
    let master_ls_content = fs::read_to_string(&master_ls_path).map_err(|e| format!("Failed to read lineString.json: {}", e))?;
    let master_ls: serde_json::Value = serde_json::from_str(&master_ls_content).map_err(|e| format!("Failed to parse lineString.json: {}", e))?;
    let master_coords = master_ls["coordinates"].as_array().ok_or("Invalid lineString format")?;
    let master_points_high_res: Vec<Vec<f64>> = master_coords.iter().map(|c| c.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect()).collect();

    // 3. Load Master Tracking
    let tracking_master_path = circuit_data_dir.join("tracking.json");
    let tracking_master_content = fs::read_to_string(&tracking_master_path).map_err(|e| format!("Failed to read master tracking: {}", e))?;
    let tracking_master: Vec<serde_json::Value> = serde_json::from_str(&tracking_master_content).map_err(|e| format!("Failed to parse master tracking: {}", e))?;
    let total_tracking_pts = tracking_master.len();
    let total_high_res_pts = master_points_high_res.len();

    // 4. Indexing Helper
    let find_corresponding_idx = |lon: f64, lat: f64, tracking_idx: usize, start_search_from: usize| -> usize {
        let ratio = tracking_idx as f64 / total_tracking_pts as f64;
        let estimated_idx = (ratio * total_high_res_pts as f64) as usize;
        let window_size = (total_high_res_pts / 20).max(500); 
        let min_search = estimated_idx.saturating_sub(window_size).max(start_search_from);
        let max_search = (estimated_idx + window_size).min(total_high_res_pts);
        
        let mut min_dist = f64::MAX;
        let mut best_idx = start_search_from;
        
        for i in min_search..max_search {
            let mp = &master_points_high_res[i];
            let d = crate::gpx_processor::haversine_distance(lat, lon, mp[1], mp[0]);
            if d < min_dist { min_dist = d; best_idx = i; }
        }
        best_idx
    };

    let mut features: Vec<serde_json::Value> = Vec::new();
    let mut current_master_idx = 0;
    
    // Sort modifications by start anchor index
    let mut sorted_mods = archive.modifications.clone();
    sorted_mods.sort_by_key(|m| m.get_start_anchor_index());
    let mut has_arrivee_reportee = false;

    // Helper to create Feature
    let create_feature = |points: Vec<Vec<f64>>, status: &str| -> serde_json::Value {
         serde_json::json!({
            "type": "Feature",
            "geometry": {
                "type": "LineString",
                "coordinates": points
            },
            "properties": {
                "status": status // "COMMON", "ABANDONED", "NEW"
            }
        })
    };

    for modification in &sorted_mods {
         match modification {
            VariantModification::DepartDeporte { anchor_index_on_master, full_geometry, .. } => {
                // Determine anchor position on master
                let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                let anchor_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, 0);

                // ABANDONED segment from Start (0) to Anchor
                if anchor_idx > 0 {
                    let abandoned_pts: Vec<Vec<f64>> = master_points_high_res[0..=anchor_idx].to_vec();
                    features.push(create_feature(abandoned_pts, "ABANDONED"));
                }
                current_master_idx = anchor_idx;

                 // NEW segment (the variant part)
                if let Some(geom) = full_geometry {
                     let pts: Vec<Vec<f64>> = geom.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
                     features.push(create_feature(pts, "NEW"));
                }
            },
             VariantModification::SegmentDeviation { anchor_start, anchor_end, full_geometry, .. } => {
                 let start_idx = find_corresponding_idx(anchor_start.coords[0], anchor_start.coords[1], anchor_start.index, current_master_idx);
                 
                 // COMMON segment from current to start_idx
                 if start_idx > current_master_idx {
                     let common_pts: Vec<Vec<f64>> = master_points_high_res[current_master_idx..=start_idx].to_vec();
                     if !common_pts.is_empty() {
                         features.push(create_feature(common_pts, "COMMON"));
                     }
                 }
                 
                 // Find end_idx
                 let end_idx = find_corresponding_idx(anchor_end.coords[0], anchor_end.coords[1], anchor_end.index, start_idx);
                 
                 // ABANDONED segment from start_idx to end_idx
                 if end_idx > start_idx {
                      let abandoned_pts: Vec<Vec<f64>> = master_points_high_res[start_idx..=end_idx].to_vec();
                      if !abandoned_pts.is_empty() {
                         features.push(create_feature(abandoned_pts, "ABANDONED"));
                     }
                 }
                
                // NEW segment
                if let Some(geom) = full_geometry {
                     let pts: Vec<Vec<f64>> = geom.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
                     features.push(create_feature(pts, "NEW"));
                }
                
                current_master_idx = end_idx;
                if current_master_idx < start_idx { current_master_idx = start_idx; } // Safety
             },
             VariantModification::ArriveeReportee { anchor_index_on_master, full_geometry, .. } => {
                 has_arrivee_reportee = true;
                 let anchor_coords = tracking_master[*anchor_index_on_master]["coordonnee"].as_array().unwrap();
                 let arrivee_idx = find_corresponding_idx(anchor_coords[0].as_f64().unwrap(), anchor_coords[1].as_f64().unwrap(), *anchor_index_on_master, current_master_idx);
                 
                 // COMMON segment up to Arrivee Anchor
                 if arrivee_idx > current_master_idx {
                     let common_pts: Vec<Vec<f64>> = master_points_high_res[current_master_idx..=arrivee_idx].to_vec();
                      if !common_pts.is_empty() {
                         features.push(create_feature(common_pts, "COMMON"));
                     }
                 }
                 
                // ABANDONED segment from Arrivee Anchor to End
                if arrivee_idx < master_points_high_res.len() - 1 {
                    let abandoned_pts: Vec<Vec<f64>> = master_points_high_res[arrivee_idx..].to_vec();
                    features.push(create_feature(abandoned_pts, "ABANDONED"));
                }

                // NEW segment
                if let Some(geom) = full_geometry {
                     let pts: Vec<Vec<f64>> = geom.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
                     features.push(create_feature(pts, "NEW"));
                }
                current_master_idx = master_points_high_res.len();
             }
         }
    }

    // Remaining COMMON segment if any
    if !has_arrivee_reportee && current_master_idx < master_points_high_res.len() {
        let common_pts: Vec<Vec<f64>> = master_points_high_res[current_master_idx..].to_vec();
        features.push(create_feature(common_pts, "COMMON"));
    }

    let fc = serde_json::json!({
        "type": "FeatureCollection",
        "features": features
    });

    Ok(serde_json::to_string(&fc).map_err(|e| e.to_string())?)
}

// --- Helpers for Tracking Generation ---

fn simple_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1 = lat1.to_radians();
    let lon1 = lon1.to_radians();
    let lat2 = lat2.to_radians();
    let lon2 = lon2.to_radians();
    let d_lon = lon2 - lon1;
    let y = d_lon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * d_lon.cos();
    let mut brng = y.atan2(x).to_degrees();
    if brng < 0.0 { brng += 360.0; }
    (brng * 10.0).round() / 10.0
}

fn generate_interpolated_tracking(points: &Vec<VariantPoint>, segment_length: f64, default_zoom: f64, default_pitch: f64) -> Vec<serde_json::Value> {
    if points.is_empty() { return Vec::new(); }
    
    // 1. Convert to simple coordinates for calculation
    let coords_3d: Vec<Vec<f64>> = points.iter().map(|p| vec![p.lon, p.lat, p.alt.unwrap_or(0.0)]).collect();
    
    // 2. Interpolate
    let mut calculated_points = Vec::new();
    calculated_points.push(coords_3d[0].clone());

    let mut distance_needed = segment_length;
    let mut distance_traversed = 0.0;

    for i in 0..coords_3d.len() - 1 {
        let p1 = &coords_3d[i];
        let p2 = &coords_3d[i+1];
        let dist = crate::gpx_processor::haversine_distance(p1[1], p1[0], p2[1], p2[0]);

        if dist > 0.0 {
            while distance_traversed + dist >= distance_needed {
                let fraction = (distance_needed - distance_traversed) / dist;
                let lon = p1[0] + (p2[0] - p1[0]) * fraction;
                let lat = p1[1] + (p2[1] - p1[1]) * fraction;
                let alt = p1[2] + (p2[2] - p1[2]) * fraction;
                calculated_points.push(vec![lon, lat, alt]);
                distance_needed += segment_length;
            }
        }
        distance_traversed += dist;
    }

    // 3. Add last point if needed
    if coords_3d.len() > 1 {
        let last = coords_3d.last().unwrap();
        let last_calc = calculated_points.last().unwrap();
        let d = crate::gpx_processor::haversine_distance(last[1], last[0], last_calc[1], last_calc[0]);
        if d > 1.0 {
            calculated_points.push(last.clone());
        }
    }

    // 4. Map to TrackingPoint JSON
    let count = calculated_points.len();
    let mut results: Vec<serde_json::Value> = Vec::new();
    for i in 0..count {
        let p = &calculated_points[i];
        let mut cap = 0.0;
        if i < count - 1 {
            let next = &calculated_points[i + 1];
            cap = simple_bearing(p[1], p[0], next[1], next[0]);
        } else if i > 0 {
            if let Some(prev) = results.last() {
                cap = prev["cap"].as_f64().unwrap_or(0.0);
            }
        }
        results.push(serde_json::json!({
            "increment": i,
            "coordonnee": [p[0], p[1]],
            "altitude": p[2],
            "distance": 0.0,
            "cap": cap,
            "zoom": default_zoom,
            "pitch": default_pitch,
            "coordonneeCamera": [],
            "altitudeCamera": 0.0,
            "nbrSegment": 0,
            "pointDeControl": i == 0 || i == count - 1,
            "typeTroncon": "Segment"
        }));
    }
    results
}

fn find_closest_tracking_idx(tracking: &Vec<serde_json::Value>, lat: f64, lon: f64, start_hint: usize) -> usize {
    let mut min_dist = f64::MAX;
    let mut best_idx = start_hint;
    let limit = tracking.len();
    // Search localized around hint first could be better but let's scan a reasonable window or all if small
    // Optimization: scan forward from start_hint
    for i in 0..limit {
        if let Some(coords) = tracking[i].get("coordonnee").and_then(|c| c.as_array()) {
            if coords.len() >= 2 {
                let clat = coords[1].as_f64().unwrap_or(0.0);
                let clon = coords[0].as_f64().unwrap_or(0.0);
                let d = (clat - lat).powi(2) + (clon - lon).powi(2);
                if d < min_dist {
                    min_dist = d;
                    best_idx = i;
                }
            }
        }
    }
    best_idx
}

// --- Commands ---

#[tauri::command]
pub async fn get_variant_tracking(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<String, String> {
    let tracking_data = get_variant_tracking_internal(app_handle, circuit_id, variant_id).await?;
    serde_json::to_string(&tracking_data).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_variant_slope_expression(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
    slope_colors: std::collections::HashMap<String, String>,
) -> Result<serde_json::Value, String> {
    // 1. Get Variant Tracking (Interpolated at 100m)
    let tracking_values = get_variant_tracking_internal(app_handle.clone(), circuit_id.clone(), variant_id).await?;
    
    // 2. Convert to TraceStyle TrackingPoint
    let tracking_points: Vec<crate::trace_style::TrackingPoint> = tracking_values.into_iter().map(|v| {
        let alt = v["altitude"].as_f64().unwrap_or(0.0);
        let coords = v["coordonnee"].as_array().unwrap();
        let lon = coords[0].as_f64().unwrap_or(0.0);
        let lat = coords[1].as_f64().unwrap_or(0.0);
        crate::trace_style::TrackingPoint {
            altitude: alt,
            coordonnee: [lon, lat],
        }
    }).collect();

    // 3. Get Settings for segment length
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };
    let settings_path = app_env_path.join("settings.json");
    let settings_content = std::fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;
    let segment_length = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);

     // 4. Generate Expression using TraceStyle (reusing existing logic)
     crate::trace_style::get_slope_color_expression(
         app_handle.state::<std::sync::Mutex<crate::AppState>>(),
         circuit_id,
         slope_colors,
         segment_length,
         None,
         Some(tracking_points)
     ).await
}




pub async fn get_variant_tracking_internal(
    app_handle: tauri::AppHandle,
    circuit_id: String,
    variant_id: String,
) -> Result<Vec<serde_json::Value>, String> {
     let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };

    let settings_path = app_env_path.join("settings.json");
    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let segment_length = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Tracking.parametres.LongueurSegment")
        .and_then(|v| v.as_f64())
        .unwrap_or(100.0);
    let default_zoom = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Camera.parametres.Zoom").and_then(|v| v.as_f64()).unwrap_or(16.0);
    let default_pitch = crate::get_setting_value(&settings, "data.groupes.Importation.groupes.Camera.parametres.Pitch").and_then(|v| v.as_f64()).unwrap_or(60.0);

    // 1. Load Master Tracking
    let circuit_data_dir = app_env_path.join("data").join(&circuit_id);
    let tracking_path = circuit_data_dir.join("tracking.json");
    if !tracking_path.exists() { return Err("Master tracking.json missing".to_string()); }
    
    let content = fs::read_to_string(&tracking_path).map_err(|e| e.to_string())?;
    let master_tracking: Vec<serde_json::Value> = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    // 2. Load Variant Data
    // Use get_variant_details which returns Archive
    let archive = get_variant_details(app_handle.clone(), circuit_id.clone(), variant_id.clone()).await?;

    // 3. Assemble
    let mut final_tracking: Vec<serde_json::Value> = Vec::new();
    let mut current_master_idx = 0;
    
    // Check for DepartDeporte first as it overrides start
    // Sort logic similar to comparison? Yes.
    let mut sorted_mods = archive.modifications.clone();
    sorted_mods.sort_by_key(|m| m.get_start_anchor_index());

    for modification in sorted_mods {
        match modification {
            VariantModification::DepartDeporte { full_geometry, .. } => {
                 if let Some(geom) = full_geometry {
                      // Find where it connects on Master
                      let last = geom.last().unwrap();
                      let connect_idx = find_closest_tracking_idx(&master_tracking, last.lat, last.lon, 0);
                      
                      let mut variant_tracking = generate_interpolated_tracking(&geom, segment_length, default_zoom, default_pitch);

                      // Copy camera params from anchor to first point
                      if let Some(anchor_tp) = master_tracking.get(connect_idx) {
                          if let Some(first_tp) = variant_tracking.get_mut(0).and_then(|v| v.as_object_mut()) {
                              for field in ["zoom", "pitch", "cap", "editedZoom", "editedPitch", "editedCap"] {
                                  if let Some(val) = anchor_tp.get(field) {
                                      first_tp.insert(field.to_string(), val.clone());
                                  }
                              }
                          }
                      }
                      
                      // Update LAST point of variant to point to master_tracking[connect_idx + 1]
                      if let (Some(last_tp), Some(next_master_tp)) = (variant_tracking.last_mut(), master_tracking.get(connect_idx + 1)) {
                          if let (Some(last_obj), Some(next_obj)) = (last_tp.as_object_mut(), next_master_tp.as_object()) {
                              let l_coords = last_obj["coordonnee"].as_array().unwrap();
                              let n_coords = next_obj["coordonnee"].as_array().unwrap();
                              let new_cap = simple_bearing(l_coords[1].as_f64().unwrap(), l_coords[0].as_f64().unwrap(),
                                                         n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                              last_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                              if last_obj.contains_key("editedCap") { last_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                          }
                      }

                      final_tracking.extend(variant_tracking);
                      current_master_idx = connect_idx + 1;
                 }
            },
            VariantModification::ArriveeReportee { full_geometry, .. } => {
                 if let Some(geom) = full_geometry {
                     if let Some(first) = geom.first() {
                         let connect_idx = find_closest_tracking_idx(&master_tracking, first.lat, first.lon, current_master_idx);
                         
                         // Add Master until connection
                         for i in current_master_idx..=connect_idx {
                             if i < master_tracking.len() {
                                 final_tracking.push(master_tracking[i].clone());
                             }
                         }
                         
                         // Add New Segment (skip first point as it's the anchor in master)
                         let var_tracking = generate_interpolated_tracking(&geom, segment_length, default_zoom, default_pitch);
                         // Update master anchor's cap to point to the FIRST NEW point of variant
                         if let (Some(anchor_tp), Some(next_var_tp)) = (final_tracking.last_mut(), var_tracking.get(1)) {
                             if let (Some(anchor_obj), Some(next_obj)) = (anchor_tp.as_object_mut(), next_var_tp.as_object()) {
                                 let a_coords = anchor_obj["coordonnee"].as_array().unwrap();
                                 let n_coords = next_obj["coordonnee"].as_array().unwrap();
                                 let new_cap = simple_bearing(a_coords[1].as_f64().unwrap(), a_coords[0].as_f64().unwrap(),
                                                            n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                                 anchor_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                                 if anchor_obj.contains_key("editedCap") { anchor_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                             }
                         }
                         final_tracking.extend(var_tracking.into_iter().skip(1));
                         
                         current_master_idx = master_tracking.len(); // End
                     }
                 }
            },
            VariantModification::SegmentDeviation { full_geometry, .. } => {
                if let Some(geom) = full_geometry {
                    if let Some(first) = geom.first() {
                        let start_idx = find_closest_tracking_idx(&master_tracking, first.lat, first.lon, current_master_idx);
                        
                         // Add Master until start
                         for i in current_master_idx..=start_idx {
                             if i < master_tracking.len() {
                                 final_tracking.push(master_tracking[i].clone());
                             }
                         }
                         
                         // Add New Segment (skip first point as it's the anchor in master)
                         let mut var_tracking = generate_interpolated_tracking(&geom, segment_length, default_zoom, default_pitch);
                         
                         // 1. Update master anchor to point to variant[1]
                         if let (Some(anchor_tp), Some(next_var_tp)) = (final_tracking.last_mut(), var_tracking.get(1)) {
                              if let (Some(anchor_obj), Some(next_obj)) = (anchor_tp.as_object_mut(), next_var_tp.as_object()) {
                                 let a_coords = anchor_obj["coordonnee"].as_array().unwrap();
                                 let n_coords = next_obj["coordonnee"].as_array().unwrap();
                                 let new_cap = simple_bearing(a_coords[1].as_f64().unwrap(), a_coords[0].as_f64().unwrap(),
                                                            n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                                 anchor_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                                 if anchor_obj.contains_key("editedCap") { anchor_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                             }
                         }
                         
                         // 2. Update last point of variant to point to master_tracking[end_idx + 1]
                         if let Some(last) = geom.last() {
                             let end_idx = find_closest_tracking_idx(&master_tracking, last.lat, last.lon, start_idx);
                             
                             if let (Some(last_tp), Some(next_master_tp)) = (var_tracking.last_mut(), master_tracking.get(end_idx + 1)) {
                                  if let (Some(last_obj), Some(next_obj)) = (last_tp.as_object_mut(), next_master_tp.as_object()) {
                                     let l_coords = last_obj["coordonnee"].as_array().unwrap();
                                     let n_coords = next_obj["coordonnee"].as_array().unwrap();
                                     let new_cap = simple_bearing(l_coords[1].as_f64().unwrap(), l_coords[0].as_f64().unwrap(),
                                                                n_coords[1].as_f64().unwrap(), n_coords[0].as_f64().unwrap());
                                     last_obj.insert("cap".to_string(), serde_json::json!(new_cap));
                                     if last_obj.contains_key("editedCap") { last_obj.insert("editedCap".to_string(), serde_json::json!(new_cap)); }
                                 }
                             }
                             
                             final_tracking.extend(var_tracking.into_iter().skip(1));
                             current_master_idx = end_idx + 1;
                         } else {
                             final_tracking.extend(var_tracking.into_iter().skip(1));
                         }
                    }
                }
            }
        }
    }
    
    // Add remaining master
    for i in current_master_idx..master_tracking.len() {
        final_tracking.push(master_tracking[i].clone());
    }
    
    // 4. Post-Process: Recalculate Increments
    for (i, val) in final_tracking.iter_mut().enumerate() {
        if let Some(obj) = val.as_object_mut() {
            obj.insert("increment".to_string(), serde_json::json!(i));
        }
    }
    
    Ok(final_tracking)
}

#[tauri::command]
pub async fn get_altitudes(points: Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    crate::elevation_provider::fetch_altitudes(&points).await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoutingStatus {
    pub graphhopper: String, // "EMPTY", "INVALID", "VALID"
    pub ors: String,         // "EMPTY", "INVALID", "VALID"
}

#[tauri::command]
pub async fn check_routing_services(
    app_handle: tauri::AppHandle,
) -> Result<RoutingStatus, String> {
    let app_env_path = {
        let state_mutex = app_handle.state::<std::sync::Mutex<crate::AppState>>();
        let app_state = state_mutex.lock().unwrap();
        app_state.app_env_path.clone()
    };
    
    let settings_path = app_env_path.join("settings.json");
    if !settings_path.exists() {
        return Ok(RoutingStatus { graphhopper: "EMPTY".to_string(), ors: "EMPTY".to_string() });
    }

    let settings_content = fs::read_to_string(&settings_path).map_err(|e| e.to_string())?;
    let settings: serde_json::Value = serde_json::from_str(&settings_content).map_err(|e| e.to_string())?;

    let key_gh = crate::get_setting_value(&settings, "data.groupes.Variante.groupes.Parametres.parametres.apiKeyGraphHopper").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let key_ors = crate::get_setting_value(&settings, "data.groupes.Variante.groupes.Parametres.parametres.apiKeyOpenRouteService").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let mut status = RoutingStatus {
        graphhopper: if key_gh.is_empty() { "EMPTY".to_string() } else { "UNKNOWN".to_string() },
        ors: if key_ors.is_empty() { "EMPTY".to_string() } else { "UNKNOWN".to_string() },
    };

    // Test points (very close to avoid heavy routing)
    let test_points = vec![[2.3522, 48.8566], [2.3523, 48.8567]]; 

    // Test GraphHopper
    if status.graphhopper == "UNKNOWN" {
        match call_graphhopper(&key_gh, "car", &test_points).await {
            Ok(_) => status.graphhopper = "VALID".to_string(),
            Err(e) => {
                if e.contains("401") || e.contains("403") {
                    status.graphhopper = "INVALID".to_string();
                } else {
                    // Other error (network, etc) - we might want to say INVALID for safety or leave as is.
                    // But if it's a real API rejection, it's 401/403.
                    status.graphhopper = "INVALID".to_string(); 
                }
            }
        }
    }

    // Test OpenRouteService
    if status.ors == "UNKNOWN" {
        match call_openrouteservice(&key_ors, "car", &test_points).await {
            Ok(_) => status.ors = "VALID".to_string(),
            Err(e) => {
                if e.contains("401") || e.contains("403") {
                    status.ors = "INVALID".to_string();
                } else {
                    status.ors = "INVALID".to_string();
                }
            }
        }
    }

    Ok(status)
}
