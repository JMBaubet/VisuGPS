import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Reactive state for settings, shared across the app
const settings = ref(null);

// Recursive helper to find a parameter in the settings tree
const findParameter = (groups, path) => {
  if (!groups || !path || path.length === 0) return null;

  const [currentPathPart, ...remainingPath] = path;

  for (const group of groups) {
    // Check if the current path part matches a group libelle
    if (group.libelle === currentPathPart) {
      // If it's the last part of the path, it must be a parameter identifier
      if (remainingPath.length === 1) {
        return group.parametres?.find(p => p.identifiant === remainingPath[0]);
      }
      // If there are more parts, recurse into subgroups
      if (group.groupes) {
        const found = findParameter(group.groupes, remainingPath);
        if (found) return found;
      }
    }
  }
  return null;
};

export function useSettings() {
  const loadSettings = async () => {
    if (settings.value) return;
    try {
      settings.value = await invoke('read_settings');
    } catch (error) {
      console.error("Failed to load application settings:", error);
    }
  };

  const getSettingValue = (path, defaultValue = null) => {
    if (!settings.value) return defaultValue;
    const pathParts = path.split('/');
    
    const param = findParameter(settings.value.data.groupes, pathParts);

    if (!param) return defaultValue;

    // Use surcharge if it exists and is not null, otherwise use defaut
    return param.surcharge != null ? param.surcharge : param.defaut;
  };

  onMounted(loadSettings);

  // Expose the reactive settings object and the getter function
  return { settings, getSettingValue };
}
