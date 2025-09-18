<template>
  <v-dialog v-model="dialog" max-width="500px">
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
        ></v-combobox>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="close">Annuler</v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="saveTraceur" :disabled="!selectedTraceur">Valider</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch } from 'vue';
import { core } from '@tauri-apps/api';
import { useSnackbar } from '@/composables/useSnackbar';

const { showSnackbar } = useSnackbar();

const props = defineProps({
  modelValue: Boolean,
  circuitId: String, // Le circuitId pour lequel on doit assigner un traceur
});

const emit = defineEmits(['update:modelValue', 'traceur-selected']);

const dialog = ref(props.modelValue);
const traceurs = ref([]);
const selectedTraceur = ref(null);

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    loadTraceurs();
    selectedTraceur.value = null; // Réinitialiser le traceur sélectionné à l'ouverture
  }
});

watch(dialog, (newVal) => {
  if (newVal !== props.modelValue) {
    emit('update:modelValue', newVal);
  }
});

async function loadTraceurs() {
  try {
    traceurs.value = await core.invoke('list_traceurs');
  } catch (e) {
    showSnackbar(`Erreur lors du chargement des traceurs: ${e}`, 'error');
    traceurs.value = [];
  }
}

async function saveTraceur() {
  if (!selectedTraceur.value) {
    showSnackbar('Veuillez sélectionner ou créer un traceur.', 'warning');
    return;
  }

  let traceurIdToUse = null;
  let traceurNomToUse = null;

  if (typeof selectedTraceur.value === 'string') {
    // Nouveau traceur à créer
    try {
      const newTraceur = await core.invoke('add_traceur', { nom: selectedTraceur.value });
      traceurIdToUse = newTraceur.id;
      traceurNomToUse = newTraceur.nom;
      showSnackbar(`Traceur '${newTraceur.nom}' créé.`, 'success');
      await loadTraceurs(); // Recharger la liste pour la prochaine fois
    } catch (e) {
      showSnackbar(`Erreur lors de la création du traceur: ${e}`, 'error');
      return;
    }
  } else {
    // Traceur existant sélectionné
    traceurIdToUse = selectedTraceur.value.id;
    traceurNomToUse = selectedTraceur.value.nom;
  }

  // Appeler une nouvelle commande Tauri pour mettre à jour le circuit avec le traceurId
  try {
    await core.invoke('update_circuit_traceur', { circuitId: props.circuitId, traceurId: traceurIdToUse });
    showSnackbar(`Traceur '${traceurNomToUse}' assigné au circuit ${props.circuitId}.`, 'success');
    emit('traceur-selected', { circuitId: props.circuitId, traceurId: traceurIdToUse, traceurNom: traceurNomToUse });
    close();
  } catch (e) {
    showSnackbar(`Erreur lors de l'assignation du traceur: ${e}`, 'error');
  }
}

function close() {
  dialog.value = false;
}
</script>

<style scoped>
/* Styles spécifiques au composant si nécessaire */
</style>
