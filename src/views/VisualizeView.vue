<template>
  <div id="map-container" ref="mapContainer" :class="{ 'hide-cursor': isCursorHidden }"></div>

  <center-marker v-if="isCenterMarkerVisible" :color="couleurCroixCentrale" />

  <transition name="fade">
    <v-btn v-if="!isInitializing && isBackButtonVisibleFinal" icon="mdi-arrow-left" class="back-button" @click="goBack" title="Retour à l'accueil (h)"></v-btn>
  </transition>

  <transition name="fade-opacity">
    <div v-if="!isInitializing && shouldShowCommuneWidget && isCommuneWidgetVisible" class="commune-display" :style="{ borderColor: communeWidgetBorderColor }" @wheel.stop>
      <span class="font-weight-bold">{{ currentCommuneName }}</span>
    </div>
  </transition>



  <div class="top-center-container">
    <transition name="fade-opacity">
      <v-card v-if="!isInitializing && isDistanceDisplayVisible" variant="elevated" class="distance-display" @wheel.stop>
            <div class="d-flex align-center justify-center fill-height px-4">
              <span class="font-weight-bold">Distance :&nbsp;</span>
              <span class="font-weight-bold">{{ globalProgressStats.current.toFixed(2) }}</span> <span class="font-weight-bold">&nbsp;/ {{ globalProgressStats.total.toFixed(2) }} km</span>
            </div>  
      </v-card>
    </transition>


  </div>

  <div class="top-right-container" style="position: absolute; top: 10px; right: 10px; z-index: 1000; pointer-events: none;">
    <transition name="fade">
        <WeatherWidgetDynamic 
            v-if="!isInitializing && currentWeather && (isWeatherInfoVisible || isCompassVisible)" 
            :weather="currentWeather" 
            :bearing="currentCameraBearing" 
            :trace-bearing="currentTraceBearing" 
            :orientation-mode="currentOrientationMode"
            :show-info="isWeatherInfoVisible"
            :show-compass="isCompassVisible"
            :scenarios="circuitScenarios"
            :weather-matrix="weatherForecasts"
            :current-distance="currentDistance"
            :simulation-start-date="simulationStartDate"
        />
    </transition>
    <transition name="fade">
        <WeatherWidgetStatic 
            v-if="showWeatherTable && weatherForecasts.length > 0" 
            :weather-matrix="weatherForecasts" 
            :scenarios="circuitScenarios"
            :date="simulationStartDate"
            @close="showWeatherTable = false" 
            style="pointer-events: auto;"
        />
    </transition>
  </div>

              <div class="bottom-center-container">
    <transition name="fade">
      <div v-if="!isInitializing && isAltitudeVisible" class="altitude-svg-container" @wheel.stop>
          <altitude-s-v-g 
            :circuit-id="props.circuitId" 
            :current-distance="currentDistanceInMeters" 
            :total-distance="totalDistanceRef > 0 ? totalDistanceRef * 1000 : 1"
            :tracking-points="trackingPointsWithDistanceRef"
            :is-variant-comparison="isComparisonModeRef"
            :main-trace-points="mainTraceComparisonPointsRef"
            :variant-segments="activeVariantSegments"
            :current-segment-index="currentSegmentIndex"
          />
      </div>
    </transition>
    <transition name="fade">
      <v-btn v-if="!isInitializing && isPaused && !isControlsCardVisible"
             color="warning"
             @click="isAnimationFinished ? resetAnimation() : isPaused = false"
             class="bottom-controls"
             size="x-large"
             rounded
             title="Reprendre l'animation (P)"
             @wheel.stop
      >
        Reprise
      </v-btn>
    </transition>
    <transition name="fade-opacity">
      <div v-if="!isInitializing && isControlsCardVisible" class="bottom-controls" title="Afficher/Masquer (Espace)" @wheel.stop>
        <v-card variant="elevated" class="controls-card">
            <div class="d-flex align-center pa-1">
                            <v-btn icon="mdi-rewind" variant="text" size="x-small"
                                   @mousedown="isRewinding = true"
                                   @mouseup="isRewinding = false" @mouseleave="isRewinding = false"></v-btn>
                            <v-btn :icon="isPaused ? 'mdi-play' : 'mdi-pause'" variant="text" @click="isPaused = !isPaused" :disabled="isAnimationFinished"></v-btn>
                            <v-divider vertical class="mx-2"></v-divider>
                            <v-slider
                                v-model="sliderPosition"
                                :min="0"
                                :max="100"
                                :step="1"
                                hide-details
                                class="align-center speed-slider"
                                :disabled="isAnimationFinished"
                            >
                                <template v-slot:append>
                                    <span class="speed-value-display">{{ currentSpeed.toFixed(1) }}x</span>
                                    <v-btn icon="mdi-numeric-1-box-outline" variant="text" @click="currentSpeed = defaultSpeedValue" :disabled="isAnimationFinished"></v-btn>
                                </template>
                            </v-slider>
                            <!-- Variant / Segment UI -->
                            <template v-if="variants.length > 0">
                                        <v-divider vertical class="mx-2"></v-divider>
                                        
                                        <!-- Case 1: No Variant Selected -->
                                        <template v-if="!currentVariantId">
                                            <!-- Multiple Variants: Menu -->
                                            <v-menu v-if="variants.length > 1">
                                                <template v-slot:activator="{ props }">
                                                    <v-btn 
                                                        v-bind="props"
                                                        icon="mdi-source-branch" 
                                                        variant="text" 
                                                        title="Choisir une variante"
                                                        :disabled="!isPaused"
                                                    ></v-btn>
                                                </template>
                                                <v-list density="compact">
                                                    <v-list-item 
                                                        v-for="variant in variants" 
                                                        :key="variant.id" 
                                                        :title="variant.name" 
                                                        @click="handleSelectVariant(variant)"
                                                        prepend-icon="mdi-source-branch"
                                                    ></v-list-item>
                                                </v-list>
                                            </v-menu>
                                            <!-- Single Variant: Direct Action -->
                                            <v-btn 
                                                v-else
                                                icon="mdi-source-branch" 
                                                variant="text" 
                                                title="Charger la variante"
                                                :disabled="!isPaused"
                                                @click="handleSelectVariant(variants[0])"
                                            ></v-btn>
                                        </template>

                                        <!-- Case 2: Variant Selected -->
                                        <template v-else>
                                             <!-- Back to Main Trace Button -->
                                             <v-btn 
                                                icon="mdi-location-exit" 
                                                variant="text" 
                                                title="Retour à la trace maîtresse" 
                                                @click="loadMainTrace" 
                                                style="transform: rotate(180deg);"
                                                color="warning"
                                             ></v-btn>
                                             
                                             <!-- Unified Segment Navigation (Icons) -->
                                             <div v-if="isMultisegmentVariant" class="d-flex align-center ml-2">
                                                <v-tooltip v-for="(seg, i) in activeVariantSegments" :key="i" location="top">
                                                    <template v-slot:activator="{ props }">
                                                        <v-btn
                                                            v-bind="props"
                                                            :icon="getSegmentIcon(seg.type)"
                                                            variant="text" 
                                                            density="compact"
                                                            :color="currentDistanceInMeters >= seg.startDistKm*1000 && currentDistanceInMeters < seg.endDistKm*1000 ? 'primary' : 'grey'"
                                                            @click="focusSegment(i)"
                                                        ></v-btn>
                                                    </template>
                                                    <span>{{ seg.type.replace('_',' ') }}</span>
                                                </v-tooltip>
                                             </div>
                                             
                                             <!-- Legacy Fallback (Should not be reached if loadFullVariant used) -->
                                             <v-menu v-else-if="selectedVariant && getVariantSegments(selectedVariant).length > 1">
                                                <template v-slot:activator="{ props }">
                                                    <v-btn
                                                        v-bind="props"
                                                        icon="mdi-menu"
                                                        variant="text"
                                                        :disabled="!isPaused"
                                                        :color="currentSegmentType ? getSegmentColor({ type: currentSegmentType }) : ''"
                                                        title="Changer de segment"
                                                    ></v-btn>
                                                </template>
                                                <v-list density="compact">
                                                    <v-list-item 
                                                        v-for="(mod, index) in getVariantSegments(selectedVariant)" 
                                                        :key="index"
                                                        @click="loadVariantSegment(selectedVariant.id, mod, mod.originalIndex)"
                                                    >
                                                        <template v-slot:prepend>
                                                            <v-icon :color="getSegmentColor(mod)" class="mr-2">{{ getModIcon(mod) }}</v-icon>
                                                        </template>
                                                        <v-list-item-title :class="'text-' + getSegmentColor(mod)">{{ getSegmentTitle(mod) }}</v-list-item-title>
                                                    </v-list-item>
                                                </v-list>
                                             </v-menu>
                                        </template>
                                    </template>
          </div>
        </v-card>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch, nextTick, reactive } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import mapboxgl from 'mapbox-gl';
import * as turf from '@turf/turf';
import { useTheme } from 'vuetify';
import { useSettings } from '@/composables/useSettings';
import { useSnackbar } from '@/composables/useSnackbar';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import { useSharedUiState } from '@/composables/useSharedUiState';
import { useMessageDisplay } from '@/composables/useMessageDisplay.js';
import AltitudeSVG from '@/components/Visualize/AltitudeSVG.vue';
import CenterMarker from '@/components/CenterMarker.vue';
import { useVariantCalculator } from '@/composables/useVariantCalculator';

const { calculateVariantStats } = useVariantCalculator();
const mainTraceTotalDistanceValue = ref(0);

import WeatherWidgetDynamic from '@/components/Visualize/WeatherWidgetDynamic.vue';
import WeatherWidgetStatic from '@/components/Visualize/WeatherWidgetStatic.vue';
import WeatherService from '@/services/WeatherService';



const props = defineProps({
  circuitId: {
    type: String,
    required: true,
  },
});

const animationState = ref('Initialisation');

watch(animationState, (newState) => {
  invoke('update_animation_state', { newState });
  // Automatically control isBackButtonVisible based on animationState
  if (newState === 'En_Pause' || newState === 'Termine' || newState === 'En_Pause_au_Depart') {
    isBackButtonVisible.value = true;
  } else {
    isBackButtonVisible.value = false;
  }
});



const router = useRouter();
const { settings, getSettingValue } = useSettings();
const { showSnackbar } = useSnackbar();
const { interruptUpdate } = useCommunesUpdate();
const { current: theme } = useTheme();
const { toHex } = useVuetifyColors();
const { createMessageSVG } = useMessageDisplay();

// --- Center Marker Logic ---
const afficherCroixCentrale = computed(() => getSettingValue('Visualisation/Lecture/afficherCroixCentrale'));
const couleurCroixCentrale = computed(() => getSettingValue('Visualisation/Lecture/couleurCroixCentrale'));
const zoomMinimum = computed(() => (getSettingValue('Visualisation/Lecture/zoomMinimum') ?? 100) / 10);
const isCenterMarkerVisible = computed(() => {
  return afficherCroixCentrale.value && animationState.value === 'En_Pause';
});

const { isBackButtonVisible, toggleBackButtonVisibility } = useSharedUiState();

const shouldShowBackButtonBasedOnAnimationState = computed(() => {
    const state = animationState.value;
    return state === 'En_Pause' || state === 'Termine' || state === 'En_Pause_au_Depart';
});

const isBackButtonVisibleFinal = computed(() => {
    return isBackButtonVisible.value && shouldShowBackButtonBasedOnAnimationState.value;
});

const isInitializing = ref(true);
const isDistanceDisplayVisible = ref(getSettingValue('Visualisation/Widgets/distance') ?? true);
const isControlsCardVisible = ref(getSettingValue('Visualisation/Widgets/commandes') ?? true);
const isCommuneWidgetVisible = ref(getSettingValue('Visualisation/Widgets/communes') ?? true);
const isAltitudeVisible = ref(getSettingValue('Visualisation/Widgets/altitude') ?? true);
const isCursorHidden = ref(false);

// --- Weather State ---
const weatherForecasts = ref([]);
const simulationStartDate = ref(null);
const currentWeather = ref(null);
// const isWeatherVisible = ref(true); // Removed
const isStaticWeatherVisible = ref(getSettingValue('Visualisation/Météo/Widgets/informationMeteo') ?? true);
const showWeatherTable = ref(false); // Widget Météo Tableau (Shift+M)
const isDynamicWeatherVisible = ref(getSettingValue('Visualisation/Météo/Widgets/boussole') ?? true);
const isWeatherInfoVisible = ref(isStaticWeatherVisible.value);
const isCompassVisible = ref(isDynamicWeatherVisible.value);
const currentTraceBearing = ref(0);
const currentCameraBearing = ref(0);
const currentCircuitRef = ref(null); // Added for settings priority

// Weather Settings
// Prioritize circuit settings if available, otherwise fallback to global settings
const meteoActif = computed(() => getSettingValue('Visualisation/Météo/meteoActif') ?? true);

const defaultHeureDepart = computed(() => {
    if (currentCircuitRef.value?.meteoConfig?.heureDepart) {
        return currentCircuitRef.value.meteoConfig.heureDepart;
    }
    return getSettingValue('Visualisation/Météo/heureDepart') || "08:30";
});

const defaultVitesseMoyenne = computed(() => {
    if (currentCircuitRef.value?.meteoConfig?.vitesseMoyenne) {
        return currentCircuitRef.value.meteoConfig.vitesseMoyenne;
    }
    return getSettingValue('Visualisation/Météo/vitesseMoyenne') || 20.0;
});

const orientationBoussole = computed(() => {
    if (currentCircuitRef.value?.meteoConfig?.orientationBoussole) {
        return currentCircuitRef.value.meteoConfig.orientationBoussole;
    }
    return getSettingValue('Visualisation/Météo/orientationBoussole') || "Trace";
});

const currentOrientationMode = ref(orientationBoussole.value);
watch(orientationBoussole, (newVal) => {
    currentOrientationMode.value = newVal;
});

const globalProgressStats = computed(() => {
    return calculateVariantStats(
        mainTraceTotalDistanceValue.value,
        selectedVariant.value?.details?.modifications || [],
        0.1, // segmentLength (estimate or from settings)
        currentSegmentIndex.value,
        currentDistanceInMeters.value / 1000
    );
});

const currentDistance = computed(() => currentDistanceInMeters.value / 1000);
const circuitScenarios = computed(() => currentCircuitRef.value?.meteoConfig?.scenarios || []);

// --- Commune Widget State ---
const avancementCommunes = ref(0);
const currentCommuneName = ref('');
const shouldShowCommuneWidget = computed(() => avancementCommunes.value > 6);
const communeWidgetBorderColor = computed(() => theme.value.colors['red-darken-3'] || '#C62828');

const unlistenFunctions = [];

const mapContainer = ref(null);
let map = null;
let animationFrameId = null;
let finalizationTimeoutId = null;
const coloredSegmentsGeoJsonRef = ref(null);

let cursorTimer = null;
let isMapInitialized = false;
let warningShown = false;
let accumulatedTime = 0;
let lastTimestamp = 0;
let activePopups = new Map();

const lineStringRef = ref(null);
const trackingDataRef = ref(null);
const totalDistanceRef = ref(0);
const totalDurationAt1xRef = ref(0);
const trackingPointsWithDistanceRef = ref([]);
// --- Comparison Refs ---
const isComparisonModeRef = ref(false);
const mainTraceComparisonPointsRef = ref([]);
const mainTraceComparisonStartRef = ref(0);
const mainTraceComparisonEndRef = ref(0);
const mainTraceComparisonTypeRef = ref('segment');

// --- Variant Stitching State ---
const activeVariantSegments = ref([]); // List of segments: { index, type, startDist, endDist, data... }
const isMultisegmentVariant = ref(false); // Flag for UI logic
const totalVariantDistanceRef = ref(0);
 // 'start', 'end', or 'segment'
const controlPointIndicesRef = ref([]);
const pauseIncrements = ref([]);
const flytoEvents = ref({});
const rangeEvents = ref([]);
const currentDistanceInMeters = ref(0);
const segmentMetadata = ref(null); // Métadonnées des segments superposés
const currentActiveZone = ref(null); // Zone active actuelle
const mainTraceState = ref(null); // Sauvegarde du contexte de la trace maîtresse

// Gradients pour les 4 layers de trace
const layerGradients = ref({
    main: null,
    aller: null,
    retour: null,
    neutral: null
});

const isTransitioning = ref(false);
const variantLayerIds = ref([]);

// Variants state
const variants = ref([]);
const currentVariantId = ref(null);
const currentSegmentType = ref(null);
const currentSegmentIndex = ref(null);

// Variant Helper Functions
const getVariantSegments = (variant) => {
    if (!variant.details || !variant.details.modifications) return [];
    
    // Attach original index to each modification to preserve file mapping reference
    const modsWithIndex = variant.details.modifications.map((m, i) => ({...m, originalIndex: i}));

    // Sort modifications by position on master trace
    return modsWithIndex.sort((a, b) => {
        // Force DEPART to be first, ARRIVEE to be last
        let idxA = 0;
        if (a.type === 'DEPART_DEPORTE') idxA = -1;
        else if (a.type === 'ARRIVEE_REPORTEE') idxA = Number.MAX_SAFE_INTEGER;
        else idxA = a.anchorStart ? a.anchorStart.index : 0;

        let idxB = 0;
        if (b.type === 'DEPART_DEPORTE') idxB = -1;
        else if (b.type === 'ARRIVEE_REPORTEE') idxB = Number.MAX_SAFE_INTEGER;
        else idxB = b.anchorStart ? b.anchorStart.index : 0;

        return idxA - idxB;
    });
};

