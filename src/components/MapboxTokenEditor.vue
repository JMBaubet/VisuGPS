<template>
  <div>
    <v-btn color="primary" @click="openDialog">Modifier le Token Mapbox</v-btn>

    <v-dialog v-model="dialog" persistent max-width="900px">
      <v-card>
        <v-card-title>
          <span class="headline">Éditer le Token Mapbox</span>
        </v-card-title>
        <v-card-text>
          <v-text-field
            v-model="token"
            label="Votre token Mapbox"
            outlined
            dense
          ></v-text-field>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="blue darken-1" text @click="closeDialog">Annuler</v-btn>
          <v-btn color="blue darken-1" @click="saveToken">Sauvegarder</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-snackbar v-model="snackbar" :timeout="3000" :color="snackbarColor">
      {{ snackbarText }}
    </v-snackbar>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const token = ref('');
const dialog = ref(false);
const snackbar = ref(false);
const snackbarText = ref('');
const snackbarColor = ref('success');

const openDialog = async () => {
  try {
    token.value = await invoke('read_mapbox_token');
    dialog.value = true;
  } catch (error) {
    showSnackbar(`Erreur lors de la lecture du token: ${error}`, 'error');
  }
};

const closeDialog = () => {
  dialog.value = false;
};

const saveToken = async () => {
  try {
    await invoke('write_mapbox_token', { token: token.value });
    showSnackbar('Token sauvegardé avec succès!', 'success');
    closeDialog();
  } catch (error) {
    showSnackbar(`Erreur lors de la sauvegarde du token: ${error}`, 'error');
  }
};

const showSnackbar = (text, color) => {
  snackbarText.value = text;
  snackbarColor.value = color;
  snackbar.value = true;
};
</script>