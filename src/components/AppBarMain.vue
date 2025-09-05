<template>
  <v-app-bar app density="compact">
    <v-tooltip text="Etat des services Extérieurs" location="bottom">
      <template v-slot:activator="{ props }">
        <v-app-bar-nav-icon v-bind="props">
          <v-icon :color="statusIconColor">{{ statusIcon }}</v-icon>
        </v-app-bar-nav-icon>
      </template>
    </v-tooltip>

    <v-chip v-if="envInfo" :color="envInfo.color" class="mr-4" variant="elevated">
      <v-icon start :icon="envInfo.icon"></v-icon>
      {{ envInfo.text }}
    </v-chip>

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
import { computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import ThemeToggle from './ThemeToggle.vue';
import { useServiceMonitor } from '../composables/useServiceMonitor';
import { useMessageStore } from '../composables/useMessageStore';

const router = useRouter();
const { serviceStatus, mapboxTokenInvalid } = useServiceMonitor();
const { addMessage } = useMessageStore();

const envInfo = computed(() => {
  const env = import.meta.env.VITE_APP_ENV || 'prod';
  switch (env) {
    case 'dev':
      return { text: 'DEV', color: 'orange', icon: 'mdi-wrench' };
    case 'sandbox':
      return { text: 'SANDBOX', color: 'purple', icon: 'mdi-flask' };
    case 'prod':
      // Conformément à GEMINI.md, le badge est optionnel pour la prod.
      // Retourner null le masque.
      return null;
    default:
      return null;
  }
});

watch(mapboxTokenInvalid, (newValue) => {
  if (newValue) {
    addMessage('Le token MapBox est invalide !', 'error', 2500);
  }
});

watch(serviceStatus, (newValue) => {
  if (newValue === 'mapbox-unreachable') {
    addMessage('Le serveur MapBox ne répond pas !', 'info', 5000);
  } else if (newValue === 'no-internet') {
    addMessage('Pas de connexion à Internet vérifiez votre Réseau (Wifi).', 'error', 10000);
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
