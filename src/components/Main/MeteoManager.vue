<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="800" persistent>
    <v-card>
      <v-card-title class="bg-primary text-white px-4 py-2 d-flex justify-space-between align-center">
        <div class="d-flex align-center">
           <v-icon start icon="mdi-sun-thermometer"></v-icon>
           <span>Groupes et météo : {{ circuit.nom }}</span>
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

      <v-tabs v-model="activeTab" bg-color="grey-lighten-4" color="primary" density="compact">
        <v-tab value="groups" prepend-icon="mdi-account-group">Groupes</v-tab>
        <v-tab value="weather" prepend-icon="mdi-weather-partly-cloudy">Météo</v-tab>
      </v-tabs>

      <v-divider></v-divider>

      <v-window v-model="activeTab" style="min-height: 400px; max-height: 400px; overflow: hidden;">
        <!-- Tab 1: Groups -->
        <v-window-item value="groups">
          <v-card-text class="pa-0 d-flex flex-column" style="height: 400px;">
            <!-- Zone fixe en haut : Titre, bouton Ajouter et En-tête de tri -->
            <div class="px-4 pt-4 pb-2 pb-0">
                <div class="d-flex justify-space-between align-center mb-2">  
                    <div class="text-subtitle-1 font-weight-bold">Configuration des groupes</div>
                    <v-btn size="small" color="primary" variant="tonal" prepend-icon="mdi-plus" @click="addGroup">
                        Ajouter Groupe
                    </v-btn>
                </div>

                <!-- Sorting Header (Fixe) -->
                <v-row v-if="editedScenarios.length > 0" dense class="px-2 mb-1 text-grey-darken-1">
                    <v-col cols="1" class="d-flex justify-center"></v-col>
                    <v-col cols="1" class="d-flex justify-center"></v-col>
                    <v-col :cols="colsName" class="d-flex align-center">
                        <span class="text-caption font-weight-bold cursor-pointer hover-text-primary" @click="sortScenarios('nom')">
                            NOM <v-icon size="x-small">{{ getSortIcon('nom') }}</v-icon>
                        </span>
                    </v-col>
                    <v-col :cols="colsTime" class="d-flex align-center">
                        <span class="text-caption font-weight-bold cursor-pointer hover-text-primary" @click="sortScenarios('heure')">
                            DÉPART <v-icon size="x-small">{{ getSortIcon('heure') }}</v-icon>
                        </span>
                    </v-col>
                    <v-col :cols="colsSpeed" class="d-flex align-center">
                        <span class="text-caption font-weight-bold">VITESSE</span>
                    </v-col>
                    <v-col v-if="hasVariants" cols="5" class="d-flex align-center">
                        <span class="text-caption font-weight-bold">CIRCUIT</span>
                    </v-col>
                </v-row>
            </div>

            <!-- Liste Défilante des scénarios -->
            <div class="flex-grow-1 overflow-y-auto px-4 mt-1">
                <div v-if="editedScenarios.length > 0">
                    <v-row v-for="(scen, idx) in editedScenarios" :key="scen.id" dense align="center" class="mb-1 pa-2 rounded">
                        <!-- Reference Selection -->
                        <v-col cols="1" class="d-flex justify-center">
                            <v-btn icon size="x-small" variant="text" @click="setReference(idx)" :color="scen.isReference ? 'primary' : 'grey'" title="Définir comme groupe de référence">
                                <v-icon>{{ scen.isReference ? 'mdi-radiobox-marked' : 'mdi-radiobox-blank' }}</v-icon>
                            </v-btn>
                        </v-col>

                        <!-- Delete Action (Moved) -->
                        <v-col cols="1" class="d-flex justify-center">
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

                        <!-- Group Name -->
                        <v-col :cols="colsName">
                            <div class="font-weight-bold ml-2">{{ scen.nom }}</div>
                        </v-col>
                        
                        <!-- Departure Time -->
                        <v-col :cols="colsTime">
                          <EditTime v-model="scen.heureDepart" label="Heure Départ" step="300" />
                        </v-col>
                        
                        <!-- Average Speed -->
                        <v-col :cols="colsSpeed">
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

                        <!-- Variant Selection -->
                        <v-col v-if="hasVariants" cols="5" class="d-flex align-center">
                            <v-select
                                v-model="scen.variantId"
                                :items="availableVariants"
                                item-title="title"
                                item-value="value"
                                label="Circuit"
                                density="compact"
                                variant="outlined"
                                hide-details
                                :color="scen.variantId ? 'blue' : undefined"
                                :class="scen.variantId ? 'text-blue' : ''"
                            ></v-select>
                            <v-btn
                                v-if="scen.variantId"
                                icon="mdi-download"
                                size="x-small"
                                variant="text"
                                color="primary"
                                class="ml-1"
                                @click="exportVariantGpx(scen.variantId)"
                                title="Exporter ce variant au format GPX"
                            ></v-btn>
                        </v-col>
                    </v-row>
                </div>
                <div v-else class="text-center py-4 text-grey">
                    Aucun groupe défini. Le Groupe 1 sera créé par défaut.
                </div>
            </div>
          </v-card-text>
        </v-window-item>

        <!-- Tab 2: Weather -->
        <v-window-item value="weather">
          <v-card-text style="height: 400px; overflow-y: auto;">
            <div class="text-subtitle-1 font-weight-bold mb-2">Configuration de la météo</div>

            <v-card variant="tonal" color="blue-grey" class="pa-2">
                <!-- Row 1: Date Calendar -->
                <div class="calendar-container mb-4 mt-2">
                  <!-- Header: Days of week -->
                  <div class="d-flex text-caption text-grey text-center font-weight-bold mb-1">
                    <div style="width: 70px;">Mois</div>
                    <div class="px-1" style="flex: 1 1 0%;">Lun</div>
                    <div class="px-1" style="flex: 1 1 0%;">Mar</div>
                    <div class="px-1" style="flex: 1 1 0%;">Mer</div>
                    <div class="px-1" style="flex: 1 1 0%;">Jeu</div>
                    <div class="px-1" style="flex: 1 1 0%;">Ven</div>
                    <div class="px-1" style="flex: 1 1 0%;">Sam</div>
                    <div class="px-1" style="flex: 1 1 0%;">Dim</div>
                  </div>
                  
                  <!-- Weeks -->
                  <div v-for="(week, wIdx) in calendarWeeks" :key="wIdx" class="d-flex align-center text-center mb-1">
                    <div class="text-caption font-weight-bold text-grey-darken-1" style="width: 70px; line-height: 1.2;">
                      {{ week.monthLabel }}
                    </div>
                    <div v-for="(day, dIdx) in week.days" :key="dIdx" class="px-1" style="flex: 1 1 0%;">
                      <template v-if="day.empty">
                        <div style="height: 54px; padding: 2px; border: 1px solid transparent;"></div>
                      </template>
                      <template v-else>
                        <v-card 
                          :color="editedDateDepart === day.iso ? getStatusColorForDate(day.iso) : (day.isToday ? 'blue-grey-lighten-4' : undefined)"
                          variant="flat"
                          border
                          class="d-flex flex-column align-center justify-center cursor-pointer rounded overflow-hidden"
                          @click="editedDateDepart = day.iso"
                          style="height: 54px; padding: 2px;"
                        >
                            <span class="text-subtitle-2 font-weight-bold mb-1" :class="editedDateDepart === day.iso ? 'text-white' : ''" style="line-height:1;">{{ day.day }}</span>
                            <v-icon 
                                :color="editedDateDepart === day.iso ? 'white' : getStatusColorForDate(day.iso)" 
                                size="x-large"
                            >
                                mdi-sun-thermometer
                            </v-icon>
                        </v-card>
                      </template>
                    </div>
                  </div>
                </div>

                <!-- Row 2: File Management -->
                <v-row dense align="center">
                    <v-col cols="12">
                        <div class="d-flex align-center justify-end">
                            
                            <div class="d-flex flex-column align-end mr-4">
                                <div 
                                    class="text-caption font-weight-bold" 
                                    :class="weatherGlobalExplanation.class"
                                >
                                    {{ weatherGlobalExplanation.text }}
                                </div>
                            </div>

                            <!-- Case 1: Multiple Routes -> Menu -->
                            <div v-if="usedRoutes.length > 1" class="d-flex align-center mr-4">
                                <v-menu location="bottom end">
                                    <template v-slot:activator="{ props }">
                                        <v-btn
                                            :color="globalUpdateStatus.color"
                                            variant="flat"
                                            v-bind="props"
                                            :prepend-icon="globalUpdateStatus.icon"
                                            append-icon="mdi-menu-down"
                                            size="small"
                                            :loading="isDownloadingWeather"
                                        >
                                            {{ globalUpdateStatus.text }}
                                        </v-btn>
                                    </template>
                                    <v-list density="compact" nav>
                                        <v-list-item @click="() => downloadWeather()">
                                            <template v-slot:prepend>
                                                <v-icon :icon="globalUpdateStatus.icon" size="small"></v-icon>
                                            </template>
                                            <v-list-item-title class="font-weight-bold">
                                                Tout mettre à jour
                                            </v-list-item-title>
                                        </v-list-item>
                                        <v-divider class="my-1"></v-divider>
                                        <v-list-item
                                            v-for="(route, index) in usedRoutes"
                                            :key="index"
                                            :value="index"
                                            @click="() => downloadWeather(route.value)"
                                        >
                                            <template v-slot:prepend>
                                                <v-icon 
                                                    v-if="weatherStatusMap[route.value || 'main']?.present"
                                                    color="success"
                                                    size="small"
                                                >
                                                    mdi-check-circle
                                                </v-icon>
                                                <v-icon v-else color="error" size="small">
                                                    mdi-alert-circle
                                                </v-icon>
                                            </template>
                                            <v-list-item-title>
                                                {{ route.title }}
                                                <span v-if="weatherStatusMap[route.value || 'main']?.present" class="text-caption text-grey ml-2">
                                                    ({{ weatherStatusMap[route.value || 'main']?.relative }})
                                                </span>
                                            </v-list-item-title>
                                        </v-list-item>
                                    </v-list>
                                </v-menu>
                            </div>
                            
                            <!-- Case 2: Single Route -->
                            <v-btn
                                v-else
                                size="small"
                                :color="globalUpdateStatus.color"
                                variant="flat"
                                :loading="isDownloadingWeather"
                                @click="downloadWeather()"
                                :prepend-icon="globalUpdateStatus.icon"
                                class="mr-4"
                            >
                                {{ globalUpdateStatus.text }}
                            </v-btn>

                            <!-- Viewing Action (Menu or Button) -->
                            <div v-if="usedRoutes.length > 1" class="d-flex align-center ml-4">
                                <v-menu location="bottom end">
                                    <template v-slot:activator="{ props }">
                                        <v-btn color="info" variant="flat" v-bind="props" prepend-icon="mdi-eye" append-icon="mdi-menu-down" size="small">
                                            Voir...
                                        </v-btn>
                                    </template>
                                    <v-list density="compact" nav>
                                        <v-list-item
                                            v-for="(route, index) in usedRoutes"
                                            :key="index"
                                            :value="index"
                                            @click="() => { selectedRoute = route.value; loadAndShowWeather(); }"
                                            :disabled="!weatherStatusMap[route.value || 'main']?.present"
                                        >
                                            <template v-slot:prepend>
                                                <v-icon v-if="weatherStatusMap[route.value || 'main']?.present" color="success" size="small">
                                                    mdi-check-circle
                                                </v-icon>
                                                <v-icon v-else color="error" size="small">
                                                    mdi-alert-circle
                                                </v-icon>
                                            </template>
                                            <v-list-item-title>
                                                {{ route.title }}
                                                <span v-if="weatherStatusMap[route.value || 'main']?.present" class="text-caption text-grey ml-2">
                                                    ({{ weatherStatusMap[route.value || 'main']?.relative }})
                                                </span>
                                            </v-list-item-title>
                                        </v-list-item>
                                    </v-list>
                                </v-menu>
                            </div>
                            
                            <v-btn
                                v-else
                                size="small"
                                color="info"
                                variant="flat"
                                @click="loadAndShowWeather"
                                prepend-icon="mdi-eye"
                                :disabled="!isValid || !weatherFilePresent"
                            >
                                Voir
                            </v-btn>
                        </div>
                    </v-col>
                </v-row>
            </v-card>
          </v-card-text>
        </v-window-item>
      </v-window>

      <v-divider></v-divider>
      <v-card-actions class="px-4 py-3">
        <v-spacer></v-spacer>
        <v-btn color="primary" variant="flat" @click="saveMeteo" :disabled="!hasChanges">Enregistrer</v-btn>
      </v-card-actions>
    </v-card>

  </v-dialog>

  <v-dialog v-model="showDocDialog" max-width="800px" height="80%">
      <DocDisplay :doc-path="currentDocPath" @close="showDocDialog = false" />
  </v-dialog>

  <ConfirmationDialog
    v-model="showConfirmClose"
    title="Modifications non enregistrées"
    message="Vous avez des modifications qui n'ont pas encore été enregistrées.<br>Voulez-vous vraiment quitter sans sauvegarder ?"
    confirmText="Quitter sans sauvegarder"
    cancelText="Rester"
    color="warning"
    icon="mdi-alert"
    @confirm="forceClose"
  />

  <!-- GPX Export Confirmation Dialog -->
  <v-dialog v-model="showExportGpxDialog" max-width="450px" persistent>
    <v-card>
      <v-card-title class="bg-blue-darken-3 text-white px-4 py-2 d-flex align-center">
        <v-icon start icon="mdi-download"></v-icon>
        Configuration de l'export GPX
      </v-card-title>
      <v-card-text class="pa-4">
        <v-text-field
          v-model="gpxExportName"
          label="Nom interne de la trace (dans le GPX)"
          variant="outlined"
          density="compact"
          class="mb-4"
          hint="Format par défaut : Ville_Groupe_Jour"
          persistent-hint
        ></v-text-field>
        <v-text-field
          v-model="gpxExportFileName"
          label="Nom de fichier suggéré"
          variant="outlined"
          density="compact"
          hint="Format par défaut : Ville_Groupe_Jour.gpx"
          persistent-hint
        ></v-text-field>
      </v-card-text>
      <v-card-actions class="pa-4 pt-0">
        <v-spacer></v-spacer>
        <v-btn variant="text" @click="showExportGpxDialog = false">Annuler</v-btn>
        <v-btn color="primary" variant="flat" @click="handleConfirmExportGpx">Exporter</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <Teleport to="body">
    <WeatherWidgetStatic 
        v-if="showWeatherWidget"
        :weather-matrix="weatherMatrix"
        :scenarios="filteredScenarios"
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
import ConfirmationDialog from '@/components/ConfirmationDialog.vue';

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

