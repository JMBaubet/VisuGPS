import { ref, onMounted, watch } from 'vue'; // Added watch
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from './useSnackbar'; // Added useSnackbar

export function useServiceStatus() {
  const serviceStatus = ref('checking'); // 'checking', 'connected', 'disconnected', 'mapbox_unreachable', 'invalid_token'
  const statusMessage = ref('Vérification des services...');
  const { showSnackbar } = useSnackbar(); // Use useSnackbar

  const checkAllServices = async (mapboxToken) => {
    serviceStatus.value = 'checking';
    statusMessage.value = 'Vérification des services...';

    // 1. Check Internet Connectivity (Rust backend for reliability)
    try {
      const isOnline = await invoke('check_internet_connectivity');
      if (!isOnline) {
        serviceStatus.value = 'disconnected';
        statusMessage.value = 'Pas de connexion Internet.';
        return;
      }
    } catch (e) {
      console.error('Error checking internet connectivity:', e);
      serviceStatus.value = 'disconnected';
      statusMessage.value = 'Erreur lors de la vérification de la connexion Internet.';
      return;
    }

    // 2. Check Mapbox Token (basic check for presence)
    if (!mapboxToken || mapboxToken.trim() === '') {
      serviceStatus.value = 'invalid_token';
      statusMessage.value = 'Token Mapbox manquant ou invalide.';
      return;
    }

    // 3. Check Mapbox Server Reachability & Token Validity (Rust backend)
    try {
      const result = await invoke('check_mapbox_status', { token: mapboxToken });
      if (result.success) {
        serviceStatus.value = 'connected';
        statusMessage.value = 'Tous les services sont opérationnels.';
      } else {
        if (result.reason === 'unreachable') {
          serviceStatus.value = 'mapbox_unreachable';
          statusMessage.value = 'Serveur Mapbox inaccessible.';
        } else if (result.reason === 'invalid_token') {
          serviceStatus.value = 'invalid_token';
          statusMessage.value = 'Token Mapbox invalide.';
        } else {
          serviceStatus.value = 'disconnected'; // Generic error
          statusMessage.value = 'Problème de service Mapbox.';
        }
      }
    } catch (e) {
      console.error('Error checking Mapbox status:', e);
      serviceStatus.value = 'disconnected';
      statusMessage.value = 'Erreur lors de la vérification des services.';
    }
  };

  // Watch for statusMessage changes and show snackbar
  watch(statusMessage, (newMessage) => {
    let color = 'info';
    if (serviceStatus.value === 'connected') {
      color = 'success';
    } else if (serviceStatus.value === 'disconnected' || serviceStatus.value === 'mapbox_unreachable' || serviceStatus.value === 'invalid_token') {
      color = 'error';
    }
    showSnackbar(newMessage, color);
  });

  return {
    serviceStatus,
    statusMessage,
    checkAllServices,
  };
}