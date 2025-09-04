<template>
  <v-app>
    <AppBarMain v-if="!isSettingsView" />
    <AppBarSetting v-else />
    <v-main>
      <router-view />
    </v-main>
    <v-snackbar
      v-model="snackbar.show"
      :timeout="snackbar.timeout"
      :color="snackbar.color"
      location="bottom right"
      @update:modelValue="handleSnackbarUpdate"
    >
      {{ snackbar.text }}
      
    </v-snackbar>
  </v-app>
</template>

<script setup>
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import AppBarMain from './components/AppBarMain.vue';
import AppBarSetting from './components/AppBarSetting.vue';
import { useMessageStore } from './composables/useMessageStore.js';

const route = useRoute();
const isSettingsView = computed(() => route.name === 'settings');

const { snackbar, onHidden } = useMessageStore();

const handleSnackbarUpdate = (value) => {
  if (!value) {
    // The timeout has finished or the user clicked close
    // Let the store know it can process the next message
    onHidden();
  }
};
</script>

<style>
/* Basic styling */
body {
  margin: 0;
  font-family: sans-serif;
}
html, body {
  overflow: hidden;
}
html, body, #app, .v-application {
  height: 100%;
  overflow: hidden !important; /* empêche le scroll */
}
</style>