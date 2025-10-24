const WS_SERVER_IP = "192.168.1.65"; // À remplacer par l'IP de votre machine desktop
const WS_SERVER_PORT = 9001;
const WS_URL = `ws://${WS_SERVER_IP}:${WS_SERVER_PORT}`;

let ws = null;
let clientId = localStorage.getItem('visugps_remote_client_id');
let pairingCode = generateRandomCode(8);

const statusDiv = document.getElementById('status');
const pairingCodeDiv = document.getElementById('pairing-code');
const controlsDiv = document.getElementById('controls');

// Fallback for crypto.randomUUID
function generateUUID() {
    if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
        return crypto.randomUUID();
    } else {
        // Fallback for environments without crypto.randomUUID (UUID v4)
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
            var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }
}

function generateRandomCode(length) {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
    const charactersLength = characters.length;
    for (let i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}

function updateStatus(message, isError = false) {
    statusDiv.textContent = `Statut: ${message}`;
    statusDiv.style.color = isError ? 'red' : 'green';
}

function updateRemoteInterface(appState) {
    // Masquer toutes les pages
    const pages = document.querySelectorAll('.page');
    pages.forEach(page => page.style.display = 'none');
    
    // Adapter l'interface de la télécommande selon l'état de l'application
    const statusText = document.getElementById('status');
    
    switch(appState) {
        case 'Visualize':
        case 'Visualisation':
            document.getElementById('page-visualize').style.display = 'block';
            statusText.textContent = `Statut: Connecté - Mode Visualisation`;
            statusText.style.color = 'green';
            break;
        case 'Edit':
        case 'EditView':
            document.getElementById('page-edit').style.display = 'block';
            statusText.textContent = `Statut: Connecté - Mode Édition`;
            statusText.style.color = 'blue';
            break;
        case 'Main':
        case 'MainView':
            document.getElementById('page-main').style.display = 'block';
            statusText.textContent = `Statut: Connecté - Vue Principale`;
            statusText.style.color = 'orange';
            break;
        case 'Settings':
        case 'SettingsView':
            document.getElementById('page-settings').style.display = 'block';
            statusText.textContent = `Statut: Connecté - Paramètres`;
            statusText.style.color = 'purple';
            break;
        default:
            // Page par défaut pour les vues non reconnues
            document.getElementById('page-default').style.display = 'block';
            document.getElementById('default-view-title').textContent = appState || 'Vue Inconnue';
            statusText.textContent = `Statut: Connecté - ${appState}`;
            statusText.style.color = 'green';
    }
}

function setupButtonListeners() {
    // Boutons de la page VisualizeView
    const playPauseBtn = document.getElementById('play-pause');
    if (playPauseBtn) {
        playPauseBtn.addEventListener('click', () => {
            sendCommand('toggle_play');
        });
    }
    
    const toggleCommandsBtn = document.getElementById('toggle-commands');
    if (toggleCommandsBtn) {
        toggleCommandsBtn.addEventListener('click', () => {
            sendCommand('toggle_commands_widget');
        });
    }
    
    const toggleProfileBtn = document.getElementById('toggle-profile');
    if (toggleProfileBtn) {
        toggleProfileBtn.addEventListener('click', () => {
            sendCommand('toggle_altitude_profile');
        });
    }
    
    const toggleCommunesBtn = document.getElementById('toggle-communes');
    if (toggleCommunesBtn) {
        toggleCommunesBtn.addEventListener('click', () => {
            sendCommand('toggle_communes_display');
        });
    }
    
    const toggleDistanceBtn = document.getElementById('toggle-distance');
    if (toggleDistanceBtn) {
        toggleDistanceBtn.addEventListener('click', () => {
            sendCommand('toggle_distance_display');
        });
    }
    
    // Bouton Accueil (présent sur plusieurs pages)
    const goHomeBtn = document.getElementById('go-home');
    if (goHomeBtn) {
        goHomeBtn.addEventListener('click', () => {
            sendCommand('toggle_home');
        });
    }
    
    // Boutons de la page EditView
    const saveCircuitBtn = document.getElementById('save-circuit');
    if (saveCircuitBtn) {
        saveCircuitBtn.addEventListener('click', () => {
            sendCommand('save_circuit');
        });
    }
    
    const previewCircuitBtn = document.getElementById('preview-circuit');
    if (previewCircuitBtn) {
        previewCircuitBtn.addEventListener('click', () => {
            sendCommand('preview_circuit');
        });
    }
}

function sendCommand(command) {
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({ 
            type: "command", 
            clientId: clientId, 
            command: command 
        }));
        console.log(`Commande envoyée: ${command}`);
    } else {
        console.error('WebSocket non connecté');
    }
}

function connectWebSocket() {
    updateStatus("Tentative de connexion...");
    ws = new WebSocket(WS_URL);

    ws.onopen = () => {
        updateStatus("Connecté au serveur WebSocket.");
        if (!clientId) {
            clientId = generateUUID();
            localStorage.setItem('visugps_remote_client_id', clientId);
            console.log("Nouveau Client ID généré et stocké :", clientId);
        }
        pairingCodeDiv.textContent = `Code de couplage : ${pairingCode}`;
        console.log("Client ID :", clientId);
        console.log("Code de couplage :", pairingCode);

        // Envoyer la requête de couplage
        ws.send(JSON.stringify({
            type: "pairing_request",
            clientId: clientId,
            pairingCode: pairingCode
        }));
    };

    ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        console.log("Message reçu du serveur :", message);

        if (message.type === "pairing_response") {
            if (message.status === "accepted") {
                updateStatus("Couplage accepté !", false);
                pairingCodeDiv.style.display = 'none';
                // Afficher la page appropriée selon l'état de l'application
                if (message.appState) {
                    updateRemoteInterface(message.appState);
                } else {
                    // Par défaut, afficher la page de visualisation
                    updateRemoteInterface('Visualize');
                }
            } else if (message.status === "refused") {
                updateStatus(`Couplage refusé : ${message.reason || "Raison inconnue"}`, true);
                pairingCodeDiv.style.display = 'block'; // Garder le code visible en cas de refus
                // Masquer toutes les pages
                const pages = document.querySelectorAll('.page');
                pages.forEach(page => page.style.display = 'none');
            } else if (message.status === "already_paired") {
                updateStatus("Déjà couplé.", false);
                pairingCodeDiv.style.display = 'none';
                // Afficher la page appropriée selon l'état de l'application
                if (message.appState) {
                    updateRemoteInterface(message.appState);
                } else {
                    // Par défaut, afficher la page de visualisation
                    updateRemoteInterface('Visualize');
                }
            }
        } else if (message.type === "app_state_update") {
            // Gérer les mises à jour de l'état de l'application
            console.log("État de l'application :", message.appState);
            updateRemoteInterface(message.appState);
        }
    };

    ws.onclose = (event) => {
        updateStatus(`Déconnecté du serveur. Code: ${event.code}, Raison: ${event.reason}`, true);
        // Masquer toutes les pages
        const pages = document.querySelectorAll('.page');
        pages.forEach(page => page.style.display = 'none');
        // Tenter de se reconnecter après un délai
        setTimeout(connectWebSocket, 3000);
    };

    ws.onerror = (error) => {
        updateStatus("Erreur de connexion WebSocket.", true);
        console.error("Erreur WebSocket :", error);
        ws.close();
    };
}

// Initialisation
window.onload = () => {
    if (!clientId) {
        pairingCodeDiv.textContent = `Génération d'un ID client...`;
    }
    connectWebSocket();

    // Ajout des écouteurs d'événements pour les boutons
    setupButtonListeners();
};
