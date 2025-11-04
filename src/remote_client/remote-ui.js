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
    // Le bouton play-pause est maintenant géré dynamiquement

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

function updatePlayPauseButton(state) {
    const button = document.getElementById('play-pause');
    if (!button) return;

    // Comportement par défaut : commande pour Play/Pause
    button.onclick = () => sendCommand('toggle_play');

    switch (state) {
        case 'Vol_Vers_Vue_Globale':
        case 'Pause_Observation':
        case 'Vol_Vers_Depart':
        case 'Survol_Evenementiel':
            button.innerHTML = '▶️ Play';
            button.disabled = true;
            break;

        case 'En_Pause_au_Depart':
        case 'En_Pause':
            button.innerHTML = '▶️ Play';
            button.disabled = false;
            break;

        case 'En_Animation':
            button.innerHTML = '⏸️ Pause';
            button.disabled = false;
            break;

        case 'Vol_Final':
            button.innerHTML = '🔄 Redémarrer';
            button.disabled = true;
            button.onclick = () => sendCommand('restart_animation');
            break;

        case 'Termine':
            button.innerHTML = '🔄 Redémarrer';
            button.disabled = false;
            button.onclick = () => sendCommand('restart_animation');
            break;

        default:
            button.innerHTML = '--';
            button.disabled = true;
            break;
    }
}
