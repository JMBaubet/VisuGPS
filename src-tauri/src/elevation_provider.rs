// No imports needed for now as we use serde_json::Value
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

// Global locks to prevent concurrent requests to rate-limited APIs
static OPEN_METEO_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
static OPENTOPODATA_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

#[derive(Debug, PartialEq)]
pub enum ElevationProvider {
    IGN,
    OpenMeteo,
    OpenTopoData,
}

impl std::fmt::Display for ElevationProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ElevationProvider::IGN => write!(f, "IGN"),
            ElevationProvider::OpenMeteo => write!(f, "Open-Meteo"),
            ElevationProvider::OpenTopoData => write!(f, "OpenTopoData"),
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
const OPEN_METEO_URL: &str = "https://api.open-meteo.com/v1/elevation";
const OPENTOPODATA_URL: &str = "https://api.opentopodata.org/v1/srtm30m";

pub async fn fetch_altitudes(points: &Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    if points.is_empty() {
        return Ok(Vec::new());
    }

    let provider = get_best_provider(points);
    println!("Utilisation du fournisseur d'altitude : {}", provider);

    let result = match provider {
        ElevationProvider::IGN => fetch_ign(points).await,
        ElevationProvider::OpenMeteo => fetch_open_meteo(points).await,
        ElevationProvider::OpenTopoData => fetch_opentopodata(points).await,
    };

    // Fallback logic: if primary provider fails, try OpenTopoData (unless it was already the primary)
    match result {
        Ok(altitudes) => Ok(altitudes),
        Err(e) => {
            if provider != ElevationProvider::OpenTopoData {
                println!("Fournisseur d'altitude {} en échec : {}. Tentative de secours via OpenTopoData...", provider, e);
                fetch_opentopodata(points).await
            } else {
                Err(e)
            }
        }
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

        let mut attempts = 0;
        let max_attempts = 3;
        let mut loop_resp = None;

        while attempts < max_attempts {
            attempts += 1;
            match client.get(&url).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        loop_resp = Some(resp);
                        break;
                    } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS 
                           || status == reqwest::StatusCode::INTERNAL_SERVER_ERROR
                           || status == reqwest::StatusCode::SERVICE_UNAVAILABLE {
                        println!("IGN API Error {} (attempt {}/{}). Retrying...", status, attempts, max_attempts);
                        if attempts < max_attempts {
                            let wait_time = std::time::Duration::from_millis(1000 * 2_u64.pow(attempts as u32 - 1));
                            tokio::time::sleep(wait_time).await;
                        }
                    } else {
                        // Other errors (400, 404, etc.) are likely fatal
                        return Err(format!("IGN API Error: {}", status));
                    }
                },
                Err(e) => {
                    println!("IGN Request failed (attempt {}/{}): {}", attempts, max_attempts, e);
                    if attempts < max_attempts {
                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    } else {
                         return Err(format!("IGN Request failed after {} attempts: {}", max_attempts, e));
                    }
                }
            }
        }

        if loop_resp.is_none() {
             return Err(format!("IGN API failed after {} attempts.", max_attempts));
        }
        let resp = loop_resp.unwrap();

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

        // Throttle - Be nice to the API
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    
    Ok(altitudes)
}


// Public function to check status with lock
pub async fn check_open_meteo_status() -> String {
    // Try to acquire the lock without waiting
    let _guard = match OPEN_METEO_LOCK.try_lock() {
        Ok(g) => g,
        Err(_) => return "BUSY".to_string(), // Lock is held, meaning a fetch is in progress
    };

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5)) // Short timeout for check
        .build()
        .unwrap_or_default();

    // Simple request to check connectivity
    let url = format!("{}?latitude=52.52&longitude=13.41&elevation=nan", OPEN_METEO_URL);

    match client.get(&url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                "OK".to_string()
            } else if resp.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                "RATE_LIMITED".to_string()
            } else {
                "UNREACHABLE".to_string()
            }
        },
        Err(_) => "UNREACHABLE".to_string(),
    }
}