const selectedRoute = ref(null); // null = Main, string = Variant ID
const weatherStatusMap = ref({}); // Stores status for each route

const weatherStatus = ref('Inconnu'); // Legacy
const isDownloadingWeather = ref(false);

const activeTab = ref('groups');

const showDocDialog = ref(false);
const currentDocPath = ref('');
const sortKey = ref(null);
const sortAsc = ref(true);
const showConfirmClose = ref(false);

// GPX Export Dialog State
const showExportGpxDialog = ref(false);
const gpxExportName = ref('');
const gpxExportFileName = ref('');
const exportingVariantId = ref(null);

// reference state for "hasChanges" check
const lastSavedState = ref({ date: "", scenarios: "" });

const openDoc = (path) => {
  currentDocPath.value = path;
  showDocDialog.value = true;
};

// Computed identifying available routes based on scenarios
const usedRoutes = computed(() => {
    const routes = new Map();
    editedScenarios.value.forEach(s => {
        const vId = s.variantId || null;
        if (!routes.has(vId)) {
            let title = 'Principale';
            if (vId) {
                const variant = availableVariants.value.find(v => v.value === vId);
                title = variant ? variant.title : 'Variante Inconnue';
            }
            routes.set(vId, { value: vId, title });
        }
    });
    // Sort: Main first, then alphabetic
    return Array.from(routes.values()).sort((a, b) => {
        if (a.value === null) return -1;
        if (b.value === null) return 1;
        return a.title.localeCompare(b.title);
    });
});

