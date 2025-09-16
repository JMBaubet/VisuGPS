<template>
  <v-container fluid class="fill-height d-flex flex-column align-center pa-0">
    <AppMainBar @open-gpx-import-dialog="gpxImportDialog = true" />
    <v-btn color="primary">
      Aller aux Paramètres
    </v-btn>
    <GpxImportDialog v-model="gpxImportDialog" />
  </v-container>
</template>

<script setup>
import { ref } from 'vue';
import AppMainBar from '../components/AppMainBar.vue';
import GpxImportDialog from '../components/GpxImportDialog.vue';

const gpxImportDialog = ref(false);
</script>