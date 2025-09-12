<template>
  <v-card color="transparent" flat>
    <v-card-title>Arbre des Paramètres</v-card-title>
    <v-card-text class="scrolling-card-text">
      <v-list dense bg-color="transparent">
        <template v-if="settings && settings.data && settings.data.groupes">
          <SettingsNode 
            v-for="group in settings.data.groupes" 
            :key="group.identifiant" 
            :node="group" 
            currentPath="" 
            @settings-updated="loadSettings"
          />
        </template>
        <v-list-item v-else>
          <v-list-item-title>Chargement des paramètres...</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { useSettings } from '@/composables/useSettings';
import SettingsNode from './SettingsNode.vue';

const { settings } = useSettings();

</script>

<style scoped>
.scrolling-card-text {
  height: 75vh; /* Adjust this value as needed */
  overflow-y: auto;
}
</style>