// Auto-select route if current selection is invalid
watch(usedRoutes, (routes) => {
    if (routes.length > 0) {
        // If current selection is not in list, default to first (usually Main)
        const exists = routes.find(r => r.value === selectedRoute.value);
        if (!exists) {
            selectedRoute.value = routes[0].value;
        }
    } else {
        selectedRoute.value = null;
    }
}, { immediate: true });

const currentRouteTitle = computed(() => {
    const r = usedRoutes.value.find(x => x.value === selectedRoute.value);
    return r ? r.title : 'Route';
});

// Status Computed Properties based on selectedRoute
const currentRouteStatus = computed(() => {
    const key = selectedRoute.value || 'main'; // Use string 'main' for dictionary key if null
    return weatherStatusMap.value[key] || { present: false, age: 0, relative: '', missing: true };
});

const weatherFilePresent = computed(() => currentRouteStatus.value.present);
const weatherFileAgeHours = computed(() => currentRouteStatus.value.age);
const weatherFileRelativeTime = computed(() => currentRouteStatus.value.relative);
const weatherMissingCount = computed(() => 0); // Deprecated/Unused in new UI but kept for safety

const weatherGlobalExplanation = computed(() => {
    const iso = editedDateDepart.value;
    const color = getStatusColorForDate(iso);
    
    // Re-calculer l'age relatif maximum
    const routes = usedRoutes.value;
    if (routes.length === 0) return { text: "Aucun parcours défini.", class: "text-grey" };
    
    let maxAge = -1;
    let relativeStr = '';
    
    routes.forEach(r => {
        const status = weatherStatusMap.value[r.value || 'main'];
        if (status && status.present && status.age > maxAge) {
            maxAge = status.age;
            relativeStr = status.relative;
        }
    });

    if (color === 'red-accent-4' || color === 'error') {
        return { text: `Aucune donnée météo pour le ${formattedDateLong.value}.`, class: 'text-red-accent-4' };
    }
    if (color === 'deep-orange') {
        return { text: `Données manquantes pour au moins un parcours pour le ${formattedDateLong.value}.`, class: 'text-deep-orange' };
    }
    if (color === 'orange-lighten-1' || color === 'orange') {
        return { text: `Données obsolètes (> 12h) [${relativeStr}] pour le ${formattedDateLong.value}.`, class: 'text-orange-darken-1' };
    }
    if (color === 'info' || color === 'blue') {
        return { text: `Données à rafraîchir (> 4h) [${relativeStr}] pour le ${formattedDateLong.value}.`, class: 'text-blue-darken-1' };
    }
    
    // Success / Green
    return { text: `Météo mise à jour il y-a ${relativeStr}, pour le ${formattedDateLong.value}.`, class: 'text-success' };
});

