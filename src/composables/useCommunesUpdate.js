import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const majCommuneIsRunning = ref(false);
const circuitsProgress = ref({});
const updatingCircuitId = ref(null);
const updatingCircuitName = ref(null);
const isIgnEnabled = ref(true);
const isMapboxEnabled = ref(true);

export function useCommunesUpdate() {

    let unlistenStatus = null;
    let unlistenProgress = null;

    const startUpdate = async (circuitId) => {
        try {
            await invoke('start_communes_update', { circuitId });
        } catch (error) {
            console.error('Failed to start communes update:', error);
        }
    };

    const interruptUpdate = async () => {
        try {
            await invoke('interrupt_communes_update');
        } catch (error) {
            console.error('Failed to interrupt communes update:', error);
        }
    };

    onMounted(async () => {
        // 1. Get current state on mount to avoid race conditions
        try {
            const initialState = await invoke('get_current_commune_task_info');
            majCommuneIsRunning.value = initialState.is_running;
            updatingCircuitId.value = initialState.circuit_id;
            updatingCircuitName.value = initialState.circuit_name;
            isIgnEnabled.value = initialState.ign_enabled;
            isMapboxEnabled.value = initialState.mapbox_enabled;
        } catch (e) {
            console.error("Failed to get initial commune task info:", e);
        }

        // 2. Then, listen for future changes
        unlistenStatus = await listen('commune-update-status-changed', (event) => {
            const payload = event.payload;
            majCommuneIsRunning.value = payload.is_running;
            updatingCircuitId.value = payload.circuit_id;
            updatingCircuitName.value = payload.circuit_name;
            // Update API states only when the task is running
            if (payload.is_running) {
                isIgnEnabled.value = payload.ign_enabled;
                isMapboxEnabled.value = payload.mapbox_enabled;
            }
        });

        unlistenProgress = await listen('commune-progress-changed', (event) => {
            const [circuitId, progress] = event.payload;
            circuitsProgress.value[circuitId] = progress;
        });
    });

    onUnmounted(() => {
        if (unlistenStatus) {
            unlistenStatus();
        }
        if (unlistenProgress) {
            unlistenProgress();
        }
    });

    return {
        majCommuneIsRunning,
        circuitsProgress,
        updatingCircuitId,
        updatingCircuitName,
        isIgnEnabled, // Export new state
        isMapboxEnabled, // Export new state
        startUpdate,
        interruptUpdate,
    };
}
