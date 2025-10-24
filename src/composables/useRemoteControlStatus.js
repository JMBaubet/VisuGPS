import { ref, computed, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

const isRemoteConnected = ref(false);

export function useRemoteControlStatus() {
    let unlisten = null;

    const updateStatus = (status) => {
        isRemoteConnected.value = status === 'connected';
    };

    onMounted(async () => {
        try {
            const initialState = await invoke('get_remote_control_status');
            updateStatus(initialState);
        } catch (e) {
            console.error("Failed to get initial remote control status:", e);
            isRemoteConnected.value = false;
        }

        unlisten = await listen('remote_control_status_changed', (event) => {
            updateStatus(event.payload);
        });
    });

    onUnmounted(() => {
        if (unlisten) {
            unlisten();
        }
    });

    const remoteStatusIcon = computed(() => 
        isRemoteConnected.value ? 'mdi-remote' : 'mdi-remote-off'
    );

    const remoteStatusColor = computed(() => 
        isRemoteConnected.value ? 'success' : 'primary'
    );

    return {
        isRemoteConnected,
        remoteStatusIcon,
        remoteStatusColor
    };
}
