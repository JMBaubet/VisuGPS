use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use log::info;
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlacklistedClient {
    pub client_id: String,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlacklistFile {
    pub version: String,
    pub description: String,
    pub blacklisted_clients: Vec<BlacklistedClient>,
}

impl Default for BlacklistFile {
    fn default() -> Self {
        BlacklistFile {
            version: "1.0".to_string(),
            description: "Liste des clients de télécommande bloqués.".to_string(),
            blacklisted_clients: Vec::new(),
        }
    }
}

fn get_blacklist_path(app_env_path: &PathBuf) -> PathBuf {
    app_env_path.join("remote_blacklist.json")
}

fn read_blacklist_file(app_env_path: &PathBuf) -> Result<BlacklistFile, String> {
    let path = get_blacklist_path(app_env_path);
    if !path.exists() {
        return Ok(BlacklistFile::default());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

fn write_blacklist_file(app_env_path: &PathBuf, data: &BlacklistFile) -> Result<(), String> {
    let path = get_blacklist_path(app_env_path);
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

pub fn add_to_blacklist(app_env_path: &PathBuf, client_id: String, reason: String) -> Result<(), String> {
    let mut blacklist = read_blacklist_file(app_env_path)?;
    if !blacklist.blacklisted_clients.iter().any(|c| c.client_id == client_id) {
        info!("Ajout du client {} à la blacklist.", client_id);
        let new_entry = BlacklistedClient {
            client_id,
            reason,
            timestamp: Utc::now(),
        };
        blacklist.blacklisted_clients.push(new_entry);
        write_blacklist_file(app_env_path, &blacklist)?;
    }
    Ok(())
}

pub fn is_client_blacklisted(app_env_path: &PathBuf, client_id: &str) -> Result<bool, String> {
    let blacklist = read_blacklist_file(app_env_path)?;
    Ok(blacklist.blacklisted_clients.iter().any(|c| c.client_id == client_id))
}
