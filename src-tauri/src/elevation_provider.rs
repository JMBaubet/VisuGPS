// No imports needed for now as we use serde_json::Value

#[derive(Debug, PartialEq)]
pub enum ElevationProvider {
    IGN,
    OpenMeteo,
}

impl std::fmt::Display for ElevationProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElevationProvider::IGN => write!(f, "IGN"),
            ElevationProvider::OpenMeteo => write!(f, "Open-Meteo"),
        }
    }
}

/// Détermine si une liste de points se trouve intégralement en France Métropolitaine.
/// Bounding box approximatif de la France (incluant la Corse).
pub fn is_entirely_in_france(points: &[[f64; 2]]) -> bool {
    if points.is_empty() {
        return false;
    }

    // BBox France Continentale (approx)
    let cont_min_lon = -5.5;
    let cont_max_lon = 8.5;
    let cont_min_lat = 42.3;
    let cont_max_lat = 51.1;

    // BBox Corse (approx)
    let corse_min_lon = 8.5;
    let corse_max_lon = 9.6;
    let corse_min_lat = 41.3;
    let corse_max_lat = 43.1;

    points.iter().all(|p| {
        let lon = p[0];
        let lat = p[1];
        
        // Est-ce dans le continent ?
        let in_continent = lon >= cont_min_lon && lon <= cont_max_lon && lat >= cont_min_lat && lat <= cont_max_lat;
        // Est-ce en Corse ?
        let in_corse = lon >= corse_min_lon && lon <= corse_max_lon && lat >= corse_min_lat && lat <= corse_max_lat;

        in_continent || in_corse
    })
}

pub fn get_best_provider(points: &[[f64; 2]]) -> ElevationProvider {
    if is_entirely_in_france(points) {
        ElevationProvider::IGN
    } else {
        ElevationProvider::OpenMeteo
    }
}

// Re-implement providers here or keep them in variant_processor for now?
// Centralizing them here is cleaner.

const IGN_API_URL: &str = "https://data.geopf.fr/altimetrie/1.0/calcul/alti/rest/elevation.json";
const OPEN_METEO_URL: &str = "https://elevation-api.open-meteo.com/v1/elevation";

pub async fn fetch_altitudes(points: &Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    if points.is_empty() {
        return Ok(Vec::new());
    }

    let provider = get_best_provider(points);
    println!("Utilisation du fournisseur d'altitude : {}", provider);

    match provider {
        ElevationProvider::IGN => fetch_ign(points).await,
        ElevationProvider::OpenMeteo => fetch_open_meteo(points).await,
    }
}

async fn fetch_ign(points: &Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    let client = reqwest::Client::new();
    let mut altitudes = Vec::new();

    for chunk in points.chunks(50) {
        let lons: Vec<String> = chunk.iter().map(|p| p[0].to_string()).collect();
        let lats: Vec<String> = chunk.iter().map(|p| p[1].to_string()).collect();

        let url = format!(
            "{}?lon={}&lat={}&resource=ign_rge_alti_wld&delimiter=|&indent=false&zonly=true",
            IGN_API_URL,
            lons.join("|"),
            lats.join("|")
        );

        let resp = client.get(&url).send().await.map_err(|e| format!("IGN Request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("IGN API Error: {}", resp.status()));
        }

        let json: serde_json::Value = resp.json().await.map_err(|e| format!("IGN Parse error: {}", e))?;

        if let Some(elevations_array) = json.get("elevations").and_then(|v| v.as_array()) {
            for val in elevations_array {
                let alt = val.as_f64().unwrap_or(0.0);
                // Handle IGN error value -99999.0
                altitudes.push(if alt < -90000.0 { 0.0 } else { alt });
            }
        } else {
            return Err("Format de réponse IGN invalide".to_string());
        }
    }
    
    Ok(altitudes)
}

async fn fetch_open_meteo(points: &Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    let client = reqwest::Client::new();
    let mut altitudes = Vec::new();

    for chunk in points.chunks(100) {
        let lons: Vec<String> = chunk.iter().map(|p| p[0].to_string()).collect();
        let lats: Vec<String> = chunk.iter().map(|p| p[1].to_string()).collect();

        let url = format!(
            "{}?latitude={}&longitude={}",
            OPEN_METEO_URL,
            lats.join(","),
            lons.join(",")
        );

        let mut attempts = 0;
        let max_attempts = 3;
        let mut loop_resp = None;

        while attempts < max_attempts {
            attempts += 1;
            match client.get(&url).send().await {
                Ok(resp) => {
                    loop_resp = Some(resp);
                    break;
                },
                Err(e) => {
                    println!("Open-Meteo Request failed (attempt {}/{}): {}", attempts, max_attempts, e);
                    if attempts < max_attempts {
                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    } else {
                         return Err(format!("Open-Meteo Request failed after {} attempts: {}", max_attempts, e));
                    }
                }
            }
        }

        let resp = loop_resp.unwrap();

        if !resp.status().is_success() {
            return Err(format!("Open-Meteo API Error: {}", resp.status()));
        }

        let json: serde_json::Value = resp.json().await.map_err(|e| format!("Open-Meteo Parse error: {}", e))?;

        if let Some(elevations_array) = json.get("elevation").and_then(|v| v.as_array()) {
            for val in elevations_array {
                altitudes.push(val.as_f64().unwrap_or(0.0));
            }
        } else {
            return Err("Format de réponse Open-Meteo invalide".to_string());
        }
    }

    Ok(altitudes)
}
