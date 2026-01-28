// Remote Client V2 using SSE and HTTP POST
// Replaces remote-websocket.js

// Global variables (shared with main.js)
window.evtSource = null;

window.logToScreen = function (msg) {
    const logDiv = document.getElementById('debug-log');
    if (logDiv) {
        logDiv.innerHTML += `<div>${new Date().toLocaleTimeString()} ${msg}</div>`;
        logDiv.scrollTop = logDiv.scrollHeight;
    }
    console.log(msg);
}

window.logToScreen("V2 Client Script Loaded");


function updateStatus(message, isError = false, isConnecting = false) {
    const statusDiv = document.getElementById('status');
    const mainTitle = document.getElementById('main-title');
    if (!statusDiv || !mainTitle) return;

    statusDiv.textContent = `Statut: ${message}`;
    statusDiv.style.display = 'block';

    if (isError) {
        statusDiv.style.color = 'red';
        mainTitle.style.color = 'red';
    } else if (isConnecting) {
        statusDiv.style.color = 'blue';
        mainTitle.style.color = 'blue';
    } else {
        statusDiv.style.color = 'green';
        mainTitle.style.color = 'green';
    }
}

// Send command via HTTP POST
window.sendCommand = function (command, payload = {}) {
    fetch('/api/command', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            command: command,
            payload: payload
        })
    }).catch(error => {
        console.error('Erreur lors de l\'envoi de la commande:', error);
        updateStatus("Erreur communication serveur", true);
    });
};

function connectRemote() {
    if (window.evtSource) {
        window.evtSource.close();
    }

    updateStatus("Connexion SSE...", false, true);

    // Initialize ID if needed
    if (!window.clientId) {
        window.clientId = localStorage.getItem('visugps_remote_client_id');
        if (!window.clientId) {
            window.clientId = generateRandomCode(16); // Assuming generateRandomCode available or use UUID
            localStorage.setItem('visugps_remote_client_id', window.clientId);
        }
    }

    // Connect to SSE endpoint
    logToScreen("Connecting to SSE /api/events...");
    window.evtSource = new EventSource('/api/events');

    window.evtSource.onopen = function (e) {
        logToScreen("SSE Connected!");
        updateStatus("Connecté (SSE)", false);

        // Démarrer la procédure de pairing
        initiatePairing();
    };

    window.evtSource.onerror = function (e) {
        logToScreen(`SSE Error: readyState=${window.evtSource.readyState}`);
        if (window.evtSource.readyState == EventSource.CLOSED) {
            updateStatus("Déconnecté (SSE)", true);
        } else {
            updateStatus("Reconnexion SSE...", false, true);
        }
    };

    // --- Event Listeners ---

    window.evtSource.addEventListener("app_state_update", function (e) {
        const data = JSON.parse(e.data);
        updateRemoteInterface(data.appState);
    });

    window.evtSource.addEventListener("visualize_view_state_update", function (e) {
        const data = JSON.parse(e.data);
        const state = data; // data IS the state object directly or wrapped? In Rust I sent json!(state).

        if (state) {
            // Update UI switches
            const setChecked = (id, val) => {
                const el = document.getElementById(id);
                if (el) el.checked = val;
            };

            setChecked('toggle-commands', state.isControlsCardVisible);
            setChecked('toggle-profile', state.isAltitudeVisible);
            setChecked('toggle-communes', state.isCommuneWidgetVisible);
            setChecked('toggle-distance', state.isDistanceDisplayVisible);
            setChecked('toggle-weather-dynamic', state.isDynamicWeatherVisible);
            setChecked('toggle-weather-static', state.isStaticWeatherVisible);

            if (state.currentSpeed !== undefined) {
                updateSpeedDisplay(state.currentSpeed);
            }
            if (state.animationState) {
                updatePlayPauseButton(state.animationState);
            }
        }
    });

    window.evtSource.addEventListener("animation_speed_update", function (e) {
        const data = JSON.parse(e.data);
        if (data.speed !== undefined) {
            updateSpeedDisplay(data.speed);
        }
    });

    window.evtSource.addEventListener("animation_state_update", function (e) {
        const data = JSON.parse(e.data);
        if (data.animationState) {
            updatePlayPauseButton(data.animationState);
        }
    });

    window.evtSource.addEventListener("pause_state_update", function (e) {
        // Handle pause state specific logic if needed
    });

    window.evtSource.addEventListener("pairing_approved", function (e) {
        const data = JSON.parse(e.data);
        if (data.clientId === window.clientId) {
            logToScreen("Pairing approved! Re-initiating to get session...");
            // Hide the pairing code div immediately
            if (window.pairingCodeDiv) window.pairingCodeDiv.style.display = 'none';
            // Stop blinking or blue status
            updateStatus("Couplage accepté", false);
            // Re-initiate pairing to get the session token and settings
            initiatePairing();
        }
    });

    window.evtSource.addEventListener("pairing_refused", function (e) {
        const data = JSON.parse(e.data);
        if (data.clientId === window.clientId) {
            updateStatus(`Pairing refusé: ${data.reason}`, true);
            // On arrête de polluer le serveur
            if (window.evtSource) window.evtSource.close();
        }
    });

    window.evtSource.addEventListener("remote_disconnect", function (e) {
        const data = JSON.parse(e.data);
        updateStatus(`Déconnecté : ${data.reason}`, true);
        // On masque tout pour montrer qu'on n'est plus en contrôle
        const pages = document.querySelectorAll('.page');
        pages.forEach(page => page.style.display = 'none');
        // On arrête le SSE
        if (window.evtSource) window.evtSource.close();
    });
}

