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

  const checkAllServices = async (skipOpenMeteo = false) => {
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
        // --- Auto-update Status from Start to MapBoxOK if token is valid ---
        // On le fait DES QUE Mapbox est validé, sans attendre Open-Meteo
        if (status.value === 'Start') {
          console.log("Valid token detected globally, updating Status to MapBoxOK");
          updateReferenceField('Status', 'MapBoxOK');
        }

        if (!skipOpenMeteo) {
          // Nouvelle vérification Open-Meteo (NON BLOQUANTE)
          try {
            const status = await invoke('check_open_meteo_status');
            // status can be: "OK", "BUSY", "RATE_LIMITED", "UNREACHABLE"

            if (status === 'OK') {
              updateStatus('connected', 'Tous les services sont opérationnels (Mapbox & Open-Meteo).');
            } else if (status === 'BUSY') {
              // If busy, it means it's working (fetching), so we consider it connected
              console.log("Open-Meteo is BUSY (Lock held), considering confirmed.");
              updateStatus('connected', 'Services opérationnels (Open-Meteo actif).');
            } else if (status === 'RATE_LIMITED') {
              updateStatus('open_meteo_unreachable', 'Erreur 429: Trop de requêtes Open-Meteo. Veuillez patienter.');
            } else {
              // UNREACHABLE or other
              updateStatus('open_meteo_unreachable', 'Service Météo (Open-Meteo) injoignable (Réseau/Timeout).');
            }
          } catch (e) {
            console.error('Error checking Open-Meteo status:', e);
            updateStatus('open_meteo_unreachable', 'Erreur technique vérification Open-Meteo.');
          }
        } else {
          // If skipping Open-Meteo, we assume it's fine if Mapbox is fine (or keep previous status if it was specifically Open-Meteo related?)
          // If we skip, we set status to 'connected' because Mapbox is OK.
          // But if Open-Meteo was previously in error...? 
          // For now, simpler approach: if Mapbox is OK and we skip Open-Meteo, we say 'connected'.
          updateStatus('connected', 'Connexion active (Mapbox OK).');
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