const getSegmentTitle = (mod) => {
    if (mod.name) return mod.name;
    if (mod.type === 'DEPART_DEPORTE') return 'Départ';
    if (mod.type === 'ARRIVEE_REPORTEE') return 'Arrivée';
    // Use originalIndex if available, otherwise 0
    const idx = typeof mod.originalIndex === 'number' ? mod.originalIndex : 0;
    if (mod.type === 'SEGMENT_DEVIATION') return `Segment ${idx + 1}`;
    return 'Segment';
};

const getModIcon = (mod) => {
    if (mod.type === 'DEPART_DEPORTE') return 'mdi-ray-start-arrow';
    if (mod.type === 'ARRIVEE_REPORTEE') return 'mdi-ray-end-arrow';
    return 'mdi-source-branch';
};

const getSegmentColor = (mod) => {
    if (mod.type === 'DEPART_DEPORTE') return 'success';
    if (mod.type === 'ARRIVEE_REPORTEE') return 'error';
    if (mod.type === 'SEGMENT_DEVIATION') return 'primary';
    return 'grey';
};

const selectedVariant = computed(() => variants.value.find(v => v.id === currentVariantId.value));

const handleSelectVariant = (variant) => {
    loadFullVariant(variant.id, variant);
};

// Helper pour convertir les couleurs (supporte Vuetify + noms CSS)
const toHexImproved = (value) => {
    if (!value) return '#FF0000';
    // Essayer d'abord la conversion Vuetify
    const vuetifyHex = toHex(value);
    if (vuetifyHex && vuetifyHex.startsWith('#')) return vuetifyHex;

    if (typeof value === 'string') {
        if (value.startsWith('#') || value.startsWith('rgb')) return value;
        const colors = {
            'red': '#FF0000', 'blue': '#0000FF', 'green': '#008000',
            'yellow': '#FFFF00', 'white': '#FFFFFF', 'black': '#000000',
            'gray': '#808080', 'light-blue': '#ADD8E6', 'orange': '#FFA500'
        };
        return colors[value] || value;
    }
    return '#FF0000';
};

const loadVariantSegment = async (variantId, modification, index) => {
    // 1. Sauvegarder la progression actuelle si on est sur la trace maîtresse
    // On vérifie currentSegmentType car currentVariantId peut avoir été défini par le menu de sélection
    if (currentSegmentType.value === null && mainTraceState.value) {
        // On capture la valeur PRECISE du compteur de temps global
        mainTraceState.value.accumulatedTime = Number(accumulatedTime);
        mainTraceState.value.isPaused = isPaused.value;
        // Save Main Trace points for comparison
        mainTraceState.value.trackingPoints = [...trackingPointsWithDistanceRef.value];
        console.log(`[STATE SAVE] Main Trace -> Variant: Time=${mainTraceState.value.accumulatedTime}ms, Paused=${isPaused.value}`);
    }

    currentVariantId.value = variantId;
    currentSegmentType.value = modification.type;
    currentSegmentIndex.value = index;

    let suffix = "";
    if (modification.type === 'DEPART_DEPORTE') suffix = "DEPART";
    else if (modification.type === 'ARRIVEE_REPORTEE') suffix = "ARRIVEE";
    else if (modification.type === 'SEGMENT_DEVIATION') suffix = `SEGMENT_${index}`;

    const trackingFilename = `tracking_${variantId}_${suffix}.json`;
    const lineStringFilename = `lineString_${variantId}_${suffix}.json`;
    const variantIdForEvents = `${variantId}_${suffix}`;
    
    try {
    isInitializing.value = true;
    
    // Stop any running animation loop immediately
    if (animationFrameId) {
        cancelAnimationFrame(animationFrameId);
        animationFrameId = null;
    }

    // 2. Charger les nouvelles données
        const [rawTrackingData, rawLineStringData, fetchedEvents] = await Promise.all([
            invoke('read_tracking_file', { circuitId: props.circuitId, filename: trackingFilename }),
            invoke('read_line_string_file', { circuitId: props.circuitId, filename: lineStringFilename }),
            invoke('get_events', { circuitId: props.circuitId, variantId: variantIdForEvents })
        ]);

        if (!rawTrackingData || rawTrackingData.length < 1) throw new Error("Données de tracking invalides");

        lineStringRef.value = rawLineStringData;
        
        const processedData = await invoke('process_tracking_data', {
            lineStringGeojson: rawLineStringData,
            trackingPointsJs: rawTrackingData
        });

        trackingPointsWithDistanceRef.value = processedData.processedPoints;
        totalDistanceRef.value = processedData.totalDistanceKm;
        totalDurationAt1xRef.value = totalDistanceRef.value * (animationSpeed?.value || 10);

        // --- Comparaison Profile Logic ---
        isComparisonModeRef.value = false;
        mainTraceComparisonPointsRef.value = [];
        mainTraceComparisonStartRef.value = 0;
        mainTraceComparisonEndRef.value = 0;
        mainTraceComparisonTypeRef.value = 'segment';

        // TENTATIVE: Try to enable comparison if data available
        if (mainTraceState.value?.trackingPoints) {
             const type = modification.type;
             let startIdx = null;
             let endIdx = null;
             let compType = 'segment';

             if (type === 'SEGMENT_DEVIATION') {
                  startIdx = modification.anchorStart?.index;
                  endIdx = modification.anchorEnd?.index;
                  compType = 'segment';
             } else if (type === 'DEPART_DEPORTE') {
                  startIdx = 0;
                  endIdx = modification.anchorIndexOnMaster || 0;
                  compType = 'start'; // Align to the right (to anchorEnd)
             } else if (type === 'ARRIVEE_REPORTEE') {
                  startIdx = modification.anchorIndexOnMaster || 0;
                  const mainPoints = mainTraceState.value.trackingPoints;
                  endIdx = mainPoints.length - 1;
                  compType = 'end'; // Align to the left (to anchorStart)
             }
             
             if (typeof startIdx === 'number' && typeof endIdx === 'number' && startIdx < endIdx) {
                  const mainPoints = mainTraceState.value.trackingPoints;
                  if (mainPoints && endIdx < mainPoints.length) {
                       const startKm = mainPoints[startIdx].distance;
                       const endKm = mainPoints[endIdx].distance; 
                       const mainSectionLenKm = endKm - startKm;
                       const variantLenKm = totalDistanceRef.value;
                       
                       // Check condition: Variant Shorter than Main Trace Section
                       if (variantLenKm < mainSectionLenKm - 0.001) {
                            isComparisonModeRef.value = true;
                            mainTraceComparisonStartRef.value = startKm;
                            mainTraceComparisonEndRef.value = endKm;
                            mainTraceComparisonTypeRef.value = compType;
                            
                            mainTraceComparisonPointsRef.value = mainPoints.map(p => ({
                                distance: p.distance,
                                altitude: p.altitude,
                            }));
                            
                            console.log(`[COMPARISON MODE] Enabled (${compType}). Main: ${mainSectionLenKm.toFixed(3)}km, Variant: ${variantLenKm.toFixed(3)}km`);
                       } else {
                           console.log(`[COMPARISON MODE] Skipped. Variant (${variantLenKm.toFixed(3)}km) >= Main (${mainSectionLenKm.toFixed(3)}km)`);
                       }
                  }
             }
        }

        // 3. Reset segments/events
        if (fetchedEvents) {
            pauseIncrements.value = Object.keys(fetchedEvents.pointEvents || {})
                .filter(inc => fetchedEvents.pointEvents[inc].some(e => e.type === 'Pause'))
                .map(Number);
                
            const flytos = {};
            for (const inc in fetchedEvents.pointEvents || {}) {
                const flytoE = fetchedEvents.pointEvents[inc].find(e => e.type === 'Flyto');
                if (flytoE) flytos[Number(inc)] = flytoE.data;
            }
            flytoEvents.value = flytos;
            rangeEvents.value = fetchedEvents.rangeEvents || [];
        }

        // 4. Reset Animation Engine
        
        // Initial setup for Accumulator/Distance
        // IF Comparison Mode:
        // - 'start': we start at 0 (beginning of variant = beginning of trace)
        // - 'segment' or 'end': we might want to start "at the variant" visually?
        // Actually, for consistency, let's start everything at 0 (time 0 on the variant timeline).
        // BUT, visually on the graph, for 'end' variant, 0 is at globalOffset. 
        // AltitudeSVG handles getX(0 + offset). So sending distance=0 is correct for the logic "Beginning of Variant".
        
        accumulatedTime = 0;
        currentDistanceInMeters.value = 0;
        
        lastTimestamp = 0;
        isPaused.value = true;
        isAnimationFinished.value = false;
        triggeredPauseIncrement.value = null;
        triggeredFlytoIncrement.value = null;
        cameraMovedDuringPause.value = false; // Reset camera movement tracking
        
        // Update Map Source (Variant Display)
        if (map) {
             const variantSourceId = 'variant-line';
             const variantLayerId = 'variant-line-layer';
             const originalOpacity = traceOpacity.value ?? 1.0;
             const baseWidth = traceWidth.value ?? 4;
             
             // Générer le gradient de couleur pour la variante (Lissé)
             let useSlopeColors = false;
             let gradientExpression = null;

             if (colorTraceBySlope.value) {
                 try {
                     const slopeColors = {
                        TrancheNegative: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative')),
                        Tranche1: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1')),
                        Tranche2: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2')),
                        Tranche3: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3')),
                        Tranche4: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4')),
                        Tranche5: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5')),
                     };
                     
                     const points = trackingPointsWithDistanceRef.value;
                     if (points && points.length > 1) {
                         const startDist = points[0].distance;
                         const totalSegmentLen = points[points.length-1].distance - startDist;
                         
                         // Construire l'expression 'interpolate' ['line-progress']
                         const expr = ['interpolate', ['linear'], ['line-progress']];
                         
                         // Helper: calculer couleur par pente
                         const getCol = (p1, p2) => {
                             const distM = (p2.distance - p1.distance) * 1000;
                             const elev = (p2.altitude || 0) - (p1.altitude || 0);
                             const slope = distM > 0 ? (elev / distM) * 100 : 0;
                             if (slope < 0) return slopeColors.TrancheNegative || '#0000FF';
                            if (slope < 3) return slopeColors.Tranche1 || '#00FF00';
                            if (slope < 6) return slopeColors.Tranche2;
                            if (slope < 9) return slopeColors.Tranche3;
                            if (slope < 12) return slopeColors.Tranche4;
                            return slopeColors.Tranche5;
                         };
                         
                         // Point départ
                         expr.push(0);
                         let firstColor = getCol(points[0], points[1]);
                         expr.push(firstColor);

                         const transitionDistKm = 0.025; // 25m transition (comme backend)
                         let lastRatioAdded = 0;

                         // Parcourir les segments pour les transitions
                         for (let i = 0; i < points.length - 1; i++) {
                             const p1 = points[i];
                             const p2 = points[i+1];
                             const currentColor = getCol(p1, p2);
                             
                             // Regarder le segment suivant
                             let nextColor = currentColor;
                             if (i < points.length - 2) {
                                 nextColor = getCol(points[i+1], points[i+2]);
                             }
                             
                             if (currentColor !== nextColor) {
                                 const junctionDist = p2.distance - startDist;
                                 
                                 // Zone de transition autour de la jonction
                                 const startTrans = Math.max(0, junctionDist - transitionDistKm);
                                 const endTrans = Math.min(totalSegmentLen, junctionDist + transitionDistKm);
                                 
                                 const ratio1 = startTrans / totalSegmentLen;
                                 const ratio2 = endTrans / totalSegmentLen;
                                 
                                 // Ajout des points de transition si on avance
                                 if (ratio1 > lastRatioAdded && ratio1 < 1) {
                                     expr.push(ratio1);
                                     expr.push(currentColor);
                                     lastRatioAdded = ratio1;
                                 }
                                 
                                 if (ratio2 > lastRatioAdded && ratio2 < 1) {
                                     expr.push(ratio2);
                                     expr.push(nextColor);
                                     lastRatioAdded = ratio2;
                                 }
                             } else if (i === points.length - 2) {
                                 // Dernier segment (fin de variante)
                                 expr.push(1);
                                 expr.push(currentColor);
                             }
                         }
                         
                         // Fermeture sécurité
                         if (expr.length > 3 && expr[expr.length-2] < 1) {
                             expr.push(1);
                             expr.push(expr[expr.length-1]);
                         }
                         
                         gradientExpression = expr;
                         useSlopeColors = true;
                     }
                 } catch (e) {
                     console.warn("Client-side slope gradient failed", e);
                 }
             }

             // Add or Update Variant Layer (Single Line with Gradient)
             if (map.getSource(variantSourceId)) {
                 map.getSource(variantSourceId).setData(lineStringRef.value);
                 
                 // UPDATE STYLE (Gradient) because source data changed
                 if (useSlopeColors && gradientExpression) {
                     map.setPaintProperty(variantLayerId, 'line-gradient', gradientExpression);
                     // S'assurer que line-color ne rentre pas en conflit (bien que gradient soit prioritaire)
                     map.setPaintProperty(variantLayerId, 'line-color', null); // Reset color
                 } else {
                     map.setPaintProperty(variantLayerId, 'line-gradient', null); // Remove gradient
                     map.setPaintProperty(variantLayerId, 'line-color', '#FF9800');
                 }
             } else {
                 // IMPORTANT: lineMetrics: true requis pour line-gradient
                 map.addSource(variantSourceId, { type: 'geojson', data: lineStringRef.value, lineMetrics: true });
                 
                 const beforeId = map.getLayer('comet-layer') ? 'comet-layer' : undefined;
                 
                 const paintProps = {
                     'line-width': baseWidth + 2,
                     'line-opacity': 1.0
                 };
                 
                 if (useSlopeColors && gradientExpression) {
                     paintProps['line-gradient'] = gradientExpression;
                 } else {
                     paintProps['line-color'] = '#FF9800';
                 }
                 
                 map.addLayer({
                     id: variantLayerId,
                     type: 'line',
                     source: variantSourceId,
                     layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
                     paint: paintProps
                 }, beforeId);
             }
             
             // Dim Main Trace Layers (50%)
             const layersToDim = ['trace-complete', 'trace-overlap-aller', 'trace-overlap-retour'];
             layersToDim.forEach(layerId => {
                 if (map.getLayer(layerId)) {
                     map.setPaintProperty(layerId, 'line-opacity', originalOpacity * 0.5);
                 }
             });
        }
        if (map && map.getSource('comet-source')) {
             map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
        }

        // 5. FlyTo
        const startPoint = trackingPointsWithDistanceRef.value[0];
        if (startPoint && startPoint.coordonnee) {
            const flyToDuration = await getSettingValue('Variante/Visualisation/dureeFlytoSegment') || 2.0;
            
            await flyToPromise(map, {
                 center: startPoint.coordonnee,
                 zoom: 17,
                 pitch: 60,
                 bearing: startPoint.editedCap || startPoint.cap || 0,
                 duration: flyToDuration * 1000
            });
            
            // Afficher les messages de départ du segment
            const { atKm0, nearKm0 } = getMessagesForKm0();
            const messagesStart = [...atKm0, ...nearKm0];
            if (messagesStart.length > 0) {
                 console.log(`[Variant] Displaying ${messagesStart.length} messages at start`);
                 // Petite pause pour laisser la caméra se stabiliser
                 setTimeout(() => displayMessagesWithFade(messagesStart, 800), 200);
            }
        }

        showSnackbar(`Segment chargé: ${getSegmentTitle(modification)}`, "success");

    } catch (e) {
        console.error("Failed to load variant segment", e);
        showSnackbar(`Erreur: ${e.message}`, 'error');
    } finally {
        isInitializing.value = false;
    }
};

const getSegmentIcon = (type) => {
    switch (type) {
        case 'DEPART_DEPORTE': return 'mdi-flag-checkered';
        case 'ARRIVEE_REPORTEE': return 'mdi-flag-checkered';
        case 'SEGMENT_DEVIATION': return 'mdi-map-marker-path';
        default: return 'mdi-map-marker';
    }
};



/**
 * Loads ALL segments of a variant and stitches them into a single timeline.
 */
