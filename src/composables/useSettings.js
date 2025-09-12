import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { format } from 'date-fns';

// State
const settings = ref(null);

// Helper function to navigate a path and find a parameter
const findParameterNode = (path) => {
  if (!settings.value || !path) return null;

  const parts = path.split('.');
  const paramId = parts.pop();
  let current = settings.value.data;

  for (const part of parts) {
    if (!current.groupes) return null;
    const nextGroup = current.groupes.find(g => g.libelle === part);
    if (!nextGroup) return null;
    current = nextGroup;
  }

  if (!current.parametres) return null;
  return current.parametres.find(p => p.identifiant === paramId);
};

// Helper function to navigate a path and find a group
const findGroupNode = (path) => {
    if (!settings.value || !path) return settings.value.data;
    let current = settings.value.data;
    const parts = path.split('/');
    for (const part of parts) {
        if (!part) continue;
        const nextGroup = current.groupes.find(g => g.libelle === part);
        if (!nextGroup) return null;
        current = nextGroup;
    }
    return current;
};


export function useSettings() {

  const initSettings = async () => {
    try {
      settings.value = await invoke('read_settings');
    } catch (error) {
      console.error("Erreur lors du chargement initial des paramètres:", error);
      settings.value = null;
    }
  };

  const getSettingValue = (path) => {
    const paramNode = findParameterNode(path);
    if (!paramNode) {
      // console.warn(`[useSettings] Paramètre non trouvé pour le chemin: ${path}`);
      return undefined;
    }
    if (paramNode.surcharge != null && paramNode.surcharge !== '') {
      return paramNode.surcharge;
    }
    return paramNode.defaut;
  };

  const updateSetting = async (groupPath, paramId, newValue) => {
    try {
      // 1. Call Rust to persist the change
      await invoke('update_setting', {
        groupPath: groupPath,
        paramId: paramId,
        newValue: newValue
      });

      // 2. Update local reactive state for instant feedback
      const groupNode = findGroupNode(groupPath);
      if (groupNode && groupNode.parametres) {
        const param = groupNode.parametres.find(p => p.identifiant === paramId);
        if (param) {
          param.surcharge = newValue;
        }
      }
      
      // 3. Update revision date in local state
      if (settings.value && settings.value.référence) {
        settings.value.référence.date_revision = format(new Date(), "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'");
      }

    } catch (error) {
      console.error(`Erreur lors de la mise à jour du paramètre ${paramId}:`, error);
      // Optionally, revert local changes here or show a snackbar
      throw error; // Re-throw to let the component know
    }
  };

  return {
    settings: computed(() => settings.value),
    initSettings,
    getSettingValue,
    updateSetting
  };
}