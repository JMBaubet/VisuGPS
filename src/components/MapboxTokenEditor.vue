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
import { ref, defineExpose } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useServiceMonitor } from '../composables/useServiceMonitor';

const token = ref('');
const dialog = ref(false);
const snackbar = ref(false);
const snackbarText = ref('');
const snackbarColor = ref('success');

const openDialog = async () => {
  try {
    const readToken = await invoke('read_mapbox_token');
    token.value = readToken;
    dialog.value = true;
  } catch (error) {
    showSnackbar(`Erreur lors de la lecture des informations Mapbox: ${error}`, 'error');
  }
};

const closeDialog = () => {
  dialog.value = false;
};

const saveToken = async () => {
  const tokenValue = token.value;

  const mapboxTokenRegex = /^(pk|sk|tk)\.[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+$/;

  if (!mapboxTokenRegex.test(tokenValue)) {
    showSnackbar('Le format du token Mapbox est invalide. Il doit commencer par "pk.", "sk." ou "tk.", contenir deux points et des caractères Base64URL.', 'error');
    return;
  }

  const prefix = tokenValue.substring(0, 3);
  const tokenLength = tokenValue.length;

  if (prefix === 'pk.' && (tokenLength < 80 || tokenLength > 100)) {
    showSnackbar('Le token public (pk.) semble avoir une longueur incorrecte (attendu ~90 caractères).', 'error');
    return;
  }

  if (prefix === 'sk.' && (tokenLength < 115 || tokenLength > 135)) {
    showSnackbar('Le token secret (sk.) semble avoir une longueur incorrecte (attendu ~125 caractères).', 'error');
    return;
  }

  try {
    await invoke('write_mapbox_token', { token: tokenValue });
    setMapboxToken(tokenValue);
    showSnackbar('Informations Mapbox sauvegardées avec succès!', 'success');
    closeDialog();
  } catch (error) {
    showSnackbar(`Erreur lors de la sauvegarde des informations Mapbox: ${error}`, 'error');
  }
};

const showSnackbar = (text, color) => {
  snackbarText.value = text;
  snackbarColor.value = color;
  snackbar.value = true;
};

const { setMapboxToken } = useServiceMonitor();

// Explicitly expose functions for template access
defineExpose({
  openDialog,
  closeDialog,
  saveToken,
  showSnackbar,
  token,
  dialog,
  snackbar,
  snackbarText,
  snackbarColor
});
</script>