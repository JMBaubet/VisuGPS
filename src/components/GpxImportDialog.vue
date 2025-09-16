<template>
  <v-dialog v-model="dialog" max-width="600px">
    <v-card>
      <v-card-title>
        <span class="text-h5">Importer un fichier GPX</span>
      </v-card-title>
      <v-card-text style="max-height: 400px; overflow-y: auto;">
        <v-list v-model:selected="selectedFile" selectable lines="one">
          <v-list-item
            v-for="file in gpxFiles"
            :key="file"
            :value="file"
            color="primary"
          >
            <v-list-item-title>{{ file }}</v-list-item-title>
          </v-list-item>
        </v-list>
        <div v-if="error">{{ error }}</div>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="close">Annuler</v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="importFile" :disabled="selectedFile.length === 0">Importer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch } from 'vue';
import { core } from '@tauri-apps/api';

const props = defineProps({
  modelValue: Boolean
});

const emit = defineEmits(['update:modelValue']);

const dialog = ref(props.modelValue);
const gpxFiles = ref([]);
const selectedFile = ref([]); // In Vuetify 3, v-model:selected returns an array
const error = ref(null);

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    loadGpxFiles();
    selectedFile.value = []; // Reset selection on open
  }
});

watch(dialog, (newVal) => {
  if (newVal !== props.modelValue) {
    emit('update:modelValue', newVal);
  }
});

async function loadGpxFiles() {
  try {
    gpxFiles.value = await core.invoke('list_gpx_files');
    error.value = null;
  } catch (e) {
    error.value = `Erreur lors du chargement des fichiers GPX: ${e}`;
    gpxFiles.value = [];
  }
}

function close() {
  dialog.value = false;
}

function importFile() {
  if (selectedFile.value.length > 0) {
    console.log(`Importing ${selectedFile.value[0]}`);
    // La logique d'importation sera ajoutée ici dans une future étape
  }
  close();
}
</script>