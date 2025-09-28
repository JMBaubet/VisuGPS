import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const majCommuneIsRunning = ref(false);
const circuitsProgress = ref({});
const updatingCircuitId = ref(null);
const updatingCircuitName = ref(null);

export function useCommunesUpdate() {

    let unlistenStatus = null;
    let unlistenProgress = null;

    const fetchUpdateState = async () => {
        try {
            majCommuneIsRunning.value = await invoke('get_communes_update_status');
        } catch (e) {
            console.error("Failed to get initial communes update status", e)
        }
    };

    const startUpdate = async (circuitId, circuitName) => {
        try {
            updatingCircuitId.value = circuitId; // Set ID immediately
            updatingCircuitName.value = circuitName; // Set name immediately
            await invoke('start_communes_update', { circuitId });
            fetchUpdateState(); // Refresh running status
        } catch (error) {
            console.error('Failed to start communes update:', error);
            updatingCircuitId.value = null; // Clear on error
            updatingCircuitName.value = null;
        }
    };

    const interruptUpdate = async () => {
        try {
            await invoke('interrupt_communes_update');
            // The event listener will handle clearing the state
        } catch (error) {
            console.error('Failed to interrupt communes update:', error);
        }
    };

    onMounted(async () => {
        fetchUpdateState();

        unlistenStatus = await listen('commune-update-status-changed', (event) => {
            const isRunning = event.payload;
            majCommuneIsRunning.value = isRunning;
            if (!isRunning) {
                updatingCircuitId.value = null; // Clear ID when task stops
                updatingCircuitName.value = null; // Clear name when task stops
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
        startUpdate,
        interruptUpdate,
    };
}
