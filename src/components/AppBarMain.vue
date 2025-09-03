<template>
  <v-app-bar app density="compact">
    <v-tooltip text="Etat des services Extérieurs" location="bottom">
      <template v-slot:activator="{ props }">
        <v-app-bar-nav-icon v-bind="props">
          <v-icon :color="statusIconColor">{{ statusIcon }}</v-icon>
        </v-app-bar-nav-icon>
      </template>
    </v-tooltip>

    <v-spacer></v-spacer>

    <v-btn text @click="downloadGpx">
      Télécharger GPX
    </v-btn>
    <v-btn text @click="goToEditView">
      Édition
    </v-btn>
    <v-btn icon @click="goToSettingsView">
      <v-icon>mdi-cog</v-icon>
    </v-btn>

    <ThemeToggle class="mr-4" />
  </v-app-bar>
</template>

<script setup>
import { computed, watch } from 'vue'; // Add watch
import { useRouter } from 'vue-router';
import ThemeToggle from './ThemeToggle.vue'; // Import ThemeToggle
import { useServiceMonitor } from '../composables/useServiceMonitor'; // Import useServiceMonitor
import { useMessageStore } from '../composables/useMessageStore'; // Import useMessageStore

const router = useRouter();
const { serviceStatus, mapboxTokenInvalid } = useServiceMonitor(); // Use the composable
const { addMessage } = useMessageStore(); // Use the message store

watch(mapboxTokenInvalid, (newValue) => {
  if (newValue) {
    addMessage({ message: 'Attention : Le token MapBox est invalide !', type: 'warning', duration: 5000 }); // Changed type
  }
});

watch(serviceStatus, (newValue) => {
  if (newValue === 'mapbox-unreachable') {
    addMessage({ message: 'Attention : Le serveur MapBox ne répond pas !', type: 'warning', duration: 5000 });
  } else if (newValue === 'no-internet') {
    addMessage({ message: 'Erreur : Pas de connexion à Internet vérifiez votre Réseau (Wifi).', type: 'error', duration: 10000 });
  }
});

const statusIcon = computed(() => {
  switch (serviceStatus.value) {
    case 'ok': return 'mdi-check-circle';
    case 'no-internet': return 'mdi-wifi-off';
    case 'mapbox-unreachable': return 'mdi-map-marker-off';
    case 'mapbox-invalid-token': return 'mdi-alert-circle';
    default: return 'mdi-help-circle';
  }
});

const statusIconColor = computed(() => {
  switch (serviceStatus.value) {
    case 'ok': return 'green';
    case 'no-internet': return 'red';
    case 'mapbox-unreachable': return 'blue';
    case 'mapbox-invalid-token': return 'orange';
    default: return 'grey';
  }
});

// Button actions
const downloadGpx = () => {
  // Logic to open file dialog and handle GPX download
  console.log('Download GPX clicked');
};

const goToEditView = () => {
  router.push('/edit');
};

const goToSettingsView = () => {
  router.push('/settings');
};

// TODO: Implement GPX download logic
</script>

<style scoped>
/* Add any specific styles for the app bar here */
</style>
