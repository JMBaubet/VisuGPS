import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const majCommuneIsRunning = ref(false);
const circuitsProgress = ref({});
const updatingCircuitId = ref(null);

export function useCommunesUpdate() {

    let unlistenStatus = null;
    let unlistenProgress = null;

    const fetchUpdateState = async () => {
        try {
            majCommuneIsRunning.value = await invoke('get_communes_update_status');
            const appState = await invoke('get_app_state');
            updatingCircuitId.value = appState.updatingCircuitId;
        } catch (e) {
            console.error("Failed to get initial communes update status", e)
        }
    };

    const startUpdate = async (circuitId) => {
        try {
            await invoke('start_communes_update', { circuitId });
            fetchUpdateState(); // Refresh state after starting
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
        fetchUpdateState();

        unlistenStatus = await listen('commune-update-status-changed', (event) => {
            majCommuneIsRunning.value = event.payload;
            fetchUpdateState(); // Refresh state on change
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
        startUpdate,
        interruptUpdate,
    };
}
