<template>
  <v-container fluid class="fill-height d-flex flex-column pa-0">
    <AppMainBar @open-gpx-import-dialog="gpxImportDialog = true" />

    <div class="w-100">
      <CircuitFilter 
        v-if="showFilters && filterData"
        :filter-data="filterData"
        v-model="activeFilters"
        v-model:sortValue="sortOptions"
      />
    </div>

    <v-list lines="two" class="w-100 bg-transparent">
      <CircuitListItem
        v-for="circuit in paginatedCircuits"
        :key="circuit.circuitId"
        :circuit="circuit"
        :all-communes="allCommunes"
        :all-traceurs="allTraceurs"
        @circuit-deleted="handleCircuitDeleted"
        @circuit-updated="handleCircuitUpdated"
      />
    </v-list>

    <v-pagination
      v-if="pageCount > 1"
      v-model="currentPage"
      :length="pageCount"
      :total-visible="7"
      class="mt-4"
    ></v-pagination>

    <GpxImportDialog v-model="gpxImportDialog" @gpx-imported="handleGpxImported" />
  </v-container>
</template>

<script setup>
import { ref, onMounted, computed, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import AppMainBar from '../components/AppMainBar.vue';
import GpxImportDialog from '../components/GpxImportDialog.vue';
import CircuitListItem from '@/components/CircuitListItem.vue';
import CircuitFilter from '@/components/CircuitFilter.vue';
import { useSettings } from '@/composables/useSettings';
import { showRemoteDialog } from '@/composables/useRemoteControlDialog';

const gpxImportDialog = ref(false);
const allCircuits = ref([]);
const allCommunes = ref([]);
const allTraceurs = ref([]);
const filterData = ref(null);

const { getSettingValue } = useSettings();

const activeFilters = ref(null);
const sortOptions = ref({ by: 'isoDateTime', order: 'desc' });

const currentPage = ref(1);
const itemsPerPage = computed(() => getSettingValue('Accueil/circuitsPerPage') || 10);

const showFilters = computed(() => {
  return true;
});

const filteredAndSortedCircuits = computed(() => {
  if (!allCircuits.value.length || !activeFilters.value) {
    return [];
  }

  let result = [...allCircuits.value];

  // 1. Filter
  // Name
  if (activeFilters.value.nom) {
    const search = activeFilters.value.nom.toLowerCase();
    result = result.filter(c => c.nom.toLowerCase().includes(search));
  }
  // City
  if (activeFilters.value.villeId) {
    result = result.filter(c => c.villeDepartId === activeFilters.value.villeId);
  }
  // Tracer
  if (activeFilters.value.traceurId) {
    result = result.filter(c => c.traceurId === activeFilters.value.traceurId);
  }
  // Distance
  if (activeFilters.value.distance) {
    const [min, max] = activeFilters.value.distance;
    result = result.filter(c => c.distanceKm >= min && c.distanceKm <= max);
  }
  // Elevation
  if (activeFilters.value.denivele) {
    const [min, max] = activeFilters.value.denivele;
    result = result.filter(c => c.deniveleM >= min && c.deniveleM <= max);
  }

  // 2. Sort
  result.sort((a, b) => {
    const field = sortOptions.value.by;
    let valA = a[field];
    let valB = b[field];

    if (typeof valA === 'string') {
      valA = valA.toLowerCase();
      valB = valB.toLowerCase();
    }

    if (valA < valB) return sortOptions.value.order === 'asc' ? -1 : 1;
    if (valA > valB) return sortOptions.value.order === 'asc' ? 1 : -1;
    return 0;
  });

  return result;
});

const pageCount = computed(() => {
  if (!itemsPerPage.value) return 0;
  return Math.ceil(filteredAndSortedCircuits.value.length / itemsPerPage.value);
});

const paginatedCircuits = computed(() => {
  if (!itemsPerPage.value) return [];
  const start = (currentPage.value - 1) * itemsPerPage.value;
  const end = start + itemsPerPage.value;
  return filteredAndSortedCircuits.value.slice(start, end);
});

async function refreshCircuits() {
  try {
    allCircuits.value = await invoke('get_circuits_for_display');
    // Reset page to 1 on refresh
    currentPage.value = 1;
  } catch (error) {
    console.error("Failed to fetch circuits:", error);
  }
}

async function loadFilterData() {
  try {
    const data = await invoke('get_filter_data');
    filterData.value = data;
    allCommunes.value = data.villes;
    allTraceurs.value = data.traceurs;
    // Initialize filters with default values
    activeFilters.value = {
      nom: '',
      villeId: null,
      traceurId: null,
      distance: [data.minDistance, data.maxDistance],
      denivele: [data.minDenivele, data.maxDenivele],
    };
  } catch (error) {
    console.error("Failed to fetch filter data:", error);
  }
}

function handleGpxImported() {
  refreshCircuits();
  loadFilterData();
}

function handleCircuitDeleted() {
  refreshCircuits();
  loadFilterData();
}

function handleCircuitUpdated(updatedCircuit) {
  const index = allCircuits.value.findIndex(c => c.circuitId === updatedCircuit.circuitId);
  if (index !== -1) {
    allCircuits.value[index] = updatedCircuit;
  }
  loadFilterData(); // Refresh filter data as traceurs might have changed
}

onMounted(async () => {
  await useSettings().initSettings(); // Ensure settings are loaded
  await refreshCircuits();
  await loadFilterData();

  listen('ask_pairing_approval', () => {
    if (showRemoteDialog.value) {
      console.log("Pairing request received, closing QR code dialog.");
      showRemoteDialog.value = false;
    }
  });
});
</script>

<style scoped>
.v-list :deep(.v-list-item:nth-child(even)) {
    background-color: rgba(var(--v-theme-on-surface), 0.04);
}

.v-list {
    border-top: 1px solid rgba(var(--v-theme-on-surface), 0.12);
}
</style>