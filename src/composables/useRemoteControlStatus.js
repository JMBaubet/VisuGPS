import { ref, computed, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

const isRemoteConnected = ref(false);
let isInitialized = false;

async function initRemoteStatus() {
    if (isInitialized) return;
    isInitialized = true;

    try {
        const initialState = await invoke('get_remote_control_status');
        isRemoteConnected.value = initialState === 'connected';
    } catch (e) {
        console.error("Failed to get initial remote control status:", e);
    }

    await listen('remote_control_status_changed', (event) => {
        isRemoteConnected.value = event.payload === 'connected';
    });
}

export function useRemoteControlStatus() {
    onMounted(() => {
        initRemoteStatus();
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
