use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantPoint {
    lat: f64,
    lon: f64,
    #[serde(rename = "type")]
    point_type: Option<String>, // "start_point", "end_point", "waypoint"
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum VariantModification {
    #[serde(rename = "DEPART_DEPORTE")]
    DepartDeporte {
        anchor_index_on_master: usize,
        points: Vec<VariantPoint>,
        longueur: f64,
    },
    #[serde(rename = "ARRIVEE_REPORTEE")]
    ArriveeReportee {
        anchor_index_on_master: usize,
        points: Vec<VariantPoint>,
        longueur: f64,
    },
    #[serde(rename = "SEGMENT_DEVIATION")]
    SegmentDeviation {
        anchor_start: AnchorPoint,
        anchor_end: AnchorPoint,
        waypoints: Vec<VariantPoint>,
        longueur: f64,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AnchorPoint {
    index: usize,
    coords: [f64; 2],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantStats {
    total_distance: f64,
    total_ascent: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VariantMetadata {
    id: String,
    name: String,
    description: String,
    creation_date: String,
    color: String,
    stats: VariantStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Archive {
    metadata: VariantMetadata,
    modifications: Vec<VariantModification>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateVariantRequest {
    circuit_id: String,
    metadata: VariantMetadata,
    modifications: Vec<VariantModification>,
}

// Function placeholders
const IGN_API_URL: &str = "https://data.geopf.fr/altimetrie/1.0/calcul/alti/rest/elevation.json";

#[derive(Deserialize, Debug)]
struct IgnElevationResponse {
    #[allow(dead_code)]
    elevations: Vec<IgnElevationPoint>,
}

#[derive(Deserialize, Debug)]
struct IgnElevationPoint {
    #[allow(dead_code)]
    z: f64,
}

async fn fetch_altitudes(points: &Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    if points.is_empty() {
        return Ok(Vec::new());
    }

    let client = reqwest::Client::new();
    let mut altitudes = Vec::new();
    
    // Chunk requests to avoid URL length limits or API payload limits (e.g. 50 points per request)
    for chunk in points.chunks(50) {
        let lons: Vec<String> = chunk.iter().map(|p| p[0].to_string()).collect();
        let lats: Vec<String> = chunk.iter().map(|p| p[1].to_string()).collect();
        
        let url = format!(
            "{}?lon={}&lat={}&resource=ign_rge_alti_wld&delimiter=|&indent=false&zonly=true",
            IGN_API_URL,
            lons.join("|"),
            lats.join("|")
        );

        let resp = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !resp.status().is_success() {
             return Err(format!("IGN API Error: {}", resp.status()));
        }

        let json: serde_json::Value = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;
        
        // Handle "elevations": [z1, z2, ...] response format when zonly=true
        if let Some(elevations_array) = json.get("elevations").and_then(|v| v.as_array()) {
            for val in elevations_array {
                altitudes.push(val.as_f64().unwrap_or(0.0));
            }
        } else {
             return Err("Invalid API response format".to_string());
        }
    }

    if altitudes.len() != points.len() {
        return Err(format!("Mismatch in altitude count: expected {}, got {}", points.len(), altitudes.len()));
    }

    Ok(altitudes)
}

#[tauri::command]
pub async fn create_variant_files(
    app_handle: tauri::AppHandle,
    request: CreateVariantRequest,
) -> Result<String, String> {
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

    // Process each modification
    for (index, modification) in request.modifications.iter().enumerate() {
        let (suffix, points_2d) = match modification {
            VariantModification::DepartDeporte { points, .. } => ("DEPART", points),
            VariantModification::ArriveeReportee { points, .. } => ("ARRIVEE", points),
            VariantModification::SegmentDeviation { waypoints, .. } => {
                // For segment, we construct the full path: Start -> Waypoints -> End
                // Actually, the 'waypoints' in struct might just be the intermediate points.
                // We need to form a single chain for the LineString.
                // TODO: Verify if 'waypoints' includes anchors or not. Assuming it DOES NOT.
                // So list = [anchor_start] + waypoints + [anchor_end]
                // But AnchorPoint struct has coords.
                // Let's create a temporary vec for processing
                // Wait, map matching or routing logic should have already happened in frontend?
                // The prompt says: "Récupération des altitudes (API) ... Appel à generate_tracking_file"
                // The frontend sends "waypoints".
                // Let's assume 'points' in the variant mod are the ordered list of coordinates for that segment.
                ("SEGMENT", waypoints)
            }
        };

        if points_2d.is_empty() {
             continue;
        }
        
        let suffix_full = match modification {
             VariantModification::SegmentDeviation { .. } => format!("{}_{}", suffix, index),
             _ => suffix.to_string() 
        };

        // If it's a segment, we might need to prepend start/end anchors to the geometry?
        // The implementation_plan says: "Construction de la LineString complète."
        // Let's assume the frontend sends the COMPLETE geometry in `points` or `waypoints` including connections.
        // Checking VariantModification struct...
        // SegmentDeviation has anchor_start, anchor_end, AND waypoints.
        let mut full_points: Vec<[f64; 2]> = Vec::new();
        
        match modification {
             VariantModification::SegmentDeviation { anchor_start, anchor_end, waypoints, .. } => {
                 full_points.push(anchor_start.coords);
                 for p in waypoints {
                     full_points.push([p.lon, p.lat]);
                 }
                 full_points.push(anchor_end.coords);
             },
             VariantModification::DepartDeporte { points, .. } | VariantModification::ArriveeReportee { points, .. } => {
                 for p in points {
                     full_points.push([p.lon, p.lat]);
                 }
             }
        }
        
        // 1. Fetch Altitudes
        let altitudes = fetch_altitudes(&full_points).await.map_err(|e| format!("Altitude fetch failed: {}", e))?;
        
        // 2. Merge into [lon, lat, ele]
        let track_points_3d: Vec<Vec<f64>> = full_points.iter().zip(altitudes.iter()).map(|(p, alt)| {
             vec![p[0], p[1], *alt]
        }).collect();

        // 3. Write LineString file
        let linestring_filename = format!("lineString_{}_{}.json", request.metadata.id, suffix_full);
        let linestring_path = circuit_data_dir.join(&linestring_filename);
        let linestring_json = serde_json::json!({
            "type": "LineString",
            "coordinates": track_points_3d
        });
        fs::write(&linestring_path, serde_json::to_string_pretty(&linestring_json).unwrap())
            .map_err(|e| e.to_string())?;

        // 4. Generate Tracking file (Resampling 100m)
        let tracking_filename = format!("tracking_{}_{}.json", request.metadata.id, suffix_full);
        crate::tracking_processor::generate_tracking_file(
            &app_env_path,
            &request.circuit_id,
            &track_points_3d,
            &settings,
            Some(&tracking_filename)
        )?;
    }

    // 5. Save Archive file
    // Use the sanitized name from metadata or a default
    // Using simple alphanumeric replacement for safety
    let safe_name = request.metadata.name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
        
    let _archive_filename = format!("archive_var_{}.json", safe_name); 
    // The plan says "archive_[nom].json". Let's use ID if name is unsafe? 
    // Metadata has ID "var_...". Let's use that.
    let archive_filename = format!("archive_{}.json", request.metadata.id);
    let archive_path = circuit_data_dir.join(&archive_filename);
    
    let archive = Archive {
        metadata: request.metadata,
        modifications: request.modifications,
    };
    
    fs::write(&archive_path, serde_json::to_string_pretty(&archive).unwrap())
        .map_err(|e| e.to_string())?;

    Ok("Variante créée avec succès".to_string())
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
                let archive: Archive = serde_json::from_str(&content).map_err(|e| e.to_string())?;
                variants.push(archive.metadata);
            }
        }
    }

    // Sort by creation date DESC
    variants.sort_by(|a, b| b.creation_date.cmp(&a.creation_date));

    Ok(variants)
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

#[tauri::command]
pub async fn calculate_route(
    app_handle: tauri::AppHandle,
    service: String,
    profile: String,
    points: Vec<[f64; 2]>,
) -> Result<String, String> {
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

    let api_key = crate::get_setting_value(
        &settings,
        "data.groupes.Variante.parametres.routingApiKey",
    )
    .and_then(|v| v.as_str())
    .unwrap_or("")
    .to_string();

    if service == "GraphHopper" {
        if api_key.is_empty() {
             return Err("Clé API GraphHopper manquante dans les paramètres.".to_string());
        }
        
        // https://docs.graphhopper.com/#tag/Routing-API/operation/getRoute
        // GET /route?point=...&point=...&profile=...&key=...&points_encoded=false
        
        let client = reqwest::Client::new();
        let mut url = format!("https://graphhopper.com/api/1/route?key={}&profile={}&points_encoded=false&elevation=false", api_key, profile);
        
        for p in points {
            url.push_str(&format!("&point={},{}", p[1], p[0])); // Lat,Lon
        }

        let resp = client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !resp.status().is_success() {
             return Err(format!("GraphHopper API Error: {}", resp.status()));
        }

        let gh_resp: GraphHopperResponse = resp.json().await.map_err(|e| format!("Parse error: {}", e))?;

        if let Some(path) = gh_resp.paths.first() {
             let geojson = serde_json::json!({
                 "type": "LineString",
                 "coordinates": path.points.coordinates
             });
             return Ok(serde_json::to_string(&geojson).unwrap());
        } else {
             return Err("Aucun chemin trouvé.".to_string());
        }

    } else if service == "OpenRouteService" {
        return Err("OpenRouteService non implémenté pour le moment.".to_string());
    }

    Err("Service de routage inconnu.".to_string())
}
