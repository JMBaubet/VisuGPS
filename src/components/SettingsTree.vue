<template>
  <v-card>
    <v-card-title>Arbre des Paramètres</v-card-title>
    <v-card-text>
      <v-list dense>
        <template v-if="settings && settings.data && settings.data.groupes">
          <SettingsNode v-for="(group, index) in settings.data.groupes" :key="index" :node="group" />
        </template>
        <v-list-item v-else>
          <v-list-item-title>Chargement des paramètres...</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import SettingsNode from './SettingsNode.vue';

const settings = ref(null);

onMounted(async () => {
  try {
    settings.value = await invoke('read_settings');
  } catch (error) {
    console.error("Erreur lors de la lecture des paramètres:", error);
  }
});
</script>

<style scoped>
/* Styles spécifiques au composant */
</style>
