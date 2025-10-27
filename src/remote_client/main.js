const WS_SERVER_IP = "192.168.1.65"; // À remplacer par l'IP de votre machine desktop
const WS_SERVER_PORT = 9001;
const WS_URL = `ws://${WS_SERVER_IP}:${WS_SERVER_PORT}`;

let ws = null;
let clientId = localStorage.getItem('visugps_remote_client_id');
let pairingCode = generateRandomCode(8);
let manualDisconnect = false; // Flag to prevent reconnect on server-side disconnect

// Variables pour la gestion des tentatives de reconnexion
let retryCount = 0;
const MAX_RETRY_ATTEMPTS = 3;
let isRetrying = false;
let retryTimeout = null;

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

function resetRetryCount() {
    retryCount = 0;
    isRetrying = false;
    if (retryTimeout) {
        clearTimeout(retryTimeout);
        retryTimeout = null;
    }
}

function showRetryButton() {
    // Créer un bouton de reconnexion manuelle si il n'existe pas
    let retryButton = document.getElementById('retry-button');
    if (!retryButton) {
        retryButton = document.createElement('button');
        retryButton.id = 'retry-button';
        retryButton.textContent = '🔄 Réessayer la connexion';
        retryButton.style.cssText = `
            display: block;
            width: 100%;
            padding: 15px;
            margin-top: 15px;
            background-color: #28a745;
            color: white;
            border: none;
            border-radius: 8px;
            cursor: pointer;
            font-size: 1.1em;
            transition: background-color 0.3s;
        `;
        retryButton.addEventListener('click', () => {
            retryButton.remove();
            resetRetryCount();
            connectWebSocket();
        });
        statusDiv.parentNode.insertBefore(retryButton, statusDiv.nextSibling);
    }
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

function handleHijackedConnection() {
    manualDisconnect = true; // Prevent any further reconnection attempts
    updateStatus("Connexion privée. Cet appareil n'est plus autorisé.", true);
    
    const pages = document.querySelectorAll('.page');
    pages.forEach(page => page.style.display = 'none');

    // Ensure the retry button is removed
    const retryButton = document.getElementById('retry-button');
    if (retryButton) {
        retryButton.remove();
    }

    ws.close();
}

function setupButtonListeners() {
    // --- Page VisualizeView ---
    const playPauseBtn = document.getElementById('play-pause');
    if (playPauseBtn) {
        playPauseBtn.addEventListener('click', () => sendCommand('toggle_play'));
    }

    // Switches
    const toggleCommandsSwitch = document.getElementById('toggle-commands');
    if (toggleCommandsSwitch) {
        toggleCommandsSwitch.addEventListener('change', () => sendCommand('toggle_commands_widget'));
    }

    const toggleProfileSwitch = document.getElementById('toggle-profile');
    if (toggleProfileSwitch) {
        toggleProfileSwitch.addEventListener('change', () => sendCommand('toggle_altitude_profile'));
    }

    const toggleCommunesSwitch = document.getElementById('toggle-communes');
    if (toggleCommunesSwitch) {
        toggleCommunesSwitch.addEventListener('change', () => sendCommand('toggle_communes_display'));
    }

    const toggleDistanceSwitch = document.getElementById('toggle-distance');
    if (toggleDistanceSwitch) {
        toggleDistanceSwitch.addEventListener('change', () => sendCommand('toggle_distance_display'));
    }

    const goHomeSwitch = document.getElementById('go-home');
    if (goHomeSwitch) {
        goHomeSwitch.addEventListener('change', () => sendCommand('toggle_home'));
    }

    // --- Page EditView ---
    const saveCircuitBtn = document.getElementById('save-circuit');
    if (saveCircuitBtn) {
        saveCircuitBtn.addEventListener('click', () => sendCommand('save_circuit'));
    }

    const previewCircuitBtn = document.getElementById('preview-circuit');
    if (previewCircuitBtn) {
        previewCircuitBtn.addEventListener('click', () => sendCommand('preview_circuit'));
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
    manualDisconnect = false; // Reset the flag on new connection attempt
    if (isRetrying) {
        console.log("Tentative de reconnexion déjà en cours, ignorée.");
        return;
    }
    
    isRetrying = true;
    updateStatus(`Tentative de connexion... (${retryCount + 1}/${MAX_RETRY_ATTEMPTS})`);
    ws = new WebSocket(WS_URL);

    ws.onopen = () => {
        // Connexion réussie, réinitialiser le compteur
        resetRetryCount();
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
                // Afficher la page appropriée selon l'état de l'application reçu du serveur
                console.log("Message de couplage reçu:", message);
                if (message.appState) {
                    console.log("✅ État de l'application reçu lors du couplage:", message.appState);
                    updateRemoteInterface(message.appState);
                } else {
                    console.log("⚠️ Aucun état d'application reçu, utilisation de la page par défaut");
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
                // Afficher la page appropriée selon l'état de l'application reçu du serveur
                if (message.appState) {
                    console.log("État de l'application reçu lors du couplage (déjà couplé):", message.appState);
                    updateRemoteInterface(message.appState);
                } else {
                    console.log("Aucun état d'application reçu (déjà couplé), utilisation de la page par défaut");
                    // Par défaut, afficher la page de visualisation
                    updateRemoteInterface('Visualize');
                }
            }
        } else if (message.type === "app_state_update") {
            // Security check: ensure the message is for this client
            if (message.clientId && message.clientId !== clientId) {
                handleHijackedConnection();
                return;
            }
            // Gérer les mises à jour de l'état de l'application
            console.log("État de l'application :", message.appState);
            updateRemoteInterface(message.appState);
        } else if (message.type === "server_shutdown") {
            manualDisconnect = true;
            updateStatus(message.reason || "Déconnexion demandée par le serveur", true);

            // Masquer les pages immédiatement pour une UI réactive
            const pages = document.querySelectorAll('.page');
            pages.forEach(page => page.style.display = 'none');

            // Afficher le bouton de reconnexion immédiatement
            showRetryButton();

        } else if (message.type === "visualize_view_state_update") {
            console.log("Visualize View State Update received:", message.state);
            if (message.state) {
                // Update switches based on received state
                document.getElementById('go-home').checked = message.state.isBackButtonVisible;
                document.getElementById('toggle-commands').checked = message.state.isControlsCardVisible;
                document.getElementById('toggle-profile').checked = message.state.isAltitudeVisible;
                document.getElementById('toggle-communes').checked = message.state.isCommuneWidgetVisible;
                document.getElementById('toggle-distance').checked = message.state.isDistanceDisplayVisible;
            }
        }
    };

    ws.onclose = (event) => {
        if (manualDisconnect) {
            return; // Important pour ne pas déclencher la logique de reconnexion auto
        }
        isRetrying = false;
        updateStatus(`Déconnecté du serveur. Code: ${event.code}, Raison: ${event.reason}`, true);
        // Masquer toutes les pages
        const pages = document.querySelectorAll('.page');
        pages.forEach(page => page.style.display = 'none');
        
        // Vérifier si on peut encore essayer de se reconnecter
        if (retryCount < MAX_RETRY_ATTEMPTS) {
            retryCount++;
            updateStatus(`Tentative de reconnexion dans 3 secondes... (${retryCount}/${MAX_RETRY_ATTEMPTS})`, true);
            retryTimeout = setTimeout(() => {
                connectWebSocket();
            }, 3000);
        } else {
            updateStatus(`Connexion échouée après ${MAX_RETRY_ATTEMPTS} tentatives.`, true);
            showRetryButton();
        }
    };

    ws.onerror = (error) => {
        isRetrying = false;
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