const globalUpdateStatus = computed(() => {
    const routes = usedRoutes.value;
    if (routes.length === 0) return { color: 'error', text: 'Télécharger', icon: 'mdi-download' };

    const statuses = routes.map(r => weatherStatusMap.value[r.value || 'main'] || { present: false, age: 0 });
    
    // 1. Red if everything is missing
    const allMissing = statuses.every(s => !s.present);
    if (allMissing) return { color: 'error', text: 'Télécharger', icon: 'mdi-download' };

    // 2. Dark Orange if some are missing
    const anyMissing = statuses.some(s => !s.present);
    if (anyMissing) return { color: 'deep-orange', text: 'Télécharger...', icon: 'mdi-download' };

    // 3. Light Orange if date > 12h
    const anyVeryOld = statuses.some(s => s.age > 12);
    if (anyVeryOld) return { color: 'orange', text: 'Mettre à jour...', icon: 'mdi-update' };

    // 4. Blue if date > 4h
    const anyOld = statuses.some(s => s.age > 4);
    if (anyOld) return { color: 'info', text: 'Mettre à jour', icon: 'mdi-update' };

    // 5. Green if all fresh (< 4h)
    return { color: 'success', text: 'Mettre à jour', icon: 'mdi-check' };
});

const ageColorClass = computed(() => {
    return weatherFileAgeHours.value < 3 ? 'text-green' : 'text-blue';
});

