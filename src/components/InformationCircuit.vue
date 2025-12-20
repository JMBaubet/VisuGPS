<template>
  <v-card>
    <v-card-title class="headline d-flex justify-space-between align-center">
      <div class="d-flex flex-column">
        <span>{{ circuit.nom }}</span>  <!-- circuit.nom as first element -->
        <a v-if="circuit.url" :href="circuit.url" target="_blank" class="circuit-url">{{ circuit.url }}</a>
      </div>
      <div class="d-flex align-center"> <!-- Group the alert and close buttons -->
        <v-btn icon
          v-if="props.circuit.hasErrors"
          color="error"
          class="mr-2"
          @click.stop="showErrorsModal"
          size="default"
        >
          <v-icon>mdi-alert-circle</v-icon>
        </v-btn>
        <v-btn icon @click="exportCircuit" color="info" class="mr-2" title="Exporter le circuit">
          <v-icon>mdi-export</v-icon>
        </v-btn>
        <v-btn icon @click="closeDialog">
          <v-icon>mdi-close</v-icon>
        </v-btn>
      </div>
    </v-card-title>
    <v-card-text>
      <v-row>
        <v-col cols="12">
          <v-list density="compact">
            <v-row no-gutters>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Distance :</strong> {{ circuit.distanceKm.toFixed(2) }} km
                  </v-list-item-title>
                </v-list-item>
              </v-col>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Ville de départ :</strong> {{ communeNom }}
                  </v-list-item-title>
                </v-list-item>
              </v-col>
            </v-row>
            <v-row no-gutters>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Dénivelé positif :</strong> {{ circuit.deniveleM }} m
                  </v-list-item-title>
                </v-list-item>
              </v-col>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Point le plus haut :</strong> {{ circuit.sommet.altitudeM }} m (à {{ circuit.sommet.km.toFixed(2) }} km)
                  </v-list-item-title>
                </v-list-item>
              </v-col>
            </v-row>
            <v-row no-gutters>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Édition caméra :</strong> <span :style="{ color: trackingProgressColor }">{{ trackingProgress.toFixed(2) }} % ({{ circuit.trackingKm.toFixed(2) }} km)</span>
                  </v-list-item-title>
                </v-list-item>
              </v-col>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Recherche Communes :</strong> <span :style="{ color: communeProgressColor }">{{ communeProgress.toFixed(2) }} %</span>
                  </v-list-item-title>
                </v-list-item>
              </v-col>
            </v-row>
            </v-list>
            <div class="d-flex flex-row align-start " style="width: 100%; max-width: 500px;">
              <span class="text-subtitle-1 mr-2 align-self-center" style="white-space: nowrap; margin-bottom: 20px; padding-left: 16px;">Traceur : </span>
              <v-combobox
                v-model="editedTraceur"
                :items="traceurNames"
                variant="outlined"
                density="compact"
                class="w-100 mr-2"
              ></v-combobox>
              <div class="d-flex align-center">
                <v-btn
                  color="primary"
                  :disabled="editedTraceur === props.circuit.traceur"
                  @click="saveTraceur"
                  style="margin-top: 2px;"
                >Sauvegarder</v-btn>
              </div>
            </div>
            <v-row class="mt-4">
              <v-col cols="12" md="6" class="d-flex justify-center">
                <v-img
                  v-if="qrCodePath"
                  :src="qrCodePath"
                  alt="QR Code du circuit"
                  contain
                  max-height="200px"
                  max-width="200px"
                ></v-img>
                <span v-else>QR Code non disponible</span>
              </v-col>
              <v-col cols="12" md="6">
                <v-list density="compact">
                  <v-list-item>
                    <v-list-item-title>
                      <strong>Éditeur :</strong> {{ circuit.editeur }}
                    </v-list-item-title>
                  </v-list-item>
                </v-list>
                <v-btn 
                  color="primary" 
                  variant="tonal" 
                  @click="showDistanceMarkersDialog = true"
                  class="mt-2"
                  block
                >
                  <v-icon class="mr-2">mdi-map-marker-distance</v-icon>
                  Configurer les distances
                </v-btn>
              </v-col>
            </v-row>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>

  <v-dialog v-model="showErrorsDialog" max-width="800">
    <v-card>
      <v-card-title class="headline d-flex justify-space-between align-center">
        Erreurs pour le circuit : {{ circuit.nom }}
        <v-btn icon @click="showErrorsDialog = false">
          <v-icon>mdi-close</v-icon>
        </v-btn>
      </v-card-title>
      <v-card-text>
        <v-list v-if="circuitErrors.length > 0">
          <v-list-item v-for="(error, index) in circuitErrors" :key="index">
            <v-list-item-title>
              <strong>Type d'erreur :</strong> {{ error.errorType }}
            </v-list-item-title>
            <v-list-item-subtitle>
              <p><strong>Description :</strong> {{ error.description }}</p>
              <p v-if="error.messageId"><strong>ID du message :</strong> {{ error.messageId }}</p>
              <p v-if="error.anchorIncrement !== undefined"><strong>Incrément :</strong> {{ error.anchorIncrement }}</p>
              <p v-if="error.eventId"><strong>ID de l'événement :</strong> {{ error.eventId }}</p>
              <p v-if="error.timestamp"><strong>Timestamp :</strong> {{ new Date(error.timestamp).toLocaleString() }}</p>
            </v-list-item-subtitle>
          </v-list-item>
        </v-list>
        <v-alert v-else type="info">
          Aucune erreur détaillée trouvée pour ce circuit.
        </v-alert>
      </v-card-text>
    </v-card>
  </v-dialog>

  <DistanceMarkersDialog
    v-model="showDistanceMarkersDialog"
    :circuit-id="circuit.circuitId"
    :total-distance-km="circuit.distanceKm"
    @updated="handleDistanceMarkersUpdated"
  />
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue';
import { useEnvironment } from '@/composables/useEnvironment';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import DistanceMarkersDialog from './DistanceMarkersDialog.vue';

