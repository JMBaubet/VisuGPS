<template>
  <v-toolbar flat class="pa-0">
    <v-row align="center" no-gutters>
      <v-col cols="12" md="4">
        <!-- Service Status Icon -->
        <v-icon :color="serviceStatusColor" class="pl-4" size="36">{{ serviceStatusIcon }}</v-icon>

        <!-- Chip for APP_ENV -->
        <v-chip
          v-if="showAppEnvChip"
          class="ma-2 blinking-chip"
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
        <!-- Dark/Light Mode Switch -->
        <v-switch
          v-model="isDarkTheme"
          hide-details
          inset
          color="primary"
          class="pr-4"
        >
          <template v-slot:prepend>
            <v-icon :color="isDarkTheme ? 'primary' : 'grey'">mdi-weather-night</v-icon>
          </template>
          <template v-slot:append>
            <v-icon :color="isDarkTheme ? 'grey' : '#FFA000'">mdi-weather-sunny</v-icon> <!-- Darker yellow/amber -->
          </template>
        </v-switch>
      </v-col>
    </v-row>
  </v-toolbar>
</template>

<script setup>
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useTheme } from 'vuetify';
import { useEnvironment } from '../composables/useEnvironment';
import { useServiceStatus } from '../composables/useServiceStatus';

const theme = useTheme();
const { appEnv, executionMode, mapboxToken } = useEnvironment();

const { serviceStatus, statusMessage, checkAllServices } = useServiceStatus();

const serviceStatusIcon = computed(() => {
  switch (serviceStatus.value) {
    case 'connected': return 'mdi-web-check';
    case 'disconnected': return 'mdi-web-off';
    case 'mapbox_unreachable': return 'mdi-mapbox'; // Changed icon
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

let serviceCheckInterval; // Declare interval variable

onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'dark') {
    theme.global.name.value = 'dark';
  } else {
    theme.global.name.value = 'light';
  }
});

// Watch mapboxToken to trigger checks only when it's available
watch(mapboxToken, (newToken) => {
  // Clear any existing interval to prevent multiple intervals
  if (serviceCheckInterval) {
    clearInterval(serviceCheckInterval);
  }

  if (newToken && newToken.trim() !== '') {
    // Initial service check
    checkAllServices(newToken);

    // Set up periodic service checks (e.g., every 30 seconds)
    serviceCheckInterval = setInterval(() => {
      checkAllServices(newToken);
    }, 10000); // 10 seconds
  } else {
    // If token becomes invalid/empty, stop checks and set status
    serviceStatus.value = 'invalid_token';
    statusMessage.value = 'Token Mapbox manquant ou invalide.';
  }
}, { immediate: true }); // Run immediately to catch initial token value

onUnmounted(() => {
  // Clear the interval when the component is unmounted
  if (serviceCheckInterval) {
    clearInterval(serviceCheckInterval);
  }
});

const isDarkTheme = computed({
  get() {
    return theme.global.current.value.dark;
  },
  set(value) {
    theme.global.name.value = value ? 'dark' : 'light';
    localStorage.setItem('theme', value ? 'dark' : 'light');
  }
});

const showAppEnvChip = computed(() => {
  return executionMode.value === 'EVAL' || executionMode.value === 'TEST';
});

const chipColor = computed(() => {
  if (executionMode.value === 'EVAL') {
    return 'orange';
  } else if (executionMode.value === 'TEST') {
    return 'red';
  }
  return 'primary';
});
</script>

<style scoped>
@keyframes blink {
  0% { opacity: 1; }
  49% { opacity: 1; }
  50% { opacity: 0; }
  99% { opacity: 0; }
  100% { opacity: 1; }
}

.blinking-chip {
  animation: blink 1s linear infinite;
}
</style>