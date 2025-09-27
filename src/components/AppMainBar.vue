<template>
  <v-toolbar flat class="pa-0">
    <v-row align="center" no-gutters>
      <v-col cols="12" md="4">
        <!-- Service Status Icon -->
        <v-icon :color="serviceStatusColor" class="pl-4" size="36">{{ serviceStatusIcon }}</v-icon>

        <!-- Commune Update Status Icon -->
        <v-icon v-if="majCommuneIsRunning" color="green" class="pl-2" size="36">mdi-city-variant</v-icon>

        <!-- Chip for APP_ENV -->
        <v-chip
          v-if="showAppEnvChip"
          class="my-2 ml-4"
          :color="chipColor"
          label
        >
          {{ appEnv }}
        </v-chip>
      </v-col>

      <v-col clos="12" md="4" class="d-flex justify-center"> <!-- This column will take remaining space and center content -->
        <!-- Centered Title -->
        <div class="text-center">Accueil</div>
      </v-col>

      <v-col cols="12" md="4" class="d-flex justify-end align-center">
        <!-- Import GPX Button -->
        <v-btn icon @click="openGpxImportDialog">
          <v-icon>mdi-file-import-outline</v-icon>
        </v-btn>

        <!-- Settings Button -->
        <v-btn icon to="/settings">
          <v-icon>mdi-cog</v-icon>
        </v-btn>

        <!-- Dark/Light Mode Switch -->
        
      </v-col>
    </v-row>
  </v-toolbar>
</template>

<script setup>
import { computed, onMounted, onUnmounted, watch, ref } from 'vue';
import { useEnvironment } from '../composables/useEnvironment';
import { useServiceStatus } from '../composables/useServiceStatus';
import { useSettings } from '../composables/useSettings';
import { useCommunesUpdate } from '../composables/useCommunesUpdate';

const emit = defineEmits(['open-gpx-import-dialog']);

function openGpxImportDialog() {
  emit('open-gpx-import-dialog');
}

// Environment composable is only for display purposes now (chip)
const { appEnv, executionMode } = useEnvironment();

// Commune update status
const { majCommuneIsRunning } = useCommunesUpdate();

// Service status composable handles all checking logic
const { serviceStatus, checkAllServices } = useServiceStatus();

// Settings composable to get the polling interval
const { getSettingValue } = useSettings();

// Reactive polling interval from settings
const networkPollingInterval = computed(() => {
  return getSettingValue('Système.Timers.networkPolling');
});

const serviceStatusIcon = computed(() => {
  switch (serviceStatus.value) {
    case 'connected': return 'mdi-web-check';
    case 'disconnected': return 'mdi-web-off';
    case 'mapbox_unreachable': return 'mdi-mapbox';
    case 'invalid_token': return 'mdi-key-alert';
    case 'checking':
    default: return 'mdi-sync';
  }
});

const serviceStatusColor = computed(() => {
  switch (serviceStatus.value) {
    case 'connected': return 'green';
    case 'disconnected': return 'red';
    case 'mapbox_unreachable': return 'blue';
    case 'invalid_token': return 'red';
    case 'checking':
    default: return 'blue';
  }
});

let serviceCheckInterval;

const setupPolling = (interval) => {
  // Clear previous interval if it exists
  if (serviceCheckInterval) {
    clearInterval(serviceCheckInterval);
  }
  // Do not set up polling if interval is invalid or zero
  if (!interval || interval <= 0) {
    console.log('Polling disabled due to invalid interval.');
    return;
  }
  // Set up new interval
  serviceCheckInterval = setInterval(() => {
    checkAllServices();
  }, interval);
};

// Watch for changes in the polling interval setting and restart the polling
watch(networkPollingInterval, (newInterval) => {
  console.log(`Polling interval changed to ${newInterval}ms, restarting polling.`);
  setupPolling(newInterval);
});

// Perform an initial check on mount, and set up initial polling
onMounted(() => {
  console.log('AppMainBar mounted, performing initial service check and setting up polling.');
  checkAllServices();
  setupPolling(networkPollingInterval.value);
});

// Cleanup on unmount
onUnmounted(() => {
  if (serviceCheckInterval) {
    clearInterval(serviceCheckInterval);
  }
});

const showAppEnvChip = computed(() => {
  return executionMode.value === 'EVAL' || executionMode.value === 'TEST';
});

const chipColor = computed(() => {
  if (executionMode.value === 'EVAL') {
    return 'info';
  } else if (executionMode.value === 'TEST') {
    return 'warning';
  }
  return 'primary';
});
</script>

<style scoped>

</style>