const dateColorClass = computed(() => {
    if (!editedDateDepart.value) return 'text-white';
    
    const [y, m, d] = editedDateDepart.value.split('-').map(Number);
    const inputDate = new Date(y, m - 1, d);
    
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    
    // Difference in days (approx)
    const diffTime = inputDate - today;
    const diffDays = Math.round(diffTime / (1000 * 60 * 60 * 24));
    
    if (diffDays < 0) return 'text-orange'; // Past
    if (diffDays === 1) return 'text-green'; // Tomorrow
    return 'text-white';
});

const formattedDateLong = computed(() => {
    if (!editedDateDepart.value) return "";
    const [y, m, d] = editedDateDepart.value.split('-').map(Number);
    const date = new Date(y, m - 1, d);
    return new Intl.DateTimeFormat('fr-FR', { day: '2-digit', month: 'long' }).format(date);
});

const monthNamesShort = ["Janv.", "Févr.", "Mars", "Avr.", "Mai", "Juin", "Juil.", "Août", "Sept.", "Oct.", "Nov.", "Déc."];

const calendarWeeks = computed(() => {
    const days = [];
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    for (let i = 0; i < 15; i++) {
        const d = new Date(today);
        d.setDate(today.getDate() + i);
        
        const y = d.getFullYear();
        const m = String(d.getMonth() + 1).padStart(2, '0');
        const day = String(d.getDate()).padStart(2, '0');
        const iso = `${y}-${m}-${day}`;
        
        days.push({
            date: d,
            iso: iso,
            day: d.getDate(),
            isToday: i === 0
        });
    }

    const weeks = [];
    let currentWeek = [];
    
    let firstDayOfWeek = days[0].date.getDay(); 
    if (firstDayOfWeek === 0) firstDayOfWeek = 7; // Dimanche = 7
    
    const emptyPrefixCount = firstDayOfWeek - 1; // Nb de cases vides avant lundi
    
    for (let i = 0; i < emptyPrefixCount; i++) {
        currentWeek.push({ empty: true });
    }

    const formatWeek = (weekDays) => {
        const activeDays = weekDays.filter(d => !d.empty);
        const m1 = activeDays[0].date.getMonth();
        const m2 = activeDays[activeDays.length - 1].date.getMonth();
        
        let monthLabel = monthNamesShort[m1];
        if (m1 !== m2) {
            monthLabel = `${monthNamesShort[m1]} - ${monthNamesShort[m2]}`;
        }
        return { monthLabel, days: weekDays };
    };

    days.forEach(d => {
        currentWeek.push({ empty: false, ...d });
        if (currentWeek.length === 7) {
            weeks.push(formatWeek(currentWeek));
            currentWeek = [];
        }
    });

    if (currentWeek.length > 0) {
        while (currentWeek.length < 7) {
            currentWeek.push({ empty: true });
        }
        weeks.push(formatWeek(currentWeek));
    }
    
    return weeks;
});

const calendarDatesList = computed(() => {
    const list = [];
    const today = new Date();
    for (let i = 0; i < 15; i++) {
        const d = new Date(today);
        d.setDate(today.getDate() + i);
        const y = d.getFullYear();
        const m = String(d.getMonth() + 1).padStart(2, '0');
        const day = String(d.getDate()).padStart(2, '0');
        list.push(`${y}-${m}-${day}`);
    }
    return list;
});

const allDaysWeatherStatus = ref({});

