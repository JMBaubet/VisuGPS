<template>
  <v-dialog v-model="dialog" max-width="600px">
    <v-card>
      <v-card-title>
        <span class="text-h5">Importer un fichier GPX</span>
      </v-card-title>
      <v-card-text>
        <v-text-field
          v-model="filterText"
          label="Filtrer par nom"
          prepend-inner-icon="mdi-magnify"
          variant="outlined"
          hide-details
          clearable
          class="mb-4"
        ></v-text-field>



        <v-list
          v-model:selected="selectedFile"
          selectable
          lines="one"
          style="max-height: 350px; overflow-y: auto;"
        >
          <v-list-item
            v-for="file in filteredGpxFiles"
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
  <TraceurSelectionDialog ref="traceurDialog" />
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { core } from '@tauri-apps/api';
import { useSnackbar } from '@/composables/useSnackbar';
import TraceurSelectionDialog from './TraceurSelectionDialog.vue';

const { showSnackbar } = useSnackbar();

const props = defineProps({
  modelValue: Boolean
});

const emit = defineEmits(['update:modelValue', 'gpx-imported']);

const dialog = ref(props.modelValue);
const gpxFiles = ref([]);
const selectedFile = ref([]);
const error = ref(null);
const filterText = ref('');
const loading = ref(false);
const traceurDialog = ref(null);


const filteredGpxFiles = computed(() => {
  if (!filterText.value) {
    return gpxFiles.value;
  }
  return gpxFiles.value.filter(file =>
    file.toLowerCase().includes(filterText.value.toLowerCase())
  );
});

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    loadGpxFiles();
    selectedFile.value = [];
    filterText.value = ''; // Reset filter on open
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

async function importFile() {
  if (selectedFile.value.length === 0) {
    showSnackbar('Veuillez sélectionner un fichier GPX.', 'warning');
    return;
  }

  const filename = selectedFile.value[0];
  loading.value = true;
  error.value = null;

  try {
    // Phase 1: Analyse
    const draftCircuit = await core.invoke('analyze_gpx_file', { filename });

    // Fermer la boîte de dialogue actuelle avant d'en ouvrir une autre
    dialog.value = false;

    // Phase 2: Sélection du traceur
    console.log('GpxImportDialog: Opening TraceurSelectionDialog...');
    const traceurId = await traceurDialog.value.open();
    console.log('GpxImportDialog: TraceurSelectionDialog closed, received traceurId:', traceurId);

    // Phase 3: Validation
    const circuitId = await core.invoke('commit_new_circuit', { 
      draft: draftCircuit, 
      traceurId: traceurId 
    });

    // Récupérer l'état de l'application pour obtenir app_env_path
    const appState = await core.invoke('get_app_state');
    const appEnvPath = appState.app_env_path;

    // Construire le chemin du lineString.json
    const lineStringPath = `${appEnvPath}\\data\\${circuitId}\\lineString.json`;

    // Récupérer les paramètres actuels
    const settings = await core.invoke('read_settings');

    // Phase 4: Génération de la vignette
    await core.invoke('generate_gpx_thumbnail', {
      circuitId: circuitId,
      lineStringPath: lineStringPath,
      settings: settings
    });

    showSnackbar(`Circuit '${draftCircuit.nom}' importé avec succès.`, 'success');
    emit('gpx-imported', circuitId);

  } catch (e) {
    // Si l'erreur n'est pas une annulation de la part de l'utilisateur, l'afficher.
    if (typeof e === 'string' && !e.includes('annulée')) {
      showSnackbar(`Erreur lors de l\'import: ${e}`, 'error');
    }
    // Si l'utilisateur annule, ne rien faire.
  } finally {
    loading.value = false;
    // Ne pas appeler close() ici car le dialogue est déjà fermé
  }
}
</script>