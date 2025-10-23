<template>
    <v-dialog v-model="dialog" persistent max-width="400">
      <v-card>
        <v-card-title class="headline">Demande de Couplage Télécommande</v-card-title>
        <v-card-text>
          Un client distant (ID: <strong>{{ clientId }}</strong>) souhaite se connecter.
          <br>
          Code de couplage : <strong class="pairing-code">{{ pairingCode }}</strong>
          <br>
          Voulez-vous autoriser cette connexion ?
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="red darken-1" text @click="reply(false)">Refuser</v-btn>
          <v-btn color="green darken-1" text @click="reply(true)">Accepter</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </template>

  <script setup>
  import { ref, onMounted } from 'vue';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  const dialog = ref(false);
  const clientId = ref('');
  const pairingCode = ref('');

  onMounted(() => {
    listen('ask_pairing_approval', (event) => {
      console.log("ask_pairing_approval event received:", event.payload);
      clientId.value = event.payload.client_id;
      pairingCode.value = event.payload.pairing_code;
      dialog.value = true;
    });
  });

  const reply = async (accepted) => {
    dialog.value = false;
    try {
      await invoke('reply_to_pairing_request', {
        clientId: clientId.value,
        accepted: accepted,
        clientName: `Client ${clientId.value.substring(0, 8)}` // Placeholder name
      });
      console.log(`Réponse de couplage envoyée : ${accepted ? 'Accepté' : 'Refusé'}`);
    } catch (error) {
      console.error("Erreur lors de l'envoi de la réponse de couplage:", error);
    }
  };
  </script>

  <style scoped>
  .pairing-code {
    font-size: 1.5em;
    font-weight: bold;
    color: #007bff;
  }
  </style>
