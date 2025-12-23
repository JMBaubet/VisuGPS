<template>
  <v-container fluid class="fill-height d-flex flex-column pa-0">
    <AppMainBar 
      @open-import-dialog="openImport" 
      @circuit-imported="handleImported"
    />

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

    <ImportDialog
      v-model="showImportDialog"
      v-model:type="importConfig.type"
      :extensions="importConfig.extensions"
      :title="importConfig.title"
      show-type-switch
      @select="handleImportSelection"
    />
    <TraceurSelectionDialog ref="traceurDialog" />
    <OrphanCleanupDialog 
      v-model="showOrphanDialog" 
      :orphans="currentOrphans" 
      :related-ids="currentRelatedIds"
      @cleaned="handleImported"
    />
  </v-container>
</template>

<script setup>
import { ref, onMounted, computed, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import AppMainBar from '../components/AppMainBar.vue';
import ImportDialog from '../components/ImportDialog.vue';
import TraceurSelectionDialog from '../components/TraceurSelectionDialog.vue';
import CircuitListItem from '@/components/CircuitListItem.vue';
import CircuitFilter from '@/components/CircuitFilter.vue';
import OrphanCleanupDialog from '@/components/OrphanCleanupDialog.vue';
import { useSettings } from '@/composables/useSettings';
import { showRemoteDialog } from '@/composables/useRemoteControlDialog';
import { useSnackbar } from '@/composables/useSnackbar';


const { showSnackbar } = useSnackbar();

const showImportDialog = ref(false);
const importConfig = reactive({
  extensions: ['gpx'],
  title: 'Importer un fichier GPX',
  type: 'gpx'
});
const traceurDialog = ref(null);
const allCircuits = ref([]);
const allCommunes = ref([]);
const allTraceurs = ref([]);
const filterData = ref(null);

const showOrphanDialog = ref(false);
const currentOrphans = ref({ villes: [], traceurs: [], messages: [] });
const currentRelatedIds = ref({ ville: null, traceur: null, messages: [] });

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

function openImport() {
  importConfig.extensions = ['gpx']; // Default, but ImportDialog will handle it with the switch
  importConfig.title = 'Importer un fichier';
  importConfig.type = 'gpx';
  showImportDialog.value = true;
}

// Function handling the full GPX import flow
async function handleImportSelection(filename) {
    try {
        if (importConfig.type === 'gpx') {
             // GPX Import Flow
            // Phase 1: Analysis
            const draftCircuit = await invoke('analyze_gpx_file', { filename });

            // Phase 2: Traceur Selection
            if (!traceurDialog.value) return;
            const traceurId = await traceurDialog.value.open();
            
            // Phase 3: Commit
            const circuitId = await invoke('commit_new_circuit', { 
                draft: draftCircuit, 
                traceurId: traceurId 
            });

            // Phase 4: Thumbnail
            const settings = await invoke('read_settings');
            const lineStringPath = `data/${circuitId}/lineString.json`;
            
            await invoke('generate_gpx_thumbnail', {
                circuitId: circuitId,
                lineStringPath: lineStringPath,
                settings: settings
            });

            showSnackbar(`Circuit '${draftCircuit.nom}' importé avec succès.`, 'success');

        } else if (importConfig.type === 'vgps') {
             // VGPS Import Flow
             const message = await invoke('import_circuit', { filePath: filename });
             showSnackbar(message, 'success');
        }

        handleImported();

    } catch (e) {
        if (typeof e === 'string' && e.includes('annulée')) {
            console.log("Import cancelled");
        } else {
             console.error("Import error:", e);
             showSnackbar(`Erreur lors de l'import: ${e}`, 'error');
        }
    }
}

function handleImported() {
  refreshCircuits();
  loadFilterData();
}

async function handleCircuitDeleted(relatedIds) {
  refreshCircuits();
  loadFilterData();

  if (relatedIds) {
    try {
      const orphans = await invoke('get_orphans');
      if (orphans.villes.length > 0 || orphans.traceurs.length > 0 || orphans.messages.length > 0) {
        currentOrphans.value = orphans;
        currentRelatedIds.value = relatedIds;
        showOrphanDialog.value = true;
      }
    } catch (error) {
      console.error('Erreur lors de la détection des orphelins:', error);
    }
  }
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