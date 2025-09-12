<template>
  <v-toolbar density="compact">
    <v-btn icon to="/">
      <v-icon>mdi-home</v-icon>
    </v-btn>
    <v-toolbar-title>Paramètres</v-toolbar-title>
    <v-spacer></v-spacer>
    <v-btn icon @click="openExeModeDialog">
      <v-icon>mdi-folder-cog-outline</v-icon>
    </v-btn>
  </v-toolbar>
</template>

<script setup>
import { ref } from 'vue';
import ExeMode from '@/components/ExeMode.vue';
import LightDarkSwitch from '@/components/LightDarkSwitch.vue';

const emit = defineEmits(['open-exe-mode']);

const openExeModeDialog = () => {
  emit('open-exe-mode');
};
</script>

<style scoped>
/* Styles spécifiques au composant */
</style>
