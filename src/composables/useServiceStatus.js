import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from './useSnackbar';
import { useSettings } from './useSettings';

// --- Singleton State ---
const serviceStatus = ref('checking');
const statusMessage = ref('Vérification des services...');
let previousServiceStatus = null;
// --- End Singleton State ---

export function useServiceStatus() {
  const { showSnackbar } = useSnackbar();
  const { getSettingValue, status, updateReferenceField } = useSettings();

  const mapboxToken = computed(() => getSettingValue('Système.Tokens.mapbox'));

  const updateStatus = (newStatus, newMessage) => {
    if (newStatus !== previousServiceStatus) {
      if (newStatus === 'checking' && previousServiceStatus === 'connected') {
        serviceStatus.value = newStatus;
        previousServiceStatus = newStatus;
        return;
      }
      serviceStatus.value = newStatus;
      statusMessage.value = newMessage;
      previousServiceStatus = newStatus;
    }
  };

  const checkAllServices = async () => {
    const token = mapboxToken.value;
    updateStatus('checking', 'Vérification des services...');

    try {
      const isOnline = await invoke('check_internet_connectivity');
      if (!isOnline) {
        updateStatus('disconnected', 'Pas de connexion Internet.');
        return;
      }
    } catch (e) {
      console.error('Error checking internet connectivity:', e);
      updateStatus('disconnected', 'Erreur lors de la vérification de la connexion Internet.');
      return;
    }

    if (!token || token.trim() === '') {
      updateStatus('invalid_token', 'Token Mapbox manquant ou invalide.');
      return;
    }

    try {
      const result = await invoke('check_mapbox_status', { token });
      if (result.success) {

        // Nouvelle vérification Open-Meteo
        try {
          const openMeteoOk = await invoke('check_open_meteo_status');
          if (openMeteoOk) {
            updateStatus('connected', 'Tous les services sont opérationnels (Mapbox & Open-Meteo).');
          } else {
            updateStatus('mapbox_unreachable', 'Service Météo (Open-Meteo) injoignable.');
            // Note : on utilise 'mapbox_unreachable' pour déclencher l'icône bleue/warning 
            // ou on crée un nouveau statut. 
            // L'implémentation plan prévoyait 'open_meteo_unreachable'.
            // Ajustons pour utiliser un nouveau statut.
            updateStatus('open_meteo_unreachable', 'Service Météo (Open-Meteo) injoignable ou erreur.');
            return;
          }
        } catch (e) {
          console.error('Error checking Open-Meteo status:', e);
          updateStatus('open_meteo_unreachable', 'Erreur vérification Open-Meteo.');
          return;
        }

        // Auto-update Status from Start to MapBoxOK if token is valid
        if (status.value === 'Start') {
          console.log("Valid token detected globally, updating Status to MapBoxOK");
          updateReferenceField('Status', 'MapBoxOK');
        }
      } else {
        if (result.reason === 'unreachable') {
          updateStatus('mapbox_unreachable', 'Serveur Mapbox inaccessible.');
        } else if (result.reason === 'invalid_token') {
          updateStatus('invalid_token', 'Token Mapbox invalide.');
        } else {
          updateStatus('disconnected', 'Problème de service Mapbox.');
        }
      }
    } catch (e) {
      console.error('Error checking Mapbox status:', e);
      updateStatus('disconnected', 'Erreur lors de la vérification des services.');
    }
  };

  // Re-check services whenever the token changes
  watch(mapboxToken, (newToken, oldToken) => {
    if (newToken !== oldToken) {
      console.log('Mapbox token changed, re-checking services...');
      checkAllServices();
    }
  });

  // Watch for statusMessage changes and show snackbar
  watch(statusMessage, (newMessage) => {
    if (serviceStatus.value === 'checking') {
      return;
    }
    let color = 'info';
    if (serviceStatus.value === 'connected') {
      color = 'success';
    } else if (serviceStatus.value === 'disconnected' || serviceStatus.value === 'invalid_token') {
      color = 'error';
    } else if (serviceStatus.value === 'mapbox_unreachable') {
      color = 'info';
    } else if (serviceStatus.value === 'open_meteo_unreachable') {
      color = 'warning';
    }
    showSnackbar(newMessage, color);
  });

  return {
    serviceStatus,
    statusMessage,
    checkAllServices,
  };
}