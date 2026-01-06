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
            <v-row dense align="center">
                <v-col cols="3">
                    <EditTime v-model="editedHeureDepart" label="Heure Départ" />
                </v-col>
                <v-col cols="3">
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
                <v-col cols="3">
                    <EditDate v-model="editedDateDepart" label="Date Départ" />
                </v-col>
                <v-col cols="3" class="d-flex justify-center">
                     <v-btn
                      color="primary"
                      size="small"
                      @click="saveMeteo"
                      :disabled="!hasMeteoChanges"
                    >Sauvegarder</v-btn>
                </v-col>
            </v-row>
            <v-row dense class="mt-1 mb-2">
                <v-col cols="12" class="d-flex justify-space-between align-center px-4">
                    <div class="text-body-2" :class="{'text-success': weatherStatus.includes('Présent'), 'text-grey': !weatherStatus.includes('Présent')}">
                        <v-icon size="small" class="mr-1" :color="weatherStatus.includes('Présent') ? 'success' : 'grey'">mdi-weather-cloudy</v-icon>
                        Fichier Météo : <strong>{{ weatherStatus }}</strong>
                    </div>
                    <v-btn
                        size="small"
                        :color="weatherStatus.includes('Présent') ? 'warning' : 'primary'"
                        variant="tonal"
                        :loading="isDownloadingWeather"
                        @click="downloadWeather"
                        prepend-icon="mdi-download"
                    >
                        {{ weatherStatus.includes('Présent') ? 'Mettre à jour' : 'Télécharger' }}
                    </v-btn>
                </v-col>
            </v-row>

            <v-divider class="my-3"></v-divider>
            <div class="d-flex justify-space-between align-center mb-2 px-2">
                <div class="text-subtitle-2">Groupes / Scénarios Multi-Météo</div>
                <v-btn size="x-small" color="primary" variant="flat" @click="addScenario" prepend-icon="mdi-plus">
                    Ajouter un groupe
                </v-btn>
            </div>
            
            <div v-if="editedScenarios.length > 0" class="px-2">
                <v-row v-for="(scen, idx) in editedScenarios" :key="idx" dense align="center" class="mb-1 pa-1 rounded border">
                    <v-col cols="4">
                        <v-text-field v-model="scen.nom" label="Nom" density="compact" hide-details variant="underlined" />
                    </v-col>
                    <v-col cols="3">
                        <EditTime v-model="scen.heureDepart" label="Départ" />
                    </v-col>
                    <v-col cols="3">
                        <v-text-field v-model.number="scen.vitesseMoyenne" label="Km/h" type="number" density="compact" hide-details variant="underlined" />
                    </v-col>
                    <v-col cols="2" class="d-flex justify-end">
                        <v-btn icon size="x-small" color="error" variant="text" @click="removeScenario(idx)">
                            <v-icon>mdi-delete</v-icon>
                        </v-btn>
                    </v-col>
                </v-row>
            </div>
            <div v-else class="text-caption text-grey text-center py-2">
                Aucun scénario spécifique configuré.
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
import { ref, computed, onMounted, watch, nextTick } from 'vue';
import { useEnvironment } from '@/composables/useEnvironment';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import { useSettings } from '@/composables/useSettings';
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
const { getSettingValue } = useSettings();

const communeNom = ref('');
const qrCodePath = ref('');
const editedTraceur = ref(props.circuit.traceur);

// Weather Editing
const editedHeureDepart = ref("08:30");
const editedVitesseMoyenne = ref(20.0);
const editedDateDepart = ref("");
const editedScenarios = ref([]);

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
    
    // Date Logic
    let dateStr = config.dateDepart;
    let computedDate = null;
    
    // Calculate Today (Local)
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    // Check if stored date is valid and in the future
    if (dateStr) {
        const [y, m, d] = dateStr.split('-').map(Number);
        const storedDate = new Date(y, m - 1, d);
        if (storedDate > today) {
            computedDate = dateStr;
        }
    }

    // Default to Tomorrow if no valid future date
    if (!computedDate) {
        const tomorrow = new Date(today);
        tomorrow.setDate(tomorrow.getDate() + 1);
        const yyyy = tomorrow.getFullYear();
        const mm = String(tomorrow.getMonth() + 1).padStart(2, '0');
        const dd = String(tomorrow.getDate()).padStart(2, '0');
        computedDate = `${yyyy}-${mm}-${dd}`;
    }
    editedDateDepart.value = computedDate;
    
    // Scenarios
    editedScenarios.value = config.scenarios ? JSON.parse(JSON.stringify(config.scenarios)) : [];
    
    nextTick(() => checkWeatherStatus());
};

const addScenario = () => {
    editedScenarios.value.push({
        nom: `Groupe ${editedScenarios.value.length + 1}`,
        heureDepart: editedHeureDepart.value,
        vitesseMoyenne: editedVitesseMoyenne.value
    });
};

const removeScenario = (idx) => {
    editedScenarios.value.splice(idx, 1);
};

const hasMeteoChanges = computed(() => {
    const config = props.circuit.meteoConfig || {};
    const oldHeure = config.heureDepart || "08:30";
    const oldVitesse = config.vitesseMoyenne || 20.0;
    const oldDate = config.dateDepart || ((d) => { 
        d.setDate(d.getDate()+1); 
        const yyyy = d.getFullYear();
        const mm = String(d.getMonth() + 1).padStart(2, '0');
        const dd = String(d.getDate()).padStart(2, '0');
        return `${yyyy}-${mm}-${dd}`;
    })(new Date());

    // Simple comparison
    return editedHeureDepart.value !== oldHeure ||
           editedVitesseMoyenne.value !== oldVitesse ||
           editedDateDepart.value !== (config.date_depart || config.dateDepart || "") ||
           JSON.stringify(editedScenarios.value) !== JSON.stringify(config.scenarios || []); 
});

