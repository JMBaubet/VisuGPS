
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const messages = ref([]);
const loaded = ref(false);

export function useMessages() {
    const { toHex, getContrastColor } = useVuetifyColors();

    const fetchMessages = async () => {
        try {
            if (!loaded.value) {
                messages.value = await invoke('get_message_library');
                loaded.value = true;
            }
        } catch (error) {
            console.error("Erreur lors de la récupération des messages:", error);
        }
    };

    const getMessage = (id) => {
        if (!id) return null;
        return messages.value.find(m => m.id === id);
    };

    // Force refresh if needed
    const refreshMessages = async () => {
        try {
            messages.value = await invoke('get_message_library');
            loaded.value = true;
        } catch (error) {
            console.error("Erreur lors du rafraîchissement des messages:", error);
        }
    };

    return {
        messages,
        fetchMessages,
        getMessage,
        refreshMessages,
        toHex,
        getContrastColor
    };
}
