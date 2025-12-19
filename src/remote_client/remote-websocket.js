function updateStatus(message, isError = false, isConnecting = false) {
    const statusDiv = document.getElementById('status');
    const mainTitle = document.getElementById('main-title');
    if (!statusDiv || !mainTitle) return;

    statusDiv.textContent = `Statut: ${message}`;
    statusDiv.style.display = 'block'; // Ensure the status div is visible
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

function sendCommand(command, payload = {}) {
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
            type: "command",
            clientId: clientId,
            command: command,
            payload: payload
        }));
    } else {
        console.error('WebSocket non connecté');
    }
}

function connectWebSocket() {
    manualDisconnect = false; // Reset the flag on new connection attempt
    if (isRetrying) {
        return;
    }

    isRetrying = true;
    updateStatus(`Tentative de connexion... (${retryCount + 1}/${MAX_RETRY_ATTEMPTS})`, false, true);
    ws = new WebSocket(WS_URL);

    ws.onopen = () => {
        // Connexion réussie, réinitialiser le compteur
        resetRetryCount();
        updateStatus("Connecté au serveur WebSocket.");

        if (!clientId) {
            clientId = generateUUID();
            localStorage.setItem('visugps_remote_client_id', clientId);
        }
        pairingCodeDiv.textContent = `Code de couplage : ${pairingCode}`;

        // Envoyer la requête de couplage
        ws.send(JSON.stringify({
            type: "pairing_request",
            clientId: clientId,
            pairingCode: pairingCode
        }));
    };

    ws.onmessage = (event) => {
        const message = JSON.parse(event.data);

        if (message.type === "pairing_response") {
            if (message.status === "accepted") {
                updateStatus("Couplage accepté !", false);
                pairingCodeDiv.style.display = 'none';

                if (message.settings) {

                }

                // Afficher la page appropriée selon l'état de l'application reçu du serveur
                if (message.appState) {
                    updateRemoteInterface(message.appState);
                }
                // Demander l'état complet pour synchroniser l'UI
                sendCommand('request_full_state');

            } else if (message.status === "refused") {
                updateStatus(`Couplage refusé : ${message.reason || "Raison inconnue"}`, true);
                pairingCodeDiv.style.display = 'none';
                const pages = document.querySelectorAll('.page');
                pages.forEach(page => page.style.display = 'none');

            } else if (message.status === "already_paired") {
                updateStatus("Déjà couplé.", false);
                pairingCodeDiv.style.display = 'none';
                if (message.appState) {
                    updateRemoteInterface(message.appState);
                }
                // Demander l'état complet pour synchroniser l'UI
                sendCommand('request_full_state');
            }
        } else if (message.type === "app_state_update") {
            // Security check: ensure the message is for this client
            if (message.clientId && message.clientId !== clientId) {
                handleHijackedConnection();
                return;
            }
            // Gérer les mises à jour de l'état de l'application
            updateRemoteInterface(message.appState);
        } else if (message.type === "server_shutdown") {
            manualDisconnect = true;
            updateStatus(message.reason || "Déconnexion demandée par le serveur", true);

            pairingCodeDiv.style.display = 'none';

            // Masquer les pages immédiatement pour une UI réactive
            const pages = document.querySelectorAll('.page');
            pages.forEach(page => page.style.display = 'none');

            // Afficher le bouton de reconnexion immédiatement
            showRetryButton();

        } else if (message.type === "full_state_update") {
            handleFullStateUpdate(message.state);
        } else if (message.type === "visualize_view_state_update") {
            if (message.state) {
                // Update switches based on received state
                document.getElementById('toggle-commands').checked = message.state.isControlsCardVisible;
                document.getElementById('toggle-profile').checked = message.state.isAltitudeVisible;
                document.getElementById('toggle-communes').checked = message.state.isCommuneWidgetVisible;
                document.getElementById('toggle-distance').checked = message.state.isDistanceDisplayVisible;
            }
        } else if (message.type === "pause_state_update") {
        } else if (message.type === "animation_state_update") {
            // const titleElement = document.getElementById('visualize-view-title'); // Supprimé
            // if (titleElement) {
            //     titleElement.textContent = message.animationState;
            // } // Supprimé
            updatePlayPauseButton(message.animationState);
        } else if (message.type === "animation_speed_update") {
            updateSpeedDisplay(message.speed);
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