import WeatherService from '@/services/WeatherService';

const weatherStatus = ref('Inconnu');
const isDownloadingWeather = ref(false);

const getFilenameForDate = (dateStr) => {
    const d = new Date(dateStr);
    const yyyy = d.getFullYear();
    const mm = String(d.getMonth() + 1).padStart(2, '0');
    const dd = String(d.getDate()).padStart(2, '0');
    const datePart = `${yyyy}${mm}${dd}`;

    const startH = getSettingValue('Visualisation/Météo/heureDebutJournee') || 6;
    const endH = getSettingValue('Visualisation/Météo/heureFinJournee') || 20;
    
    const sH = String(startH).padStart(2, '0');
    const eH = String(endH).padStart(2, '0');

    return `${datePart}-${sH}-to-${eH}.json`;
};

const formatRelativeTime = (isoDate) => {
    const d = new Date(isoDate);
    const now = new Date();
    const diffMs = now - d;
    
    if (diffMs < 0) return "à l'instant"; 

    const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
    const days = Math.floor(diffHours / 24);
    const hours = diffHours % 24;

    if (days > 0) {
        return `il y a ${days}j et ${hours}h`;
    } else if (hours > 0) {
        return `il y a ${hours}h`;
    } else {
        return `il y a moins d'1h`;
    }
};

const checkWeatherStatus = async () => {
    if (!props.circuit?.circuitId || !editedDateDepart.value) return;
    
    const filename = getFilenameForDate(editedDateDepart.value);
    try {
        const metadata = await invoke('check_weather_cache_metadata', { 
            circuitId: props.circuit.circuitId, 
            filename 
        });
        
        if (metadata) {
            weatherStatus.value = `Présent (MAJ ${formatRelativeTime(metadata)})`;
        } else {
            weatherStatus.value = 'Non téléchargé';
        }
    } catch (e) {
        console.warn("Check weather failed", e);
        weatherStatus.value = 'Erreur vérification';
    }
};

const downloadWeather = async () => {
    if (!props.circuit?.circuitId || !editedDateDepart.value) return;
    isDownloadingWeather.value = true;
    
    try {
        // 1. Load tracking points
        let trackData = await invoke('read_tracking_file', { circuitId: props.circuit.circuitId });
        // trackData is the array of points
        if (!trackData || trackData.length === 0) {
            throw new Error("Aucun point de tracking trouvé");
        }
        
        // 2. Sample (1km resolution)
        const segmentLengthValue = Number(getSettingValue('Importation/Tracking/LongueurSegment')) || 100;
        const sampled = [];
        trackData.forEach((p, i) => {
            if (i % 10 === 0 || i === trackData.length - 1) {
                const inc = p.increment !== undefined ? p.increment : Math.round(i / 10);
                // Check if coordonnee exists
                if (p.coordonnee) {
                    sampled.push({
                        lat: p.coordonnee[1],
                        lon: p.coordonnee[0],
                        increment: inc,
                        km: p.distance || (inc * segmentLengthValue) / 1000
                    });
                }
            }
        });

        if (sampled.length === 0) throw new Error("Échantillonnage vide");

        // 3. Fetch Matrix
        const startH = getSettingValue('Visualisation/Météo/heureDebutJournee') || 6;
        const endH = getSettingValue('Visualisation/Météo/heureFinJournee') || 20;
        
        const matrix = await WeatherService.fetchWeatherMatrix(sampled, editedDateDepart.value, startH, endH);
        
        if (matrix && matrix.length > 0) {
            // 4. Save
            const filename = getFilenameForDate(editedDateDepart.value);
            await invoke('save_weather_cache', {
                circuitId: props.circuit.circuitId,
                filename,
                content: JSON.stringify(matrix, null, 2)
            });
            showSnackbar('Météo téléchargée avec succès', 'success');
            checkWeatherStatus();
        } else {
            throw new Error("Aucune donnée reçue de l'API");
        }

    } catch (e) {
        console.error("Download weather failed", e);
        showSnackbar("Erreur téléchargement météo: " + e.message, 'error');
    } finally {
        isDownloadingWeather.value = false;
    }
};

const saveMeteo = async () => {
    try {
        await invoke('update_circuit_meteo', {
            circuitId: props.circuit.circuitId,
            heureDepart: editedHeureDepart.value,
            vitesseMoyenne: Number(editedVitesseMoyenne.value),
            dateDepart: editedDateDepart.value,
            scenarios: editedScenarios.value.length > 0 ? editedScenarios.value : null
        });
        
        // Update local object via event to parent
        const newMeteoConfig = {
            heureDepart: editedHeureDepart.value,
            vitesseMoyenne: Number(editedVitesseMoyenne.value),
            dateDepart: editedDateDepart.value,
            scenarios: editedScenarios.value.length > 0 ? editedScenarios.value : null
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

watch(editedDateDepart, () => {
  checkWeatherStatus();
});

onMounted(() => {
  getCommuneNom();
  getQrCodePath();
  getMeteoConfig();
  // checkWeatherStatus called by watcher or getMeteoConfig implicitly?
  // Watcher might not fire on initial set if equal? 
  // Let's call it explicitly after a tick or in getMeteoConfig.
  nextTick(() => checkWeatherStatus());
});

watch(() => props.circuit, () => {
  getCommuneNom();
  getQrCodePath();
  editedTraceur.value = props.circuit.traceur;
  getMeteoConfig();
  nextTick(() => checkWeatherStatus());
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
