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

pub fn read_blacklist_file(app_env_path: &PathBuf) -> Result<BlacklistFile, String> {
    let path = get_blacklist_path(app_env_path);
    if !path.exists() {
        log::debug!("Blacklist file {:?} does not exist, using default.", path);
        return Ok(BlacklistFile::default());
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    if content.trim().is_empty() {
        log::debug!("Blacklist file {:?} is empty, using default.", path);
        return Ok(BlacklistFile::default());
    }
    
    match serde_json::from_str::<BlacklistFile>(&content) {
        Ok(data) => {
            log::debug!("Read blacklist file with {} clients.", data.blacklisted_clients.len());
            Ok(data)
        },
        Err(e) => {
            log::error!("Failed to parse blacklist file: {}. Returning default.", e);
            Ok(BlacklistFile::default())
        }
    }
}

fn write_blacklist_file(app_env_path: &PathBuf, data: &BlacklistFile) -> Result<(), String> {
    let path = get_blacklist_path(app_env_path);
    let content = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| e.to_string())
}

pub fn add_to_blacklist(app_env_path: &PathBuf, client_id: String, reason: String) -> Result<(), String> {
    let mut blacklist = read_blacklist_file(app_env_path)?;
    if !blacklist.blacklisted_clients.iter().any(|c| c.client_id == client_id) {
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

pub fn prune_blacklisted_clients(app_env_path: &PathBuf, max_days: i64) -> Result<(), String> {
    let mut blacklist = read_blacklist_file(app_env_path)?;
    let now = Utc::now();
    let initial_len = blacklist.blacklisted_clients.len();
    
    blacklist.blacklisted_clients.retain(|c| {
        let duration = now.signed_duration_since(c.timestamp);
        duration.num_days() < max_days
    });

    if blacklist.blacklisted_clients.len() < initial_len {
        write_blacklist_file(app_env_path, &blacklist)?;
        info!("Pruned {} old blacklisted clients", initial_len - blacklist.blacklisted_clients.len());
    }
    
    Ok(())
}

pub fn clear_blacklist(app_env_path: &PathBuf) -> Result<(), String> {
    let blacklist = BlacklistFile::default();
    write_blacklist_file(app_env_path, &blacklist)?;
    info!("Remote blacklist cleared");
    Ok(())
}