const loadFullVariant = async (variantId, variantStructure) => {
    // Save Main Trace State if switching from Main Trace
    if (!currentVariantId.value) {
        mainTraceState.value = {
            lineString: lineStringRef.value,
            trackingPoints: [...trackingPointsWithDistanceRef.value],
            totalDistance: totalDistanceRef.value,
            totalDuration: totalDurationAt1xRef.value,
            pauseIncrements: [...pauseIncrements.value],
            flytoEvents: JSON.parse(JSON.stringify(flytoEvents.value)),
            rangeEvents: [...rangeEvents.value],
            accumulatedTime: Number(accumulatedTime),
            isPaused: isPaused.value
        };
        console.log(`[STATE SAVE] Main Trace Saved: Time=${accumulatedTime}, Dist=${totalDistanceRef.value}`);
    } else if (mainTraceState.value && !isMultisegmentVariant.value) {
         // Fallback legacy update
         mainTraceState.value.accumulatedTime = Number(accumulatedTime);
    }
    
    // Enable Comparison Mode and reference Main Trace
    isComparisonModeRef.value = true;
    
    // Robustly set Main Trace Points
    if (mainTraceState.value && mainTraceState.value.trackingPoints && mainTraceState.value.trackingPoints.length > 0) {
         mainTraceComparisonPointsRef.value = mainTraceState.value.trackingPoints;
    } else {
         // Fallback: Use current points if state not yet saved (e.g. first load)
         // Assuming we are on Main Trace before this switch
         if (trackingPointsWithDistanceRef.value && trackingPointsWithDistanceRef.value.length > 0) {
             console.log("[loadFullVariant] capturing current points as Main Trace");
             mainTraceComparisonPointsRef.value = [...trackingPointsWithDistanceRef.value];
             
             // Ensure state is saved for anchors calculation later
             if (!mainTraceState.value) {
                 mainTraceState.value = {
                     trackingPoints: mainTraceComparisonPointsRef.value,
                     // Other fields might be missing but points are critical for AltitudeSVG
                 };
             } else if (!mainTraceState.value.trackingPoints) {
                 mainTraceState.value.trackingPoints = mainTraceComparisonPointsRef.value;
             }
         } else {
             console.warn("[loadFullVariant] No Main Trace points found!");
         }
    }

    currentVariantId.value = variantId;
    isMultisegmentVariant.value = true;
    activeVariantSegments.value = [];
    isInitializing.value = true;
    currentSegmentType.value = null; 
    
    if (animationFrameId) {
        cancelAnimationFrame(animationFrameId);
        animationFrameId = null;
    }

    console.log("[loadFullVariant] Structure:", variantStructure);

    try {
        const segmentsToLoad = [];
        const sortedMods = getVariantSegments(variantStructure);
        
        sortedMods.forEach(mod => {
             let suffix = "";
             if (mod.type === 'DEPART_DEPORTE') suffix = "DEPART";
             else if (mod.type === 'ARRIVEE_REPORTEE') suffix = "ARRIVEE";
             else if (mod.type === 'SEGMENT_DEVIATION') suffix = `SEGMENT_${mod.originalIndex}`;
             
             segmentsToLoad.push({ 
                 type: mod.type, 
                 index: mod.originalIndex, 
                 suffix: suffix, 
                 meta: mod 
             });
        });

        console.log("[loadFullVariant] Segments to load:", segmentsToLoad.length, segmentsToLoad);

        const promises = segmentsToLoad.map(async (seg) => {
            const trackFile = `tracking_${variantId}_${seg.suffix}.json`;
            const lsFile = `lineString_${variantId}_${seg.suffix}.json`;
            const evtVarId = `${variantId}_${seg.suffix}`;
            
            try {
                const [rawTrack, rawLS, events] = await Promise.all([
                    invoke('read_tracking_file', { circuitId: props.circuitId, filename: trackFile }),
                    invoke('read_line_string_file', { circuitId: props.circuitId, filename: lsFile }),
                    invoke('get_events', { circuitId: props.circuitId, variantId: evtVarId })
                ]);
                
                let processed = { processedPoints: [], totalDistanceKm: 0 };
                if (rawTrack && rawTrack.length > 0) {
                     processed = await invoke('process_tracking_data', {
                        lineStringGeojson: rawLS,
                        trackingPointsJs: rawTrack
                    });
                }
                
                return { 
                    ...seg, 
                    rawTrack, 
                    rawLS, 
                    events, 
                    processedPoints: processed.processedPoints, 
                    segmentDistKm: processed.totalDistanceKm 
                };
            } catch (e) {
                console.error(`Failed to load segment ${seg.suffix}`, e);
                return null;
            }
        });
        
        const results = await Promise.all(promises);
        const validResults = results.filter(r => r !== null);
        console.log("[loadFullVariant] Valid results:", validResults.length);
        
        let globalDistKm = 0;
        const allPoints = [];
        const visualCoordinates = [];
        const mergedFlytos = {};
        const mergedPauses = [];
        const mergedRanges = [];
        
        activeVariantSegments.value = [];
        
        validResults.forEach(seg => {
            const startDist = globalDistKm;
            const adjustedPoints = seg.processedPoints.map(p => ({
                ...p,
                distance: p.distance + startDist, 
            }));
            
            allPoints.push(...adjustedPoints);
            
            let segmentCoords = [];
            if (seg.rawLS) {
                if (seg.rawLS.type === 'FeatureCollection' && seg.rawLS.features && seg.rawLS.features.length > 0) {
                     segmentCoords = seg.rawLS.features[0].geometry.coordinates;
                } else if (seg.rawLS.type === 'Feature' && seg.rawLS.geometry) {
                     segmentCoords = seg.rawLS.geometry.coordinates;
                } else if (seg.rawLS.type === 'LineString' && seg.rawLS.coordinates) {
                     segmentCoords = seg.rawLS.coordinates;
                } else if (seg.rawLS.geometry && seg.rawLS.geometry.coordinates) {
                     segmentCoords = seg.rawLS.geometry.coordinates;
                }
            }
            
            if (segmentCoords && segmentCoords.length > 0) {
                visualCoordinates.push(segmentCoords);
            } else {
                 console.warn(`[loadFullVariant] No coordinates found for segment ${seg.suffix}`);
            }
            
            const len = seg.segmentDistKm;
            // Calculate Main Trace Anchors
            let mainStart = 0;
            let mainEnd = 0;
            // Use saved main trace state if available, or current refs if not yet swapped (but likely swapped/cleared)
            const mp = mainTraceState.value?.trackingPoints || [];
            
            


            
            if (mp.length > 0) {
                 if (seg.type === 'DEPART_DEPORTE') {
                     mainStart = 0;
                     // DEPART: anchorIndexOnMaster = Where it joins (End of Departe)
                     const idx = seg.meta?.anchorIndexOnMaster ?? seg.meta?.anchorEnd?.index ?? seg.meta?.endPointIndex;
                     mainEnd = mp[idx]?.distance || 0;
                     
                 } else if (seg.type === 'ARRIVEE_REPORTEE') {
                     // ARRIVEE: anchorIndexOnMaster = Where it leaves (Start of Arrivee)
                     const idx = seg.meta?.anchorIndexOnMaster ?? seg.meta?.anchorStart?.index ?? seg.meta?.startPointIndex;
                     mainStart = mp[idx]?.distance || 0;
                     mainEnd = mp[mp.length - 1]?.distance || 0;

                 } else {
                     // SEGMENT: Expects anchorStart/End objects usually
                     const idxStart = seg.meta?.anchorStart?.index ?? seg.meta?.startPointIndex;
                     const idxEnd = seg.meta?.anchorEnd?.index ?? seg.meta?.endPointIndex;
                     mainStart = mp[idxStart]?.distance || 0;
                     mainEnd = mp[idxEnd]?.distance || 0;
                 }
            }

            activeVariantSegments.value.push({
                type: seg.type,
                index: seg.index,
                uiIndex: activeVariantSegments.value.length,
                startDistKm: startDist,
                endDistKm: startDist + len,
                lengthKm: len,
                firstPoint: adjustedPoints[0] || null,
                coordinates: segmentCoords, 
                points: adjustedPoints,
                // Anchors for Profile Comparison
                mainStartDistKm: mainStart,
                mainEndDistKm: mainEnd
            });
            
            const pointOffset = allPoints.length - adjustedPoints.length;

             if (seg.events?.pointEvents) {
                 const pEvents = seg.events.pointEvents;
                 Object.keys(pEvents).forEach(incStr => {
                     const inc = Number(incStr);
                     const globalInc = pointOffset + inc;
                     
                     if (pEvents[inc].some(e => e.type === 'Pause')) {
                         mergedPauses.push(globalInc);
                     }
                     const flyto = pEvents[inc].find(e => e.type === 'Flyto');
                     if (flyto) {
                         mergedFlytos[globalInc] = flyto.data;
                     }
                 });
             }
             if (seg.events?.rangeEvents) {
                 seg.events.rangeEvents.forEach(re => {
                     mergedRanges.push({
                         ...re,
                         start: re.start + pointOffset,
                         end: re.end + pointOffset
                     });
                 });
             }
             
            globalDistKm += len;
        });
        
        trackingPointsWithDistanceRef.value = allPoints;
        totalDistanceRef.value = globalDistKm;
        
        totalDurationAt1xRef.value = globalDistKm * (animationSpeed?.value || 10);
        
        flytoEvents.value = mergedFlytos;
        pauseIncrements.value = mergedPauses;
        rangeEvents.value = mergedRanges;

        // Prepare Slope Colors if needed
        let slopeColors = null;
        if (colorTraceBySlope.value) {
             try {
                 slopeColors = {
                    TrancheNegative: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative')),
                    Tranche1: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1')),
                    Tranche2: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2')),
                    Tranche3: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3')),
                    Tranche4: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4')),
                    Tranche5: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5')),
                 };
             } catch (e) {}
        }

        // Helper for Gradient
        const getCol = (p1, p2) => {
             if (!slopeColors) return '#FF00FF';
             const distM = (p2.distance - p1.distance) * 1000;
             const elev = (p2.altitude || 0) - (p1.altitude || 0);
             const slope = distM > 0 ? (elev / distM) * 100 : 0;
             if (slope < 0) return slopeColors.TrancheNegative || '#0000FF';
             if (slope < 3) return slopeColors.Tranche1 || '#00FF00';
             if (slope < 6) return slopeColors.Tranche2;
             if (slope < 9) return slopeColors.Tranche3;
             if (slope < 12) return slopeColors.Tranche4;
             return slopeColors.Tranche5;
        };

        const multiLineString = {
            type: "Feature",
            geometry: {
                type: "MultiLineString",
                coordinates: visualCoordinates
            },
            properties: {}
        };
        lineStringRef.value = multiLineString;
        
        // isComparisonModeRef is set to true at start of function

        accumulatedTime = 0;
        currentDistanceInMeters.value = 0;
        lastTimestamp = 0;
        isPaused.value = true;
        isAnimationFinished.value = false;
        
        if (map) {
             // 1. Cleanup old layers/sources
             variantLayerIds.value.forEach(id => {
                 if (map.getLayer(id)) map.removeLayer(id);
                 // Source ID assumption: id.replace('-layer', '')
                 const sourceId = id.replace('-layer', '');
                 if (map.getSource(sourceId)) map.removeSource(sourceId);
             });
             variantLayerIds.value = [];
             
             // Legacy cleanup
             if (map.getLayer('variant-line-layer')) map.removeLayer('variant-line-layer');
             if (map.getSource('variant-line')) map.removeSource('variant-line');

             // 2. Create new layers per segment
             activeVariantSegments.value.forEach((seg, index) => {
                 if (!seg.coordinates || seg.coordinates.length < 2) return;
                 
                 const sourceId = `variant-seg-${index}`;
                 const layerId = `variant-seg-layer-${index}`;
                 
                 const segGeoJSON = {
                     type: 'Feature',
                     geometry: { type: 'LineString', coordinates: seg.coordinates },
                     properties: {}
                 };
                 
                 // Build Gradient for this segment
                 let gradientExpression = null;
                 if (slopeColors && seg.points && seg.points.length > 1) {
                     const expr = ['interpolate', ['linear'], ['line-progress']];
                     const segStartDist = seg.points[0].distance;
                     const segLen = seg.points[seg.points.length-1].distance - segStartDist;
                     
                     let lastRatio = -1;
                     const pushStop = (r, color) => {
                         let ratio = Math.max(0, Math.min(1, r));
                         // Ensure strictly ascending
                         if (ratio <= lastRatio) ratio = lastRatio + 0.00001;
                         if (ratio > 1) ratio = 1;
                         // If still equal to lastRatio (precision limit), skip? 
                         // Mapbox needs strict >. 
                         if (ratio > lastRatio) {
                             expr.push(ratio);
                             expr.push(color);
                             lastRatio = ratio;
                         }
                     };

                     pushStop(0, getCol(seg.points[0], seg.points[1] || seg.points[0]));
                     
                     for (let i = 0; i < seg.points.length - 1; i++) {
                         const p1 = seg.points[i];
                         // Use center of segment or start? Logic used p1 start.
                         if (segLen > 0) {
                             const rawRatio = (p1.distance - segStartDist) / segLen;
                             // Avoid 0 duplicates
                             if (rawRatio > 0.0001) {
                                 const col = getCol(p1, seg.points[i+1]);
                                 pushStop(rawRatio, col);
                             }
                         }
                     }
                     // Always ensure 1 is reached
                     pushStop(1, getCol(seg.points[seg.points.length-2], seg.points[seg.points.length-1]));
                     
                     gradientExpression = expr;
                 }

                 if (map.getSource(sourceId)) {
                     map.getSource(sourceId).setData(segGeoJSON);
                 } else {
                     map.addSource(sourceId, { type: 'geojson', data: segGeoJSON, lineMetrics: true });
                 }
                 
                 const paint = {
                     'line-width': traceWidth.value || 4
                 };
                 
                 if (gradientExpression) {
                     paint['line-gradient'] = gradientExpression;
                     paint['line-color'] = 'rgba(255, 255, 255, 0)'; 
                     paint['line-opacity'] = 1;
                 } else {
                     paint['line-color'] = '#FF00FF';
                 }
                 
                 if (!map.getLayer(layerId)) {
                     map.addLayer({
                        id: layerId,
                        type: 'line',
                        source: sourceId,
                        layout: { 'line-join': 'round', 'line-cap': 'round' },
                        paint: paint
                    });
                 } else {
                    if (gradientExpression) {
                        map.setPaintProperty(layerId, 'line-gradient', gradientExpression);
                        map.setPaintProperty(layerId, 'line-color', 'rgba(255, 255, 255, 0)');
                    } else {
                        map.setPaintProperty(layerId, 'line-gradient', null);
                        map.setPaintProperty(layerId, 'line-color', '#FF00FF');
                    }
                 }
                 if (!variantLayerIds.value.includes(layerId)) {
                     variantLayerIds.value.push(layerId);
                 }
             });
             
             // Clear Comet
             if (map.getSource('comet-source')) {
                 map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
             }

             // Restore Opacity Dimming for Main Trace
             const layersToDim = ['trace-complete', 'trace-overlap-aller', 'trace-overlap-retour'];
             layersToDim.forEach(lid => {
                 if (map.getLayer(lid)) map.setPaintProperty(lid, 'line-opacity', 0.5);
             });
             
             // Ensure Comet is on Top
             if (map.getLayer('comet-layer')) {
                 map.moveLayer('comet-layer');
             }
             
             const startPt = trackingPointsWithDistanceRef.value[0];
             if (startPt) {
                 await flyToPromise(map, {
                     center: startPt.coordonnee,
                     zoom: startPt.editedZoom ?? 16,
                     pitch: startPt.editedPitch ?? 60,
                     bearing: startPt.editedCap ?? 0,
                     duration: 1500
                 });
             }
        }
          
             // Initialize current segment to the first one
             if (activeVariantSegments.value.length > 0) {
                 const firstSeg = activeVariantSegments.value[0];
                 currentSegmentIndex.value = firstSeg.index;
                 currentSegmentType.value = firstSeg.type;
             }
        
    } catch (e) {
        console.error("Full Variant Load Failed", e);
        showSnackbar("Erreur chargement variant complet", "error");
    } finally {
        isInitializing.value = false;
    }
};

const focusSegment = async (arrayIndex) => {
    if (arrayIndex < 0 || arrayIndex >= activeVariantSegments.value.length) return;
    const seg = activeVariantSegments.value[arrayIndex];
    
    currentSegmentIndex.value = seg.index; 
    currentSegmentType.value = seg.type;
    
    // Set Distance/Time relative to global timeline
    currentDistanceInMeters.value = seg.startDistKm * 1000;
    
    const totalDist = totalDistanceRef.value;
    const ratio = totalDist > 0 ? (seg.startDistKm / totalDist) : 0;
    const totalDur = totalDurationAt1xRef.value;
    accumulatedTime = totalDur * ratio;
    
    // Manually force updates if paused
    if (isPaused.value) {
        lastTimestamp = 0; // Reset delta logic
    }
    
    if (seg.firstPoint) {
         await flyToPromise(map, {
             center: seg.firstPoint.coordonnee,
             zoom: seg.firstPoint.editedZoom ?? 16,
             pitch: seg.firstPoint.editedPitch ?? 60,
             bearing: seg.firstPoint.editedCap ?? 0,
             duration: 1000
         });
    }
};

