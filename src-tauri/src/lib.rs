use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::env;

use dirs::config_dir;

// -------------------------------------------------------------------
// Environment and Path Helpers
// -------------------------------------------------------------------

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const DEV_ENV_CANDIDATES: [&str; 1] = ["../.env"];
const KEY_FRONTEND: &str = "VITE_MAPBOX_TOKEN";
const KEY_BACKEND: &str = "MAPBOX_TOKEN";
const SETTINGS_FILENAME: &str = "settings.json";

// Reads the APP_ENV variable from .env file, defaults to "prod".
fn get_app_env() -> String {
    eprintln!("DEBUG: get_app_env() called.");
    let env_path_option = find_dev_env_file();
    eprintln!("DEBUG: find_dev_env_file() returned: {:?}", env_path_option);

    if let Some(env_path) = env_path_option {
        eprintln!("DEBUG: Attempting to load .env from: {:?}", env_path);
        if dotenvy::from_path(&env_path).is_ok() {
            eprintln!("DEBUG: .env loaded successfully.");
        } else {
            eprintln!("DEBUG: Failed to load .env from: {:?}", env_path);
        }
    } else {
        eprintln!("DEBUG: No .env file path provided by find_dev_env_file().");
    }

    let app_env_var = env::var("APP_ENV");
    eprintln!("DEBUG: env::var(\"APP_ENV\") result: {:?}", app_env_var);

    app_env_var.unwrap_or_else(|_| {
        eprintln!("DEBUG: APP_ENV not found, defaulting to 'prod'.");
        "prod".to_string()
    })
}

// Gets the application's configuration directory based on the environment.
// e.g., C:\Users\<user>\AppData\Roaming\visuGPS\<env>
fn get_app_config_dir() -> Result<PathBuf, String> {
    let mut path = config_dir().ok_or("Failed to get config dir")?;
    path.push(APP_NAME);
    path.push(get_app_env());
    if fs::create_dir_all(&path).is_err() {
        return Err("Failed to create app config directory".into());
    }
    Ok(path)
}

// -------------------------------------------------------------------
// Helpers for Mapbox Token
// -------------------------------------------------------------------

fn get_config_path() -> Option<PathBuf> {
    let mut base = get_app_config_dir().ok()?;
    base.push("config.env"); // simple key=value store
    Some(base)
}

fn read_kv_file(path: &Path, key: &str) -> Option<String> {
    let file = std::fs::File::open(path).ok()?;
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') { continue; }
        if let Some(rest) = l.strip_prefix(&(key.to_string() + "=")) {
            return Some(trim_quotes(rest.trim()).to_string());
        }
        if let Some(rest) = l.strip_prefix(&(key.to_string() + " = ")) {
            return Some(trim_quotes(rest.trim()).to_string());
        }
        if let Some(eq_idx) = l.find('=') {
            let (k, v) = l.split_at(eq_idx);
            if k.trim() == key {
                return Some(trim_quotes(v[1..].trim()).to_string());
            }
        }
    }
    None
}

fn write_kv_file(path: &Path, key: &str, value: &str) -> Result<(), String> {
    let mut lines: Vec<String> = if path.exists() {
        std::fs::read_to_string(path)
            .map_err(|e| e.to_string())?
            .lines()
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };

    let mut found = false;
    for l in &mut lines {
        let trimmed = l.trim_start();
        if trimmed.starts_with(&(key.to_string() + "=")) || trimmed.starts_with(&(key.to_string() + " = ")) {
            *l = format!("{}={}", key, value);
            found = true;
            break;
        }
        if let Some(eq_idx) = trimmed.find('=') {
            if trimmed[..eq_idx].trim() == key {
                *l = format!("{}={}", key, value);
                found = true;
                break;
            }
        }
    }
    if !found {
        lines.push(format!("{}={}", key, value));
    }
    let mut out = lines.join("\n");
    if !out.ends_with('\n') { out.push('\n'); }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

fn trim_quotes(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('v') && s.ends_with('v')) {
        &s[1..s.len()-1]
    } else {
        s
    }
}

