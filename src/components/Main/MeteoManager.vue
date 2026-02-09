<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="800" persistent>
    <v-card>
      <v-card-title class="bg-primary text-white px-4 py-2 d-flex justify-space-between align-center">
        <div class="d-flex align-center">
           <v-icon start icon="mdi-sun-thermometer"></v-icon>
           <span>Gestion Météo : {{ circuit.nom }}</span>
        </div>
        <div class="d-flex align-center">
             <v-btn icon @click="openDoc('/docs/DocUtilisateur/meteo_manager.md')" color="white" variant="text" class="mr-2" title="Documentation">
                <v-icon>mdi-book-open-page-variant-outline</v-icon>
            </v-btn>
            <v-btn icon @click="closeDialog" color="white" variant="text">
                <v-icon>mdi-close</v-icon>
            </v-btn>
        </div>
      </v-card-title>

      <v-card-text>
        <!-- Global Weather Configuration -->
        <v-card variant="tonal" color="blue-grey" class="mb-4 pa-2">
            <!-- Row 1: Date -->
            <v-row dense align="center" class="mb-2">
                <v-col cols="12">
                    <v-select
                        v-model="editedDateDepart"
                        :items="availableDateOptions"
                        item-title="title"
                        item-value="value"
                        label="Date de départ"
                        density="compact"
                        variant="underlined"
                        hide-details
                        prepend-icon="mdi-calendar"
                    ></v-select>
                </v-col>
            </v-row>

            <!-- Row 2: File Management -->
            <v-row dense align="center">
                <v-col cols="12">
                     <!-- Status & Action when file exists -->
                     <div v-if="weatherFilePresent" class="d-flex align-center justify-end">
                        <div 
                            class="text-caption mr-2 text-right font-weight-bold" 
                            :class="weatherFileAgeHours < 3 ? 'text-green' : 'text-blue'"
                        >
                            Le fichier météo pour le {{ formattedDateLong }} a été mis à jour {{ weatherFileRelativeTime }}.
                        </div>
                        <v-btn
                            size="small"
                            color="info"
                            variant="flat"
                            class="mr-2"
                            @click="loadAndShowWeather"
                            prepend-icon="mdi-eye"
                            :disabled="!isValid"
                        >
                            Voir
                        </v-btn>
                        <v-btn
                            size="small"
                            color="warning"
                            variant="flat"
                            :loading="isDownloadingWeather"
                            @click="downloadWeather"
                            prepend-icon="mdi-update"
                        >
                            Mettre à jour
                        </v-btn>
                     </div>

                     <!-- Status & Action when missing -->
                     <div v-else class="d-flex align-center justify-end">
                        <div class="text-caption text-red mr-2 text-right font-weight-bold">
                           Aucun fichier météo pour le {{ formattedDateLong }}.
                        </div>
                        <v-btn
                            size="small"
                            color="primary"
                            variant="flat"
                            :loading="isDownloadingWeather"
                            @click="downloadWeather"
                            prepend-icon="mdi-download"
                            :disabled="!isValid"
                        >
                            Télécharger
                        </v-btn>
                     </div>
                </v-col>
            </v-row>
        </v-card>

        <v-divider class="mb-4"></v-divider>

        <!-- Groups Management -->
        <div class="d-flex justify-space-between align-center mb-2">
            <div class="text-subtitle-1 font-weight-bold">Groupes</div>
            <v-btn size="small" color="primary" variant="tonal" prepend-icon="mdi-plus" @click="addGroup">
                Ajouter Groupe
            </v-btn>
        </div>

        <div v-if="editedScenarios.length > 0" class="scenarios-list">
            <!-- Sorting Header -->
            <v-row dense class="px-2 mb-1 text-grey-darken-1">
                <v-col cols="1" class="d-flex justify-center"></v-col>
                <v-col cols="2" class="d-flex align-center">
                    <span class="text-caption font-weight-bold cursor-pointer hover-text-primary" @click="sortScenarios('nom')">
                        NOM <v-icon size="x-small">{{ getSortIcon('nom') }}</v-icon>
                    </span>
                </v-col>
                <v-col cols="4" class="d-flex align-center">
                    <span class="text-caption font-weight-bold cursor-pointer hover-text-primary" @click="sortScenarios('heure')">
                        DÉPART <v-icon size="x-small">{{ getSortIcon('heure') }}</v-icon>
                    </span>
                </v-col>
                <v-col cols="4"></v-col>
                <v-col cols="1"></v-col>
            </v-row>

            <v-row v-for="(scen, idx) in editedScenarios" :key="scen.id" dense align="center" class="mb-1 pa-2 rounded">
                <!-- Reference Selection -->
                <v-col cols="1" class="d-flex justify-center">
                    <v-btn icon size="x-small" variant="text" @click="setReference(idx)" :color="scen.isReference ? 'primary' : 'grey'" title="Définir comme groupe de référence">
                        <v-icon>{{ scen.isReference ? 'mdi-radiobox-marked' : 'mdi-radiobox-blank' }}</v-icon>
                    </v-btn>
                </v-col>

                <!-- Group Name (Read-only or strict) -->
                <v-col cols="2">
                    <div class="font-weight-bold ml-2">{{ scen.nom }}</div>
                </v-col>
                
                <!-- Departure Time -->
                <v-col cols="4">
                   <EditTime v-model="scen.heureDepart" label="Heure Départ" step="300" />
                </v-col>
                
                <!-- Average Speed -->
                <v-col cols="4">
                    <v-text-field
                        v-model.number="scen.vitesseMoyenne"
                        label="Vitesse (km/h)"
                        type="number"
                        min="5"
                        max="50"
                        step="0.5"
                        density="compact"
                        hide-details
                        variant="outlined"
                    ></v-text-field>
                </v-col>
                
                <!-- Delete Action (Only for the group with highest number) -->
                <v-col cols="1" class="d-flex justify-end">
                    <v-btn 
                        v-if="isHighestGroup(scen) && editedScenarios.length > 1"
                        icon 
                        size="x-small" 
                        color="error" 
                        variant="text" 
                        @click="removeGroup(idx)" 
                        title="Supprimer ce groupe (dernier numéro)"
                    >
                        <v-icon>mdi-delete</v-icon>
                    </v-btn>
                </v-col>
            </v-row>
        </div>
        <div v-else class="text-center py-4 text-grey">
            Aucun groupe défini. Le Groupe 1 sera créé par défaut.
        </div>
      </v-card-text>

      <v-card-actions class="pa-4 pt-0">
        <v-spacer></v-spacer>
        <v-btn variant="text" @click="closeDialog">Fermer</v-btn>
        <v-btn color="primary" variant="flat" @click="saveMeteo" :disabled="!hasChanges">Enregistrer</v-btn>
      </v-card-actions>
    </v-card>

  </v-dialog>

  <v-dialog v-model="showDocDialog" max-width="800px" height="80%">
      <DocDisplay :doc-path="currentDocPath" @close="showDocDialog = false" />
  </v-dialog>

  <Teleport to="body">
    <WeatherWidgetStatic 
        v-if="showWeatherWidget"
        :weather-matrix="weatherMatrix"
        :scenarios="editedScenarios"
        :date="weatherDate"
        @close="showWeatherWidget = false"
        style="z-index: 2500;"
    />
  </Teleport>
