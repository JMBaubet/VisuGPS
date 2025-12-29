<template>
  <v-app>
    <v-container :class="[{ 'app-frame': showFrame }, frameColorClass]" fluid class="pa-0">
      <Suspense>
        <router-view />
      </Suspense>
    </v-container>
    <SnackbarContainer />
    <PairingDialog /> <!-- Added PairingDialog -->
    <MigrationReportModal v-model="showMigrationModal" :report="migrationReport" />
    <WelcomeModal />
  </v-app>
</template>

<script setup>
import { onMounted, computed, ref } from 'vue';
import { useTheme } from 'vuetify';
import { listen } from '@tauri-apps/api/event';
import { useEnvironment } from '@/composables/useEnvironment';
import { useSettings } from '@/composables/useSettings';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import { useSharedUiState } from '@/composables/useSharedUiState';
import SnackbarContainer from '@/components/SnackbarContainer.vue';
import PairingDialog from '@/components/PairingDialog.vue';
import MigrationReportModal from '@/components/MigrationReportModal.vue';
import WelcomeModal from '@/components/WelcomeModal.vue';
import { invoke } from '@tauri-apps/api/core';

const { executionMode } = useEnvironment();
const { initSettings, status, updateReferenceField } = useSettings();
const theme = useTheme();
const { toggleBackButtonVisibility } = useSharedUiState();
useCommunesUpdate(); // Initialize the composable

const showMigrationModal = ref(false);
const migrationReport = ref('');

const showFrame = computed(() => {
  return executionMode.value === 'EVAL' || executionMode.value === 'TEST';
});

const frameColorClass = computed(() => {
  if (executionMode.value === 'EVAL') {
    return 'app-frame-eval';
  }
  if (executionMode.value === 'TEST') {
    return 'app-frame-test';
  }
  return '';
});

// Load settings on app mount
onMounted(async () => {
  await initSettings();
  // Restore theme from localStorage
  const savedTheme = window.localStorage.getItem('theme');
  if (savedTheme) {
    theme.change(savedTheme);
  }



  // --- DIAGNOSTIC TEST RE-ADD ---
  listen('test-event', (event) => {
    console.log('[DIAGNOSTIC] Received test-event from backend:', event.payload);
  });
  // --- END DIAGNOSTIC ---

  // Check for migration report
  try {
    const report = await invoke('get_migration_report');
    if (report) {
      migrationReport.value = report;
      // Si on a un rapport de migration, on passe en mode Update.
      // Cela permet d'afficher update.md même si c'est la toute première fois
      // que VisuGPS gère le statut (cas d'une montée de version < 1.0.0).
      await updateReferenceField('Status', 'Update');
    }
  } catch (e) {
    console.error("Failed to check for migration report:", e);
  }
});
</script>

<style>
/* Basic styling */
body {
  margin: 0;
  font-family: sans-serif;
}
html, body, #app {
  height: 100%;
  margin: 0;
  padding: 0;
}

.ope-border {
  border: 0px solid rgba(255, 255, 255, 0);
}
.eval-border {
  border: 5px solid rgb(var(--v-theme-info));
}

.test-border {
  border: 5px solid rgb(var(--v-theme-warning));
}

.app-frame {
  border-width: 5px; /* Changed to 5px */
  border-style: solid;
  /* Removed border-radius */
  height: 100%; /* Added for full height */
}

.app-frame-eval {
  border-color: rgb(var(--v-theme-info));
  /* Removed box-shadow */
}

.app-frame-test {
  border-color: rgb(var(--v-theme-warning)) !important;
  /* Removed box-shadow */
}

/* Hide scrollbar for Chrome, Safari and Opera */
body::-webkit-scrollbar,
html::-webkit-scrollbar,
*::-webkit-scrollbar {
  display: none;
}

/* Hide scrollbar for IE, Edge and Firefox */
body,
html {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}
</style>