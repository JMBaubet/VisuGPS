// Main entry point for Remote Client V2

// Variables globales attachées à window
window.clientId = localStorage.getItem('visugps_remote_client_id');
window.pairingCode = generateRandomCode(8);
window.manualDisconnect = false;

// Variables UI
window.statusDiv = document.getElementById('status');
window.pairingCodeDiv = document.getElementById('pairing-code');
window.controlsDiv = document.getElementById('controls');

// NoSleep management
window.noSleep = null;
window.noSleepEnabled = false;
window.nosleepControl = document.getElementById('nosleep-control');
window.nosleepButton = document.getElementById('toggle-nosleep');

// Fonction utilitaire si pas dans remote-utils
function generateRandomCode(length) {
    const chars = '0123456789';
    let result = '';
    for (let i = 0; i < length; i++) {
        result += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return result;
}

// Initialisation
window.onload = () => {
    // Appel à la fonction définie dans remote-client-v2.js
    if (window.connectRemote) {
        window.connectRemote();
    } else {
        console.error("remote-client-v2.js non chargé !");
        if (window.statusDiv) window.statusDiv.textContent = "Erreur: script client manquant";
    }

    // Ajout des écouteurs d'événements pour les boutons
    if (window.setupButtonListeners) {
        window.setupButtonListeners();
    }

    // Initialize NoSleep
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

        updateNoSleepUI();

        if (nosleepButton) {
            nosleepButton.addEventListener('click', () => {
                if (!noSleepEnabled) {
                    window.noSleep.enable();
                    window.noSleepEnabled = true;
                } else {
                    window.noSleep.disable();
                    window.noSleepEnabled = false;
                }
                updateNoSleepUI();
            });
        }
    } else {
        if (nosleepButton) {
            nosleepButton.textContent = "NoSleep non disponible";
            nosleepButton.classList.add('btn-danger');
        }
    }

    // Auto-reconnect when tab becomes visible
    document.addEventListener('visibilitychange', () => {
        if (document.visibilityState === 'visible') {
            if (window.evtSource && window.evtSource.readyState === EventSource.CLOSED) {
                console.log("App became visible, attempting to reconnect...");
                window.connectRemote();
            }
        }
    });
};