const props = defineProps({
  circuit: {
    type: Object,
    required: true,
  },
  allCommunes: {
    type: Array,
    default: () => [],
  },
  allTraceurs: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(['close', 'update-circuit']);

const { appEnvPath } = useEnvironment();
const { showSnackbar } = useSnackbar();

const communeNom = ref('');
const qrCodePath = ref('');
const editedTraceur = ref(props.circuit.traceur);

const showErrorsDialog = ref(false);
const circuitErrors = ref([]);
const showDistanceMarkersDialog = ref(false);

const showErrorsModal = async () => {
  try {
    circuitErrors.value = await invoke('read_errors_file', { circuitId: props.circuit.circuitId });
    showErrorsDialog.value = true;
  } catch (error) {
    showSnackbar(`Erreur lors du chargement des erreurs : ${error}`, 'error');
    console.error('Error loading errors:', error);
  }
};

const trackingProgress = computed(() => {
  if (props.circuit.distanceKm === 0) return 0;
  return (props.circuit.trackingKm / props.circuit.distanceKm) * 100;
});

const trackingProgressColor = computed(() => {
  const progress = trackingProgress.value;
  if (progress === 0) return 'red';
  if (progress === 100) return 'green';
  return 'orange';
});

const communeProgress = computed(() => {
  return props.circuit.avancementCommunes;
});

const communeProgressColor = computed(() => {
  const progress = communeProgress.value;
  if (progress === 0) return 'purple';
  if (progress < 7) return 'red';
  if (progress < 13) return 'orange';
  if (progress < 25) return 'yellow';
  if (progress < 50) return 'blue';
  if (progress < 100) return 'teal';
  return 'green';
});

const traceurNames = computed(() => {
  return props.allTraceurs.map(t => t.nom);
});

const getCommuneNom = () => {
  const commune = props.allCommunes.find(c => c.id === props.circuit.villeDepartId);
  communeNom.value = commune ? commune.nom : 'Inconnu';
};

const exportCircuit = async () => {
  try {
    const message = await invoke('export_circuit', { circuitId: props.circuit.circuitId });
    showSnackbar(message, 'success');
  } catch (error) {
    console.error('Error exporting circuit:', error);
    showSnackbar(`Erreur lors de l'exportation : ${error}`, 'error');
  }
};

const getQrCodePath = async () => {
  if (props.circuit.circuitId) {
    try {
      qrCodePath.value = await invoke('get_qrcode_as_base64', { circuitId: props.circuit.circuitId });
    } catch (error) {
      console.error('Failed to load QR Code:', error);
      qrCodePath.value = '';
    }
  }
};

const saveTraceur = async () => {
  const newTraceurName = editedTraceur.value;
  if (newTraceurName === props.circuit.traceur) return;

  let traceurId = props.allTraceurs.find(t => t.nom === newTraceurName)?.id;

  if (!traceurId) {
    // New traceur, create a new ID
    const newId = `tr-${(props.allTraceurs.length + 1).toString().padStart(4, '0')}`;
    traceurId = newId;
    showSnackbar('Nouveau traceur ajouté : ' + newTraceurName, 'info');
  }

  try {
    await invoke('update_circuit_traceur', {
      circuitId: props.circuit.circuitId,
      newTraceur: newTraceurName,
      newTraceurId: traceurId,
    });
    emit('update-circuit', { ...props.circuit, traceur: newTraceurName, traceurId: traceurId });
    showSnackbar('Traceur mis à jour avec succès !', 'success');
  } catch (error) {
    showSnackbar('Erreur lors de la mise à jour du traceur : ' + error, 'error');
    editedTraceur.value = props.circuit.traceur; // Revert on error
  }
};

const handleDistanceMarkersUpdated = () => {
  // Optionally refresh circuit data or notify parent
  showSnackbar('Les bornes kilométriques ont été mises à jour', 'success');
};

const closeDialog = () => {
  emit('close');
};

onMounted(() => {
  getCommuneNom();
  getQrCodePath();
});

watch(() => props.circuit, () => {
  getCommuneNom();
  getQrCodePath();
  editedTraceur.value = props.circuit.traceur;
}, { deep: true });

watch(appEnvPath, () => {
  getQrCodePath();
});
</script>

<style scoped>
.v-card-title {
  background-color: rgb(var(--v-theme-primary));
  color: white;
}

.circuit-url {
  font-size: 0.7rem;
  color: rgba(255, 255, 255, 0.8);
  text-decoration: none;
  margin-top: 2px;
}

.circuit-url:hover {
  color: rgba(255, 255, 255, 1);
  text-decoration: underline;
}

.blinking-button {
  animation: blink 1s linear infinite;
}

@keyframes blink {
  0% { opacity: 0; }
  50% { opacity: 1; }
  100% { opacity: 0; }
}
</style>