const loadMainTrace = async () => {
    if (!mainTraceState.value) return;
    
    isInitializing.value = true;
    try {
        // Stop any running animation loop immediately
        if (animationFrameId) {
            cancelAnimationFrame(animationFrameId);
            animationFrameId = null;
        }

        currentVariantId.value = null;
        currentSegmentType.value = null;
        currentSegmentIndex.value = null;

        // --- Reset Comparison State ---
        isComparisonModeRef.value = false;
        mainTraceComparisonPointsRef.value = [];
        mainTraceComparisonStartRef.value = 0;
        mainTraceComparisonEndRef.value = 0;

        lineStringRef.value = mainTraceState.value.lineString;
        trackingPointsWithDistanceRef.value = [...mainTraceState.value.trackingPoints];
        totalDistanceRef.value = mainTraceState.value.totalDistance;
        totalDurationAt1xRef.value = mainTraceState.value.totalDuration;
        pauseIncrements.value = [...mainTraceState.value.pauseIncrements];
        flytoEvents.value = JSON.parse(JSON.stringify(mainTraceState.value.flytoEvents));
        rangeEvents.value = [...mainTraceState.value.rangeEvents];
        
        // Restaurer le temps et l'état
        accumulatedTime = Number(mainTraceState.value.accumulatedTime);
        lastTimestamp = 0;
        isAnimationFinished.value = false;
        
        isMultisegmentVariant.value = false; // Reset flag
        
        // CRITIQUE : on désactive la détection de mouvement avant le FlyTo de repositionnement
        cameraMovedDuringPause.value = false; 

        console.log(`[STATE RESTORE] Back to Main: Time=${accumulatedTime}ms, TotalDist=${totalDistanceRef.value}`);

        // Update Map - Restore Main Trace Visualization
        if (map) {
             // Cleanup Dynamic Variant Layers
             variantLayerIds.value.forEach(id => {
                 if (map.getLayer(id)) map.removeLayer(id);
                 const sourceId = id.replace('-layer', ''); // Approximation, assume convention
                 // Actually my convention was sourceId = variant-seg-i, layerId = variant-seg-layer-i.
                 // layerId.replace('-layer', '') -> variant-seg-i. Correct.
                 if (map.getSource(sourceId)) map.removeSource(sourceId);
             });
             variantLayerIds.value = [];

             // Remove Legacy Variant Layer/Source
             if (map.getLayer('variant-line-layer')) map.removeLayer('variant-line-layer');
             if (map.getSource('variant-line')) map.removeSource('variant-line');
             
             // Clear Comet
             if (map.getSource('comet-source')) {
                 map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
             }
             
             // Restore Opacity
             const originalOpacity = traceOpacity.value ?? 1.0;
             const originalWidth = traceWidth.value ?? 4;
             const layersToRestore = ['trace-complete', 'trace-overlap-aller', 'trace-overlap-retour'];
             layersToRestore.forEach(layerId => {
                 if (map.getLayer(layerId)) {
                     map.setPaintProperty(layerId, 'line-opacity', originalOpacity);
                     map.setPaintProperty(layerId, 'line-width', originalWidth);
                 }
             });
        }
        
        // FlyTo position sur la trace maîtresse
        const currentTotalDur = totalDurationAt1xRef.value || 1;
        const phase = Math.min(accumulatedTime / currentTotalDur, 1);
        const dist = totalDistanceRef.value * phase;
        
        // Mettre à jour l'affichage de la distance immédiatement
        distanceDisplay.value = dist.toFixed(2);
        currentDistanceInMeters.value = dist * 1000;
        
        console.log(`[REPOSITIONING] Phase=${phase.toFixed(4)}, Target KM=${dist.toFixed(3)}`);

        let pIdx = 0;
        if (trackingPointsWithDistanceRef.value.length > 0) {
            for (let i = trackingPointsWithDistanceRef.value.length - 1; i >= 0; i--) {
                if (trackingPointsWithDistanceRef.value[i].distance <= dist) {
                    pIdx = i;
                    break;
                }
            }
            const p = trackingPointsWithDistanceRef.value[pIdx];
            if (p) {
                console.log(`[FLYTO START] Index=${pIdx}, Coord=${p.coordonnee}`);
                await flyToPromise(map, {
                    center: p.coordonnee,
                    zoom: p.editedZoom ?? p.zoom ?? 17,
                    pitch: p.editedPitch ?? p.pitch ?? 60,
                    bearing: p.editedCap || p.cap || 0,
                    duration: 1500
                });
                
                // On force la capture de cette position comme "Position de Pause" officielle
                pausedCameraOptions.value = {
                    center: map.getCenter(),
                    zoom: map.getZoom(),
                    pitch: map.getPitch(),
                    bearing: map.getBearing(),
                };
                cameraMovedDuringPause.value = false; // On re-confirme que ce n'est pas un mouvement utilisateur
            }
        }
        
        // Enfin, on restaure l'état de pause
        isPaused.value = mainTraceState.value.isPaused;
        
        // If not paused, render loop needs to be jump-started (as isPaused was potentially true before)
        if (!isPaused.value) {
            lastTimestamp = 0; // Reset for new loop
            requestAnimationFrame(animate); 
        }

        console.log(`[RESTORE END] isPaused=${isPaused.value}, final Km=${(totalDistanceRef.value * (accumulatedTime / (totalDurationAt1xRef.value || 1))).toFixed(2)}`);
        
        showSnackbar("Retour à la trace maîtresse", "info");

    } catch (e) {
        console.error("Failed to restore main trace", e);
    } finally {
        isInitializing.value = false;
    }
};

const fetchVariants = async () => {
  try {
    const result = await invoke('get_variants', { circuitId: props.circuitId });
    // Fetch details for each to build the menu (needed for getVariantSegments)
    const detailsPromises = result.map(v => invoke('get_variant_details', { circuitId: props.circuitId, variantId: v.id }));
    const detailsResults = await Promise.all(detailsPromises);
    
    variants.value = result.map((v, index) => ({
      ...v,
      details: detailsResults[index]
    }));
  } catch (e) {
    console.error("Failed to fetch variants", e);
  }
};

onMounted(async () => {
    await fetchVariants();
});

// Function to send the current state to the backend
const sendVisualizeViewStateUpdate = async () => {
    try {
        await invoke('update_visualize_view_state', {
            state: {
                isControlsCardVisible: isControlsCardVisible.value,
                isAltitudeVisible: isAltitudeVisible.value,
                isCommuneWidgetVisible: isCommuneWidgetVisible.value,
                isDistanceDisplayVisible: isDistanceDisplayVisible.value,
                isStaticWeatherVisible: isWeatherInfoVisible.value,
                isDynamicWeatherVisible: isCompassVisible.value,
                currentSpeed: currentSpeed.value,
                animationState: animationState.value
            }
        });
    } catch (error) {
        console.error("Failed to broadcast visual state update:", error);
    }
};

// Watch for changes and send updates
watch(isControlsCardVisible, sendVisualizeViewStateUpdate);
watch(isAltitudeVisible, sendVisualizeViewStateUpdate);
watch(isCommuneWidgetVisible, sendVisualizeViewStateUpdate);
watch(isDistanceDisplayVisible, sendVisualizeViewStateUpdate);
watch(isWeatherInfoVisible, sendVisualizeViewStateUpdate);
watch(isCompassVisible, sendVisualizeViewStateUpdate);

// Update remote control when initialization finishes
watch(isInitializing, (newVal) => {
    if (!newVal) {
        sendVisualizeViewStateUpdate();
    }
});


const centerEurope = computed(() => getSettingValue('Visualisation/Lancement/centerEurope'));
const zoomEurope = computed(() => getSettingValue('Visualisation/Lancement/zoomEurope'));
const durationEuropeToTrace = computed(() => {
    const val = getSettingValue('Visualisation/Lancement/durationEuropeToTrace');
    return val > 100 ? val : val * 1000;
});
const pauseBeforeStart = computed(() => {
    const val = getSettingValue('Visualisation/Lancement/pauseBeforeStart');
    return val > 100 ? val : val * 1000;
});
const durationTraceToStart = computed(() => {
    const val = getSettingValue('Visualisation/Lancement/durationTraceToStart');
    return val > 100 ? val : val * 1000;
});


// Helper function to promisify map.flyTo
function flyToPromise(mapInstance, options) {
    return new Promise(resolve => {
        mapInstance.flyTo(options);
        mapInstance.once('moveend', () => resolve());
    });
}




const triggeredPauseIncrement = ref(null);
const triggeredFlytoIncrement = ref(null);
const isFlytoActive = ref(false);
const preFlytoCameraOptions = ref(null);
const globalTraceCameraOptions = ref(null);

