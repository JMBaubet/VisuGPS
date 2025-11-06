function sendCommand(command, payload = {}) {
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
            type: "command",
            clientId: clientId,
            command: command,
            payload: payload
        }));
        console.log(`Commande envoyée: ${command}`, payload);
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
    updateStatus(`Tentative de connexion... (${retryCount + 1}/${MAX_RETRY_ATTEMPTS})`, false, true);
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

                if (message.settings) {
                    console.log("Received settings from server:", message.settings);
                    g_speed_min_value = message.settings.speedMinValue;
                    g_speed_max_value = message.settings.speedMaxValue;
                }

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
                // document.getElementById('go-home').checked = message.state.isBackButtonVisible; // Supprimé
                document.getElementById('toggle-commands').checked = message.state.isControlsCardVisible;
                document.getElementById('toggle-profile').checked = message.state.isAltitudeVisible;
                document.getElementById('toggle-communes').checked = message.state.isCommuneWidgetVisible;
                document.getElementById('toggle-distance').checked = message.state.isDistanceDisplayVisible;
            }
        } else if (message.type === "pause_state_update") {
            console.log("Pause state updated:", message.payload);
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
