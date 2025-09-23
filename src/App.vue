<template>
  <v-app>
    <v-container :class="[{ 'app-frame': showFrame }, frameColorClass]" fluid class="pa-0">
      <router-view />
    </v-container>
    <SnackbarContainer /> <!-- Added SnackbarContainer -->
  </v-app>
</template>

<script setup>
import { onMounted, computed } from 'vue';
import { useTheme } from 'vuetify';
import { useEnvironment } from '@/composables/useEnvironment';
import { useSettings } from '@/composables/useSettings';
import SnackbarContainer from '@/components/SnackbarContainer.vue';

const { executionMode } = useEnvironment();
const { initSettings } = useSettings();
const theme = useTheme();

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