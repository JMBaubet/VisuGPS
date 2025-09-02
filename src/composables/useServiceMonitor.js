import { ref, onMounted, onUnmounted, watch } from 'vue'; // Add watch

export function useServiceMonitor() {
  const serviceStatus = ref('ok'); // 'ok', 'no-internet', 'mapbox-unreachable', 'mapbox-invalid-token'
  const mapboxTokenInvalid = ref(false); // New ref for specific token error
  const mapboxToken = import.meta.env.VITE_MAPBOX_TOKEN;

  let intervalId = null;
  const POLLING_INTERVAL_NORMAL = 30000; // 30 seconds
  const POLLING_INTERVAL_FAST = 5000;    // 5 seconds

  const checkInternetConnectivity = async () => {
    if (!navigator.onLine) {
      serviceStatus.value = 'no-internet';
      return false;
    }
    try {
      // Try to fetch a small, reliable resource
      await fetch('https://www.google.com/favicon.ico', { mode: 'no-cors' });
      return true;
    } catch (e) {
      serviceStatus.value = 'no-internet';
      return false;
    }
  };

  const checkMapboxService = async () => {
    mapboxTokenInvalid.value = false; // Reset specific token error
    if (!mapboxToken) { // Only check if token is falsy (null, undefined, empty string)
      serviceStatus.value = 'mapbox-invalid-token';
      mapboxTokenInvalid.value = true; // Set specific token error
      return false;
    }
    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 5000); // 5 seconds timeout

      const response = await fetch(`https://api.mapbox.com/geocoding/v5/mapbox.places/London.json?access_token=${mapboxToken}`, { signal: controller.signal });
      clearTimeout(timeoutId);

      if (!response.ok) {
        if (response.status === 401 || response.status === 403) {
          serviceStatus.value = 'mapbox-invalid-token';
          mapboxTokenInvalid.value = true; // Set specific token error
        } else {
          serviceStatus.value = 'mapbox-unreachable';
        }
        return false;
      }
      return true;
    } catch (e) {
      if (e.name === 'AbortError') {
        serviceStatus.value = 'mapbox-unreachable'; // Timeout
      } else {
        serviceStatus.value = 'mapbox-unreachable'; // Network error
      }
      return false;
    }
  };

  const monitorServices = async () => {
    serviceStatus.value = 'ok'; // Reset general status
    mapboxTokenInvalid.value = false; // Reset specific token error

    const isOnline = await checkInternetConnectivity();
    if (!isOnline) return;

    const isMapboxOk = await checkMapboxService();
    if (!isMapboxOk) return;

    // If all checks pass, status remains 'ok'
  };

  const startPolling = (interval) => {
    if (intervalId) {
      clearInterval(intervalId);
    }
    intervalId = setInterval(monitorServices, interval);
  };

  onMounted(() => {
    monitorServices(); // Initial check
    startPolling(POLLING_INTERVAL_NORMAL); // Start with normal polling
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

  return { serviceStatus, mapboxTokenInvalid };
}