const getStatusColorForDate = (iso) => {
    return allDaysWeatherStatus.value[iso]?.color || 'red-accent-4';
};

const checkAllWeatherStatuses = async () => {
    if (!props.circuit?.circuitId) return;
    
    const routesToScan = usedRoutes.value;
    const datesToScan = calendarDatesList.value;
    
    const promises = [];
    
    for (const dIso of datesToScan) {
        for (const route of routesToScan) {
            const varId = route.value;
            const fname = varId 
                ? `weather_variant_${varId}_${dIso}.json` 
                : getFilenameForDate(dIso);
                
            promises.push(
                invoke('check_weather_cache_metadata', { 
                    circuitId: props.circuit.circuitId, 
                    filename: fname
                }).then(metadata => {
                    return { date: dIso, route: varId, metadata };
                }).catch(e => {
                    return { date: dIso, route: varId, metadata: null };
                })
            );
        }
    }
    
    const results = await Promise.all(promises);
    
    const datesMap = {}; 
    
    results.forEach(res => {
        if (!datesMap[res.date]) datesMap[res.date] = { statuses: [] };
        let present = false;
        let age = 0;
        if (res.metadata) {
            const updatedTime = new Date(res.metadata);
            const diffMs = new Date() - updatedTime;
            const hours = diffMs / (1000 * 60 * 60);
            present = true;
            age = hours;
        }
        datesMap[res.date].statuses.push({ present, age });
    });
    
    const newGlobalMap = {};
    for (const dIso of datesToScan) {
        const statuses = datesMap[dIso]?.statuses || [];
        
        if (statuses.length === 0) {
            newGlobalMap[dIso] = { color: 'red-accent-4' };
            continue;
        }
        
        const allMissing = statuses.every(s => !s.present);
        if (allMissing) {
            newGlobalMap[dIso] = { color: 'red-accent-4' };
            continue;
        }
        
        const anyMissing = statuses.some(s => !s.present);
        if (anyMissing) {
            newGlobalMap[dIso] = { color: 'deep-orange' };
            continue;
        }
        
        const anyVeryOld = statuses.some(s => s.age > 12);
        if (anyVeryOld) {
            newGlobalMap[dIso] = { color: 'orange-lighten-1' };
            continue;
        }
        
        const anyOld = statuses.some(s => s.age > 4);
        if (anyOld) {
             newGlobalMap[dIso] = { color: 'info' };
             continue;
        }
        
        newGlobalMap[dIso] = { color: 'success' };
    }
    
    allDaysWeatherStatus.value = newGlobalMap;
    checkWeatherStatus();
};

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
            id: s.id || `scen-${Date.now()}-${i}`,
            // Ensure variantId is reactive even if undefined in source
            variantId: s.variantId || null 
        }));
        
        // Ensure at least one reference exists (default to 1st if none)
        if (!editedScenarios.value.some(s => s.isReference)) {
            if (editedScenarios.value[0]) editedScenarios.value[0].isReference = true;
        }
    } else {
        // Default Group 1 logic
        createDefaultGroup();
    }
    
    // Capture initial state for change detection
    const normalize = (s) => ({
        nom: s.nom,
        heureDepart: s.heureDepart,
        vitesseMoyenne: parseFloat(s.vitesseMoyenne),
        isReference: !!s.isReference,
        variantId: s.variantId || null
    });
    
    lastSavedState.value = {
        date: editedDateDepart.value,
        scenarios: JSON.stringify(editedScenarios.value.map(normalize))
    };
    
    // Check cache status
    nextTick(() => {
        checkAllWeatherStatuses();
        loadVariants();
    });
};

const availableVariants = ref([]);

const loadVariants = async () => {
    try {
        const variants = await invoke('get_variants', { circuitId: props.circuit.circuitId });
        // Transform for selection
        availableVariants.value = [
            { title: 'Principale', value: null },
            ...variants.map(v => ({ title: v.name, value: v.id }))
        ];
    } catch (e) {
        console.error("Failed to load variants:", e);
        // Fallback to just main trace
        availableVariants.value = [{ title: 'Principale', value: null }];
    }
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
        isReference: true,
        variantId: null
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
});

// Computed
const hasVariants = computed(() => availableVariants.value.length > 1);

const colsName = computed(() => hasVariants.value ? 1 : 4);
const colsTime = computed(() => hasVariants.value ? 2 : 3);
const colsSpeed = computed(() => hasVariants.value ? 2 : 3);

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
    // Normalisation pour une comparaison fiable
    const normalize = (s) => ({
        nom: s.nom,
        heureDepart: s.heureDepart,
        vitesseMoyenne: parseFloat(s.vitesseMoyenne),
        isReference: !!s.isReference,
        variantId: s.variantId || null
    });

    const newScenarios = JSON.stringify(editedScenarios.value.map(normalize));
    const newDate = editedDateDepart.value;
    
    return newDate !== lastSavedState.value.date || newScenarios !== lastSavedState.value.scenarios;
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
        isReference: false,
        variantId: null
    });
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
};

