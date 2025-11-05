const mainTitle = document.getElementById('main-title');

function updateStatus(message, isError = false, isConnecting = false) {
    statusDiv.textContent = `Statut: ${message}`;
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
    const visualizeViewTitle = document.getElementById('visualize-view-title');

    // Réinitialiser l'affichage du statut et du titre de la vue visualisation
    statusText.style.display = 'block';
    visualizeViewTitle.style.display = 'block';

    switch(appState) {
        case 'Visualize':
        case 'Visualisation':
            document.getElementById('page-visualize').style.display = 'block';
            mainTitle.textContent = 'VisuGPS Visualisation';
            statusText.style.display = 'none'; // Masquer le statut
            visualizeViewTitle.style.display = 'none'; // Masquer le titre de la vue visualisation
            break;
        case 'Edit':
        case 'EditView':
            document.getElementById('page-edit').style.display = 'block';
            mainTitle.textContent = 'VisuGPS Édition';
            break;
        case 'Main':
        case 'MainView':
            document.getElementById('page-main').style.display = 'block';
            mainTitle.textContent = 'VisuGPS Accueil';
            break;
        case 'Settings':
        case 'SettingsView':
            document.getElementById('page-settings').style.display = 'block';
            mainTitle.textContent = 'VisuGPS Configuration';
            break;
        case 'Pairing':
            mainTitle.textContent = 'VisuGPS Couplage';
            break;
        default:
            // Page par défaut pour les vues non reconnues
            document.getElementById('page-default').style.display = 'block';
            document.getElementById('default-view-title').textContent = appState || 'Vue Inconnue';
            mainTitle.textContent = 'VisuGPS ' + (appState || 'Inconnue');
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

    // Speed Slider
    const speedSlider = document.getElementById('speed-slider');
    const speedDisplay = document.getElementById('speed-display');

    if (speedSlider && speedDisplay) {
        speedSlider.addEventListener('input', () => {
            const speedValue = parseFloat(speedSlider.value);
            speedDisplay.textContent = `${speedValue.toFixed(1)}x`;
            sendCommand('update_speed', { speed: speedValue });
        });

        speedSlider.addEventListener('dblclick', () => {
            speedSlider.value = 1.0;
            speedDisplay.textContent = `1.0x`;
            sendCommand('set_speed_to_1x');
        });
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
    const playPauseButton = document.getElementById('play-pause');

    if (!playPauseButton) return;

    // Comportement par défaut : commande pour Play/Pause
    playPauseButton.onclick = () => sendCommand('toggle_play');

    switch (state) {
        case 'Vol_Vers_Vue_Globale':
        case 'Pause_Observation':
        case 'Vol_Vers_Depart':
        case 'Survol_Evenementiel':
            playPauseButton.innerHTML = '▶️ Play';
            playPauseButton.disabled = true;
            break;

        case 'En_Pause_au_Depart':
        case 'En_Pause':
            playPauseButton.innerHTML = '▶️ Play';
            playPauseButton.disabled = false;
            break;

        case 'En_Animation':
            playPauseButton.innerHTML = '⏸️ Pause';
            playPauseButton.disabled = false;
            break;

        case 'Vol_Final':
            playPauseButton.innerHTML = '🔄 Redémarrer';
            playPauseButton.disabled = true;
            playPauseButton.onclick = () => sendCommand('restart_animation');
            break;

        case 'Termine':
            playPauseButton.innerHTML = '🔄 Redémarrer';
            playPauseButton.disabled = false;
            playPauseButton.onclick = () => sendCommand('restart_animation');
            break;

        default:
            playPauseButton.innerHTML = '--';
            playPauseButton.disabled = true;
            break;
    }
}

function updateSpeedDisplay(speed) {
    const speedDisplay = document.getElementById('speed-display');
    const speedSlider = document.getElementById('speed-slider');
    if (speedDisplay) {
        speedDisplay.textContent = `${speed.toFixed(1)}x`;
    }
    if (speedSlider) {
        speedSlider.value = speed;
    }
}
