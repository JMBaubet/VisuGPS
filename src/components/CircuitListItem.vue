<template>
  <v-list-item :value="circuit.circuitId">
    <div>
      <v-list-item-title>{{ circuit.nom }}</v-list-item-title>
      <v-list-item-subtitle>
        {{ circuit.distanceKm }} km | {{ circuit.deniveleM }} m | {{ circuit.villeDepart }}
      </v-list-item-subtitle>
    </div>

    <template v-slot:append>
      <v-btn icon="mdi-bug" variant="text" v-if="isDev" @click.stop="debugCircuit" color="warning"></v-btn>
      <v-btn icon="mdi-pencil" variant="text" @click.stop="editTracking"></v-btn>
      <v-btn icon="mdi-cube-scan" variant="text" @click.stop="view3D"></v-btn>
      <v-btn icon="mdi-delete" variant="text" @click.stop="deleteCircuit" color="error"></v-btn>
    </template>
  </v-list-item>

  <ConfirmationDialog
    v-model="showConfirmDialog"
    title="Confirmation de suppression"
      :message="`Êtes-vous sûr de vouloir supprimer le circuit '${props.circuit.nom}' ? Cette action est irréversible.`"    confirm-text="Supprimer"
    cancel-text="Annuler"
    @confirm="proceedDeletion"
    @cancel="showConfirmDialog = false"
  />
</template>

<script setup>
import { ref, defineEmits } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import ConfirmationDialog from '@/components/ConfirmationDialog.vue';

const props = defineProps({
  circuit: {
    type: Object,
    required: true,
  },
});

const emit = defineEmits(['circuit-deleted']);

const isDev = ref(import.meta.env.DEV);
const router = useRouter();
const { showSnackbar } = useSnackbar();

const showConfirmDialog = ref(false);

const debugCircuit = () => {
  router.push({ name: 'DebugTracking', params: { circuitId: props.circuit.circuitId } });
};

const editTracking = () => {
  router.push({ name: 'EditView', params: { circuitId: props.circuit.circuitId } });
};

const view3D = () => {
  console.log('View 3D for circuit:', props.circuit.circuitId);
  // Future implementation
};

const deleteCircuit = async () => {
  showConfirmDialog.value = true; // Open the custom dialog
};

const proceedDeletion = async () => {
  try {
    await invoke('delete_circuit', { circuitId: props.circuit.circuitId });
    showSnackbar('Circuit supprimé avec succès.', 'success');
    emit('circuit-deleted');
  } catch (error) {
    showSnackbar(`Erreur lors de la suppression du circuit : ${error}`, 'error');
    console.error('Error deleting circuit:', error);
  }
};
</script>

<style scoped>
</style>
