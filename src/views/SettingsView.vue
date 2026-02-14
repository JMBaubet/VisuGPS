<template>
  <v-container fluid class="pa-0">
    <SettingsToolbar 
      @open-exe-mode="showExeModeDialog = true" 
      @open-doc="isDocDialogVisible = true" 
      @purge-blacklist="handlePurgeClick"
    />
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

    <!-- Information Liste Déjà Vide -->
    <v-dialog v-model="showEmptyInfo" max-width="500px">
      <v-card>
        <v-card-title class="bg-info text-white px-4 py-2 d-flex align-center">
          <v-icon start icon="mdi-information-outline"></v-icon>
          Information
        </v-card-title>
        <v-card-text class="pa-4">
          Le fichier de la liste noire est déjà purgé.
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn variant="flat" color="info" @click="showEmptyInfo = false">Fermer</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Confirmation Purge Liste Noire -->
    <ConfirmationDialog
      v-model="showPurgeConfirm"
      title="Purge de la liste noire des télécommandes"
      message="Êtes-vous sûr de vouloir purger la liste noire des télécommandes ?<br>Toutes les restrictions passées seront supprimées."
      confirmText="Purger"
      cancelText="Annuler"
      color="error"
      icon="mdi-remote-off"
      @confirm="confirmPurge"
    />
  </v-container>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import SettingsToolbar from '@/components/Settings/SettingsToolbar.vue';
import SettingsTree from '@/components/Settings/SettingsTree.vue';
import ExeMode from '@/components/Settings/ExeMode.vue';
import LightDarkSwitch from '@/components/Settings/LightDarkSwitch.vue';
import DocDisplay from '@/components/DocDisplay.vue';
import ConfirmationDialog from '@/components/ConfirmationDialog.vue';

const showExeModeDialog = ref(false);
const isDocDialogVisible = ref(false);
const showPurgeConfirm = ref(false);
const showEmptyInfo = ref(false);
const { showSnackbar } = useSnackbar();

const handlePurgeClick = async () => {
  try {
    const isEmpty = await invoke('is_blacklist_empty');
    if (isEmpty) {
      showEmptyInfo.value = true;
    } else {
      showPurgeConfirm.value = true;
    }
  } catch (error) {
    showSnackbar(`Erreur lors de la vérification : ${error}`, 'error');
  }
};

const confirmPurge = async () => {
  try {
    const response = await invoke('purge_remote_blacklist');
    showSnackbar(response.message, 'success');
  } catch (error) {
    showSnackbar(`Erreur lors de la purge : ${error}`, 'error');
  }
};
</script>

<style scoped>
/* Styles pour la vue des paramètres */
</style>
