import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Reactive state for settings, shared across the app
const settings = ref(null);

export function useSettings() {
  // Function to load settings if they haven't been loaded yet
  const loadSettings = async () => {
    if (settings.value) return;
    try {
      settings.value = await invoke('read_settings');
    } catch (error) {
      console.error("Failed to load application settings:", error);
    }
  };

  // Load settings when the composable is first used
  onMounted(loadSettings);

  // Expose the reactive settings object
  return { settings };
}
