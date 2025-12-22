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

// NoSleep management
window.noSleep = null;
window.noSleepEnabled = false;
window.nosleepControl = document.getElementById('nosleep-control');
window.nosleepButton = document.getElementById('toggle-nosleep');

// Initialisation
window.onload = () => {
    if (!clientId) {
        pairingCodeDiv.textContent = `Génération d'un ID client...`;
    }
    connectWebSocket();

    // Ajout des écouteurs d'événements pour les boutons
    setupButtonListeners();

    // Initialize NoSleep if available
    const NoSleepClass = window.NoSleep || (typeof NoSleep !== 'undefined' ? NoSleep : null);

    if (NoSleepClass) {
        window.noSleep = new NoSleepClass();

        const updateNoSleepUI = () => {
            if (nosleepButton) {
                if (noSleepEnabled) {
                    nosleepButton.textContent = "💡 Maintenir l'écran : ON";
                    nosleepButton.classList.remove('btn-secondary', 'btn-outline-secondary');
                    nosleepButton.classList.add('btn-success');
                } else {
                    nosleepButton.textContent = "🌙 Maintenir l'écran : OFF";
                    nosleepButton.classList.remove('btn-success', 'btn-secondary');
                    nosleepButton.classList.add('btn-outline-secondary');
                }
            }
        };

        // Initial UI state update to change from "Veuillez patienter..."
        updateNoSleepUI();

        if (nosleepButton) {
            nosleepButton.addEventListener('click', () => {
                if (!noSleepEnabled) {
                    window.noSleep.enable();
                    window.noSleepEnabled = true;
                    console.log("NoSleep enabled");
                } else {
                    window.noSleep.disable();
                    window.noSleepEnabled = false;
                    console.log("NoSleep disabled");
                }
                updateNoSleepUI();
            });
        }
    } else {
        if (nosleepButton) {
            nosleepButton.textContent = "NoSleep non disponible";
            nosleepButton.classList.add('btn-danger');
        }
        if (statusDiv) {
            statusDiv.innerHTML += "<br><span style='color:orange;'>Avertissement: La bibliothèque NoSleep n'a pas été détectée.</span>";
        }
        console.warn("NoSleep library not found");
    }

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