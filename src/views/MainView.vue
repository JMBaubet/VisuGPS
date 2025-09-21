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
      :total-visible="7"
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
import { useSettings } from '@/composables/useSettings';

const gpxImportDialog = ref(false);
const circuits = ref([]);
const { getSettingValue } = useSettings();

const currentPage = ref(1);
const itemsPerPage = computed(() => getSettingValue('Accueil/circuitsPerPage') || 10);

const pageCount = computed(() => {
  if (!itemsPerPage.value) return 0;
  return Math.ceil(circuits.value.length / itemsPerPage.value);
});

const paginatedCircuits = computed(() => {
  if (!itemsPerPage.value) return [];
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