const getSortIcon = (criteria) => {
    if (sortKey.value !== criteria) return 'mdi-sort';
    return sortAsc.value ? 'mdi-sort-ascending' : 'mdi-sort-descending';
};

const closeDialog = () => {
    if (hasChanges.value) {
        showConfirmClose.value = true;
    } else {
        forceClose();
    }
};

const forceClose = () => {
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

        // Update local reference state after success
        const normalize = (s) => ({
            nom: s.nom,
            heureDepart: s.heureDepart,
            vitesseMoyenne: parseFloat(s.vitesseMoyenne),
            isReference: !!s.isReference,
            variantId: s.variantId || null
        });
        
        lastSavedState.value = {
            date: editedDateDepart.value,
            scenarios: JSON.stringify(editedScenarios.value.map(normalize))
        };
        
        showSnackbar('Configuration météo enregistrée', 'success');
        emit('saved');
        checkAllWeatherStatuses(); // Re-check all
    } catch (e) {
        console.error("Save error:", e);
        showSnackbar("Erreur sauvegarde: " + e, 'error');
    }
};

const exportVariantGpx = async (variantId) => {
    if (!variantId) return;

    // Trouver le premier groupe (scénario) concerné par ce variant
    const firstGroup = editedScenarios.value.find(s => s.variantId === variantId);
    const groupName = firstGroup ? firstGroup.nom : "Variante";

    try {
        // Obtenir les noms par défaut via le backend (avec override du groupe)
        const defaults = await invoke('get_gpx_export_defaults', { 
            circuitId: props.circuit.circuitId,
            groupOverride: groupName 
        });
        
        gpxExportName.value = defaults.gpxName;
        gpxExportFileName.value = defaults.fileName;
        exportingVariantId.value = variantId;
        showExportGpxDialog.value = true;
    } catch (e) {
        console.error("Erreur lors de l'export GPX depuis MeteoManager:", e);
        showSnackbar("Erreur lors de l'export GPX : " + e, "error");
    }
};

const handleConfirmExportGpx = async () => {
    showExportGpxDialog.value = false;
    try {
        await invoke('export_variant_gpx', {
            circuitId: props.circuit.circuitId,
            variantId: exportingVariantId.value,
            variantName: gpxExportName.value,
            defaultFilename: gpxExportFileName.value
        });
    } catch (e) {
        if (e !== "Export annulé par l'utilisateur.") {
            console.error("Erreur lors de l'export GPX (confirmation):", e);
            showSnackbar("Erreur lors de l'export GPX : " + e, "error");
        }
    }
};

