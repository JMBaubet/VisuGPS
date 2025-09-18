<template>
  <v-container fluid class="fill-height d-flex flex-column align-center pa-0">
    <AppMainBar @open-gpx-import-dialog="gpxImportDialog = true" />
    <v-btn color="primary">
      Aller aux Paramètres
    </v-btn>
    <GpxImportDialog v-model="gpxImportDialog" @gpx-imported="handleGpxImported" />
    <TraceurSelectionDialog v-model="showTraceurSelectionDialog" :circuitId="circuitIdToAssignTraceur" />
  </v-container>
</template>

<script setup>
import { ref } from 'vue';
import AppMainBar from '../components/AppMainBar.vue';
import GpxImportDialog from '../components/GpxImportDialog.vue';
import TraceurSelectionDialog from '@/components/TraceurSelectionDialog.vue'; // Import du nouveau composant

const gpxImportDialog = ref(false);
const showTraceurSelectionDialog = ref(false);
const circuitIdToAssignTraceur = ref(null);

function handleGpxImported(circuitId) {
  circuitIdToAssignTraceur.value = circuitId;
  showTraceurSelectionDialog.value = true;
}
</script>