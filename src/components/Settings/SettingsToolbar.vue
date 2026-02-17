<template>
  <v-toolbar density="compact">
    <v-btn icon to="/">
      <v-icon>mdi-home</v-icon>
    </v-btn>
    <v-toolbar-title>Configuration de VisuGPS</v-toolbar-title>
    <v-spacer></v-spacer>
    <v-btn icon @click="$emit('purge-blacklist')" title="Purger la liste noire des télécommandes" color="error">
      <v-icon>mdi-remote-off</v-icon>
    </v-btn>
    <v-btn icon @click="openExeModeDialog">
      <v-icon>mdi-database-cog-outline</v-icon>
    </v-btn>
    <v-btn icon @click="$emit('open-doc')" title="Documentation Paramètres" color="blue">
      <v-icon>mdi-book-open-page-variant-outline</v-icon>
    </v-btn>
    <div class="ml-4 mr-2">
      <slot name="append"></slot>
    </div>
  </v-toolbar>
</template>

<script setup>
import { ref } from 'vue';
import ExeMode from './ExeMode.vue';
import LightDarkSwitch from './LightDarkSwitch.vue';

const emit = defineEmits(['open-exe-mode', 'open-doc']);

const openExeModeDialog = () => {
  emit('open-exe-mode');
};
</script>

<style scoped>
/* Styles spécifiques au composant */
</style>
