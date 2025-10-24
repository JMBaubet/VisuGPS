use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use log::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteClient {
    pub client_id: String,
    pub name: String, // Nom donné par l'utilisateur lors du couplage
    pub last_seen: String, // Date/heure de la dernière connexion
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteClientsFile {
    pub version: String,
    pub description: String,
    pub clients: Vec<RemoteClient>,
}

impl Default for RemoteClientsFile {
    fn default() -> Self {
        RemoteClientsFile {
            version: "1.0".to_string(),
            description: "Fichier de configuration des clients de télécommande autorisés.".to_string(),
            clients: Vec::new(),
        }
    }
}

pub fn get_remote_clients_path(app_env_path: &PathBuf) -> PathBuf {
    app_env_path.join("remote.json")
}

pub fn read_remote_clients(app_env_path: &PathBuf) -> Result<RemoteClientsFile, String> {
    let path = get_remote_clients_path(app_env_path);
    if !path.exists() {
        info!("Le fichier remote.json n'existe pas, création d'un fichier par défaut.");
        let default_clients = RemoteClientsFile::default();
        write_remote_clients(app_env_path, &default_clients)?;
        return Ok(default_clients);
    }

    let file_content = fs::read_to_string(&path)
        .map_err(|e| format!("Impossible de lire remote.json: {}", e))?;
    serde_json::from_str(&file_content)
        .map_err(|e| format!("Impossible de parser remote.json: {}", e))
}

pub fn write_remote_clients(app_env_path: &PathBuf, clients_data: &RemoteClientsFile) -> Result<(), String> {
    let path = get_remote_clients_path(app_env_path);
    let new_content = serde_json::to_string_pretty(clients_data)
        .map_err(|e| format!("Impossible de sérialiser remote.json: {}", e))?;
    fs::write(&path, new_content)
        .map_err(|e| format!("Impossible d'écrire remote.json: {}", e))
}

pub fn is_client_authorized(app_env_path: &PathBuf, client_id: &str) -> Result<bool, String> {
    let clients_file = read_remote_clients(app_env_path)?;
    Ok(clients_file.clients.iter().any(|c| c.client_id == client_id))
}

pub fn add_authorized_client(app_env_path: &PathBuf, client_id: String, name: String) -> Result<(), String> {
    let mut clients_file = read_remote_clients(app_env_path)?;
    if !clients_file.clients.iter().any(|c| c.client_id == client_id) {
        let new_client = RemoteClient {
            client_id,
            name,
            last_seen: chrono::Utc::now().to_rfc3339(),
        };
        clients_file.clients.push(new_client);
        write_remote_clients(app_env_path, &clients_file)?;
    }
    Ok(())
}

pub fn update_client_last_seen(app_env_path: &PathBuf, client_id: &str) -> Result<(), String> {
    let mut clients_file = read_remote_clients(app_env_path)?;
    if let Some(client) = clients_file.clients.iter_mut().find(|c| c.client_id == client_id) {
        client.last_seen = chrono::Utc::now().to_rfc3339();
        write_remote_clients(app_env_path, &clients_file)?;
    }
    Ok(())
}

pub fn remove_authorized_client(app_env_path: &PathBuf, client_id_to_remove: &str) -> Result<(), String> {
    let mut clients_file = read_remote_clients(app_env_path)?;
    
    let initial_len = clients_file.clients.len();
    clients_file.clients.retain(|c| c.client_id != client_id_to_remove);

    if clients_file.clients.len() < initial_len {
        write_remote_clients(app_env_path, &clients_file)?;
    }
    
    Ok(())
}
