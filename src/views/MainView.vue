<template>
  <v-container fluid class="fill-height d-flex flex-column pa-0">
    <AppMainBar @open-gpx-import-dialog="gpxImportDialog = true" />

    <v-list lines="two" class="w-100">
      <CircuitListItem
        v-for="circuit in paginatedCircuits"
        :key="circuit.circuitId"
        :circuit="circuit"
      />
    </v-list>

    <v-pagination
      v-model="currentPage"
      :length="pageCount"
      class="mt-4"
    ></v-pagination>

    <GpxImportDialog v-model="gpxImportDialog" @gpx-imported="handleGpxImported" />
  </v-container>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import AppMainBar from '../components/AppMainBar.vue';
import GpxImportDialog from '../components/GpxImportDialog.vue';
import CircuitListItem from '@/components/CircuitListItem.vue';

const gpxImportDialog = ref(false);
const circuits = ref([]);

const currentPage = ref(1);
const itemsPerPage = ref(10);

const pageCount = computed(() => {
  return Math.ceil(circuits.value.length / itemsPerPage.value);
});

const paginatedCircuits = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage.value;
  const end = start + itemsPerPage.value;
  return circuits.value.slice(start, end);
});

async function refreshCircuits() {
  try {
    circuits.value = await invoke('get_circuits_for_display');
  } catch (error) {
    console.error("Failed to fetch circuits:", error);
  }
}

function handleGpxImported() {
  refreshCircuits();
}

onMounted(() => {
  refreshCircuits();
});
</script>