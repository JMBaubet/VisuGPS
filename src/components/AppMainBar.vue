<template>
  <v-toolbar flat class="pa-0">
    <v-row align="center" no-gutters>
      <v-col cols="12" md="4" class="d-flex align-center">
        <!-- Service Status Icon -->
        <v-icon :color="serviceStatusColor" class="pl-4" size="36">{{ serviceStatusIcon }}</v-icon>

        <!-- Remote Control Status Icon -->
        <v-icon :color="remoteStatusColor" class="pl-2" size="36" @click="handleRemoteIconClick">{{ remoteStatusIcon }}</v-icon>

        <!-- Commune Update Status Component -->
        <MajCommunesInfo class="ml-4" />

      </v-col>

      <v-col clos="12" md="4" class="d-flex justify-center align-center"> <!-- This column will take remaining space and center content -->
        <!-- Centered Title -->
        <div class="text-center">Gestion des circuits</div>
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

      <v-col cols="12" md="4" class="d-flex justify-end align-center">
        <!-- Import Button -->
        <v-btn icon @click="openImportDialog" title="Importer un fichier (GPX ou VGPS)">
          <v-icon >mdi-file-import-outline</v-icon>
        </v-btn>

        <!-- Settings Button -->
        <v-btn icon to="/settings">
          <v-icon>mdi-cog</v-icon>
        </v-btn>

        <!-- Help Button -->
        <v-btn icon @click="showHelpDialog = true">
          <v-icon color="info">mdi-book-open-page-variant-outline</v-icon>
        </v-btn>

        <!-- Dark/Light Mode Switch -->
        
      </v-col>
    </v-row>
  </v-toolbar>
  <RemoteControlDialog v-model="showRemoteDialog" />
  
  <!-- Help Dialog -->
  <v-dialog v-model="showHelpDialog" max-width="800px">
    <DocDisplay doc-path="/docs/DocUtilisateur/index.md" @close="showHelpDialog = false" />
  </v-dialog>
</template>

<script setup>
import { computed, onMounted, onUnmounted, watch, ref } from 'vue';
import { useEnvironment } from '../composables/useEnvironment';
import { useServiceStatus } from '../composables/useServiceStatus';
import { useSettings } from '../composables/useSettings';
import { useRemoteControlStatus } from '@/composables/useRemoteControlStatus';
import { showRemoteDialog } from '@/composables/useRemoteControlDialog';
import MajCommunesInfo from './MajCommunesInfo.vue';
import RemoteControlDialog from './RemoteControlDialog.vue';
import DocDisplay from './DocDisplay.vue'; // Added this import
import { invoke } from '@tauri-apps/api/core';
import { confirm } from '@tauri-apps/plugin-dialog';
import { useSnackbar } from '../composables/useSnackbar';

const emit = defineEmits(['open-import-dialog', 'circuit-imported']);
const { showSnackbar } = useSnackbar();

function openImportDialog() {
  emit('open-import-dialog');
}

const showHelpDialog = ref(false); // Added this line

// Environment composable is only for display purposes now (chip)
const { appEnv, executionMode } = useEnvironment();

// Service status composable handles all checking logic
const { serviceStatus, checkAllServices } = useServiceStatus();

// Remote control status composable
const { isRemoteConnected, remoteStatusIcon, remoteStatusColor } = useRemoteControlStatus();

async function handleRemoteIconClick() {
  if (isRemoteConnected.value) {
    const confirmed = await confirm("Voulez-vous vraiment déconnecter la télécommande ?", {
      title: 'Confirmation de déconnexion',
      okLabel: 'Déconnecter',
      cancelLabel: 'Annuler'
    });
    if (confirmed) {
      invoke('disconnect_active_remote_client').catch(err => {
        console.error("Erreur lors de la déconnexion de la télécommande:", err);
      });
    }
  } else {
    showRemoteDialog.value = true;
  }
}

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