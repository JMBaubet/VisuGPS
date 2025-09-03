import { ref, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core'; // Import invoke

export function useServiceMonitor() {
  const serviceStatus = ref('ok'); // 'ok', 'no-internet', 'mapbox-unreachable', 'mapbox-invalid-token'
  const mapboxTokenInvalid = ref(false); // New ref for specific token error
  const mapboxToken = ref(null); // Reactive ref for Mapbox token

  let intervalId = null;
  const POLLING_INTERVAL_NORMAL = 30000; // 30 seconds
  const POLLING_INTERVAL_FAST = 5000;    // 5 seconds

  const loadMapboxToken = async () => {
    try {
      mapboxToken.value = await invoke('read_mapbox_token');
    } catch (error) {
      console.error('Failed to read Mapbox token from backend:', error);
      mapboxToken.value = null;
    }
  };

  const checkInternetConnectivity = async () => {
    if (!navigator.onLine) {
      serviceStatus.value = 'no-internet';
      return false;
    }
    try {
      await fetch('https://www.google.com/favicon.ico', { mode: 'no-cors' });
      return true;
    } catch (e) {
      serviceStatus.value = 'no-internet';
      return false;
    }
  };

  const checkMapboxService = async () => {
    mapboxTokenInvalid.value = false; // Reset specific token error
    if (!mapboxToken.value) {
      serviceStatus.value = 'mapbox-invalid-token';
      mapboxTokenInvalid.value = true;
      return false;
    }
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 5000);

      const url = `https://api.mapbox.com/tokens/v2?access_token=${mapboxToken.value}`;

      const response = await fetch(url, { signal: controller.signal });
      clearTimeout(timeoutId);

      if (!response.ok) {
        if (response.status === 401 || response.status === 403) {
          serviceStatus.value = 'mapbox-invalid-token';
          mapboxTokenInvalid.value = true;
        } else {
          serviceStatus.value = 'mapbox-unreachable';
        }
        return false;
      }

      const data = await response.json();

      // Check if the response indicates a valid token
      if (data && data.code === 'TokenValid') {
        return true;
      } else {
        serviceStatus.value = 'mapbox-invalid-token';
        mapboxTokenInvalid.value = true;
        return false;
      }
    } catch (e) {
      console.error('Fetch error:', e);
      if (e.name === 'AbortError') {
        serviceStatus.value = 'mapbox-unreachable';
      } else {
        serviceStatus.value = 'mapbox-unreachable';
      }
      return false;
    }
  };

  const monitorServices = async () => {
    serviceStatus.value = 'ok'; // Reset general status
    mapboxTokenInvalid.value = false; // Reset specific token error

    const isOnline = await checkInternetConnectivity();
    if (!isOnline) return;

    await loadMapboxToken();

    const isMapboxOk = await checkMapboxService();
    if (!isMapboxOk) return;
  };

  const startPolling = (interval) => {
    if (intervalId) {
      clearInterval(intervalId);
    }
    intervalId = setInterval(monitorServices, interval);
  };

  const setMapboxToken = (newToken) => {
    mapboxToken.value = newToken;
    monitorServices();
  };

  onMounted(() => {
    monitorServices();
    startPolling(POLLING_INTERVAL_NORMAL);
  });

  onUnmounted(() => {
    if (intervalId) {
      clearInterval(intervalId);
    }
  });

  watch(serviceStatus, (newStatus) => {
    if (newStatus === 'mapbox-unreachable' || newStatus === 'mapbox-invalid-token' || newStatus === 'no-internet') {
      startPolling(POLLING_INTERVAL_FAST);
    } else {
      startPolling(POLLING_INTERVAL_NORMAL);
    }
  });

  return { serviceStatus, mapboxTokenInvalid, setMapboxToken };
}
