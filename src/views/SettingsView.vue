<template>
  <v-container fluid class="pa-0">
    <SettingsToolbar @open-exe-mode="showExeModeDialog = true" @open-doc="isDocDialogVisible = true" />
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
    <v-dialog v-model="isDocDialogVisible" max-width="800px">
      <DocDisplay doc-path="docs/DocUtilisateur/parametres.md" @close="isDocDialogVisible = false" />
    </v-dialog>
  </v-container>
</template>

<script setup>
import { ref } from 'vue';
import SettingsToolbar from '@/components/SettingsToolbar.vue';
import SettingsTree from '@/components/SettingsTree.vue';
import ExeMode from '@/components/ExeMode.vue';
import LightDarkSwitch from '@/components/LightDarkSwitch.vue';
import DocDisplay from '@/components/DocDisplay.vue';

const showExeModeDialog = ref(false);
const isDocDialogVisible = ref(false);
</script>

<style scoped>
/* Styles pour la vue des paramètres */
</style>
