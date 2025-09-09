import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from './useSnackbar';

let previousServiceStatus = null; // Moved outside and initialized to null

export function useServiceStatus() {
  const serviceStatus = ref('checking'); // 'checking', 'connected', 'disconnected', 'mapbox_unreachable', 'invalid_token'
  const statusMessage = ref('Vérification des services...');
  const { showSnackbar } = useSnackbar();

  const updateStatus = (newStatus, newMessage) => {
    // Only update if status changed
    if (newStatus !== previousServiceStatus) {
      // Special case: if transitioning from 'connected' to 'checking', do not update statusMessage
      // This prevents repeated 'checking' messages when already connected
      if (newStatus === 'checking' && previousServiceStatus === 'connected') {
        serviceStatus.value = newStatus; // Still update serviceStatus for icon
        previousServiceStatus = newStatus;
        return; // Do not update statusMessage, thus no snackbar
      }

      serviceStatus.value = newStatus;
      statusMessage.value = newMessage;
      previousServiceStatus = newStatus;
    }
  };

  const checkAllServices = async (mapboxToken) => {
    // Initial state for this check cycle
    updateStatus('checking', 'Vérification des services...');

    // 1. Check Internet Connectivity (Rust backend for reliability)
    try {
      const isOnline = await invoke('check_internet_connectivity');
      if (!isOnline) {
        updateStatus('disconnected', 'Pas de connexion Internet.');
        return;
      }
    }
    catch (e) {
      console.error('Error checking internet connectivity:', e);
      updateStatus('disconnected', 'Erreur lors de la vérification de la connexion Internet.');
      return;
    }

    // 2. Check Mapbox Token (basic check for presence)
    if (!mapboxToken || mapboxToken.trim() === '') {
      updateStatus('invalid_token', 'Token Mapbox manquant ou invalide.');
      return;
    }

    // 3. Check Mapbox Server Reachability & Token Validity (Rust backend)
    try {
      const result = await invoke('check_mapbox_status', { token: mapboxToken });
      if (result.success) {
        updateStatus('connected', 'Tous les services sont opérationnels.');
      } else {
        if (result.reason === 'unreachable') {
          updateStatus('mapbox_unreachable', 'Serveur Mapbox inaccessible.');
        } else if (result.reason === 'invalid_token') {
          updateStatus('invalid_token', 'Token Mapbox invalide.');
        } else {
          updateStatus('disconnected', 'Problème de service Mapbox.'); // Generic error
        }
      }
    }
    catch (e) {
      console.error('Error checking Mapbox status:', e);
      updateStatus('disconnected', 'Erreur lors de la vérification des services.');
    }
  };

  // Watch for statusMessage changes and show snackbar
  watch(statusMessage, (newMessage) => {
    if (serviceStatus.value === 'checking') { // Do not show snackbar for 'checking' status
      return;
    }
    let color = 'info';
    if (serviceStatus.value === 'connected') {
      color = 'success';
    } else if (serviceStatus.value === 'disconnected' || serviceStatus.value === 'invalid_token') {
      color = 'error';
    } else if (serviceStatus.value === 'mapbox_unreachable') {
      color = 'info'; // Or 'blue'
    }
    showSnackbar(newMessage, color);
  });

  return {
    serviceStatus,
    statusMessage,
    checkAllServices,
  };
}