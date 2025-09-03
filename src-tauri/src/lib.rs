use std::fs;
use std::path::Path;
use std::io::{BufReader, BufRead};

#[tauri::command]
fn read_mapbox_token() -> String {
    let env_path_str = "../.env"; // Assuming CWD is src-tauri during dev
    let key_to_find = "VITE_MAPBOX_TOKEN";

    if let Ok(file) = fs::File::open(env_path_str) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(l) = line {
                if l.starts_with(&format!("{} = ", key_to_find)) { // Added space for robustness
                    return l.trim_start_matches(&format!("{} = ", key_to_find)).to_string();
                } else if l.starts_with(&format!("{} =", key_to_find)) { // Without space
                    return l.trim_start_matches(&format!("{} =", key_to_find)).to_string();
                }
            }
        }
    }
    // If file not found or key not found, return default
    String::default()
}

#[tauri::command]
fn write_mapbox_token(token: String) -> Result<(), String> {
    let env_path_str = "../.env"; // Assuming CWD is src-tauri during dev
    let key_to_set = "VITE_MAPBOX_TOKEN";
    
    let lines: Vec<String> = if Path::new(env_path_str).exists() {
        fs::read_to_string(env_path_str)
            .map_err(|e| e.to_string())? 
            .lines()
            .map(String::from)
            .collect()
    } else {
        Vec::new()
    };

    let mut new_lines: Vec<String> = Vec::new();
    let mut found = false;

    for line in lines {
        if line.starts_with(&format!("{} =", key_to_set)) {
            new_lines.push(format!("{} = {}", key_to_set, token));
            found = true;
        } else {
            new_lines.push(line);
        }
    }

    if !found {
        new_lines.push(format!("{} = {}", key_to_set, token));
    }

    let new_content = new_lines.join("\n");
    fs::write(env_path_str, new_content).map_err(|e| e.to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![read_mapbox_token, write_mapbox_token]) // Removed read_mapbox_username, write_mapbox_username
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