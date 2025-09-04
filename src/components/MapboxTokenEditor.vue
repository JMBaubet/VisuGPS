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

    
  </div>
</template>

<script setup>
import { ref, defineExpose } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useServiceMonitor } from '../composables/useServiceMonitor';
import { useMessageStore } from '../composables/useMessageStore.js';

const { addMessage } = useMessageStore();

const token = ref('');
const dialog = ref(false);

const openDialog = async () => {
  try {
    const readToken = await invoke('read_mapbox_token');
    token.value = readToken;
    dialog.value = true;
  } catch (error) {
    addMessage(`Erreur lors de la lecture des informations Mapbox: ${error}`, 'error');
  }
};

const closeDialog = () => {
  dialog.value = false;
};

const saveToken = async () => {
  const tokenValue = token.value;

  const mapboxTokenRegex = /^(pk|sk|tk)\.[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+$/;

  if (!mapboxTokenRegex.test(tokenValue)) {
    addMessage('Le format du token Mapbox est invalide. Il doit commencer par "pk.", "sk." ou "tk.", contenir deux points et des caractères Base64URL.', 'error', 10000);
    return;
  }

  const prefix = tokenValue.substring(0, 3);
  const tokenLength = tokenValue.length;

  if (prefix === 'pk.' && (tokenLength < 80 || tokenLength > 100)) {
    addMessage('Le token public (pk.) semble avoir une longueur incorrecte (attendu ~90 caractères).', 'error', 5000);
    return;
  }

  if (prefix === 'sk.' && (tokenLength < 115 || tokenLength > 135)) {
    addMessage('Le token secret (sk.) semble avoir une longueur incorrecte (attendu ~125 caractères).', 'error', 5000);
    return;
  }

  try {
    await invoke('write_mapbox_token', { token: tokenValue });
    setMapboxToken(tokenValue);
    addMessage('Votre token Mapbox a été sauvegardé avec succès !', 'success', 5000);
    closeDialog();
  } catch (error) {
    addMessage(`Erreur lors de la sauvegarde des informations Mapbox: ${error}`, 'error');
  }
};

const { setMapboxToken } = useServiceMonitor();

// Explicitly expose functions for template access
defineExpose({
  openDialog,
  closeDialog,
  saveToken,
  token,
  dialog
});
</script>