async function executeFlytoSequence(flytoData) {
    isWatcherActive = false; // Désactivation du watcher principal
    isFlytoActive.value = true;
    isPaused.value = true; // Pause de la boucle d'animation principale

    // --- Phase 1: Vol vers la cible ---
    animationState.value = 'Survol_Evenementiel';
    preFlytoCameraOptions.value = {
        center: map.getCenter(),
        zoom: map.getZoom(),
        pitch: map.getPitch(),
        bearing: map.getBearing(),
    };
    //showSnackbar('Début du survol programmé...', 'info');
    
    // Ajuster la durée en fonction de la vitesse, avec une durée minimale.
    // Ajuster la durée en fonction de la vitesse, avec une durée minimale.
    // Heuristique : si la durée est > 100, on considère qu'elle est déjà en ms (rétrocompatibilité).
    // console.log(`[FlyTo] Execute Sequence for increment. Dist adjustment?`);
    const durationInMs = flytoData.duree > 100 ? flytoData.duree : flytoData.duree * 1000;
    const durationToTarget = Math.max(200, durationInMs / currentSpeed.value);

    // Give time for Vue to process the cursor update (AltitudeSVG) before freezing/flying
    console.log('[FlyTo] Waiting 50ms for cursor update...');
    await new Promise(resolve => setTimeout(resolve, 50));

    console.log('[FlyTo] STARTING flyToPromise (Camera Should Move now)');
    await flyToPromise(map, {
        center: flytoData.coord,
        zoom: flytoData.zoom,
        pitch: flytoData.pitch,
        bearing: flytoData.cap,
        duration: durationToTarget,
    });
    console.log('[FlyTo] FINISHED flyToPromise');

    // --- Phase 2: En pause sur la cible ---
    animationState.value = 'En_Pause';
    //showSnackbar('Survol en pause. Appuyez sur Play pour continuer.', 'info');

    // Activer l'interaction cartographique pendant la pause Flyto
    if (map) {
        map.interactive = true;
        map.dragRotate.enable();
        map.dragPan.enable();
        map.scrollZoom.enable({ around: 'center' });
        
        // Détecter si l'utilisateur bouge la caméra pour le vol de retour (optionnel mais propre)
        map.on('move', onMapInteraction);
        map.on('zoom', onMapInteraction);
        map.on('pitch', onMapInteraction);
        map.on('rotate', onMapInteraction);
        map.on('zoom', handleMapZoom);
    }

    // Attendre que l'utilisateur appuie sur "Play"
    await new Promise(resolve => {
        const unwatch = watch(() => isPaused.value, (newVal, oldVal) => {
            if (oldVal === true && newVal === false) {
                unwatch();
                resolve();
            }
        });
    });

    // Désactiver l'interaction avant le vol de retour
    if (map) {
        map.interactive = false;
        map.dragRotate.disable();
        map.dragPan.disable();
        map.scrollZoom.disable();
        
        map.off('move', onMapInteraction);
        map.off('zoom', onMapInteraction);
        map.off('pitch', onMapInteraction);
        map.off('rotate', onMapInteraction);
        map.off('zoom', handleMapZoom);
    }

    // --- Phase 3: Vol de retour vers la trace ---
    animationState.value = 'Survol_Evenementiel';
    //showSnackbar('Retour à la trace...', 'info');

    // Ajuster également la durée du vol de retour.
    const durationBackToTrace = Math.max(200, flytoData.duree / currentSpeed.value);

    await flyToPromise(map, {
        ...preFlytoCameraOptions.value,
        duration: durationBackToTrace,
    });

    // --- Phase 4: Reprise de l'animation ---
    isFlytoActive.value = false;
    isPaused.value = false; // On relance l'animation
    animationState.value = 'En_Animation'; // On définit l'état final explicitement
    lastTimestamp = 0; // On réinitialise le timer pour une reprise propre
    isWatcherActive = true; // Réactivation du watcher principal
}

  const animate = (timestamp) => {
  if (isInitializing.value || isResuming.value) {
      if (map) map.triggerRepaint();
      animationFrameId = requestAnimationFrame(animate);
      return;
  }

  if (isFlytoActive.value) {
    if (map) map.triggerRepaint();
    lastTimestamp = timestamp; 
    animationFrameId = requestAnimationFrame(animate);
    return;
  }

  // New, robust timing logic
  if ((isPaused.value && !isRewinding.value) || isTransitioning.value) {
    if (isTransitioning.value) {
        console.log('[animate] BLOCKED by isTransitioning');
    }
    lastTimestamp = 0; // Invalidate lastTimestamp while paused
    animationFrameId = requestAnimationFrame(animate);
    return;
  }
  if (lastTimestamp === 0) {
    // First frame after start or pause, just set the timestamp and skip a frame
    lastTimestamp = timestamp;
    animationFrameId = requestAnimationFrame(animate);
    return;
  }

  // Cap deltaTime to prevent large jumps when the tab is inactive
  const deltaTime = Math.min(timestamp - lastTimestamp, 100);
  lastTimestamp = timestamp;

  if (isRewinding.value) {
      accumulatedTime = Math.max(0, accumulatedTime - (deltaTime * 2 * currentSpeed.value));
  }
  else {
      accumulatedTime += deltaTime * currentSpeed.value;
  }

  const totalDur = totalDurationAt1xRef.value || 0;
  const phase = totalDur > 0 ? Math.min(accumulatedTime / totalDur, 1) : 1;
  const distanceTraveled = totalDistanceRef.value * phase;
  distanceDisplay.value = distanceTraveled.toFixed(2);
  currentDistanceInMeters.value = distanceTraveled * 1000;

  // --- Variant Segment Auto-Pause Logic ---
  if (isMultisegmentVariant.value && !isRewinding.value && activeVariantSegments.value.length > 0) {
      // Determine current UI index from currentSegmentIndex
      let uiIndex = activeVariantSegments.value.findIndex(s => s.index === currentSegmentIndex.value);
      
      // If lost sync, re-sync based on distance
      if (uiIndex === -1) {
           uiIndex = activeVariantSegments.value.findIndex(s => distanceTraveled >= s.startDistKm && distanceTraveled < s.endDistKm);
           if (uiIndex >= 0) {
               currentSegmentIndex.value = activeVariantSegments.value[uiIndex].index;
               currentSegmentType.value = activeVariantSegments.value[uiIndex].type;
           }
      }

      if (uiIndex >= 0) {
          const currentSeg = activeVariantSegments.value[uiIndex];
          // Check if we reached the end of this segment
          // Tolerance: 5 meters or 1 frame? 
          // If we passed it, we pause.
          if (distanceTraveled >= currentSeg.endDistKm - 0.005) { 
               // Only pause if we are NOT already at the very end of the whole track (which is handled by phase >= 1)
               // and if we haven't already processed this pause/segment switch for this location.
               // Simply Pausing works. User clicks Play -> isPaused becomes false.
               // But we need to switch Segment Index so the NEXT frame doesn't pause again immediately.
               
               const nextSeg = activeVariantSegments.value[uiIndex + 1];
               if (nextSeg) {
                   console.log('[Auto-Pause] End of segment reached. Pausing and switching to nextSeg.index=', nextSeg.index);
                   isPaused.value = true;
                   currentSegmentIndex.value = nextSeg.index;
                   currentSegmentType.value = nextSeg.type;
                   // Snap time to exactly the boundary to avoid missing/skipping?
                   // accumulatedTime = (nextSeg.startDistKm / totalDistanceRef.value) * totalDur;
                   showSnackbar("Fin du segment. Appuyez sur Run pour continuer.", "info");
               }
          }
      }
  }

  // --- Refonte Phase 8 : Bascule Globale Aller -> Retour ---
  // On ne gère plus des zones individuelles, mais une phase globale.
  // Tant qu'on n'a pas dépassé le dernier segment "Aller", on affiche le calque Aller.
  // Ensuite, on bascule sur le calque Retour pour toujours.
  
  if (segmentMetadata.value && segmentMetadata.value.overlappingZones && segmentMetadata.value.overlappingZones.length > 0) {
      // Trouver le point de bascule ultime (le max de tous les aller_end_km)
      // Correction: On utilise KM et pas Index, car Index Backend (LineString) != Index Frontend (Tracking)
      
      // Nouvelle logique : Bascule dynamique basée sur la position actuelle
      // Si on est dans un segment identifié comme "Retour", on active la vue Retour.
      // Sinon, par défaut, on reste en vue "Aller".
      const isPhaseRetour = segmentMetadata.value.overlappingZones.some(z => 
          distanceTraveled >= z.retourStartKm && distanceTraveled <= z.retourEndKm
      );
      
      // Variable pour le log (pour garder une info pertinente)
      // On cherche la zone retour active pour l'afficher dans les logs si besoin
      const currentRetourZone = isPhaseRetour ? segmentMetadata.value.overlappingZones.find(z => 
          distanceTraveled >= z.retourStartKm && distanceTraveled <= z.retourEndKm
      ) : null;
      
      if (map) {
          if (isPhaseRetour) {
              // Mode Retour : Afficher uniquement l'overlay retour (qui contient tout sauf aller_overlap)
              if (map.getLayer('trace-overlap-aller') && map.getLayoutProperty('trace-overlap-aller', 'visibility') !== 'none') {
                  map.setLayoutProperty('trace-overlap-aller', 'visibility', 'none');
              }
              if (map.getLayer('trace-overlap-retour') && map.getLayoutProperty('trace-overlap-retour', 'visibility') !== 'visible') {
                  map.setLayoutProperty('trace-overlap-retour', 'visibility', 'visible');
                  if (currentRetourZone) {
                      console.log(`🔀 [Visualize] Bascule Phase RETOUR (Zone ${currentRetourZone.zoneId}: Km ${distanceTraveled.toFixed(2)})`);
                  }
              }
          } else {
              // Mode Aller : Afficher uniquement l'overlay aller (qui contient tout sauf retour_overlap)
              if (map.getLayer('trace-overlap-retour') && map.getLayoutProperty('trace-overlap-retour', 'visibility') !== 'none') {
                  map.setLayoutProperty('trace-overlap-retour', 'visibility', 'none');
              }
              if (map.getLayer('trace-overlap-aller') && map.getLayoutProperty('trace-overlap-aller', 'visibility') !== 'visible') {
                  map.setLayoutProperty('trace-overlap-aller', 'visibility', 'visible');
                  // console.log(`🔀 [Visualize] Phase ALLER`);
              }
          }
      }
  } else {
      // Pas de metadata (trace simple) : "Aller" par défaut
      if (map && map.getLayer('trace-overlap-aller')) {
           if (map.getLayoutProperty('trace-overlap-aller', 'visibility') !== 'visible') {
              map.setLayoutProperty('trace-overlap-aller', 'visibility', 'visible');
           }
      }
  }

  const cometLengthKm = cometLength.value / 1000;
  const startDistance = Math.max(0, distanceTraveled - cometLengthKm);
  if (distanceTraveled > startDistance) {

        // Comet Logic
        try {
            if (isMultisegmentVariant.value && activeVariantSegments.value.length > 0) {
                 const currentSeg = activeVariantSegments.value.find(s => distanceTraveled >= s.startDistKm && distanceTraveled < s.endDistKm) 
                                    || activeVariantSegments.value[activeVariantSegments.value.length-1];
                 
                 if (currentSeg && currentSeg.coordinates && currentSeg.coordinates.length > 1) {
                      const segmentLine = turf.lineString(currentSeg.coordinates);
                      // Calculate local distance
                      const localDist = distanceTraveled - currentSeg.startDistKm;
                      const localStart = Math.max(0, localDist - cometLengthKm);
                      const localEnd = Math.min(localDist, currentSeg.lengthKm);
                      
                      if (localEnd > localStart) {
                           const cometSlice = turf.lineSliceAlong(segmentLine, localStart, localEnd, { units: 'kilometers' });
                           if (map.getSource('comet-source')) map.getSource('comet-source').setData(cometSlice);
                      } else {
                           if (map.getSource('comet-source')) map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
                      }
                 }
            } else {
                // Standard Logic
                const cometSlice = turf.lineSliceAlong(lineStringRef.value, startDistance, distanceTraveled, { units: 'kilometers' });
                if (map.getSource('comet-source')) {
                    map.getSource('comet-source').setData(cometSlice);
                }
            }
        } catch (e) {
            console.warn("Comet slice error:", e);
        }
  }
  else {
      if (map && map.getSource('comet-source')) {
          map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
      }
  }

  // --- Event and Camera Logic ---
  if (!trackingPointsWithDistanceRef.value || trackingPointsWithDistanceRef.value.length < 2) return;

  // 1. Find the current, real-time increment on the dense track for event handling
  let currentPointIndex = 0;
  for (let i = trackingPointsWithDistanceRef.value.length - 1; i >= 0; i--) {
    if (trackingPointsWithDistanceRef.value[i].distance <= distanceTraveled) {
      currentPointIndex = i;
      break;
    }
  }

  const currentPoint = trackingPointsWithDistanceRef.value[currentPointIndex];
  if (currentPoint && currentPoint.commune) {
      currentCommuneName.value = currentPoint.commune;
  }

    const currentIncrement = currentPoint?.increment;

    // Si nous rembobinons et passons avant un événement déjà déclenché,
    // nous réinitialisons son état pour qu'il puisse se redéclencher.
    if (isRewinding.value) {
        if (triggeredPauseIncrement.value !== null && currentIncrement < triggeredPauseIncrement.value) {
            triggeredPauseIncrement.value = null;
        }
        if (triggeredFlytoIncrement.value !== null && currentIncrement < triggeredFlytoIncrement.value) {
            triggeredFlytoIncrement.value = null;
        }
    }

    // 2. Handle events using the accurate increment
    if (currentIncrement !== undefined) {
        // Popups
        if (map && rangeEvents.value.length > 0) {
            const newVisibleIds = new Set();
            for (const msg of rangeEvents.value) {
                const isVisible = currentIncrement >= msg.startIncrement && currentIncrement <= msg.endIncrement;

                if (isVisible) {
                    newVisibleIds.add(msg.eventId); // Use eventId here
                }
            }
            const currentVisibleIds = new Set(activePopups.keys());
            for (const id of currentVisibleIds) {
                if (!newVisibleIds.has(id)) {
                    activePopups.get(id)?.remove();
                    activePopups.delete(id);
                }
            }
            for (const id of newVisibleIds) {
                if (!currentVisibleIds.has(id)) {
                    const newMsg = rangeEvents.value.find(m => m.eventId === id); // Use eventId here
                    if (newMsg && newMsg.message) {
                        const svgContent = createMessageSVG(newMsg);

                        const orientation = newMsg.orientation || 'Droite';
                        const anchor = orientation === 'Gauche' ? 'bottom-right' : 'bottom-left';

                        const popup = new mapboxgl.Popup({ closeButton: false, closeOnClick: false, anchor: anchor, className: 'map-message-popup' })
                            .setLngLat(newMsg.coord)
                            .setHTML(svgContent)
                            .addTo(map);
                        activePopups.set(id, popup);
                    }
                }
            }
        }
      // Flyto
      const flytoData = flytoEvents.value[currentIncrement];
      if (flytoData && triggeredFlytoIncrement.value !== currentIncrement) {
          
          // CRITICAL: Do NOT trigger FlyTo if we just paused (e.g. at end of segment)
          if (isPaused.value) {
              console.log('[FlyTo] SKIPPED because isPaused=true. Increment:', currentIncrement);
              return; 
          }

          console.log('[FlyTo] DETECTED event at increment', currentIncrement);

          // FORCE UPDATE position to the event location (Start of Segment)
          // This ensures AltitudeSVG switches to the new segment immediately
          if (currentPoint) {
              const exactDist = currentPoint.distance;
              console.log('[FlyTo] FORCING distance update to:', exactDist * 1000);
              currentDistanceInMeters.value = exactDist * 1000;
              
              // Also sync accumulatedTime to avoid jumps when resuming
              const totalDur = totalDurationAt1xRef.value || 1;
              const ratio = totalDistanceRef.value > 0 ? (exactDist / totalDistanceRef.value) : 0;
              accumulatedTime = totalDur * ratio;
          }

          triggeredFlytoIncrement.value = currentIncrement;
          executeFlytoSequence(flytoData);
          animationFrameId = requestAnimationFrame(animate);
          return;
      }

      // Pause
      if (pauseIncrements.value.includes(currentIncrement)) {
          if (triggeredPauseIncrement.value !== currentIncrement) {
              console.log('[Pause] Triggered at increment', currentIncrement);
              isPaused.value = true;
              triggeredPauseIncrement.value = currentIncrement;
              //showSnackbar('Pause programmée atteinte.', 'info');
          }
      }
  }

  // 3. Determine camera keyframes and set camera
  let prevCamKeyframe, nextCamKeyframe;

  // Find the last control point we have passed
  let lastPassedControlPointIndex = -1;
  for (let i = controlPointIndicesRef.value.length - 1; i >= 0; i--) {
    const cpIndex = controlPointIndicesRef.value[i];
    const point = trackingPointsWithDistanceRef.value[cpIndex]; // Secure access
    if (point && point.distance <= distanceTraveled) {
      lastPassedControlPointIndex = cpIndex;
      break;
    }
  }

  if (lastPassedControlPointIndex !== -1) {
    const controlPoint = trackingPointsWithDistanceRef.value[lastPassedControlPointIndex];
    if (controlPoint.nbrSegment > 0) {
      const nextCpIndex = lastPassedControlPointIndex + controlPoint.nbrSegment;
      if (nextCpIndex < trackingPointsWithDistanceRef.value.length) {
        // We found a valid segment defined by a control point
        prevCamKeyframe = controlPoint;
        nextCamKeyframe = trackingPointsWithDistanceRef.value[nextCpIndex];
      }
    }
  }

  // If we are between two control points of a valid segment, interpolate.
  if (prevCamKeyframe && nextCamKeyframe && distanceTraveled < nextCamKeyframe.distance) {
    const prevKeyframeDist = prevCamKeyframe.distance;
    const nextKeyframeDist = nextCamKeyframe.distance;
    const segmentDist = nextKeyframeDist - prevKeyframeDist;
    const progressInSegment = segmentDist > 0 ? (distanceTraveled - prevKeyframeDist) / segmentDist : 0;

    const lookAtPointLng = lerp(prevCamKeyframe.coordonnee[0], nextCamKeyframe.coordonnee[0], progressInSegment);
    const lookAtPointLat = lerp(prevCamKeyframe.coordonnee[1], nextCamKeyframe.coordonnee[1], progressInSegment);

    const prevZoom = prevCamKeyframe.editedZoom ?? prevCamKeyframe.zoom;
    const nextZoom = nextCamKeyframe.editedZoom ?? nextCamKeyframe.zoom;
    const prevPitch = prevCamKeyframe.editedPitch ?? prevCamKeyframe.pitch;
    const nextPitch = nextCamKeyframe.editedPitch ?? nextCamKeyframe.pitch;
    const prevCap = prevCamKeyframe.editedCap ?? prevCamKeyframe.cap;
    const nextCap = nextCamKeyframe.editedCap ?? nextCamKeyframe.cap;
    const zoom = lerp(prevZoom, nextZoom, progressInSegment) * dynamicZoomCoefficient.value;
    const pitch = lerp(prevPitch, nextPitch, progressInSegment);
    const bearing = lerpAngle(prevCap, nextCap, progressInSegment);
    map.setZoom(zoom);
    map.setPitch(pitch);
    map.setBearing(bearing);
    map.setCenter([lookAtPointLng, lookAtPointLat]);
    
     // Calculate instantaneous track bearing using geometry
    if (lineStringRef.value) {
        try {
                const p1 = turf.along(lineStringRef.value, distanceTraveled, {units: 'kilometers'});
                const p2 = turf.along(lineStringRef.value, distanceTraveled + 0.01, {units: 'kilometers'});
                currentTraceBearing.value = turf.bearing(p1, p2);
        } catch (e) {
            // Fallback
             currentTraceBearing.value = bearing;
        }
    }
  } else {
    // Otherwise (no CPs, after last CP segment, or on a non-CP point), interpolate point-by-point for smooth movement.
    if (currentPoint) {
        const nextPointIndex = currentPointIndex + 1;
        if (nextPointIndex < trackingPointsWithDistanceRef.value.length) {
            const nextPoint = trackingPointsWithDistanceRef.value[nextPointIndex];

            const prevKeyframeDist = currentPoint.distance;
            const nextKeyframeDist = nextPoint.distance;
            const segmentDist = nextKeyframeDist - prevKeyframeDist;
            const progressInSegment = segmentDist > 0 ? (distanceTraveled - prevKeyframeDist) / segmentDist : 0;

            const lookAtPointLng = lerp(currentPoint.coordonnee[0], nextPoint.coordonnee[0], progressInSegment);
            const lookAtPointLat = lerp(currentPoint.coordonnee[1], nextPoint.coordonnee[1], progressInSegment);

            const prevZoom = currentPoint.editedZoom ?? currentPoint.zoom;
            const nextZoom = nextPoint.editedZoom ?? nextPoint.zoom;
            const prevPitch = currentPoint.editedPitch ?? currentPoint.pitch;
            const nextPitch = nextPoint.editedPitch ?? nextPoint.pitch;
            const prevCap = currentPoint.editedCap ?? currentPoint.cap;
            const nextCap = nextPoint.editedCap ?? nextPoint.cap;

            const zoom = lerp(currentPoint.editedZoom ?? currentPoint.zoom, nextPoint.editedZoom ?? nextPoint.zoom, progressInSegment) * dynamicZoomCoefficient.value;
            const pitch = lerp(prevPitch, nextPitch, progressInSegment);
            const bearing = lerpAngle(prevCap, nextCap, progressInSegment);
            
            map.setZoom(zoom);
            map.setPitch(pitch);
            map.setBearing(bearing);
            map.setCenter([lookAtPointLng, lookAtPointLat]);
            
            // Calculate instantaneous track bearing using geometry
            let calculatedBearing = null;
            
            // MULTI-SEGMENT LOGIC: Calculate bearing on local segment to avoid jumps
            if (isMultisegmentVariant.value && activeVariantSegments.value.length > 0) {
                 const currentSeg = activeVariantSegments.value.find(s => distanceTraveled >= s.startDistKm && distanceTraveled < s.endDistKm);
                 // If at exact end or overflow, use the last segment logic handled by clamp usually.
                 // We simply check if we found a segment with coordinates
                 if (currentSeg && currentSeg.coordinates && currentSeg.coordinates.length > 1) {
                      const localDist = distanceTraveled - currentSeg.startDistKm;
                      const segmentLine = turf.lineString(currentSeg.coordinates);
                      
                      const len = currentSeg.lengthKm;
                      const dist1 = Math.min(localDist, len);
                      const dist2 = Math.min(localDist + 0.01, len);
                      
                      const p1 = turf.along(segmentLine, dist1, {units: 'kilometers'});
                      const p2 = turf.along(segmentLine, dist2, {units: 'kilometers'});
                      calculatedBearing = turf.bearing(p1, p2);
                 }
            } 
            // DEFAULT LOGIC: Use global lineString if available (Main Trace)
            else if (lineStringRef.value && lineStringRef.value.geometry.type === 'LineString') {
                try {
                     const p1 = turf.along(lineStringRef.value, distanceTraveled, {units: 'kilometers'});
                     const p2 = turf.along(lineStringRef.value, distanceTraveled + 0.01, {units: 'kilometers'});
                     calculatedBearing = turf.bearing(p1, p2);
                } catch (e) {
                    // console.warn("Error calculating track bearing:", e);
                }
            }
            
            if (calculatedBearing !== null) {
                currentTraceBearing.value = calculatedBearing;
            }
            
        } else {
            // At the very last point
            const zoom = (currentPoint.editedZoom ?? currentPoint.zoom) * dynamicZoomCoefficient.value;
            const pitch = currentPoint.editedPitch ?? currentPoint.pitch;
            const bearing = currentPoint.editedCap ?? currentPoint.cap;
            const center = currentPoint.coordonnee;

            map.setZoom(zoom);
            map.setPitch(pitch);
            map.setBearing(bearing);
            map.setCenter(center);
            
            currentTraceBearing.value = bearing;
        }
    }
  }

  // Weather Dynamic Update
  if (weatherForecasts.value.length > 0 && simulationStartDate.value && !isInitializing.value) {
      // Calculate current simulation time based on Reference Scenario
      // Allows compass to follow specific scenario speed/time regardless of animation playback speed
      let currentSimTime;
      
      const refScen = circuitScenarios.value.find(s => s.isReference) || circuitScenarios.value[0];
      if (refScen) {
           const refStart = refScen.heureDepart || "09:00";
           const refSpeed = Number(refScen.vitesseMoyenne) || 20;

           const [h, m] = String(refStart).split(':').map(Number);
           const startDate = new Date(simulationStartDate.value);
           // Reset to Ref Start Time
           startDate.setHours(isNaN(h) ? 9 : h, isNaN(m) ? 0 : m, 0, 0);

           const hoursElapsed = currentDistance.value / refSpeed;
           currentSimTime = new Date(startDate.getTime() + hoursElapsed * 3600000);
      } else {
           // Fallback to global animation time if no scenarios defined
           currentSimTime = new Date(simulationStartDate.value.getTime() + accumulatedTime);
      }

      const currentHour = currentSimTime.getHours();

      // Calculate current increment (km)
      const currentIncrement = Math.round(currentDistanceInMeters.value / 1000);

      // Find forecast for this increment
      // Find forecast for this increment/km
      const forecast = weatherForecasts.value.find(f => {
          if (f.km !== undefined) return Math.abs(f.km - currentDistance.value) < 0.5;
          return f.increment === currentIncrement;
      });
      
      if (forecast && forecast.hours) {
          if (forecast.hours[currentHour]) {
              currentWeather.value = forecast.hours[currentHour];
          } else {
              // Fallback to closest available hour if out of range
             const keys = Object.keys(forecast.hours).map(Number).sort((a,b)=>a-b);
             if (keys.length > 0) {
                 const closestH = keys.reduce((prev, curr) => {
                     return (Math.abs(curr - currentHour) < Math.abs(prev - currentHour) ? curr : prev);
                 });
                 currentWeather.value = forecast.hours[closestH];
             }
          }
      }
  }


  if (map) map.triggerRepaint();

  if (phase < 1 || isRewinding.value) {
    animationFrameId = requestAnimationFrame(animate);
  } else {
    isAnimationFinished.value = true;
    isPaused.value = true;
    cancelAnimationFrame(animationFrameId);
    animationFrameId = null;

    // Si on est sur une variante, on reste sur la fin sans lancer la séquence globale
    if (currentSegmentType.value !== null) {
        // Optionnel: afficher un message de fin
        // showSnackbar('Fin du segment.', 'info');
        return; 
    }

    // Start finalization sequence
    finalizationTimeoutId = setTimeout(async () => {
      if (!map) return;

      // Masquer les widgets pour la vue globale finale
      isDistanceDisplayVisible.value = false;
      isCommuneWidgetVisible.value = false;
      isAltitudeVisible.value = false;
      isControlsCardVisible.value = false;
      isWeatherInfoVisible.value = false;
      isCompassVisible.value = false;
      sendVisualizeViewStateUpdate();

      animationState.value = 'Vol_Final';

      // Hide the comet
      map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });

      const traceBbox = turf.bbox(lineStringRef.value);
      
      // Changement de style vers le style de lancement pour la vue globale finale
      if (styleLancement.value !== mapStyle.value) {
          map.setStyle(styleLancement.value);
          await new Promise(resolve => map.once('style.load', resolve));
      }

      await flyToPromise(map, {
          pitch: 0,
          bearing: 0,
          duration: flyToGlobalDuration.value,
          ...(globalTraceCameraOptions.value || map.cameraForBounds(traceBbox, { padding: 40, bearing: 0, pitch: 0 }))
      });

      animationState.value = 'Termine';

      if (repriseAutomatique.value) {
        const pauseMs = pauseAvantReprise.value;
        if (pauseMs > 0) {
            await new Promise(resolve => setTimeout(resolve, pauseMs));
        }
        resetAnimation();
      }
    }, delayAfterAnimationEnd.value);
  }
};

