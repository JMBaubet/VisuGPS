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
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const props = defineProps({
  circuit: {
    type: Object,
    required: true,
  },
});

const isDev = ref(import.meta.env.DEV);
const router = useRouter();

const debugCircuit = () => {
  router.push({ name: 'DebugTracking', params: { circuitId: props.circuit.circuitId } });
};

const editTracking = () => {
  console.log('Edit tracking for circuit:', props.circuit.circuitId);
  // Future implementation
};

const view3D = () => {
  console.log('View 3D for circuit:', props.circuit.circuitId);
  // Future implementation
};

const deleteCircuit = () => {
  console.log('Delete circuit:', props.circuit.circuitId);
  // Future implementation
};
</script>

<style scoped>
</style>
