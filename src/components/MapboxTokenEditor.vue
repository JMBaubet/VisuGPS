<template>
  <v-card>
    <v-card-title>Token Mapbox</v-card-title>
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
      <v-btn color="primary" @click="saveToken">Sauvegarder</v-btn>
    </v-card-actions>
     <v-snackbar v-model="snackbar" :timeout="3000" :color="snackbarColor">
      {{ snackbarText }}
    </v-snackbar>
  </v-card>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const token = ref('');
const snackbar = ref(false);
const snackbarText = ref('');
const snackbarColor = ref('success');

onMounted(async () => {
  try {
    token.value = await invoke('read_mapbox_token');
  } catch (error) {
    showSnackbar(`Erreur lors de la lecture du token: ${error}`, 'error');
  }
});

const saveToken = async () => {
  try {
    await invoke('write_mapbox_token', { token: token.value });
    showSnackbar('Token sauvegardé avec succès!', 'success');
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