const isPaused = ref(true);
const isRewinding = ref(false);
const isAnimationFinished = ref(false);
const distanceDisplay = ref('0.00 km');

const minSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/min_value'));
const maxSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/max_value'));
const defaultSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/default_value'));
const sliderStep = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/slider_step'));

function mapSliderToSpeed(sliderValue) {
    const min = minSpeedValue.value;
    const max = maxSpeedValue.value;
    const def = defaultSpeedValue.value;
    if (![min, max, def].every(v => typeof v === 'number')) return def || 1.0;
    const halfMax = max / 2;

    if (sliderValue <= 20) {
        const t = sliderValue / 20;
        return min + t * (def - min);
    } else if (sliderValue <= 80) {
        const t = (sliderValue - 20) / 60;
        return def + t * (halfMax - def);
    } else {
        const t = (sliderValue - 80) / 20;
        return halfMax + t * (max - halfMax);
    }
}

function mapSpeedToSlider(speed) {
    const min = minSpeedValue.value;
    const max = maxSpeedValue.value;
    const def = defaultSpeedValue.value;
    if (![min, max, def].every(v => typeof v === 'number')) return 20;
    const halfMax = max / 2;

    if (speed < min) return 0;
    if (speed > max) return 100;

    if (speed <= def) {
        const range = def - min;
        if (range <= 0) return 20;
        const t = (speed - min) / range;
        return t * 20;
    } else if (speed <= halfMax) {
        const range = halfMax - def;
        if (range <= 0) return 80;
        const t = (speed - def) / range;
        return 20 + t * 60;
    } else {
        const range = max - halfMax;
        if (range <= 0) return 100;
        const t = (speed - halfMax) / range;
        return 80 + t * 20;
    }
}

const sliderPosition = ref(mapSpeedToSlider(getSettingValue('Visualisation/Lecture/Vitesse/default_value')));

const currentSpeed = computed({
  get: () => mapSliderToSpeed(sliderPosition.value),
  set: (newSpeed) => {
    sliderPosition.value = mapSpeedToSlider(newSpeed);
  }
});

const intensiteZoomDynamique = computed(() => getSettingValue('Visualisation/Lecture/ZoomDynamique/intensite_zoom_dynamique'));

const dynamicZoomCoefficient = computed(() => {
    const speed = currentSpeed.value;
    const intensite = intensiteZoomDynamique.value;

    if (speed === null || intensite === null) {
        return 1.0; // Valeur par défaut si les paramètres ne sont pas chargés
    }

    // Constantes A et type de fonction sont maintenant fixes.
    const A = 1.0;
    const B = intensite / 1000; // Conversion de l'entier (ex: 10) en flottant (ex: 0.01)

    // Implémentation de la fonction de zoom dynamique : coefficient = A / (speed^B)
    if (speed === 0) return A; // Éviter la division par zéro ou puissance de zéro
    return A / (speed ** B);
});

watch(currentSpeed, (newSpeed) => {
    if (newSpeed !== null && newSpeed !== undefined) {
        invoke('update_animation_speed', { speed: newSpeed });
    }
});

// --- Helper Functions ---
const lerp = (start, end, t) => start * (1 - t) + end * t;

const lerpAngle = (start, end, t) => {
    let delta = end - start;
    if (delta > 180) delta -= 360;
    else if (delta < -180) delta += 360;
    let result = start + delta * t;
    result = result % 360;
    if (result < 0) result += 360;
    return result;
};

// Add this function to handle the zoom event
const handleMapZoom = () => {
    if (isPaused.value && map.getZoom() < zoomMinimum.value) {
        map.setZoom(zoomMinimum.value);
    }
};

const initWeather = async (circuit, trackPoints) => {
    console.log("initWeather called", { 
        meteoActif: meteoActif.value, 
        circuitId: props.circuitId, 
        trackPointsLength: trackPoints?.length 
    });
    if (!meteoActif.value) return;

    const config = circuit?.meteoConfig || {};
    // Start Time (e.g. "09:00")
    const startTimeStr = config.heureDepart || defaultHeureDepart.value; // e.g. "09:00"
    
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
    dateStr = computedDate;

    console.log("initWeather Date Logic:", {
        rawConfigDate: config.dateDepart,
        calculatedDate: dateStr
    });

    // Set Simulation Start Date Ref
    const [h, m] = startTimeStr.split(':').map(Number);
    const startD = new Date(dateStr);
    startD.setHours(h, m, 0, 0);
    simulationStartDate.value = startD;

    try {
        if (!trackPoints || trackPoints.length === 0) return;

        // Sample points every 10 indices (approx 1km) and last point
        const sampled = [];
        trackPoints.forEach((p, i) => {
            if (i % 10 === 0 || i === trackPoints.length - 1) {
                // Determine increment (km)
                // User logic: "increment 0, 10, 20" -> these are indices in the 100m array.
                // So KM = index / 10.
                const inc = Math.round(i / 10);

                // Avoid duplicates (e.g. if last point is close to 10th)
                if (!sampled.some(s => s.increment === inc)) {
                    // trackPoints have coordonnee: [lon, lat]
                    sampled.push({
                        lat: p.coordonnee[1],
                        lon: p.coordonnee[0],
                        increment: inc,
                        km: p.distance // Use the real distance calculated by backend
                    });
                }
            }
        });

        if (sampled.length > 0) {
             // Filename: AAAMMJJ-HH-to-HH.json
             const d = new Date(dateStr);
             const yyyy = d.getFullYear();
             const mm = String(d.getMonth() + 1).padStart(2, '0');
             const dd = String(d.getDate()).padStart(2, '0');
             const datePart = `${yyyy}${mm}${dd}`;

             const startH = heureDebutJournee.value;
             const endH = heureFinJournee.value;
             
             const sH = String(startH).padStart(2, '0');
             const eH = String(endH).padStart(2, '0');

             const filename = `${datePart}-${sH}-to-${eH}.json`;
             console.log(`Checking weather cache: ${filename}`);

             let matrix = null;
             try {
                  const cacheContent = await invoke('check_weather_cache', { circuitId: props.circuitId, filename });
                  if (cacheContent) {
                      console.log("Loading weather from cache");
                      matrix = JSON.parse(cacheContent);
                  }
             } catch (err) {
                 console.warn("Cache check failed", err);
             }

             if (!matrix) {
                  console.log("Fetching weather from API (Matrix)");
                  matrix = await WeatherService.fetchWeatherMatrix(sampled, dateStr, startH, endH);
                  if (matrix && matrix.length > 0) {
                       try {
                           await invoke('save_weather_cache', { 
                               circuitId: props.circuitId, 
                               filename, 
                               content: JSON.stringify(matrix, null, 2) 
                           });
                           console.log("Weather cache saved");
                       } catch (err) {
                           console.error("Failed to save cache", err);
                       }
                  }
             }

             weatherForecasts.value = matrix;
             
             // Set initial weather
             if (matrix.length > 0) {
                 const p0 = matrix[0]; 
                 const startHour = startD.getHours();
                 if (p0.hours && p0.hours[startHour]) {
                    currentWeather.value = p0.hours[startHour];
                 } else if (p0.hours) {
                    const keys = Object.keys(p0.hours).sort();
                    if (keys.length > 0) currentWeather.value = p0.hours[keys[0]];
                 }
             }
        }
    } catch (e) {
        console.error("Weather init failed", e);
    }
};



// --- Computed settings ---
const mapboxToken = computed(() => getSettingValue('Système/Tokens/mapbox'));
const styleLancement = computed(() => getSettingValue('Visualisation/Lancement/styleLancement'));
const mapStyle = computed(() => getSettingValue('Visualisation/Vue 3D/Carte/styleVisualisation'));
const terrainExaggeration = computed(() => getSettingValue('Edition/Vue 3D/Carte/exaggeration'));
const traceColor = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/couleurTrace'));
const traceWidth = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/epaisseurTrace'));
const traceOpacity = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/opaciteTrace'));
const colorTraceBySlope = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/colorerSelonPente'));
const segmentLength = computed(() => getSettingValue('Importation/Tracking/LongueurSegment'));
const cometColor = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/couleurComete'));
const cometWidth = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/epaisseurComete'));
const cometOpacity = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/opaciteComete'));
const cometLength = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/longueurComete'));
const heureDebutJournee = computed(() => getSettingValue('Visualisation/Météo/heureDebutJournee') || 6);
const heureFinJournee = computed(() => getSettingValue('Visualisation/Météo/heureFinJournee') || 20);

const animationSpeed = computed(() => {
    const val = getSettingValue('Visualisation/Lecture/vitesse');
    return val > 100 ? val : val * 1000;
});
const timerReprisePause = computed(() => {
    const val = getSettingValue('Visualisation/Lecture/timerReprisePause');
    return val > 100 ? val : val * 1000;
});
const masquerCurseurDelai = computed(() => {
    const val = getSettingValue('Visualisation/Lecture/masquerCurseurDelai');
    return val > 100 ? val : val * 1000;
});
const delayAfterAnimationEnd = computed(() => {
    const val = getSettingValue('Visualisation/Finalisation/delayAfterAnimationEnd');
    return (val > 100 ? val : val * 1000);
}); 
const flyToGlobalDuration = computed(() => {
    const val = getSettingValue('Visualisation/Finalisation/flyToGlobalDuration');
    return val > 100 ? val : val * 1000;
});
const flyToKm0Duration = computed(() => {
    const val = getSettingValue('Visualisation/Finalisation/flyToKm0Duration');
    return val > 100 ? val : val * 1000;
});
const pauseAuKm0 = computed(() => {
    const val = getSettingValue('Visualisation/Lancement/pauseAuKm0');
    return val > 100 ? val : val * 1000;
});
const repriseAutomatique = computed(() => getSettingValue('Visualisation/Finalisation/repriseAutomatique'));
const pauseAvantReprise = computed(() => {
    const val = getSettingValue('Visualisation/Finalisation/pauseAvantReprise');
    return val > 100 ? val : val * 1000;
});
const baseMessageFontSize = computed(() => getSettingValue('Visualisation/Taille des Messages/baseFontSize'));

const sensibilityCap = computed(() => getSettingValue('Système/Télécommande/sensibiliteCap') ?? 100);
const sensibilityPointDeVueX = computed(() => getSettingValue('Système/Télécommande/sensibilitePointDeVueX') ?? 100);
const sensibilityPointDeVueY = computed(() => getSettingValue('Système/Télécommande/sensibilitePointDeVueY') ?? 100);
const sensibilityZoom = computed(() => getSettingValue('Système/Télécommande/sensibiliteZoom') ?? 100);
const sensibilityTilt = computed(() => getSettingValue('Système/Télécommande/sensibiliteTilt') ?? 50);



// --- Pause/Resume Logic ---
const pausedCameraOptions = ref(null);
const cameraMovedDuringPause = ref(false);
const isResuming = ref(false); // Flag to block animation during flyTo
let isWatcherActive = true;

const onMapInteraction = () => {
    // Ne pas marquer de mouvement si on est en train d'initialiser ou de repositionner
    if (isInitializing.value || isResuming.value || isFlytoActive.value) return;

    if (isPaused.value && !isResuming.value && !isFlytoActive.value) {
        cameraMovedDuringPause.value = true;
    }
};

watch(isPaused, (paused) => {
    if (!isWatcherActive) return; // Ne rien faire si le watcher est désactivé

    // Notify the backend about the pause state change
    invoke('update_animation_state', { newState: paused ? 'En_Pause' : 'En_Animation' });

    if (paused) {
        // On ne met à jour l'état que si un survol n'est pas déjà en train de gérer la pause.
        if (!isFlytoActive.value) {
            animationState.value = 'En_Pause';
        }
    } else {
        // On ne passe en animation que si la reprise n'est pas déclenchée par la fin d'un survol.
        if (!isFlytoActive.value) {
            animationState.value = 'En_Animation';
        }
    }

    if (!map) return;

    if (paused) {
        // --- PAUSING ---
        isResuming.value = false; // Ensure resuming flag is off
        pausedCameraOptions.value = {
            center: map.getCenter(),
            zoom: map.getZoom(),
            pitch: map.getPitch(),
            bearing: map.getBearing(),
        };
        cameraMovedDuringPause.value = false;

        // Enable interactions
        map.interactive = true;
        map.dragRotate.enable();
        map.dragPan.enable();
        map.scrollZoom.enable({ around: 'center' });

        // Listen for any interaction
        map.on('move', onMapInteraction);
        map.on('zoom', onMapInteraction);
        map.on('pitch', onMapInteraction);
        map.on('rotate', onMapInteraction);

        // Add new listener for zoom minimum
        map.on('zoom', handleMapZoom);

    } else {
        // --- RESUMING ---
        // Disable interactions first
        map.interactive = false;
        map.dragRotate.disable();
        map.dragPan.disable();
        map.scrollZoom.disable();
        map.off('move', onMapInteraction);
        map.off('zoom', onMapInteraction);
        map.off('pitch', onMapInteraction);
        map.off('rotate', onMapInteraction);

        // Remove listener for zoom minimum
        map.off('zoom', handleMapZoom);

        // On n'exécute la logique de reprise que si un survol n'est pas en cours.
        if (!isFlytoActive.value && !isInitializing.value) {
            if (cameraMovedDuringPause.value && pausedCameraOptions.value) {
                isResuming.value = true; // Block animation
                //showSnackbar('Reprise de la position initiale...', 'info');
                map.flyTo({
                    ...pausedCameraOptions.value,
                    duration: timerReprisePause.value,
                    easing: (t) => t, // linear easing
                });
                map.once('moveend', () => {
                    isResuming.value = false; // Unblock animation
                    lastTimestamp = 0; // Reset timestamp to resume animation smoothly
                });
            } else {
                 lastTimestamp = 0; // Reset timestamp for smooth resume even without flyTo
            }
        }

        // Restart animation loop if it was stopped and we are effectively resuming
        if (!isPaused.value && !animationFrameId) {
            animationFrameId = requestAnimationFrame(animate);
        }
    }
});

// Watcher pour relancer l'animation lors du rembobinage (même si terminé)
watch(isRewinding, (newVal) => {
    if (newVal) {
        // Annuler la finalisation si elle est en attente
        if (finalizationTimeoutId) {
            clearTimeout(finalizationTimeoutId);
            finalizationTimeoutId = null;
        }

        if (!animationFrameId) {
            // Si l'animation était terminée ou arrêtée, on la relance
            isAnimationFinished.value = false;
            lastTimestamp = 0; // Reset timestamp pour éviter les sauts
            animationFrameId = requestAnimationFrame(animate);
        }
    }
});

// --- Message Display Functions for Initialization ---
const getMessagesForKm0 = () => {
  if (!rangeEvents.value || rangeEvents.value.length === 0) {
    return { atKm0: [], nearKm0: [] };
  }
  
  const atKm0 = [];
  const nearKm0 = [];
  
  for (const msg of rangeEvents.value) {
    // Messages visibles au km0 (startIncrement <= 0 && endIncrement >= 0)
    if (msg.startIncrement <= 0 && msg.endIncrement >= 0) {
      if (msg.anchorIncrement === 0) {
        atKm0.push(msg);
      } else {
        nearKm0.push(msg);
      }
    }
  }
  
  return { atKm0, nearKm0 };
};

