use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::env;

use dirs::config_dir;
use serde_json::Value;

// -------------------------------------------------------------------
// Environment and Path Helpers
// -------------------------------------------------------------------

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const DEV_ENV_CANDIDATES: [&str; 3] = ["../.env", "./.env", "../../.env"];
const KEY_FRONTEND: &str = "VITE_MAPBOX_TOKEN";
const KEY_BACKEND: &str = "MAPBOX_TOKEN";
const SETTINGS_FILENAME: &str = "settings.json";

// Reads the APP_ENV variable from .env file, defaults to "prod".
fn get_app_env() -> String {
    if let Some(env_path) = find_dev_env_file() {
        dotenvy::from_path(&env_path).ok();
    }
    env::var("APP_ENV").unwrap_or_else(|_| "prod".to_string())
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
    for candidate in DEV_ENV_CANDIDATES {
        let p = Path::new(candidate);
        if p.exists() { return Some(p.to_path_buf()); }
    }
    None
}

// -------------------------------------------------------------------
// Helpers for Settings
// -------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct Setting {
    nom: String,
    description: String,
    documentation: Option<String>,
    #[serde(rename = "type")]
    setting_type: String,
    unite: String,
    valeur_par_defaut: Value,
    valeur_de_surcharge: Option<Value>,
    valeur_min: Option<Value>,
    valeur_max: Option<Value>,
    critique: bool,
    arbre: String,
}

// Gets the path to the user's settings.json file.
fn get_user_settings_path() -> Result<PathBuf, String> {
    let mut path = get_app_config_dir()?;
    path.push(SETTINGS_FILENAME);
    Ok(path)
}

// Synchronizes the user's settings file with the default one.
fn sync_settings() -> Result<(), String> {
    let user_path = get_user_settings_path()?;
    // Embed the default settings string directly into the binary at compile time.
    let default_settings_str = include_str!("../settings.default.json");

    let mut default_settings: Vec<Setting> = serde_json::from_str(&default_settings_str).map_err(|e| e.to_string())?;

    let user_settings: Vec<Setting> = if user_path.exists() {
        let user_settings_str = fs::read_to_string(&user_path).map_err(|e| e.to_string())?;
        // If file is corrupt, start fresh from default
        serde_json::from_str(&user_settings_str).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    let user_settings_map: HashMap<String, Setting> = user_settings.into_iter().map(|s| (s.nom.clone(), s)).collect();

    // Iterate through default settings. If a setting exists in user_settings, keep its override value.
    // Otherwise, add the new default setting.
    let mut final_settings = Vec::new();
    for default_setting in default_settings.iter_mut() {
        let mut final_setting = default_setting.clone();
        if let Some(user_setting) = user_settings_map.get(&default_setting.nom) {
            final_setting.valeur_de_surcharge = user_setting.valeur_de_surcharge.clone();
        }
        final_settings.push(final_setting);
    }

    let final_settings_str = serde_json::to_string_pretty(&final_settings).map_err(|e| e.to_string())?;
    fs::write(user_path, final_settings_str).map_err(|e| e.to_string())?;

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
    if cfg!(debug_assertions) {
        if let Some(env_path) = find_dev_env_file() {
            return write_kv_file(&env_path, KEY_FRONTEND, &token);
        }
    }
    if let Some(cfg) = get_config_path() {
        return write_kv_file(&cfg, KEY_BACKEND, &token);
    }
    Err("Impossible de déterminer le répertoire de configuration utilisateur".into())
}

#[tauri::command]
fn get_settings() -> Result<String, String> {
    let path = get_user_settings_path()?;
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_settings(settings: Vec<Setting>) -> Result<(), String> {
    let path = get_user_settings_path()?;
    let current_settings_str = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut current_settings: Vec<Setting> = serde_json::from_str(&current_settings_str).map_err(|e| e.to_string())?;

    let new_settings_map: HashMap<String, Setting> = settings.into_iter().map(|s| (s.nom.clone(), s)).collect();

    for setting in &mut current_settings {
        if let Some(new_setting) = new_settings_map.get(&setting.nom) {
            setting.valeur_de_surcharge = new_setting.valeur_de_surcharge.clone();
        }
    }

    let updated_settings_str = serde_json::to_string_pretty(&current_settings).map_err(|e| e.to_string())?;
    fs::write(path, updated_settings_str).map_err(|e| e.to_string())?;

    Ok(())
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
            get_settings,
            save_settings
        ])
        .setup(|app| {
            // Initialize settings
            if let Err(e) = sync_settings() {
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
