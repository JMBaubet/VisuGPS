use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use dirs::config_dir;

// -------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------

const APP_DIR: &str = env!("CARGO_PKG_NAME"); // folder name under user's config dir
const DEV_ENV_CANDIDATES: [&str; 3] = ["../.env", "./.env", "../../.env"];
const KEY_FRONTEND: &str = "VITE_MAPBOX_TOKEN"; // used in dev .env
const KEY_BACKEND: &str = "MAPBOX_TOKEN";       // used in prod config

fn get_config_path() -> Option<PathBuf> {
    let mut base = config_dir()?;
    base.push(APP_DIR);
    if std::fs::create_dir_all(&base).is_err() {
        return None;
    }
    base.push("config.env"); // simple key=value store
    Some(base)
}

fn read_kv_file(path: &Path, key: &str) -> Option<String> {
    let file = std::fs::File::open(path).ok()?;
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') { continue; }
        // Accept KEY=value or KEY = value
        if let Some(rest) = l.strip_prefix(&(key.to_string() + "=")) {
            return Some(trim_quotes(rest.trim()).to_string());
        }
        if let Some(rest) = l.strip_prefix(&(key.to_string() + " = ")) {
            return Some(trim_quotes(rest.trim()).to_string());
        }
        // Also allow `key = "value"` with any spacing around '='
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
        if trimmed.starts_with(&(key.to_string() + "=")) || trimmed.starts_with(&(key.to_string() + " =")) {
            *l = format!("{}={}", key, value);
            found = true;
            break;
        }
        // Also check 'key   =   value'
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
    let mut out = lines.join("
");
    if !out.ends_with('\n') { out.push('\n'); }
    std::fs::write(path, out).map_err(|e| e.to_string())
}

fn trim_quotes(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
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
// Public commands (names preserved for VueJS compatibility)
// -------------------------------------------------------------------

#[tauri::command]
fn read_mapbox_token() -> String {
    // DEV: first try .env so your current workflow keeps working
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

    // PROD (or fallback): read persisted config file
    if let Some(cfg) = get_config_path() {
        if let Some(val) = read_kv_file(&cfg, KEY_BACKEND) {
            return val;
        }
    }

    String::new() // keep signature; return empty if not found
}

#[tauri::command]
fn write_mapbox_token(token: String) -> Result<(), String> {
    // DEV: if a .env file exists, update it to keep the same DX you had
    if cfg!(debug_assertions) {
        if let Some(env_path) = find_dev_env_file() {
            // Prefer writing frontend key in dev
            return write_kv_file(&env_path, KEY_FRONTEND, &token);
        }
    }

    // PROD (or if no .env): write to app config file
    if let Some(cfg) = get_config_path() {
        return write_kv_file(&cfg, KEY_BACKEND, &token);
    }

    Err("Impossible de déterminer le répertoire de configuration utilisateur".into())
}

// -------------------------------------------------------------------
// Tauri entry point
// -------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_mapbox_token, write_mapbox_token])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
