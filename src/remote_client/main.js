const WS_SERVER_IP = "192.168.1.52"; // À remplacer par l'IP de votre machine desktop
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
                controlsDiv.style.display = 'block'; // Afficher les contrôles
                // Ici, on pourrait aussi mettre à jour l'interface en fonction de message.appState
            } else if (message.status === "refused") {
                updateStatus(`Couplage refusé : ${message.reason || "Raison inconnue"}`, true);
                pairingCodeDiv.style.display = 'block'; // Garder le code visible en cas de refus
                controlsDiv.style.display = 'none';
            } else if (message.status === "already_paired") {
                updateStatus("Déjà couplé.", false);
                pairingCodeDiv.style.display = 'none';
                controlsDiv.style.display = 'block';
            }
        } else if (message.type === "app_state_update") {
            // Gérer les mises à jour de l'état de l'application
            console.log("État de l'application :", message.appState);
            // Ici, la logique pour afficher/masquer les boutons en fonction de message.appState
            if (message.appState === "Visualisation") {
                controlsDiv.style.display = 'block';
            } else {
                controlsDiv.style.display = 'none';
            }
        }
    };

    ws.onclose = (event) => {
        updateStatus(`Déconnecté du serveur. Code: ${event.code}, Raison: ${event.reason}`, true);
        controlsDiv.style.display = 'none';
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

    // Ajout des écouteurs d'événements pour les boutons (pour l'étape 4)
    document.getElementById('play-pause').addEventListener('click', () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "command", clientId: clientId, command: "toggle_play" }));
        }
    });
    document.getElementById('toggle-commands').addEventListener('click', () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "command", clientId: clientId, command: "toggle_commands_widget" }));
        }
    });
    document.getElementById('toggle-profile').addEventListener('click', () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "command", clientId: clientId, command: "toggle_altitude_profile" }));
        }
    });
    document.getElementById('toggle-communes').addEventListener('click', () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "command", clientId: clientId, command: "toggle_communes_display" }));
        }
    });
    document.getElementById('toggle-distance').addEventListener('click', () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "command", clientId: clientId, command: "toggle_distance_display" }));
        }
    });
    document.getElementById('go-home').addEventListener('click', () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(JSON.stringify({ type: "command", clientId: clientId, command: "toggle_home" }));
        }
    });
};