fn find_dev_env_file() -> Option<PathBuf> {
    eprintln!("DEBUG: find_dev_env_file() called. Current dir: {:?}", env::current_dir());
    for candidate in DEV_ENV_CANDIDATES {
        let p = Path::new(candidate);
        eprintln!("DEBUG: Checking candidate: {:?} (exists: {})", p, p.exists());
        if p.exists() { return Some(p.to_path_buf()); }
    }
    eprintln!("DEBUG: No .env file found in candidates.");
    None
}

// -------------------------------------------------------------------
// Helpers for Settings
// -------------------------------------------------------------------

// Gets the path to the user\'s settings.json file.
fn get_user_settings_path() -> Result<PathBuf, String> {
    let mut path = get_app_config_dir()?;
    path.push(SETTINGS_FILENAME);
    Ok(path)
}

// Temporarily synchronizes the user\'s settings file with the default one.
// This function will be replaced later with a more robust solution.
fn sync_settings_temp() -> Result<(), String> {
    let user_path = get_user_settings_path()?;
    if !user_path.exists() {
        let default_settings_str = include_str!("../settings.default.json");
        fs::write(&user_path, default_settings_str).map_err(|e| e.to_string())?;
    }
    Ok(())
}

// -------------------------------------------------------------------
// Public commands
// -------------------------------------------------------------------

#[tauri::command]
fn read_mapbox_token() -> String {
    if cfg!(debug_assertions) {
        if let Some(env_path) = find_dev_env_file() {
            if let Some(val) = read_kv_file(&env_path, KEY_FRONTEND) {
                return val;
            }
            if let Some(val) = read_kv_file(&env_path, KEY_BACKEND) {
                return val;
            }
        }
    }
    if let Some(cfg) = get_config_path() {
        if let Some(val) = read_kv_file(&cfg, KEY_BACKEND) {
            return val;
        }
    }
    String::new()
}

#[tauri::command]
fn write_mapbox_token(token: String) -> Result<(), String> {
    eprintln!("DEBUG: write_mapbox_token() called. Token: {}", token);
    eprintln!("DEBUG: cfg!(debug_assertions) is: {}", cfg!(debug_assertions));

    if cfg!(debug_assertions) {
        eprintln!("DEBUG: In debug_assertions block.");
        if let Some(env_path) = find_dev_env_file() {
            eprintln!("DEBUG: .env file found at: {:?}", env_path);
            return write_kv_file(&env_path, KEY_FRONTEND, &token);
        } else {
            eprintln!("DEBUG: find_dev_env_file() returned None. Falling back to AppData logic.");
        }
    } else {
        eprintln!("DEBUG: Not in debug_assertions block. Using AppData logic.");
    }

    if let Some(cfg) = get_config_path() {
        eprintln!("DEBUG: Saving to AppData path: {:?}", cfg);
        return write_kv_file(&cfg, KEY_BACKEND, &token);
    }
    eprintln!("DEBUG: Failed to determine user config directory.");
    Err("Impossible de déterminer le répertoire de configuration utilisateur".into())
}


#[tauri::command]
fn get_settings() -> Result<String, String> {
    let path = get_user_settings_path()?;
    fs::read_to_string(path).map_err(|e| e.to_string())
}

// -------------------------------------------------------------------
// Tauri entry point
// -------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init()) // Added plugin initialization
        .invoke_handler(tauri::generate_handler![
            read_mapbox_token,
            write_mapbox_token,
            get_settings
        ])
        .setup(|app| {
            // Initialize settings
            if let Err(e) = sync_settings_temp() {
                eprintln!("Failed to sync settings: {}", e);
            }

            // Logging (existing logic)
            if cfg!(debug_assertions) {
                let _ = app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                );
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