const downloadWeather = async (specificVariantId = undefined) => {
    if (!props.circuit?.circuitId || !editedDateDepart.value) return;
    
    if (editedScenarios.value.length === 0) {
         showSnackbar("Veuillez définir au moins un groupe.", "warning");
         return;
    }
    
    isDownloadingWeather.value = true;
    
    try {
        // Identify all unique variants needed (null = main trace)
        const variantsToUpdate = new Set();
        
        if (specificVariantId !== undefined) {
             // Update ONLY the specific one (can be null for main)
             variantsToUpdate.add(specificVariantId);
        } else {
             // Update ALL unique used by scenarios
            editedScenarios.value.forEach(s => {
                variantsToUpdate.add(s.variantId || null);
            });
        }

        const startH = getSettingValue('Visualisation/Météo/heureDebutJournee') || 6;
        const endH = getSettingValue('Visualisation/Météo/heureFinJournee') || 20;

        let successCount = 0;

        for (const varId of variantsToUpdate) {
            if (varId === null) {
                // --- Process Main Trace ---
                let trackData = await invoke('read_tracking_file', { circuitId: props.circuit.circuitId });
                if (!trackData || trackData.length === 0) {
                    console.warn("Main trace empty, skipping");
                    continue;
                }
                
                // Sample (1km resolution)
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

                if (sampled.length > 0) {
                    const matrix = await WeatherService.fetchWeatherMatrix(sampled, editedDateDepart.value, startH, endH);
                    if (matrix && matrix.length > 0) {
                        const filename = getFilenameForDate(editedDateDepart.value);
                        await invoke('save_weather_cache', {
                            circuitId: props.circuit.circuitId,
                            filename,
                            content: JSON.stringify(matrix, null, 2)
                        });
                        successCount++;
                    }
                }

            } else {
                // --- Process Variant ---
                // Load tracking file for variant
                const variantTracking = await invoke('read_tracking_file', { 
                    circuitId: props.circuit.circuitId, 
                    filename: `tracking_${varId}_FULL.json`
                });

                if (variantTracking && variantTracking.length > 0) {
                    // Group by segments (contiguous typeTroncon)
                    const segments = [];
                    let currentSeg = null;
                    
                    variantTracking.forEach(pt => {
                        const type = pt.typeTroncon || "Commun"; // Default if missing
                        
                        if (!currentSeg || currentSeg.type !== type) {
                            if (currentSeg) segments.push(currentSeg);
                            currentSeg = {
                                id: `seg-${segments.length}`,
                                type: type,
                                points: []
                            };
                        }
                        if (pt.coordonnee) {
                            currentSeg.points.push(pt.coordonnee);
                        }
                    });
                    if (currentSeg) segments.push(currentSeg);

                    // Generate Weather
                    await WeatherService.generateVariantWeather(
                        props.circuit.circuitId,
                        varId,
                        segments,
                        editedDateDepart.value
                    );
                    successCount++;
                }
            }
        }

        if (successCount > 0) {
            showSnackbar(`${successCount} météo(s) mise(s) à jour`, 'success');
            emit('downloaded');
            checkAllWeatherStatuses();
        } else {
            showSnackbar("Aucune mise à jour effectuée check console.", "warning");
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
const filteredScenarios = ref([]); // Scenarios to display in widget

const loadAndShowWeather = async () => {
    if (!props.circuit?.circuitId || !editedDateDepart.value) return;
    
    // Select the cache file based on the CURRENT SELECTION
    const varId = selectedRoute.value; // can be null (Main) or string (Variant ID)

    // Filter scenarios to show only those matching the selected route
    filteredScenarios.value = editedScenarios.value.filter(s => s.variantId === varId);

    let filename;
    if (varId) {
        filename = `weather_variant_${varId}_${editedDateDepart.value}.json`;
    } else {
        filename = getFilenameForDate(editedDateDepart.value);
    }

    try {
        const cacheContent = await invoke('check_weather_cache', { 
            circuitId: props.circuit.circuitId, 
            filename 
        });
        
        if (cacheContent) {
            const data = JSON.parse(cacheContent);
            
            if (Array.isArray(data)) {
                 if (data.length > 0 && data[0].segmentId) {
                     // Flatten variant structure for static widget visualization
                     const flat = [];
                     let cumDist = 0;
                     data.forEach(seg => {
                         seg.points.forEach(p => {
                            flat.push({
                                km: cumDist + (p.km_local || 0),
                                hours: p.meteo
                            });
                         });
                         if (seg.points.length > 0) {
                             const maxK = Math.max(...seg.points.map(p => p.km_local));
                             cumDist += maxK;
                         }
                     });
                     weatherMatrix.value = flat;
                 } else {
                     weatherMatrix.value = data;
                 }
            }
            
            const [y, m, d] = editedDateDepart.value.split('-').map(Number);
            weatherDate.value = new Date(y, m - 1, d);
            showWeatherWidget.value = true;
        } else {
            showSnackbar("Aucun fichier météo trouvé pour ce parcours.", "warning");
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
    
    // Check status for ALL unique routes used
    const routesToScan = usedRoutes.value; // [{value: null|id, title: ...}]
    const updates = {};
    let updatesCount = 0;

    for (const route of routesToScan) {
        const varId = route.value;
        const fname = varId 
            ? `weather_variant_${varId}_${editedDateDepart.value}.json` 
            : getFilenameForDate(editedDateDepart.value);
            
        try {
            const metadata = await invoke('check_weather_cache_metadata', { 
                circuitId: props.circuit.circuitId, 
                filename: fname
            });

            if (metadata) {
                const updatedTime = new Date(metadata);
                const now = new Date();
                const diffMs = now - updatedTime;
                const hours = diffMs / (1000 * 60 * 60);

                let relative = "";
                if (hours < 1) {
                    const min = Math.round(diffMs / (1000 * 60));
                    relative = `il y a ${min} min`;
                } else if (hours < 24) {
                    relative = `il y a ${Math.round(hours)}h`;
                } else {
                    relative = `il y a ${Math.round(hours / 24)}j`;
                }

                updates[varId || 'main'] = {
                    present: true,
                    age: hours,
                    relative: relative,
                    missing: false
                };
            } else {
                updates[varId || 'main'] = {
                    present: false,
                    age: 0,
                    relative: '',
                    missing: true
                };
            }
        } catch (e) {
            console.warn(`Check failed for ${varId}`, e);
            updates[varId || 'main'] = { present: false, missing: true };
        }
        updatesCount++;
    }
    
    // Fallback if usedRoutes is empty (shouldn't happen with watcher)
    if (updatesCount === 0) {
        weatherStatusMap.value = {};
    } else {
        weatherStatusMap.value = updates;
    }
};

watch(editedScenarios, () => {
    checkAllWeatherStatuses();
}, { deep: true });
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