const displayMessagesWithFade = async (messages, duration) => {
  if (!map || messages.length === 0) return;
  
  const popupsToAnimate = [];
  
  // Créer les popups avec opacité 0
  for (const msg of messages) {
    const svgContent = createMessageSVG(msg);
    const orientation = msg.orientation || 'Droite';
    const anchor = orientation === 'Gauche' ? 'bottom-right' : 'bottom-left';
    
    const popup = new mapboxgl.Popup({
      closeButton: false,
      closeOnClick: false,
      anchor: anchor,
      className: 'map-message-popup fade-in-message'
    })
      .setLngLat(msg.coord)
      .setHTML(svgContent)
      .addTo(map);
    
    // Stocker pour animation
    popupsToAnimate.push({ popup, msg });
    activePopups.set(msg.eventId, popup);
  }
  
  // Animer l'opacité
  const startTime = performance.now();
  const animateOpacity = (timestamp) => {
    const elapsed = timestamp - startTime;
    const progress = Math.min(elapsed / duration, 1);
    
    popupsToAnimate.forEach(({ popup }) => {
      const element = popup.getElement();
      if (element) {
        element.style.opacity = progress;
      }
    });
    
    if (progress < 1) {
      requestAnimationFrame(animateOpacity);
    }
  };
  
  requestAnimationFrame(animateOpacity);
};

const goBack = () => {
  router.push({ name: 'Main' });
};

const setupMapLayersAndSources = async () => {
  if (!map) return;
  
  if (!map.getSource('mapbox-dem')) {
    map.addSource('mapbox-dem', {
      'type': 'raster-dem',
      'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
      'tileSize': 512,
      'maxzoom': 14
    });
  }
  map.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': terrainExaggeration.value });
  map.setFog({});

  if (!map.getSource('trace')) {
    map.addSource('trace', { type: 'geojson', data: lineStringRef.value, lineMetrics: true });
  }

  // Source for the colored segments (Detailed FeatureCollection)
  if (coloredSegmentsGeoJsonRef.value && !map.getSource('colored-segments')) {
    map.addSource('colored-segments', { type: 'geojson', data: coloredSegmentsGeoJsonRef.value });

    // Layer 2: Overlay Aller (Tout SAUF Retour)
    if (!map.getLayer('trace-overlap-aller')) {
        map.addLayer({
            id: 'trace-overlap-aller',
            type: 'line',
            source: 'colored-segments',
            layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' }, 
            paint: {
                'line-width': traceWidth.value ?? 4,
                'line-opacity': traceOpacity.value ?? 1,
                'line-color': ['get', 'color_raw']
            },
            filter: ['!=', ['get', 'segment_type'], 'retour_overlap']
        });
    }

    // Layer 3: Overlay Retour (Tout SAUF Aller)
    if (!map.getLayer('trace-overlap-retour')) {
        map.addLayer({
            id: 'trace-overlap-retour',
            type: 'line',
            source: 'colored-segments',
            layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' }, 
            paint: {
                'line-width': traceWidth.value ?? 4,
                'line-opacity': traceOpacity.value ?? 1,
                'line-color': ['get', 'color_raw']
            },
            filter: ['!=', ['get', 'segment_type'], 'aller_overlap']
        });
    }
  }

  // Fallback
  if (!coloredSegmentsGeoJsonRef.value && !map.getLayer('trace-complete')) {
     if (!map.getSource('trace') && lineStringRef.value) {
        map.addSource('trace', { type: 'geojson', data: lineStringRef.value });
     }
     
     if (map.getSource('trace')) {
        map.addLayer({
            id: 'trace-complete',
            type: 'line',
            source: 'trace',
            layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' },
            paint: {
                'line-width': traceWidth.value ?? 4,
                'line-opacity': traceOpacity.value ?? 1,
                'line-color': traceColor.value || '#0000FF'
            }
        });
     }
  }

  // Comète
  if (lineStringRef.value && !map.getSource('comet-source')) {
    map.addSource('comet-source', { type: 'geojson', data: { type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} } });
  }
  if (!map.getLayer('comet-layer')) {
    map.addLayer({ id: 'comet-layer', type: 'line', source: 'comet-source', paint: { 'line-width': cometWidth.value, 'line-color': cometColor.value, 'line-opacity': cometOpacity.value } });
  }
};


const resetAnimation = async () => {
    accumulatedTime = 0;
    isPaused.value = true;
    isAnimationFinished.value = false;
    warningShown = false;
    triggeredPauseIncrement.value = null;
    triggeredFlytoIncrement.value = null;
    isFlytoActive.value = false;
    preFlytoCameraOptions.value = null;
    currentSpeed.value = defaultSpeedValue.value;

    activePopups.forEach(popup => popup.remove());
    activePopups.clear();

    // Fly to Km 0
    if (map && trackingPointsWithDistanceRef.value && trackingPointsWithDistanceRef.value.length > 0) {
        // Restaurer la visibilité des widgets selon les réglages
        isDistanceDisplayVisible.value = getSettingValue('Visualisation/Widgets/distance') ?? true;
        isCommuneWidgetVisible.value = getSettingValue('Visualisation/Widgets/communes') ?? true;
        isAltitudeVisible.value = getSettingValue('Visualisation/Widgets/altitude') ?? true;
        isControlsCardVisible.value = getSettingValue('Visualisation/Widgets/commandes') ?? true;
        isWeatherInfoVisible.value = true;
        isCompassVisible.value = true;
        isBackButtonVisible.value = true;
        sendVisualizeViewStateUpdate();

        // Restauration du style de visualisation (satellite) si nécessaire
        // On force le rechargement si on n'est pas sûr, car on peut être en style Standard (fin de vol)
        if (mapStyle.value && (mapStyle.value !== (map.getStyle() && map.getStyle().style))) {
            console.log('[resetAnimation] Restoring Satellite style...');
            map.setStyle(mapStyle.value);
            await new Promise(resolve => map.once('style.load', resolve));
            setupMapLayersAndSources();
        }

        const startCameraOptions = {
            center: trackingPointsWithDistanceRef.value[0].coordonnee,
            zoom: trackingPointsWithDistanceRef.value[0].editedZoom ?? trackingPointsWithDistanceRef.value[0].zoom,
            pitch: trackingPointsWithDistanceRef.value[0].editedPitch ?? trackingPointsWithDistanceRef.value[0].pitch,
            bearing: trackingPointsWithDistanceRef.value[0].editedCap ?? trackingPointsWithDistanceRef.value[0].cap,
        };
        await flyToPromise(map, {
            ...startCameraOptions,
            duration: flyToKm0Duration.value,
        });
    }

    animationState.value = 'En_Pause_au_Depart';

    // After flying to start, update the paused state to reflect the new camera position
    // and reset the interaction flag. This prevents the "resume" logic from flying back
    // to the pre-reset position.
    if (map) {
        pausedCameraOptions.value = {
            center: map.getCenter(),
            zoom: map.getZoom(),
            pitch: map.getPitch(),
            bearing: map.getBearing(),
        };
        cameraMovedDuringPause.value = false;
    }

    // Reset comet to start
    map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
    // Restart animation loop if it was stopped
    if (!animationFrameId) {
        animationFrameId = requestAnimationFrame(animate);
    }

    // Démarrage automatique après la pause définie
    const pauseMs = pauseAuKm0.value;
    if (pauseMs > 0) {
        await new Promise(resolve => setTimeout(resolve, pauseMs));
    }
    // On vérifie que l'état est toujours celui d'une réinitialisation avant de lancer
    if (!isAnimationFinished.value) {
        isPaused.value = false;
    }
};



const handleKeyDown = (e) => {
    // Handle 'r' for reload specifically when animation is finished
    if (isAnimationFinished.value) {
        if (e.key === 'r' || e.key === 'R') {
            resetAnimation();
        }
        return; // Ignore all other keys when finished
    }

    if (isInitializing.value) return; // Ignore all keys during initialization

    if (e.key === 'p' || e.key === 'P') {
        isPaused.value = !isPaused.value;
    } else if (e.key === 'ArrowLeft') {
        isRewinding.value = true;
    } else if (e.key === 'ArrowDown') {
        sliderPosition.value = Math.max(0, sliderPosition.value - 1);
    } else if (e.key === 'ArrowUp') {
        sliderPosition.value = Math.min(100, sliderPosition.value + 1);
    } else if (e.key === 'a' || e.key === 'A') {
        isAltitudeVisible.value = !isAltitudeVisible.value;
        sendVisualizeViewStateUpdate();
    } else if (e.key === 'c' || e.key === 'C') {
        isCommuneWidgetVisible.value = !isCommuneWidgetVisible.value;
        sendVisualizeViewStateUpdate();
    } else if (e.key === 'd' || e.key === 'D') {
        isDistanceDisplayVisible.value = !isDistanceDisplayVisible.value;
        sendVisualizeViewStateUpdate();
    } else if (e.key === 'm') {
        isWeatherInfoVisible.value = !isWeatherInfoVisible.value;
    } else if (e.key === 'b') {
        isCompassVisible.value = !isCompassVisible.value;
    } else if (e.key === 'B') {
        currentOrientationMode.value = currentOrientationMode.value === 'Trace' ? 'Camera' : 'Trace';
    } else if (e.key === 'h' || e.key === 'H') {
        toggleBackButtonVisibility();
    } else if (e.code === 'Space') {
        e.preventDefault();
        isControlsCardVisible.value = !isControlsCardVisible.value;
        sendVisualizeViewStateUpdate();
    } else if (e.key === 'Delete') {
        isBackButtonVisible.value = false;
        isDistanceDisplayVisible.value = false;
        isControlsCardVisible.value = false;
        isCommuneWidgetVisible.value = false;
        isAltitudeVisible.value = false;
        isStaticWeatherVisible.value = false;
        isDynamicWeatherVisible.value = false;
        sendVisualizeViewStateUpdate();
    }
};

const handleKeyUp = (e) => {
    if (isInitializing.value) return; // Ignorer l'entrée clavier pendant l'initialisation

    if (e.key === 'ArrowLeft') {
        isRewinding.value = false;
    }
};

// Fonction pour basculer la visibilité des layers selon la zone active
const updateTraceGradient = async (zoneId, direction) => {
    if (!map) return;

    // Déterminer la visibilité des overlays
    const showAllerOverlay = (zoneId !== null && direction === 'aller');
    const showRetourOverlay = (zoneId !== null && direction === 'retour');
console.log('[Layer Toggle]', {zoneId, direction, showAller: showAllerOverlay, showRetour: showRetourOverlay});
    // trace-complete est toujours visible
    if (map.getLayer('trace-complete')) {
        map.setLayoutProperty('trace-complete', 'visibility', 'visible');
    }
    
    // Basculer les overlays
    if (map.getLayer('trace-overlap-aller')) {
        map.setLayoutProperty('trace-overlap-aller', 'visibility', showAllerOverlay ? 'visible' : 'none');
    }
    
    if (map.getLayer('trace-overlap-retour')) {
        map.setLayoutProperty('trace-overlap-retour', 'visibility', showRetourOverlay ? 'visible' : 'none');
    }
};

// Helper function to detect if current position is in an overlapping zone
const getCurrentSegmentZone = (currentKm, metadata) => {
    if (!metadata || !metadata.overlappingZones) return null;
    
    for (const zone of metadata.overlappingZones) {
        // Vérifier si on est sur l'aller
        if (currentKm >= zone.allerStartKm && currentKm <= zone.allerEndKm) {
            return { zone, direction: 'aller' };
        }
        
        // Vérifier si on est sur le retour
        if (currentKm >= zone.retourStartKm && currentKm <= zone.retourEndKm) {
            return { zone, direction: 'retour' };
        }
    }
    
    return null;
};

