import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useRemoteStore = defineStore('remote', () => {
    // --- State ---
    const connectionStatus = ref('disconnected') // 'disconnected', 'connecting', 'connected', 'pairing', 'refused'
    const statusMessage = ref('Connexion en attente...')
    const errorMessage = ref('')
    const clientId = ref(null)
    const pairingCode = ref('')
    const appState = ref(null) // Global app state from SSE
    const visualizeViewState = ref(null) // Visualize view specific state
    const remoteSettings = ref(null) // Settings received from backend
    const settingsDebug = ref("")
    const sseDebugLog = ref([]) // Log of raw SSE events for debug panel
    const heartbeatInterval = ref(null)
    const evtSource = ref(null)
    const reconnectAttempts = ref(0)
    const maxReconnectAttempts = 5

    // --- Computed ---
    const isConnected = computed(() => connectionStatus.value === 'connected')
    const isPairing = computed(() => connectionStatus.value === 'pairing')

    // --- Actions ---

    // 1. Utilities
    function generateUUID() {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
            var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    }

    function generatePairingCode() {
        const chars = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ';
        let result = '';
        for (let i = 0; i < 8; i++) {
            result += chars.charAt(Math.floor(Math.random() * chars.length));
        }
        return result;
    }

    function logDebug(type, data) {
        const timestamp = new Date().toLocaleTimeString();
        sseDebugLog.value.unshift({ timestamp, type, data: JSON.stringify(data) });
        if (sseDebugLog.value.length > 50) sseDebugLog.value.pop();
    }

    function updateStatus(message, status = 'connecting', error = '') {
        statusMessage.value = message;
        connectionStatus.value = status;
        errorMessage.value = error;
    }

    // 2. Command API
    async function sendCommand(command, payload = {}) {
        try {
            logDebug('CMD_OUT', { command, payload });
            const response = await fetch('/api/command', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ command, payload })
            });
            if (!response.ok) throw new Error(`HTTP ${response.status}`);
        } catch (error) {
            console.error('Command Exec Error:', error);
            updateStatus('Erreur communication', 'disconnected', error.message);
        }
    }

    // 3. Heartbeat
    function startHeartbeat() {
        stopHeartbeat();

        let lastHeartbeatSent = 0;
        const sendPulse = () => {
            fetch(`/api/heartbeat?clientId=${clientId.value}`)
                .then(async (response) => {
                    if (response.status === 409) {
                        const data = await response.json().catch(() => ({}));
                        updateStatus("Un autre appareil est déjà connecté", 'busy', data.reason);
                        stopHeartbeat();
                        if (evtSource.value) evtSource.value.close();
                    } else {
                        lastHeartbeatSent = Date.now();
                    }
                })
                .catch(err => console.debug("Heartbeat error", err));
        };

        sendPulse();
        heartbeatInterval.value = setInterval(() => {
            // Watchdog: if last successful heartbeat was > 10s ago, retry immediately
            if (lastHeartbeatSent > 0 && (Date.now() - lastHeartbeatSent) > 10000) {
                console.warn("Heartbeat watchdog triggered");
                sendPulse();
            } else {
                sendPulse();
            }
        }, 3000);
        console.log("Heartbeat started");
    }

    function stopHeartbeat() {
        if (heartbeatInterval.value) {
            clearInterval(heartbeatInterval.value);
            heartbeatInterval.value = null;
        }
    }

    // 4. Pairing Logic
    async function initiatePairing() {
        // Toujours régénérer le code pour chaque nouvelle tentative
        pairingCode.value = generatePairingCode();

        // Show status "Pairing..."
        updateStatus('En attente de couplage...', 'pairing');

        try {
            const response = await fetch('/api/pair', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    clientId: clientId.value,
                    pairingCode: pairingCode.value
                })
            });

            // Handle errors specially (403: blocked/pending, 409: busy)
            if (response.status === 403 || response.status === 409) {
                const data = await response.json();
                throw { status: response.status, data };
            }

            const data = await response.json();

            if (data.status === "accepted" || data.status === "already_paired") {
                handlePairingAccepted(data);
            } else if (data.status === "busy") {
                updateStatus("Un autre appareil est déjà connecté", 'busy', data.reason);
            }
        } catch (err) {
            if (err.data) {
                if (err.data.status === "pending") {
                    updateStatus(`En attente approbation (${pairingCode.value})`, 'pairing');
                } else if (err.data.status === "refused") {
                    updateStatus("Couplage refusé", 'refused', err.data.reason);
                } else if (err.data.status === "busy") {
                    updateStatus("Un autre appareil est déjà connecté", 'busy', err.data.reason);
                }
            } else {
                console.error("Pairing error:", err);
            }
        }
    }

    function handlePairingAccepted(data) {
        updateStatus("Connecté", 'connected');
        if (data.settings) {
            remoteSettings.value = data.settings;
            logDebug('SETTINGS', data.settings);
        }
        if (data.debug_info) {
            settingsDebug.value = data.debug_info;
        }
        if (data.appState) {
            appState.value = data.appState;
        }
        startHeartbeat();
        // Request full state refresh just in case
        fetch('/api/state').then(r => r.json()).then(state => {
            if (state.visualize_view) {
                visualizeViewState.value = state.visualize_view;
            }
        }).catch(e => console.error("Initial state fetch error", e));
    }


    // 5. Connection (SSE)
    function connect() {
        if (evtSource.value) {
            evtSource.value.close();
        }
        stopHeartbeat();
        updateStatus("Connexion SSE...", "connecting");

        // Init Client ID
        if (!clientId.value) {
            clientId.value = localStorage.getItem('visugps_remote_client_id');
            if (!clientId.value) {
                clientId.value = generateUUID();
                localStorage.setItem('visugps_remote_client_id', clientId.value);
            }
        }

        const es = new EventSource('/api/events');
        evtSource.value = es;

        es.onopen = () => {
            reconnectAttempts.value = 0;
            updateStatus("Connecté SSE", "connecting"); // Still connecting until paired
            initiatePairing();
            startHeartbeat(); // Start heartbeat immediately to keep alive
        };

        es.onerror = () => {
            if (es.readyState == EventSource.CLOSED) {
                updateStatus("Déconnecté (SSE)", "disconnected");
                stopHeartbeat();
            } else {
                reconnectAttempts.value++;
                if (reconnectAttempts.value >= maxReconnectAttempts) {
                    es.close();
                    updateStatus("Échec connexion", "disconnected", "Max tentatives atteintes");
                } else {
                    updateStatus(`Reconnexion... (${reconnectAttempts.value})`, "connecting");
                }
            }
        };

        // --- Event Listeners ---
        es.addEventListener("app_state_update", (e) => {
            logDebug('APP_STATE', JSON.parse(e.data));
            const data = JSON.parse(e.data);
            appState.value = data.appState;
            // If we receive updates, we are effectively connected
            if (connectionStatus.value !== 'pairing') {
                updateStatus("Connecté", "connected");
            }
        });

        es.addEventListener("visualize_view_state_update", (e) => {
            logDebug('VISU_STATE', JSON.parse(e.data));
            const data = JSON.parse(e.data);
            visualizeViewState.value = data;
        });

        es.addEventListener("animation_speed_update", (e) => {
            const data = JSON.parse(e.data);
            if (visualizeViewState.value) {
                visualizeViewState.value.currentSpeed = data.speed;
            }
        });

        es.addEventListener("animation_state_update", (e) => {
            const data = JSON.parse(e.data);
            if (visualizeViewState.value) {
                visualizeViewState.value.animationState = data.animationState;
                if (data.currentSegmentIndex !== undefined && data.currentSegmentIndex !== null) {
                    visualizeViewState.value.currentSegmentIndex = data.currentSegmentIndex;
                }
            }
        });

        es.addEventListener("pairing_approved", (e) => {
            logDebug('PAIR_APPROVED', JSON.parse(e.data));
            const data = JSON.parse(e.data);
            handlePairingAccepted(data);
        });

        es.addEventListener("pairing_refused", (e) => {
            logDebug('PAIR_REFUSED', JSON.parse(e.data));
            const data = JSON.parse(e.data);
            if (data.clientId === clientId.value) {
                updateStatus("Couplage refusé", 'refused', data.reason);
                es.close();
                stopHeartbeat();
            }
        });

        es.addEventListener("pairing_abandoned", (e) => {
            const data = JSON.parse(e.data);
            if (data.clientId === clientId.value) {
                updateStatus("Couplage ignoré", 'disconnected', "Demande ignorée par l'utilisateur.");
                es.close();
                stopHeartbeat();
            }
        });

        es.addEventListener("remote_disconnect", (e) => {
            logDebug('REMOTE_DISCO', JSON.parse(e.data));
            const data = JSON.parse(e.data);
            updateStatus("Déconnecté par le serveur", 'disconnected'); // Ignore reason to avoid redundant display
            es.close();
            stopHeartbeat();
        });
    }

    function disconnect() {
        if (evtSource.value) evtSource.value.close();
        stopHeartbeat();
        updateStatus("Déconnecté par l'utilisateur", "disconnected");
    }

    return {
        // State
        connectionStatus,
        statusMessage,
        errorMessage,
        clientId,
        pairingCode,
        appState,
        visualizeViewState,
        visualizeViewState,
        remoteSettings,
        settingsDebug,
        sseDebugLog,

        // Actions
        connect,
        disconnect,
        sendCommand,
        isConnected,
        isPairing
    }
})