function initiatePairing() {
    if (window.pairingCodeDiv) {
        window.pairingCodeDiv.style.display = 'block';
        window.pairingCodeDiv.textContent = `Code de couplage : ${window.pairingCode}`;
    }

    fetch('/api/pair', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            clientId: window.clientId,
            pairingCode: window.pairingCode
        })
    })
        .then(response => {
            if (response.status === 403) {
                // Forbidden can mean refused OR pending (if we implemented pending logic properly via status code)
                // But my Rust code returns 200 OK for accepted, and 403 Forbidden for refused/pending
                // Let's parse the body.
                return response.json().then(data => { throw { status: response.status, data }; });
            }
            return response.json();
        })
        .then(data => {
            if (data.status === "accepted") {
                updateStatus("Couplage accepté", false);
                if (window.pairingCodeDiv) window.pairingCodeDiv.style.display = 'none';

                if (data.settings) {
                    // Apply settings like min/max speed
                    if (window.applyRemoteSettings) window.applyRemoteSettings(data.settings);
                }

                if (data.appState) {
                    updateRemoteInterface(data.appState);
                }

                // Request full state refresh just in case
                fetch('/api/state').then(r => r.json()).then(state => {
                    // Manually trigger updates based on state
                    if (state.visualize_view) {
                        // Simulate event
                        const event = { data: JSON.stringify(state.visualize_view) };
                        // call handler... or simpler just reload page? No.
                        // We should refactor update logic to be reusable.
                        // specific logic here...
                    }
                });

            } else if (data.status === "already_paired") {
                updateStatus("Déjà connecté", false);
                if (window.pairingCodeDiv) window.pairingCodeDiv.style.display = 'none';
                if (data.appState) updateRemoteInterface(data.appState);
            }
        })
        .catch(err => {
            if (err.data) {
                if (err.data.status === "pending") {
                    updateStatus("En attente d'approbation...", false);
                } else if (err.data.status === "refused") {
                    updateStatus(`Refusé : ${err.data.reason}`, true);
                }
            } else {
                console.error("Pairing error:", err);
                updateStatus("Erreur pairing", true);
            }
        });
}

// Helper needed by main.js or others
if (!window.generateUUID) {
    window.generateUUID = function () {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
            var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    };
}

// Make connectRemote available globally
window.connectRemote = connectRemote;