const initializeMap = async () => {
  if (!settings.value || !mapboxToken.value) {
    return;
  }
  mapboxgl.accessToken = mapboxToken.value;

  try {
    const [fetchedLineString, fetchedTrackingData, fetchedEvents, allCircuits] = await Promise.all([
      invoke('read_line_string_file', { circuitId: props.circuitId }),
      invoke('read_tracking_file', { circuitId: props.circuitId }),
      invoke('get_events', { circuitId: props.circuitId }),
      invoke('get_circuits_for_display')
    ]);

    const currentCircuit = allCircuits.find(c => c.circuitId === props.circuitId);
    currentCircuitRef.value = currentCircuit || null; // Store in ref
    if (currentCircuit) {
        avancementCommunes.value = currentCircuit.avancementCommunes;
    }

    if (fetchedEvents) {
        if (fetchedEvents.pointEvents) {
            pauseIncrements.value = Object.keys(fetchedEvents.pointEvents)
                .filter(increment =>
                    fetchedEvents.pointEvents[increment].some(event => event.type === 'Pause')
                )
                .map(Number);
            
            const flytos = {};
            for (const incrementStr in fetchedEvents.pointEvents) {
                const increment = Number(incrementStr);
                const flytoEvent = fetchedEvents.pointEvents[increment].find(event => event.type === 'Flyto');
                if (flytoEvent) {
                    flytos[increment] = flytoEvent.data;
                }
            }
            flytoEvents.value = flytos;
        }
        rangeEvents.value = fetchedEvents.rangeEvents || []; // Utiliser les événements hydratés et filtrés

        // --- Message Error Handling for VisualizeView ---
        const missingMessageErrors = fetchedEvents.missingMessageErrors ?? []; // S'assurer que c'est un tableau
        if (missingMessageErrors.length > 0) {
            const missingMessageErrorsForLog = missingMessageErrors.map(errorDetail => ({
                errorType: 'MissingMessage',
                messageId: errorDetail.messageId,
                anchorIncrement: errorDetail.anchorIncrement,
                eventId: errorDetail.eventId,
                timestamp: new Date().toISOString(),
                description: errorDetail.description,
            }));

            try {
                await invoke('save_error_event', { circuitId: props.circuitId, newErrors: missingMessageErrorsForLog });
            } catch (error) {
                console.error("Failed to save missing message errors to file:", error);
            }
        }
    }

    if (!fetchedLineString || !fetchedTrackingData || fetchedTrackingData.length < 2) {
      console.error("Failed to load valid circuit data.");
      return;
    }

    lineStringRef.value = fetchedLineString;
    trackingDataRef.value = fetchedTrackingData;

    const processedData = await invoke('process_tracking_data', {
        lineStringGeojson: fetchedLineString,
        trackingPointsJs: fetchedTrackingData
    });

    trackingPointsWithDistanceRef.value = processedData.processedPoints;
    totalDistanceRef.value = processedData.totalDistanceKm;
    mainTraceTotalDistanceValue.value = processedData.totalDistanceKm;
    totalDurationAt1xRef.value = totalDistanceRef.value * animationSpeed.value;

    controlPointIndicesRef.value = trackingPointsWithDistanceRef.value.reduce((acc, p, index) => {
        if (p.pointDeControl) acc.push(index);
        return acc;
    }, []);

    // Sauvegarde du contexte de la trace maîtresse pour les variantes
    mainTraceState.value = {
        lineString: fetchedLineString,
        trackingPoints: [...trackingPointsWithDistanceRef.value],
        totalDistance: processedData.totalDistanceKm,
        totalDuration: processedData.totalDistanceKm * (animationSpeed?.value || 10), // Fallback if ref not yet up
        pauseIncrements: [...pauseIncrements.value],
        flytoEvents: JSON.parse(JSON.stringify(flytoEvents.value)),
        rangeEvents: [...rangeEvents.value],
        accumulatedTime: 0,
        isPaused: true // Par défaut au démarrage
    };

    // Charger les métadonnées de segments superposés
    try {
        const metadata = await invoke('get_segment_metadata', { circuitId: props.circuitId });
        segmentMetadata.value = metadata;
        
        if (metadata && metadata.overlappingZones && metadata.overlappingZones.length > 0) {
            console.log(`[Visualize] ${metadata.overlappingZones.length} zone(s) de superposition chargée(s)`);
            metadata.overlappingZones.forEach(zone => {
                console.log(`  Zone ${zone.zoneId}: Aller ${zone.allerStartKm.toFixed(1)}-${zone.allerEndKm.toFixed(1)}km, Retour ${zone.retourStartKm.toFixed(1)}-${zone.retourEndKm.toFixed(1)}km`);
            });
        } else {
            console.log('[Visualize] Aucune zone de superposition pour ce circuit');
        }
    } catch (err) {
        console.warn('[Visualize] Pas de métadonnées de segments disponibles:', err);
        segmentMetadata.value = null;
    }

    if (!trackingDataRef.value[0]) {
        console.error("Initial tracking data point is undefined.");
        return;
    }

    // Initialize Weather
    await initWeather(currentCircuit, trackingPointsWithDistanceRef.value);



    // --- Trace Color Logic avec 3 layers ---
    // On charge maintenant une FeatureCollection de segments colorés (backend Refonte Phase 7)


    if (colorTraceBySlope.value) {
        try {
            const slopeColors = {
                TrancheNegative: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative')),
                Tranche1: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1')),
                Tranche2: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2')),
                Tranche3: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3')),
                Tranche4: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4')),
                Tranche5: toHexImproved(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5')),
            };

            coloredSegmentsGeoJsonRef.value = await invoke('get_colored_segments_geojson', {
                circuitId: props.circuitId,
                slopeColors: slopeColors,
                segmentLength: segmentLength.value,
            });

            if (coloredSegmentsGeoJsonRef.value) {
                console.log('[Visualize] Segmented GeoJSON loaded');
                if (map) setupMapLayersAndSources();
            } else {
                console.warn("Failed to generate segmented GeoJSON.");
            }
        } catch (e) {
            console.error("Error getting colored segments:", e);
        }
    }


    map = new mapboxgl.Map({
      container: mapContainer.value,
      style: styleLancement.value,
      center: centerEurope.value,
      zoom: zoomEurope.value,
      pitch: 0,
      bearing: 0,
      interactive: false, // Désactiver l'interaction au démarrage
    });



    map.on('style.load', async () => {
      await setupMapLayersAndSources();
    });

    map.on('load', async () => {
      // map.on('load') sequence continues...
      // Les sources et layers sont déjà gérés par style.load via setupMapLayersAndSources

      // Update camera bearing on every move (covers rotate, flyTo, manual interaction)
      map.on('move', () => {
          currentCameraBearing.value = map.getBearing();
      });

      // --- Séquence d'animation d'initialisation ---
      isInitializing.value = true;
      map.interactive = false; // Désactiver l'interaction pendant l'animation

      const traceBbox = turf.bbox(lineStringRef.value);
      const startCameraOptions = {
          center: trackingPointsWithDistanceRef.value[0].coordonnee,
          zoom: trackingPointsWithDistanceRef.value[0].editedZoom ?? trackingPointsWithDistanceRef.value[0].zoom,
          pitch: trackingPointsWithDistanceRef.value[0].editedPitch ?? trackingPointsWithDistanceRef.value[0].pitch,
          bearing: trackingPointsWithDistanceRef.value[0].editedCap ?? trackingPointsWithDistanceRef.value[0].cap,
      };

      // Définir la vue initiale de l'Europe (sans animation)
      map.setCenter(centerEurope.value);
      map.setZoom(zoomEurope.value);
      map.setPitch(0);
      map.setBearing(0);

      // Court délai pour s'assurer que la carte rend l'état initial
      await new Promise(resolve => setTimeout(resolve, 500));

      // Séquence 1: Vol vers l'aperçu de la trace
      animationState.value = 'Vol_Vers_Vue_Globale';
      
      // Préparer l'affichage des messages avec anchorIncrement === 0
      // Ils apparaîtront pendant les derniers 33% du flyTo (de 66% à 100%)
      const { atKm0, nearKm0 } = getMessagesForKm0();
      if (atKm0.length > 0) {
        const delayBeforeStart = durationEuropeToTrace.value * 0.66; // Démarrer à 66%
        const fadeDuration = durationEuropeToTrace.value * 0.34; // Durer 34% (jusqu'à 100%)
        
        setTimeout(() => {
          displayMessagesWithFade(atKm0, fadeDuration);
        }, delayBeforeStart);
      }
      
      const globalView = map.cameraForBounds(traceBbox, { padding: 40, bearing: 0, pitch: 0 });
      globalTraceCameraOptions.value = globalView;

      await flyToPromise(map, {
          pitch: 0,
          bearing: 0,
          duration: durationEuropeToTrace.value,
          ...globalView
      });

      // Séquence 2: Pause sur la vue globale (messages atKm0 déjà visibles)
      animationState.value = 'Pause_Observation';
      
      
      await new Promise(resolve => setTimeout(resolve, pauseBeforeStart.value));

      
      // Séquence 3: Vol vers le début de la trace (km 0)
      animationState.value = 'Vol_Vers_Depart';



      // Changement de style avant le vol vers le départ
      if (mapStyle.value !== styleLancement.value) {
          map.setStyle(mapStyle.value);
          await new Promise(resolve => map.once('style.load', resolve));
      }
      
      // Afficher les messages avec anchorIncrement !== 0 pendant le flyTo
      if (nearKm0.length > 0) {
        displayMessagesWithFade(nearKm0, durationTraceToStart.value); // Pas de await, animation en parallèle
      }
      
      await flyToPromise(map, {
          ...startCameraOptions,
          duration: durationTraceToStart.value,
      });

      // Séquence 4: Afficher l'interface utilisateur et démarrer l'animation après une pause
      animationState.value = 'En_Pause_au_Depart';
      isInitializing.value = false;
      isPaused.value = true; // On reste en pause le temps du timer
      map.interactive = true;
      distanceDisplay.value = '0.00';

      // Mettre à jour l'affichage de la commune pour le km0
      if (shouldShowCommuneWidget.value && trackingPointsWithDistanceRef.value[0]?.commune) {
          currentCommuneName.value = trackingPointsWithDistanceRef.value[0].commune;
      }

      // Stocker l'état initial de la caméra pour la reprise
      pausedCameraOptions.value = {
          center: map.getCenter(),
          zoom: map.getZoom(),
          pitch: map.getPitch(),
          bearing: map.getBearing(),
      };
      // Commencer à écouter les interactions
      map.on('move', onMapInteraction);
      map.on('zoom', onMapInteraction);
      map.on('pitch', onMapInteraction);
      map.on('rotate', onMapInteraction);

      animationFrameId = requestAnimationFrame(animate);

      // Démarrage automatique après la pause définie
      const pauseMs = pauseAuKm0.value;
      if (pauseMs > 0) {
          await new Promise(resolve => setTimeout(resolve, pauseMs));
      }
      isPaused.value = false; // Démarrage de l'animation
    });

  } catch (error) {
    console.error("Error during visualization setup:", error);
  }
};

const handleMouseMove = () => {
    if (isCursorHidden.value) {
        isCursorHidden.value = false;
    }
    clearTimeout(cursorTimer);
    if (masquerCurseurDelai.value) {
        cursorTimer = setTimeout(() => {
            isCursorHidden.value = true;
        }, masquerCurseurDelai.value);
    }
};

onMounted(() => {
  interruptUpdate(); // Interrupt commune update task
  window.addEventListener('keydown', handleKeyDown);
  window.addEventListener('keyup', handleKeyUp);

  const setupRemoteListeners = async () => {
    // Bloquer le scroll global du body
    document.body.style.overflow = 'hidden';

    unlistenFunctions.push(await listen('remote_command::toggle_play', () => {
        if (isInitializing.value || isAnimationFinished.value) return;
        isPaused.value = !isPaused.value;
    }));
    unlistenFunctions.push(await listen('remote_command::toggle_commands_widget', () => {
        if (isInitializing.value) return;
        isControlsCardVisible.value = !isControlsCardVisible.value;
    }));
    unlistenFunctions.push(await listen('remote_command::toggle_altitude_profile', () => {
        if (isInitializing.value) return;
        isAltitudeVisible.value = !isAltitudeVisible.value;
    }));
    unlistenFunctions.push(await listen('remote_command::toggle_communes_display', () => {
        if (isInitializing.value) return;
        isCommuneWidgetVisible.value = !isCommuneWidgetVisible.value;
    }));
    unlistenFunctions.push(await listen('remote_command::toggle_distance_display', () => {
        if (isInitializing.value) return;
        isDistanceDisplayVisible.value = !isDistanceDisplayVisible.value;
    }));
    unlistenFunctions.push(await listen('remote_command::toggle_weather_static', () => {
        if (isInitializing.value) return;
        isWeatherInfoVisible.value = !isWeatherInfoVisible.value;
    }));
    unlistenFunctions.push(await listen('remote_command::toggle_weather_dynamic', () => {
        if (isInitializing.value) return;
        isCompassVisible.value = !isCompassVisible.value;
    }));
    unlistenFunctions.push(await listen('remote_command::toggle_home', () => {
        if (isInitializing.value) return;
        toggleBackButtonVisibility();
    }));
    unlistenFunctions.push(await listen('remote_command::restart_animation', () => {
        if (isAnimationFinished.value) {
            resetAnimation();
        }
    }));
    unlistenFunctions.push(await listen('remote_command::update_speed', (event) => {
        console.log("Received remote_command::update_speed event:", event);
        if (isInitializing.value || isAnimationFinished.value) return;
        currentSpeed.value = Math.min(maxSpeedValue.value, Math.max(minSpeedValue.value, parseFloat(event.payload.speed)));
    }));
    unlistenFunctions.push(await listen('remote_command::set_speed_to_1x', () => {
        if (isInitializing.value || isAnimationFinished.value) return;
        currentSpeed.value = defaultSpeedValue.value;
    }));
    unlistenFunctions.push(await listen('remote_command::update_camera', (event) => {
        if (!isPaused.value || !map) return;
        const payload = event.payload;
        const dx = payload.dx || 0;
        const dy = payload.dy || 0;

        switch (payload.type) {
            case 'pan':
                const panX = dx * (sensibilityPointDeVueX.value / 100) * -1;
                const panY = dy * (sensibilityPointDeVueY.value / 100) * -1;
                console.log('Pan - dx:', dx, 'dy:', dy, 'sensibilityPointDeVueX:', sensibilityPointDeVueX.value, 'sensibilityPointDeVueY:', sensibilityPointDeVueY.value, 'panX:', panX, 'panY:', panY);
                map.panBy([panX, panY], { duration: 0 });
                break;
            case 'bearing':
                const bearingDelta = dx * (sensibilityCap.value / 100) * -1;
                console.log('Bearing - dx:', dx, 'sensibilityCap:', sensibilityCap.value, 'bearingDelta:', bearingDelta);
                map.setBearing(map.getBearing() + bearingDelta);
                break;
            case 'zoom':
                const zoomDelta = dy * (sensibilityZoom.value / 1000) * -1;
                console.log('Zoom - dy:', dy, 'sensibilityZoom:', sensibilityZoom.value, 'zoomDelta:', zoomDelta);
                let newZoom = map.getZoom() + zoomDelta;
                if (isPaused.value) { // Only apply minimum zoom when paused
                    newZoom = Math.max(newZoom, zoomMinimum.value);
                }
                map.setZoom(newZoom);
                break;
            case 'tilt':
                const tiltDelta = dy * (sensibilityTilt.value / 100) * -1;
                console.log('Tilt - dy:', dy, 'sensibilityTilt:', sensibilityTilt.value, 'tiltDelta:', tiltDelta);
                map.setPitch(map.getPitch() + tiltDelta);
                break;
        }
    }));
    unlistenFunctions.push(await listen('remote_command::start_rewind', () => {
        if (isInitializing.value || isAnimationFinished.value) return;
        isRewinding.value = true;
    }));
    unlistenFunctions.push(await listen('remote_command::stop_rewind', () => {
        if (isInitializing.value || isAnimationFinished.value) return;
        isRewinding.value = false;
    }));
  };

  setupRemoteListeners();

  // Send initial state to the backend
  sendVisualizeViewStateUpdate();

  const unwatchSettings = watch(settings, (newSettings) => {
    if (newSettings) {
      // --- Update Widget Visibility from Settings ---
      isDistanceDisplayVisible.value = getSettingValue('Visualisation/Widgets/distance') ?? true;
      isControlsCardVisible.value = getSettingValue('Visualisation/Widgets/commandes') ?? true;
      isCommuneWidgetVisible.value = getSettingValue('Visualisation/Widgets/communes') ?? true;
      isAltitudeVisible.value = getSettingValue('Visualisation/Widgets/altitude') ?? true;
      
      const staticW = getSettingValue('Visualisation/Météo/Widgets/informationMeteo') ?? true;
      isStaticWeatherVisible.value = staticW;
      isWeatherInfoVisible.value = staticW;

      const dynamicW = getSettingValue('Visualisation/Météo/Widgets/boussole') ?? true;
      isDynamicWeatherVisible.value = dynamicW;
      isCompassVisible.value = dynamicW;

      // --- Cursor hide logic ---
      if (masquerCurseurDelai.value != null) {
        if (mapContainer.value) {
          mapContainer.value.addEventListener('mousemove', handleMouseMove);
          handleMouseMove(); // Initial call
        }
      }

      // --- Map init logic ---
      if (mapContainer.value && !isMapInitialized) {
        isMapInitialized = true;
        initializeMap();
      }

      // We've done all initial setup based on settings, so we can stop watching.
      nextTick(() => {
        if (unwatchSettings) {
            unwatchSettings();
        }
      });
    }
  }, { immediate: true });
});


// Watcher to handle FlyTo on Resume at Segment Boundaries
watch(isPaused, async (paused) => {
    console.log('[isPaused Watcher] Triggered. paused=', paused, 'isMultisegment=', isMultisegmentVariant.value);
    
    if (!paused) {
        console.log('[isPaused Watcher] Resuming animation...');
        // Resuming...
        if (isMultisegmentVariant.value && activeVariantSegments.value.length > 0) {
             const seg = activeVariantSegments.value.find(s => s.index === currentSegmentIndex.value);
             
             if (seg && seg.uiIndex > 0) { // Not first segment
                 // Check if distance is effectively at start of this segment
                 if (Math.abs(currentDistanceInMeters.value/1000 - seg.startDistKm) < 0.02) {
                      if (seg.firstPoint && map) {
                           console.log('[Resume Watcher] FORCING cursor update to segment start:', seg.startDistKm * 1000);
                           
                           // CRITICAL: Block animate BEFORE updating distance
                           isTransitioning.value = true;
                           
                           // FORCE cursor update BEFORE FlyTo
                           currentDistanceInMeters.value = seg.startDistKm * 1000;
                           
                           // Wait for Vue reactivity
                           await new Promise(resolve => setTimeout(resolve, 50));
                           
                           console.log('[Resume Watcher] STARTING FlyTo to segment:', seg.uiIndex);
                           
                           await flyToPromise(map, {
                               center: seg.firstPoint.coordonnee,
                               zoom: seg.firstPoint.editedZoom ?? 16,
                               pitch: seg.firstPoint.editedPitch ?? 60,
                               bearing: seg.firstPoint.editedCap ?? 0,
                               duration: 1500
                           });
                           
                           console.log('[Resume Watcher] FlyTo COMPLETED');
                           isTransitioning.value = false;
                      }
                 }
             }
        }
    }
});

onUnmounted(() => {
  // Restaurer le scroll global du body
  document.body.style.overflow = '';

  unlistenFunctions.forEach(unlisten => unlisten());
  window.removeEventListener('keydown', handleKeyDown);
  window.removeEventListener('keyup', handleKeyUp);
  if (mapContainer.value) {
      mapContainer.value.removeEventListener('mousemove', handleMouseMove);
  }
  clearTimeout(cursorTimer);
  if (animationFrameId) cancelAnimationFrame(animationFrameId);
  activePopups.forEach(popup => popup.remove());
  activePopups.clear();
  if (map) map.remove();
  map = null;
  isMapInitialized = false;
});
</script>

<style>
#map-container {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 100%;
}

.hide-cursor {
  cursor: none;
}

.back-button {
  position: absolute !important;
  top: 20px;
  left: 20px;
  z-index: 1;
  pointer-events: auto;
}

.distance-display {
  pointer-events: auto;
  width: fit-content;
  height: 48px; /* Force height to match button */
  max-height: 60px;
  overflow: hidden;
}

.top-center-container {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1; /* Ensure it's above map */
  display: flex;
  align-items: flex-start;
  gap: 20px; /* Space between widgets */
  pointer-events: none; /* Let clicks pass through gaps */
}

.bottom-center-container {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1;
  display: flex;
  flex-direction: column-reverse;
  align-items: center;
  /* gap: 8px; Removed to fix animation jump */
  pointer-events: none;
}

.bottom-controls {
  pointer-events: auto;
  margin-top: 8px;
}

.altitude-svg-container {
    justify-content: center; /* Center the child if it's smaller */
    pointer-events: auto;
    background-color: rgba(0, 0, 0, 0.7);
    border-radius: 5px;
    margin-top: 8px; /* Replaces flex gap */
    max-height: 500px;
    overflow: hidden; /* Ensure animation clipping */
}

.altitude-profile-container {
    position: absolute;
    bottom: 80px; /* Position above the bottom controls */
    left: 50%;
    transform: translateX(-50%);
    width: 80%; /* Or as desired */
    z-index: 1;
    pointer-events: auto;
    background-color: rgba(0, 0, 0, 0.7);
    border-radius: 5px;
}

.controls-card {
    pointer-events: auto;
}

.speed-display-text {
    font-family: monospace;
    font-size: 0.9em;
    padding: 0 8px;
    min-width: 45px; /* Ensure space doesn't jump around */
    text-align: center;
}

.speed-slider {
    width: 200px; /* Ajustez cette valeur selon vos besoins */
}

/* Remove the default white box and pointer/tip from our custom popups */
.map-message-popup .mapboxgl-popup-content {
  background: none;
  padding: 0;
  box-shadow: none;
}

.map-message-popup .mapboxgl-popup-tip {
  display: none;
}

/* Hide mapbox logo/attribution for cleaner view, but ensure it's compliant with Mapbox terms */
.mapboxgl-ctrl-bottom-left, .mapboxgl-ctrl-bottom-right {
  display: none;
}

.commune-display {
  position: absolute;
  top: 20px;
  left: 80px; /* Position next to the back button (20px + ~48px button + gap) */
  width: 250px;
  height: 48px; /* Match standard button height for visual alignment */
  max-height: 60px;
  overflow: hidden;
  background-color: white;
  border-width: 4px;
  border-style: solid;
  border-radius: 5px;
  color: black;
  padding: 4px;
  z-index: 1;
  pointer-events: auto;
  display: flex;
  align-items: center;
  justify-content: center;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.75s ease, max-height 0.75s ease, margin 0.75s ease, padding 0.75s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  max-height: 0 !important;
  margin-top: 0 !important;
  margin-bottom: 0 !important;
  padding-top: 0 !important;
  padding-bottom: 0 !important;
}

.fade-opacity-enter-active,
.fade-opacity-leave-active {
  transition: opacity 0.75s ease;
}

.fade-opacity-enter-from,
.fade-opacity-leave-to {
  opacity: 0;
}

/* Fade-in animation for messages during initialization */
.fade-in-message {
  opacity: 0;
}
</style>
