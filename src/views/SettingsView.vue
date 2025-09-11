<template>
  <v-container fluid>
    <SettingsToolbar @open-exe-mode="showExeModeDialog = true" />
    <v-row>
      <v-col cols="12" md="8" lg="6">
        <SettingsTree />
      </v-col>
      <v-spacer></v-spacer>
      <v-col cols="auto" class="mr-4">
        <LightDarkSwitch />
      </v-col>
    </v-row>
    <ExeMode v-model="showExeModeDialog" />
  </v-container>
</template>

<script setup>
import { ref } from 'vue';
import SettingsToolbar from '@/components/SettingsToolbar.vue';
import SettingsTree from '@/components/SettingsTree.vue';
import ExeMode from '@/components/ExeMode.vue';
import LightDarkSwitch from '@/components/LightDarkSwitch.vue';

const showExeModeDialog = ref(false);
</script>

<style scoped>
/* Styles pour la vue des paramètres */
</style>