async fn fetch_open_meteo(points: &Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    // Acquire the lock to ensure sequential requests
    let _lock = OPEN_METEO_LOCK.lock().await;

    let client = reqwest::Client::new();
    let mut altitudes = Vec::new();
    let total_chunks = (points.len() as f64 / 90.0).ceil() as usize;

    // Reduced chunk size to 90 to avoid URL too long issues and "bursty" behavior
    for (i, chunk) in points.chunks(90).enumerate() {
        println!("Open-Meteo: Fetching chunk {}/{} ({} points)", i + 1, total_chunks, chunk.len());
        
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
                    let status = resp.status();
                    if status.is_success() {
                        loop_resp = Some(resp);
                        break;
                    } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        println!("Open-Meteo Rate Limit 429 (attempt {}/{}). Retrying...", attempts, max_attempts);
                        if attempts < max_attempts {
                            let wait_time = std::time::Duration::from_millis(1500 * 2_u64.pow(attempts as u32));
                            tokio::time::sleep(wait_time).await;
                        }
                    } else {
                         return Err(format!("Open-Meteo API Error: {}", status));
                    }
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

        if loop_resp.is_none() {
             return Err(format!("Open-Meteo API failed after {} attempts (likely Rate Limit)", max_attempts));
        }
        let resp = loop_resp.unwrap();

        let json: serde_json::Value = resp.json().await.map_err(|e| format!("Open-Meteo Parse error: {}", e))?;

        if let Some(elevations_array) = json.get("elevation").and_then(|v| v.as_array()) {
            for val in elevations_array {
                altitudes.push(val.as_f64().unwrap_or(0.0));
            }
        } else {
            return Err("Format de réponse Open-Meteo invalide".to_string());
        }
        
        // Increased Throttle - Be nicer to the API (800ms pause)
        tokio::time::sleep(std::time::Duration::from_millis(800)).await;
    }

    Ok(altitudes)
}

async fn fetch_opentopodata(points: &Vec<[f64; 2]>) -> Result<Vec<f64>, String> {
    // Acquire the lock to ensure sequential requests
    let _lock = OPENTOPODATA_LOCK.lock().await;

    let client = reqwest::Client::new();
    let mut altitudes = Vec::new();

    // Respect limitations: Max 100 locations per request
    for chunk in points.chunks(100) {
        let coords: Vec<String> = chunk.iter().map(|p| format!("{},{}", p[1], p[0])).collect();
        let url = format!(
            "{}?locations={}",
            OPENTOPODATA_URL,
            coords.join("|")
        );

        let mut attempts = 0;
        let max_attempts = 3;
        let mut loop_resp = None;

        while attempts < max_attempts {
            attempts += 1;
            match client.get(&url).send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        loop_resp = Some(resp);
                        break;
                    } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        println!("OpenTopoData Rate Limit 429 (attempt {}/{}). Retrying...", attempts, max_attempts);
                        if attempts < max_attempts {
                            let wait_time = std::time::Duration::from_millis(2000 * 2_u64.pow(attempts as u32 - 1));
                            tokio::time::sleep(wait_time).await;
                        }
                    } else {
                         return Err(format!("OpenTopoData API Error: {}", status));
                    }
                },
                Err(e) => {
                    println!("OpenTopoData Request failed (attempt {}/{}): {}", attempts, max_attempts, e);
                    if attempts < max_attempts {
                        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    } else {
                         return Err(format!("OpenTopoData Request failed after {} attempts: {}", max_attempts, e));
                    }
                }
            }
        }

        if loop_resp.is_none() {
             return Err(format!("OpenTopoData API failed after {} attempts (likely Rate Limit)", max_attempts));
        }
        let resp = loop_resp.unwrap();

        let json: serde_json::Value = resp.json().await.map_err(|e| format!("OpenTopoData Parse error: {}", e))?;

        if let Some(results) = json.get("results").and_then(|v| v.as_array()) {
            for res in results {
                altitudes.push(res.get("elevation").and_then(|v| v.as_f64()).unwrap_or(0.0));
            }
        } else {
            return Err("Format de réponse OpenTopoData invalide".to_string());
        }
        
        // Respect limitations: Max 1 call per second
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
    }

    Ok(altitudes)
}
