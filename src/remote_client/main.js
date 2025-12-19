const WS_SERVER_IP = "192.168.1.65"; // À remplacer par l'IP de votre machine desktop
const WS_SERVER_PORT = 9001;
const WS_URL = `ws://${WS_SERVER_IP}:${WS_SERVER_PORT}`;

// Variables globales explicitement attachées à window pour l'accès depuis d'autres scripts
window.ws = null;
window.clientId = localStorage.getItem('visugps_remote_client_id');
window.pairingCode = generateRandomCode(8);
window.manualDisconnect = false;

// Variables pour la gestion des tentatives de reconnexion
window.retryCount = 0;
window.MAX_RETRY_ATTEMPTS = 3;
window.isRetrying = false;
window.retryTimeout = null;

// Éléments UI globaux
window.statusDiv = document.getElementById('status');
window.pairingCodeDiv = document.getElementById('pairing-code');
window.controlsDiv = document.getElementById('controls');

// Initialisation
window.onload = () => {
    if (!clientId) {
        pairingCodeDiv.textContent = `Génération d'un ID client...`;
    }
    connectWebSocket();

    // Ajout des écouteurs d'événements pour les boutons
    setupButtonListeners();

    // Auto-reconnect when tab becomes visible (wake up from sleep)
    document.addEventListener('visibilitychange', () => {
        if (document.visibilityState === 'visible') {
            if (!ws || ws.readyState === WebSocket.CLOSED || ws.readyState === WebSocket.CLOSING) {
                console.log("App became visible, attempting to reconnect...");
                // Remove manual disconnect flag to allow auto-reconnect logic to work
                manualDisconnect = false;
                // Remove retry button if it exists to avoid duplicates/confusion
                const retryBtn = document.getElementById('retry-button');
                if (retryBtn) retryBtn.remove();

                resetRetryCount();
                connectWebSocket();
            }
        }
    });
};