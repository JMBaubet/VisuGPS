<template>
  <v-dialog :model-value="dialog" v-if="dialog" max-width="500px">
    <v-card>
      <v-card-title>
        <span class="text-h5">Sélectionner un traceur pour le circuit</span>
      </v-card-title>
      <v-card-text>
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
          autocomplete="off"
          hide-no-data
          :menu-props="{ maxHeight: '150px' }"
          :filter-keys="['nom']"
        ></v-combobox>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="closeDialog">Annuler</v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="handleValidation" :disabled="!selectedTraceur">Valider</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref } from 'vue';
import { core } from '@tauri-apps/api';
import { useSnackbar } from '@/composables/useSnackbar';

const { showSnackbar } = useSnackbar();

const dialog = ref(false);
const traceurs = ref([]);
const selectedTraceur = ref(null);

let resolvePromise = null;
let rejectPromise = null;

async function open() {
  console.log('TraceurSelectionDialog: open() called, setting dialog.value = true');
  dialog.value = true;
  selectedTraceur.value = null;
  await loadTraceurs();
  return new Promise((resolve, reject) => {
    resolvePromise = resolve;
    rejectPromise = reject;
  });
}

function closeDialog() {
  console.log('TraceurSelectionDialog: closeDialog() called, setting dialog.value = false');
  dialog.value = false;
  if (rejectPromise) {
    rejectPromise('Sélection du traceur annulée.');
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

async function handleValidation() {
  console.log('TraceurSelectionDialog: handleValidation() called');
  if (!selectedTraceur.value) {
    showSnackbar('Veuillez sélectionner ou créer un traceur.', 'warning');
    return;
  }

  let traceurIdToUse = null;

  try {
    if (typeof selectedTraceur.value === 'string') {
      console.log('TraceurSelectionDialog: Creating new traceur:', selectedTraceur.value);
      const newTraceur = await core.invoke('add_traceur', { nom: selectedTraceur.value });
      traceurIdToUse = newTraceur.id;
      showSnackbar(`Traceur '${newTraceur.nom}' créé.`, 'success');
    } else {
      console.log('TraceurSelectionDialog: Using existing traceur:', selectedTraceur.value.id);
      traceurIdToUse = selectedTraceur.value.id;
    }

    if (resolvePromise) {
      console.log('TraceurSelectionDialog: Resolving promise with:', traceurIdToUse);
      resolvePromise(traceurIdToUse);
    }
  } catch (e) {
    console.error('TraceurSelectionDialog: Error during validation:', e);
    showSnackbar(`Erreur lors de la création du traceur: ${e}`, 'error');
    if (rejectPromise) {
      rejectPromise(e);
    }
  } finally {
    console.log('TraceurSelectionDialog: finally block, setting dialog.value = false');
    dialog.value = false;
  }
}

// Exposer la fonction open pour le composant parent
defineExpose({ open });

</script>

<style scoped>
/* Styles spécifiques au composant si nécessaire */
</style>