</template>

<script setup>
import { ref, computed, onMounted, watch, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import { useSettings } from '@/composables/useSettings';
import EditTime from '@/components/Settings/EditTime.vue';
import WeatherService from '@/services/WeatherService';
import DocDisplay from '@/components/DocDisplay.vue';
import WeatherWidgetStatic from '@/components/Visualize/WeatherWidgetStatic.vue';

const props = defineProps({
  modelValue: {
    type: Boolean,
    required: true
  },
  circuit: {
    type: Object,
    required: true
  }
});

const emit = defineEmits(['update:modelValue', 'saved', 'downloaded']);

const { showSnackbar } = useSnackbar();
const { getSettingValue } = useSettings();

// Local State
const editedDateDepart = ref("");
const editedScenarios = ref([]);

const weatherStatus = ref('Inconnu'); // Legacy, kept if needed, but we use split vars now
const weatherFilePresent = ref(false);
const weatherFileRelativeTime = ref("");
const weatherFileAgeHours = ref(0);

const isDownloadingWeather = ref(false);

const showDocDialog = ref(false);
const currentDocPath = ref('');
const sortKey = ref(null);
const sortAsc = ref(true);

const openDoc = (path) => {
  currentDocPath.value = path;
  showDocDialog.value = true;
};

const formattedDateLong = computed(() => {
    if (!editedDateDepart.value) return "";
    const [y, m, d] = editedDateDepart.value.split('-').map(Number);
    const date = new Date(y, m - 1, d);
    let str = new Intl.DateTimeFormat('fr-FR', { weekday: 'short', day: '2-digit', month: 'short', year: 'numeric' }).format(date);
    // Capitalize first letter (e.g., "mer." -> "Mer.")
    return str.charAt(0).toUpperCase() + str.slice(1);
});

const availableDateOptions = computed(() => {
    const options = [];
    const today = new Date();
    // Reset time to avoid issues
    today.setHours(0, 0, 0, 0);

    const formatter = new Intl.DateTimeFormat('fr-FR', { 
        weekday: 'short', 
        day: '2-digit', 
        month: 'short',
        year: 'numeric'
    });

    for (let i = 0; i < 16; i++) {
        const d = new Date(today);
        d.setDate(today.getDate() + i);
        
        // Use local time for value to match title (avoid UTC offset issues with toISOString)
        const y = d.getFullYear();
        const m = String(d.getMonth() + 1).padStart(2, '0');
        const day = String(d.getDate()).padStart(2, '0');
        const iso = `${y}-${m}-${day}`;
        
        // title example: "mer. 07 janv. 2026"
        // Capitalize first letter
        let title = formatter.format(d);
        title = title.charAt(0).toUpperCase() + title.slice(1);
        
        options.push({ title, value: iso });
    }
    return options;
});

// Initialize Data
const initData = () => {
    const config = props.circuit.meteoConfig || {};
    
    // Date Logic
    let dateStr = config.dateDepart;
    let computedDate = null;
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    if (dateStr) {
        const [y, m, d] = dateStr.split('-').map(Number);
        const storedDate = new Date(y, m - 1, d);
        if (storedDate >= today) { // Allow today
            computedDate = dateStr;
        }
    }

    if (!computedDate) {
        const tomorrow = new Date(today);
        tomorrow.setDate(tomorrow.getDate() + 1);
        const yyyy = tomorrow.getFullYear();
        const mm = String(tomorrow.getMonth() + 1).padStart(2, '0');
        const dd = String(tomorrow.getDate()).padStart(2, '0');
        computedDate = `${yyyy}-${mm}-${dd}`;
    }
    editedDateDepart.value = computedDate;

    // Scenarios Logic
    if (config.scenarios && Array.isArray(config.scenarios) && config.scenarios.length > 0) {
        // Deep copy and add unique IDs for Vue keys
        editedScenarios.value = JSON.parse(JSON.stringify(config.scenarios)).map((s, i) => ({
            ...s,
            id: s.id || `scen-${Date.now()}-${i}`
        }));
        
        // Ensure at least one reference exists (default to 1st if none)
        if (!editedScenarios.value.some(s => s.isReference)) {
            if (editedScenarios.value[0]) editedScenarios.value[0].isReference = true;
        }
    } else {
        // Default Group 1 logic
        createDefaultGroup();
    }
    
    // Check cache status
    nextTick(() => checkWeatherStatus());
};

const createDefaultGroup = () => {
    // Get defaults from settings
    const defaultTime = getSettingValue('Visualisation/Météo/heureDepart') || "08:30";
    const defaultSpeed = getSettingValue('Visualisation/Météo/vitesseMoyenne') || 20.0;
    
    editedScenarios.value = [{
        id: `scen-${Date.now()}-0`,
        nom: "Gr. 1",
        heureDepart: defaultTime,
        vitesseMoyenne: defaultSpeed,
        isReference: true
    }];
};

// Open/Close Watcher
watch(() => props.modelValue, (val) => {
    if (val) {
        initData();
    }
}, { immediate: true });

// Date Watcher
watch(editedDateDepart, () => {
    checkWeatherStatus();
    saveMeteo();
});

// Computed
const isValid = computed(() => {
    return editedScenarios.value.length > 0 && editedDateDepart.value;
});

const maxGroupNum = computed(() => {
    return Math.max(...editedScenarios.value.map(s => {
        const match = s.nom.match(/Gr\. (\d+)/);
        return match ? parseInt(match[1]) : 0;
    }));
});

const isHighestGroup = (scen) => {
    const match = scen.nom.match(/Gr\. (\d+)/);
    const num = match ? parseInt(match[1]) : 0;
    return num === maxGroupNum.value;
};

const hasChanges = computed(() => {
    const config = props.circuit.meteoConfig || {};
    const oldDate = config.dateDepart || "";
    const oldScenarios = JSON.stringify(config.scenarios || []);
    const newScenarios = JSON.stringify(editedScenarios.value);
    
    // Note: init logic might change date if invalid/past, so initially hasChanges might be true if data was stale
    return editedDateDepart.value !== oldDate || newScenarios !== oldScenarios;
});

// Actions
const addGroup = () => {
    const nextNum = maxGroupNum.value + 1;
    
    // Find the current "highest" group to inherit its values
    const lastGroup = editedScenarios.value.find(s => {
        const match = s.nom.match(/Gr\. (\d+)/);
        return match && parseInt(match[1]) === maxGroupNum.value;
    });

    const defaultTime = lastGroup?.heureDepart || getSettingValue('Visualisation/Météo/heureDepart') || "08:30";
    const defaultSpeed = lastGroup?.vitesseMoyenne || getSettingValue('Visualisation/Météo/vitesseMoyenne') || 20.0;
    
    editedScenarios.value.push({
        id: `scen-${Date.now()}-${nextNum}`,
        nom: `Gr. ${nextNum}`,
        heureDepart: defaultTime,
        vitesseMoyenne: defaultSpeed,
        isReference: false
    });
    
    saveMeteo();
};

const setReference = (idx) => {
    editedScenarios.value.forEach((s, i) => {
        s.isReference = (i === idx);
    });
};

const removeGroup = (idx) => {
    // We only allow removing the last group, so index doesn't strictly matter if we trust UI
    editedScenarios.value.splice(idx, 1);
    
    if (editedScenarios.value.length === 0) {
        createDefaultGroup();
    }
    
    saveMeteo();
};

const sortScenarios = (criteria) => {
    if (sortKey.value === criteria) {
        sortAsc.value = !sortAsc.value;
    } else {
        sortKey.value = criteria;
        sortAsc.value = true;
    }

    if (criteria === 'nom') {
        editedScenarios.value.sort((a, b) => {
            const res = a.nom.localeCompare(b.nom, undefined, { numeric: true });
            return sortAsc.value ? res : -res;
        });
    } else if (criteria === 'heure') {
        editedScenarios.value.sort((a, b) => {
            const res = a.heureDepart.localeCompare(b.heureDepart);
            return sortAsc.value ? res : -res;
        });
    }
    saveMeteo();
};

const getSortIcon = (criteria) => {
    if (sortKey.value !== criteria) return 'mdi-sort';
    return sortAsc.value ? 'mdi-sort-ascending' : 'mdi-sort-descending';
};

const closeDialog = () => {
    emit('update:modelValue', false);
};

const saveMeteo = async () => {
    if (editedScenarios.value.length === 0) {
        // Enforce at least one group on save
        createDefaultGroup();
    }

    try {
        // Prepare data for backend: remove the local 'id' field to match Rust struct
        const scenariosToSave = editedScenarios.value.map(({ id, ...rest }) => rest);

        await invoke('update_circuit_meteo', {
            circuitId: props.circuit.circuitId,
            heureDepart: scenariosToSave[0]?.heureDepart || "08:30",
            vitesseMoyenne: Number(scenariosToSave[0]?.vitesseMoyenne || 20.0),
            dateDepart: editedDateDepart.value,
            scenarios: scenariosToSave
        });
        
        showSnackbar('Configuration météo enregistrée', 'success');
        emit('saved');
        checkWeatherStatus(); // Re-check (maybe filename changed due to date)
    } catch (e) {
        console.error("Save error:", e);
        showSnackbar("Erreur sauvegarde: " + e, 'error');
    }
};

const downloadWeather = async () => {
    if (!props.circuit?.circuitId || !editedDateDepart.value) return;
    
    if (editedScenarios.value.length === 0) {
         showSnackbar("Veuillez définir au moins un groupe.", "warning");
         return;
    }
    
    isDownloadingWeather.value = true;
    
    try {
        // 1. We must save first if there are changes, otherwise download might use old config?
        // Actually weather download depends on Date (filename) and Track (lat/lon). 
        // It does NOT depend on Speed/Time for *fetching* the matrix (which covers the whole day).
        // It mainly needs the Date.
        // But for consistency we should ensure track data is loaded.
        
        let trackData = await invoke('read_tracking_file', { circuitId: props.circuit.circuitId });
        if (!trackData || trackData.length === 0) {
            throw new Error("Aucun point de tracking trouvé");
        }
        
        // 2. Sample (1km resolution - as decided)
        const segmentLengthValue = Number(getSettingValue('Importation/Tracking/LongueurSegment')) || 100;
        const sampled = [];
        trackData.forEach((p, i) => {
            if (i % 10 === 0 || i === trackData.length - 1) {
                const inc = p.increment !== undefined ? p.increment : Math.round(i / 10);
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
            // 4. Save Cache
            const filename = getFilenameForDate(editedDateDepart.value);
            await invoke('save_weather_cache', {
                circuitId: props.circuit.circuitId,
                filename,
                content: JSON.stringify(matrix, null, 2)
            });
            showSnackbar('Météo téléchargée avec succès', 'success');
            emit('downloaded');
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

// Weather Widget Display Logic
const showWeatherWidget = ref(false);
const weatherMatrix = ref([]);
const weatherDate = ref(null);

const loadAndShowWeather = async () => {
    if (!props.circuit?.circuitId || !editedDateDepart.value) return;
    
    const filename = getFilenameForDate(editedDateDepart.value);
    try {
        const cacheContent = await invoke('check_weather_cache', { 
            circuitId: props.circuit.circuitId, 
            filename 
        });
        
        if (cacheContent) {
            weatherMatrix.value = JSON.parse(cacheContent);
            const [y, m, d] = editedDateDepart.value.split('-').map(Number);
            weatherDate.value = new Date(y, m - 1, d);
            showWeatherWidget.value = true;
        } else {
            showSnackbar("Aucun fichier météo trouvé pour cette date.", "warning");
        }
    } catch (e) {
        console.error("Failed to load weather:", e);
        showSnackbar("Erreur lors du chargement de la météo.", "error");
    }
};

// Utils
const getFilenameForDate = (dateStr) => {
    // Éviter le décalage de fuseau horaire de new Date(dateStr) en extrayant les parties manuellement
    const [y, m, d] = dateStr.split('-').map(String);
    const datePart = `${y}${m.padStart(2, '0')}${d.padStart(2, '0')}`;

    const startH = getSettingValue('Visualisation/Météo/heureDebutJournee') || 6;
    const endH = getSettingValue('Visualisation/Météo/heureFinJournee') || 20;
    
    const sH = String(startH).padStart(2, '0');
    const eH = String(endH).padStart(2, '0');

    return `${datePart}-${sH}-to-${eH}.json`;
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
            weatherFilePresent.value = true;
            
            const d = new Date(metadata);
            const now = new Date();
            const diffMs = now - d;
            const diffHours = diffMs / (1000 * 60 * 60); // Float hours
            
            weatherFileAgeHours.value = diffHours;
            
            // Format relative time
            if (diffMs < 0) {
                 weatherFileRelativeTime.value = "à l'instant";
            } else {
                const days = Math.floor(diffHours / 24);
                const hours = Math.floor(diffHours % 24);
                if (days > 0) {
                    weatherFileRelativeTime.value = `il y a ${days}j et ${hours}h`;
                } else if (hours > 0) {
                    weatherFileRelativeTime.value = `il y a ${hours}h`;
                } else {
                    weatherFileRelativeTime.value = `il y a moins d'1h`;
                }
            }
            
            weatherStatus.value = `Présent (MAJ ${weatherFileRelativeTime.value})`;
        } else {
            weatherFilePresent.value = false;
            weatherFileAgeHours.value = 9999;
            weatherFileRelativeTime.value = "";
            weatherStatus.value = 'Non téléchargé';
        }
    } catch (e) {
        console.warn("Check weather failed", e);
        weatherStatus.value = 'Erreur vérification';
        weatherFilePresent.value = false;
        weatherFileAgeHours.value = 9999;
    }
};

</script>

<style scoped>
.scenarios-list {
  max-height: 350px;
  overflow-y: auto;
  overflow-x: hidden;
}
.hover-text-primary:hover {
  color: rgb(var(--v-theme-primary)) !important;
}
.cursor-pointer {
  cursor: pointer;
}
</style>
