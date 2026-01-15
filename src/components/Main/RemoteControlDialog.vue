<template>
  <v-dialog v-model="dialog" max-width="400">
    <v-card>
      <v-card-title class="headline">Connecter une télécommande</v-card-title>
      <v-card-text class="text-center">
        <p class="mb-4">
          Scannez ce QR code avec votre appareil mobile ou ouvrez l'URL suivante dans un navigateur :
        </p>
        <div v-if="qrCodeBase64" class="d-flex flex-column align-center">
          <img :src="qrCodeBase64" style="max-width: 250px; max-height: 250px;" alt="QR Code pour la télécommande" />
          <p class="mt-4 font-weight-bold">
            <a :href="url" target="_blank">{{ url }}</a>
          </p>
        </div>
        <div v-else class="d-flex flex-column align-center justify-center pa-8">
          <v-progress-circular indeterminate color="primary"></v-progress-circular>
          <p class="mt-4">Génération du QR code...</p>
        </div>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="primary" text @click="dialog = false">Fermer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  modelValue: Boolean,
});
const emit = defineEmits(['update:modelValue']);

const dialog = ref(props.modelValue);
const url = ref(null);
const qrCodeBase64 = ref(null);

watch(() => props.modelValue, async (newValue) => {
  dialog.value = newValue;
  if (newValue && !qrCodeBase64.value) { // Only generate if not already generated
    url.value = null;
    qrCodeBase64.value = null;
    try {
      const remoteUrl = await invoke('get_remote_control_url');
      url.value = remoteUrl;
      
      const base64Data = await invoke('generate_qrcode_base64', { url: remoteUrl });
      qrCodeBase64.value = base64Data;
    } catch (err) {
      console.error("Failed to generate QR Code:", err);
      url.value = "Erreur lors de la génération.";
      qrCodeBase64.value = null; // Ensure no broken image is shown
    }
  }
});

watch(dialog, (newValue) => {
  emit('update:modelValue', newValue);
});
</script>