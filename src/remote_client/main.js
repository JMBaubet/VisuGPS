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

// Initialisation
window.onload = () => {
    if (!clientId) {
        pairingCodeDiv.textContent = `Génération d'un ID client...`;
    }
    connectWebSocket();

    // Ajout des écouteurs d'événements pour les boutons
    setupButtonListeners();
};