<template>
  <v-dialog v-model="dialog" max-width="500">
    <v-card>
      <v-card-title class="headline">Connecter une télécommande</v-card-title>
      <v-card-text class="text-center">
        <p class="mb-4">
          Sélectionnez l'adresse IP de votre ordinateur sur le réseau local :
        </p>
        
        <v-select
          v-model="selectedIp"
          :items="availableIps"
          item-title="title"
          item-value="value"
          label="Interface Réseau"
          variant="outlined"
          density="comfortable"
          :loading="isLoading"
          hide-details="auto"
          class="mb-4"
        ></v-select>

        <p class="mb-2 text-caption text-grey">
          Scannez ce QR code avec votre mobile ou ouvrez l'URL :
        </p>

        <div v-if="qrCodeBase64" class="d-flex flex-column align-center">
          <img :src="qrCodeBase64" style="max-width: 250px; max-height: 250px;" alt="QR Code" />
          <p class="mt-4 font-weight-bold user-select-all">
            <a :href="url" target="_blank" class="text-decoration-none">{{ url }}</a>
          </p>
        </div>
        <div v-else-if="isLoading" class="d-flex flex-column align-center justify-center pa-8">
          <v-progress-circular indeterminate color="primary"></v-progress-circular>
          <p class="mt-4">Chargement des interfaces...</p>
        </div>
         <div v-else class="text-center pa-4 text-grey">
            Sélectionnez une IP pour générer le QR Code.
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
import { ref, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const props = defineProps({
  modelValue: Boolean,
});
const emit = defineEmits(['update:modelValue']);

const dialog = ref(props.modelValue);
const url = ref(null);
const qrCodeBase64 = ref(null);
const availableIps = ref([]);
const selectedIp = ref(null);
const isLoading = ref(false);

const loadInterfaces = async () => {
    isLoading.value = true;
    try {
        const interfaces = await invoke('get_network_interfaces');
        availableIps.value = interfaces.map((item) => ({
            title: `${item[0]} (${item[1]})`,
            value: item[1]
        }));

        if (availableIps.value.length > 0) {
             const preferred = availableIps.value.find(i => i.value.startsWith('192.168.'));
             if (preferred) {
                 selectedIp.value = preferred.value;
             } else {
                 selectedIp.value = availableIps.value[0].value;
             }
        }
    } catch (e) {
        console.error("Error loading network interfaces:", e);
    } finally {
        isLoading.value = false;
    }
};

onMounted(() => {
    listen('remote_pairing_request', () => {
        // Close the dialog immediately when a client tries to pair
        // making room for the Approval Dialog
        dialog.value = false;
    });
});

watch(() => props.modelValue, (newValue) => {
  dialog.value = newValue;
  if (newValue) {
      loadInterfaces();
  }
});

watch(selectedIp, async (newIp) => {
    if (!newIp) return;
    
    // Assuming port 9001 as default. 
    const port = 9001; 
    const newUrl = `http://${newIp}:${port}/`;
    url.value = newUrl;
    
    try {
        qrCodeBase64.value = await invoke('generate_qrcode_base64', { url: newUrl });
    } catch(e) {
        console.error("Error generating QR code:", e);
    }
});

watch(dialog, (newValue) => {
  emit('update:modelValue', newValue);
});
</script>

<style scoped>
.user-select-all {
    user-select: all;
}
</style>