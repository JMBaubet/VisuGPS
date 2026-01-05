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

            <!-- Weather Configuration -->
            <v-divider class="my-3"></v-divider>
            <div class="text-subtitle-2 mb-2">Paramètres Météo</div>
            <v-row dense>
                <v-col cols="4">
                    <EditTime v-model="editedHeureDepart" label="Heure Départ" />
                </v-col>
                <v-col cols="4">
                    <v-text-field
                        v-model.number="editedVitesseMoyenne"
                        label="Vitesse Moy. (km/h)"
                        type="number"
                        min="5"
                        max="50"
                        step="0.5"
                        variant="underlined"
                        density="compact"
                        prepend-icon="mdi-speedometer"
                        hide-details
                    ></v-text-field>
                </v-col>
                <v-col cols="4">
                    <EditDate v-model="editedDateDepart" label="Date Départ" />
                </v-col>
            </v-row>
            <div class="d-flex justify-end mt-2">
                 <v-btn
                  color="primary"
                  size="small"
                  @click="saveMeteo"
                  :disabled="!hasMeteoChanges"
                >Sauvegarder Météo</v-btn>
            </div>
            <v-divider class="my-3"></v-divider>

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
import EditTime from './EditTime.vue';
import EditDate from './EditDate.vue';

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

// Weather Editing
const editedHeureDepart = ref("08:30");
const editedVitesseMoyenne = ref(20.0);
const editedDateDepart = ref("");

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
  
  // If traceurId is undefined, backend will handle creation

  try {
    const finalId = await invoke('update_circuit_traceur', {
      circuitId: props.circuit.circuitId,
      newTraceur: newTraceurName,
      newTraceurId: traceurId || null,
    });
    
    emit('update-circuit', { ...props.circuit, traceur: newTraceurName, traceurId: finalId });
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

// Meteo Logic
const getMeteoConfig = () => {
    const config = props.circuit.meteoConfig || {};
    editedHeureDepart.value = config.heureDepart || "08:30";
    editedVitesseMoyenne.value = config.vitesseMoyenne || 20.0;
    
    // Default date is tomorrow if not set
    if (config.dateDepart) {
        editedDateDepart.value = config.dateDepart;
    } else {
        const tomorrow = new Date();
        tomorrow.setDate(tomorrow.getDate() + 1);
        editedDateDepart.value = tomorrow.toISOString().split('T')[0];
    }
};

const hasMeteoChanges = computed(() => {
    const config = props.circuit.meteoConfig || {};
    const oldHeure = config.heureDepart || "08:30";
    const oldVitesse = config.vitesseMoyenne || 20.0;
    const oldDate = config.dateDepart || ((d) => { d.setDate(d.getDate()+1); return d.toISOString().split('T')[0];})(new Date()); // approx check

    // Simple comparison
    return editedHeureDepart.value !== oldHeure ||
           editedVitesseMoyenne.value !== oldVitesse ||
           editedDateDepart.value !== (config.dateDepart || ""); 
           // Note on date: if config.dateDepart is undefined, edited is tomorrow, so it might be different. 
           // If we want to detect if values *changed* from *saved* values:
           // If saved is None, and edited is 'Tomorrow', is that a change? Yes if we want to save it explicitly.
});

const saveMeteo = async () => {
    try {
        await invoke('update_circuit_meteo', {
            circuitId: props.circuit.circuitId,
            heureDepart: editedHeureDepart.value,
            vitesseMoyenne: Number(editedVitesseMoyenne.value),
            dateDepart: editedDateDepart.value
        });
        
        // Update local object via event to parent
        const newMeteoConfig = {
            heureDepart: editedHeureDepart.value,
            vitesseMoyenne: Number(editedVitesseMoyenne.value),
            dateDepart: editedDateDepart.value
        };
        
        emit('update-circuit', { 
            ...props.circuit, 
            meteoConfig: newMeteoConfig
        });
        
        showSnackbar('Paramètres météo mis à jour', 'success');
    } catch (e) {
        showSnackbar('Erreur sauvegarde météo: ' + e, 'error');
    }
};

onMounted(() => {
  getCommuneNom();
  getQrCodePath();
  getMeteoConfig();
});

watch(() => props.circuit, () => {
  getCommuneNom();
  getQrCodePath();
  editedTraceur.value = props.circuit.traceur;
  getMeteoConfig();
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
