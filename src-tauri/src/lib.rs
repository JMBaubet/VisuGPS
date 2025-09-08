use tauri::{App, Manager, State};
use std::fs;
use std::path::PathBuf;

#[derive(serde::Serialize, Clone)]
pub struct AppState {
    app_env: String,
    execution_mode: String,
    app_env_path: PathBuf,
}

#[tauri::command]
fn get_app_state(state: State<AppState>) -> AppState {
    state.inner().clone()
}

fn get_execution_mode(app_env: &str) -> String {
    if app_env.starts_with("Sandbox_") {
        "EVAL".to_string()
    } else if app_env.starts_with("Test_") {
        "TEST".to_string()
    } else {
        "OPE".to_string()
    }
}

fn setup_environment(app: &mut App) -> Result<AppState, Box<dyn std::error::Error>> {
    let app_data_dir = app.path().app_data_dir()?;
    let visugps_dir = app_data_dir.join("VisuGPS");

    if !visugps_dir.exists() {
        fs::create_dir_all(&visugps_dir)?;
    }

    let env_path = visugps_dir.join(".env");

    if !env_path.exists() {
        fs::write(&env_path, "APP_ENV=OPE")?;
    }

    let mut app_env = "OPE".to_string();
    if let Ok(iter) = dotenvy::from_path_iter(&env_path) {
        for item in iter {
            if let Ok((key, val)) = item {
                if key == "APP_ENV" {
                    app_env = val;
                    break;
                }
            }
        }
    }

    let execution_mode = get_execution_mode(&app_env);
    
    // Create the environment-specific directory
    let app_env_path = visugps_dir.join(&app_env);
    if !app_env_path.exists() {
        fs::create_dir_all(&app_env_path)?;
    }

    Ok(AppState {
        app_env,
        execution_mode,
        app_env_path,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let state = setup_environment(app)?;
            app.manage(state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_app_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
