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

        <v-combobox
          v-model="selectedTraceur"
          :items="traceurs"
          item-title="nom"
          item-value="id"
          label="Sélectionner ou créer un traceur"
          variant="outlined"
          clearable
          class="mb-4"
          :rules="[v => !!v || 'Un traceur est requis']"
          required
          return-object
        ></v-combobox>

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
        <v-btn color="blue-darken-1" variant="text" @click="importFile" :disabled="selectedFile.length === 0 || !selectedTraceur">Importer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { core } from '@tauri-apps/api';
import { useSnackbar } from '@/composables/useSnackbar';

const { showSnackbar } = useSnackbar();

const props = defineProps({
  modelValue: Boolean
});

const emit = defineEmits(['update:modelValue']);

const dialog = ref(props.modelValue);
const gpxFiles = ref([]);
const selectedFile = ref([]);
const error = ref(null);
const filterText = ref('');
const traceurs = ref([]);
const selectedTraceur = ref(null);

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
    loadTraceurs();
    selectedFile.value = [];
    filterText.value = ''; // Reset filter on open
    selectedTraceur.value = null; // Reset selected traceur
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

async function loadTraceurs() {
  try {
    traceurs.value = await core.invoke('list_traceurs');
  } catch (e) {
    showSnackbar(`Erreur lors du chargement des traceurs: ${e}`, 'error');
    traceurs.value = [];
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
  if (!selectedTraceur.value) {
    showSnackbar('Veuillez sélectionner ou créer un traceur.', 'warning');
    return;
  }

  let traceurIdToUse = null;
  if (typeof selectedTraceur.value === 'string') {
    // Nouveau traceur à créer
    try {
      const newTraceur = await core.invoke('add_traceur', { nom: selectedTraceur.value });
      traceurIdToUse = newTraceur.id;
      showSnackbar(`Traceur '${newTraceur.nom}' créé.`, 'success');
      // Recharger la liste des traceurs pour inclure le nouveau
      await loadTraceurs();
      // Réinitialiser selectedTraceur pour forcer la mise à jour du v-combobox
      selectedTraceur.value = newTraceur; // Sélectionner le nouveau traceur
    } catch (e) {
      showSnackbar(`Erreur lors de la création du traceur: ${e}`, 'error');
      return;
    }
  } else {
    // Traceur existant sélectionné
    traceurIdToUse = selectedTraceur.value.id;
  }

  const filename = selectedFile.value[0];
  try {
    const result = await core.invoke('process_gpx_file', { filename, traceurId: traceurIdToUse });
    showSnackbar(result, 'success');
  } catch (e) {
    showSnackbar(`Erreur lors de l'import: ${e}`, 'error');
  }

  close();
}
</script>