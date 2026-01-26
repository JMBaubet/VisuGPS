<template>
  <VisualizeMapArea
    ref="visualizeMapAreaRef"
    :is-cursor-hidden="isCursorHidden"
    :is-center-marker-visible="isCenterMarkerVisible"
    :center-marker-color="couleurCroixCentrale"
    :show-back-button="!isInitializing && isBackButtonVisibleFinal"
    @go-back="goBack"
    @register-map-container="(el) => { mapContainer = el }"
  />

  <VisualizeInfoDisplay
    :is-visible="!isInitializing"
    :show-distance="isDistanceDisplayVisible"
    :distance-display="distanceDisplay"
    :total-distance="totalDistanceRef / 1000"
    :show-commune="shouldShowCommuneWidget && isCommuneWidgetVisible"
    :commune-name="currentCommuneName"
    :commune-border-color="communeWidgetBorderColor"
  />

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
            :current-distance="currentDistanceInMeters / 1000"
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

  <VisualizeControls
    v-if="!isInitializing"
    :is-visible="!isInitializing"
    :is-altitude-visible="isAltitudeVisible"
    v-model:is-paused="isPaused"
    :is-animation-finished="isAnimationFinished"
    v-model:is-rewinding="isRewinding"
    v-model:current-speed="currentSpeed"
    :controls-visible="isControlsCardVisible"
    :min-speed="minSpeedValue"
    :max-speed="maxSpeedValue"
    :default-speed="defaultSpeedValue"
    @reset="resetAnimation"
    @trigger-final-view="handleEndSequence"
  >
    <template #altitude-chart>
        <AltitudeVariantSVG 
            :key="`altitude-variant-${props.circuitId}`"
            :current-distance="currentDistanceInMeters" 
            :total-main-distance="masterTraceTotalDistance"
            :main-trace-points="masterTrackingPoints"
            :abandoned-segments="abandonedSegmentsRef"
            :variant-blue-segments="variantBlueSegmentsRef"
        />
    </template>


    <template #extra-controls>
        <!-- Variant Switching & Return -->
        <v-divider vertical class="mx-2"></v-divider>
        
        <!-- Select another variant (if multiple) -->
        <v-btn v-if="availableVariants.length > 1" 
            icon="mdi-format-list-bulleted" 
            variant="text" 
            title="Changer de variante" 
            @click="showVariantSelection = true">
        </v-btn>

        <!-- Return to Main Trace -->
        <v-btn icon="mdi-arrow-u-left-top" 
            variant="text" 
            color="secondary"
            title="Retour Trace Principale" 
            :disabled="!isPaused"
            @click="returnToMainTrace">
        </v-btn>
    </template>
  </VisualizeControls>

    <!-- Variant Selection Dialog -->
    <v-dialog v-model="showVariantSelection" persistent max-width="500" scrim="black" opacity="0.5">
        <v-card>
            <v-card-title class="text-h5 bg-primary text-white">Choisir une variante</v-card-title>
            <v-list>
                <v-list-item v-for="v in availableVariants" :key="v.id" @click="selectVariant(v.id)" link>
                    <template v-slot:prepend>
                        <v-icon icon="mdi-source-branch" color="primary"></v-icon>
                    </template>
                    <v-list-item-title>{{ v.name }}</v-list-item-title>
                    <v-list-item-subtitle>
                        {{ (v.stats.totalDistance).toFixed(1) }} km • {{ v.stats.totalAscent.toFixed(0) }}m D+
                    </v-list-item-subtitle>
                </v-list-item>
            </v-list>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn text @click="goBack">Annuler</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch, nextTick, shallowRef } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import mapboxgl from 'mapbox-gl';
import * as turf from '@turf/turf';
import { useSettings } from '@/composables/useSettings';
import { useSnackbar } from '@/composables/useSnackbar';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import { useSharedUiState } from '@/composables/useSharedUiState';
import { useMessageDisplay } from '@/composables/useMessageDisplay.js';
import AltitudeSVG from '@/components/Visualize/AltitudeSVG.vue';
import AltitudeVariantSVG from '@/components/Visualize/AltitudeVariantSVG.vue';
import WeatherWidgetDynamic from '@/components/Visualize/WeatherWidgetDynamic.vue';
import WeatherWidgetStatic from '@/components/Visualize/WeatherWidgetStatic.vue';
import WeatherService from '@/services/WeatherService';
// Shared Components
import VisualizeMapArea from '@/components/Visualize/Shared/VisualizeMapArea.vue';
import VisualizeInfoDisplay from '@/components/Visualize/Shared/VisualizeInfoDisplay.vue';
import VisualizeControls from '@/components/Visualize/Shared/VisualizeControls.vue';

// --- Nouveaux Composables ---
import { useMapEngine } from '@/composables/visualize/useMapEngine.js';
import { useCameraManager } from '@/composables/visualize/useCameraManager.js';
import { useCircuitData } from '@/composables/visualize/useCircuitData.js';
import { useTraceLayers } from '@/composables/visualize/useTraceLayers.js';
import { useAnimationController } from '@/composables/visualize/useAnimationController.js';
import { useCameraInterpolator } from '@/composables/visualize/useCameraInterpolator.js'; // NEW

const props = defineProps({
  circuitId: { type: String, required: true },
  variantId: { type: String, default: null } // Optional
});

const router = useRouter();
const route = useRoute();

const showVariantSelection = ref(false);
const availableVariants = ref([]);
const selectedVariantId = ref(props.variantId || route.query.variantId);

const selectVariant = (id) => {
    selectedVariantId.value = id;
    showVariantSelection.value = false;
    // Update URL without reloading? Or just continue.
    // router.replace({ name: 'VisualizeVariant', params: { circuitId: props.circuitId, variantId: id } });
    initializeVisualization();
};
const { settings, getSettingValue } = useSettings();
const { showSnackbar } = useSnackbar();
const { interruptUpdate } = useCommunesUpdate();
const { toHex } = useVuetifyColors();
const { createMessageSVG } = useMessageDisplay();
const { isBackButtonVisible, toggleBackButtonVisibility } = useSharedUiState();

// --- Initialization + Refs ---
const mapContainer = ref(null);
const isCursorHidden = ref(false); // Fix warning
const animationState = ref('Initialisation'); // Initialisation, Vol_Vers_Vue_Globale, Vol_Vers_Depart, En_Animation, En_Pause, Termine
const hasVariants = ref(false);

// --- Settings Computed ---
const mapboxToken = computed(() => getSettingValue('Système/Tokens/mapbox'));
const mapStyle = computed(() => getSettingValue('Visualisation/Vue 3D/Carte/styleVisualisation'));
const styleLancement = computed(() => getSettingValue('Visualisation/Lancement/styleLancement'));
// Use Edition setting as verified source
const terrainExaggeration = computed(() => getSettingValue('Edition/Vue 3D/Carte/exaggeration')); 
const centerEurope = computed(() => getSettingValue('Visualisation/Lancement/centerEurope'));
const zoomEurope = computed(() => getSettingValue('Visualisation/Lancement/zoomEurope'));
const durationEuropeToTrace = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/durationEuropeToTrace')));
const durationTraceToStart = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/durationTraceToStart')));
const pauseBeforeStart = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/pauseBeforeStart')));
const pauseAuKm0 = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/pauseAuKm0'))); 
const flyToKm0Duration = computed(() => formatDuration(getSettingValue('Visualisation/Finalisation/flyToKm0Duration'))); // Fixed path
const flyToGlobalDuration = computed(() => formatDuration(getSettingValue('Visualisation/Finalisation/flyToGlobalDuration'))); // Fixed path
const delayAfterAnimationEnd = computed(() => formatDuration(getSettingValue('Visualisation/Finalisation/delayAfterAnimationEnd'))); // Fixed path
const traceWidth = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/epaisseurTrace')); // Fixed path
const traceOpacity = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/opaciteTrace')); // Fixed path
const traceColor = computed(() => toHex(getSettingValue('Visualisation/Vue 3D/Trace/couleurTrace'))); // Fixed path
const cometWidth = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/epaisseurComete')); // Fixed path
const cometColor = computed(() => toHex(getSettingValue('Visualisation/Vue 3D/Trace/couleurComete'))); // Fixed path
const cometOpacity = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/opaciteComete')); // Fixed path
const cometLength = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/longueurComete')); // Fixed path
const dynamicZoomIntensity = computed(() => getSettingValue('Visualisation/Lecture/ZoomDynamique/intensite_zoom_dynamique') || 10);
const colorTraceBySlope = computed(() => getSettingValue('Visualisation/Vue 3D/Trace/colorerSelonPente')); // Fixed path - wait, verify Visualisation vs Edition?
const segmentLength = computed(() => getSettingValue('Importation/Tracking/LongueurSegment') || 100); // Fixed path
const jumpDuration = computed(() => getSettingValue('Visualisation/Lecture/jumpDuration') ?? 2.0);

// --- Variant Visualization Settings ---
const showSegments = computed(() => getSettingValue('Variante/Visualisation/afficherSegments'));
const showSlope = computed(() => getSettingValue('Variante/Visualisation/afficherPente'));
const segmentThickness = computed(() => getSettingValue('Variante/Visualisation/epaisseurSegments'));
const segmentOpacity = computed(() => getSettingValue('Variante/Visualisation/opaciteSegments'));
const slopeThickness = computed(() => getSettingValue('Variante/Visualisation/epaisseurPente'));
const slopeOpacity = computed(() => getSettingValue('Variante/Visualisation/opacitePente'));
const colorNew = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurNouveau')));
const colorCommon = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurCommun')));
const colorAbandoned = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurAbandonne')));
const colorTraceVariant = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurTrace')));
const showAbandoned = computed(() => getSettingValue('Variante/Visualisation/afficherSegmentAbandonne'));

const formatDuration = (val) => (val > 100 ? val : val * 1000);

// --- Using New Composables ---
// 1. Map Engine
const { map, isMapLoaded, initializeMap: initMapEngine, flyToPromise, cleanupMap } = useMapEngine(mapContainer, mapboxToken, mapStyle, terrainExaggeration);

// 2. Camera Manager
const { pausedCameraOptions, currentCameraBearing: camBearing, saveCameraState, restoreCameraState, enableInteraction, disableInteraction, startBearingTracking } = useCameraManager(map);
const currentCameraBearing = computed(() => camBearing.value);
const currentOrientationMode = ref('Trace');

// 3. Circuit Data
const { loadCircuitData, processTrackingData, lineStringRef, trackingPointsWithDistanceRef, eventsRef, segmentMetadata } = useCircuitData();
const totalDistanceRef = ref(0); 
const totalDurationAt1xRef = ref(0);
const controlPointIndicesRef = ref([]);
const pauseIncrements = ref([]);
const flytoEvents = ref({});
const rangeEvents = ref([]);
const currentCircuitRef = shallowRef(null);
const triggeredPauseIncrement = ref(null);
const triggeredFlytoIncrement = ref(null);
const isFlytoActive = ref(false); // Mode FlyTo exclusif
const preFlytoCameraOptions = ref(null);

// 4. Trace Layers
const { setupTraceLayers, updateLayerVisibility, updateTraceOverlapVisibility, updateVariantSlopeMode, updateVariantStyle, coloredSegmentsGeoJsonRef, slopeExpressionRef } = useTraceLayers(map);

// 5. Animation Controller
// Note: accumulatedTime can be manipulated directly via composable exposed ref if needed
const { isPaused, isRewinding, isAnimationFinished, currentSpeed, currentDistanceInMeters, distanceDisplay, currentTraceBearing, startAnimation, pauseAnimation, resetTime, updateTime, accumulatedTime, setTimeFromDistance } = useAnimationController();

// 6. Camera Interpolator (NEW)
const { updateCameraPosition } = useCameraInterpolator(map);

const modifications = ref([]);
const fullRouteGeoJson = ref(null);
const masterTraceGeoJson = ref(null);
const variantStats = ref({ total: 0, current: 0 });

// New Refs for AltitudeVariantSVG
const masterTraceTotalDistance = ref(1);
const masterTrackingPoints = ref([]);
const abandonedSegmentsRef = ref([]);
const variantBlueSegmentsRef = ref([]);



// --- Legacy State ---
const isInitializing = ref(true);
const avancementCommunes = ref('Non calculé');
const activePopups = new Map();
let unlistenFunctions = [];

// Widgets State
const isDistanceDisplayVisible = ref(true);
const isControlsCardVisible = ref(true);
const isCommuneWidgetVisible = ref(true);
const isAltitudeVisible = ref(true);
const isWeatherInfoVisible = ref(true); 
const isCompassVisible = ref(true);
const isStaticWeatherVisible = ref(true);
const isDynamicWeatherVisible = ref(true);
const isCenterMarkerVisible = computed(() => getSettingValue('Visualisation/Lecture/afficherCroixCentrale') && isPaused.value);
const couleurCroixCentrale = computed(() => getSettingValue('Visualisation/Lecture/couleurCroixCentrale'));
const zoomMinimum = computed(() => (getSettingValue('Visualisation/Lecture/zoomMinimum') ?? 100) / 10);

const currentCommuneName = ref("N/A");
const shouldShowCommuneWidget = computed(() => (avancementCommunes.value === 'Terminé' || avancementCommunes.value == 100) && currentCommuneName.value !== "N/A");
const communeWidgetBorderColor = computed(() => '#F44336'); 

const isBackButtonVisibleFinal = computed(() => isBackButtonVisible.value && (animationState.value === 'En_Pause' || animationState.value === 'Termine' || animationState.value === 'En_Phase_au_Depart'));

// Weather State
const weatherForecasts = ref([]);
const simulationStartDate = ref(null);
const currentWeather = ref(null);
const showWeatherTable = ref(false);
const circuitScenarios = ref([]);

// Speed Slider Constants
const minSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/min_value'));
const maxSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/max_value'));
const defaultSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/default_value'));

// --- Methods ---

const returnToMainTrace = () => { 
    let query = { directStart: 'true' };
    if (map.value) {
        const center = map.value.getCenter();
        query = {
            ...query,
            lat: center.lat,
            lng: center.lng,
            zoom: map.value.getZoom(),
            bearing: map.value.getBearing(),
            pitch: map.value.getPitch()
        };
    }
    router.push({ 
        name: 'Visualize', 
        params: { circuitId: props.circuitId },
        query: query
    }); 
};
const goBack = () => { router.push({ name: 'Main' }); };
const getToHexImproved = (n) => toHex(getSettingValue(n));

// --- User Interactions ---
const togglePlayPauseOrReset = () => {
    if (isAnimationFinished.value) {
        resetAnimation();
        isPaused.value = false;
    } else {
        isPaused.value = !isPaused.value;
    }
};

// --- Initialization Logic ---
const initializeVisualization = async () => {
    resetTime(); 
    try {
        // 0. Map Init (Pre-load to avoid black screen)
        let initialCenter = centerEurope.value;
        let initialZoom = zoomEurope.value;
        if (route.query.lat && route.query.lng && route.query.zoom) {
             initialCenter = [parseFloat(route.query.lng), parseFloat(route.query.lat)];
             initialZoom = parseFloat(route.query.zoom);
        }
        
        if (!map.value) {
            let instance = await initMapEngine(initialCenter, initialZoom);
            if(!instance) throw new Error("Map failed to init");
            instance.setMinZoom(zoomMinimum.value);
            
            if (route.query.bearing && route.query.pitch) {
                instance.jumpTo({
                    bearing: parseFloat(route.query.bearing),
                    pitch: parseFloat(route.query.pitch)
                });
            }
            startBearingTracking();
        }

        // 0.5 Ensure we have a variant ID
        if (!selectedVariantId.value) {
             showSnackbar("Aucune variante spécifiée.", "error");
             setTimeout(() => goBack(), 1000);
             return;
        }

        // 1. Load Data
        const { circuit } = await loadCircuitData(props.circuitId); // Metadata
        currentCircuitRef.value = circuit;
        if (circuit) avancementCommunes.value = circuit.avancementCommunes;

        // A. Load MASTER Trace (Source of truth for original segments - High Resolution)
        const masterGeoJson = await invoke('read_line_string_file', { circuitId: props.circuitId });
        masterTraceGeoJson.value = masterGeoJson;
        
        // Calculate Master Trace Total Distance (using Turf)
        if (masterGeoJson && masterGeoJson.coordinates) {
             const line = turf.lineString(masterGeoJson.coordinates);
             masterTraceTotalDistance.value = turf.length(line, { units: 'meters' });
        } else if (masterGeoJson && masterGeoJson.geometry && masterGeoJson.geometry.coordinates) {
             const line = turf.lineString(masterGeoJson.geometry.coordinates);
             masterTraceTotalDistance.value = turf.length(line, { units: 'meters' });
        }
        
        // A.0 Load MASTER Tracking Points (for Altitude Profile)
        const masterTrackingRaw = await invoke('read_tracking_file', { circuitId: props.circuitId, filename: 'tracking.json' });
        const masterProcessed = await invoke('process_tracking_data', { 
            lineStringGeojson: masterGeoJson,
            trackingPointsJs: masterTrackingRaw 
        });
        masterTrackingPoints.value = masterProcessed.processedPoints;


        // A. Load Variant Details (Metadata + Stats)
        const variantArchive = await invoke('get_variant_details', { circuitId: props.circuitId, variantId: selectedVariantId.value });
        modifications.value = variantArchive.modifications || [];
        variantStats.value = {
            total: (variantArchive.metadata?.stats?.totalDistance || 0),
            current: 0
        };

        // A.5 Load Overlap Metadata (Aller/Retour)
        try {
            segmentMetadata.value = await invoke('get_variant_overlap_metadata', { 
                circuitId: props.circuitId, 
                variantId: selectedVariantId.value 
            });
        } catch (e) {
            console.warn("[VisualizeVariant] Could not fetch variant overlap metadata. This is normal for old variants needing re-save:", e);
            segmentMetadata.value = null;
        }

        if (segmentMetadata.value) {
            console.log(`[VisualizeVariant] Metadata loaded: ${segmentMetadata.value.overlappingZones?.length || 0} overlap zones detected.`);
        }

        // B. Load RECONSTITUTED Geometry (FULL)
        const fullLineString = await invoke('read_line_string_file', { 
            circuitId: props.circuitId, 
            filename: `lineString_${selectedVariantId.value}_FULL.json` 
        });
        fullRouteGeoJson.value = fullLineString;
        lineStringRef.value = fullLineString; 

        // C. Load RECONSTITUTED Tracking (FULL)
        let trackingData = await invoke('read_tracking_file', { 
            circuitId: props.circuitId, 
            filename: `tracking_${selectedVariantId.value}_FULL.json` 
        });
        
        trackingPointsWithDistanceRef.value = trackingData;
        
        // Use exact total distance from tracking
        const nominalTotalKm = trackingData[trackingData.length - 1].distance;
        totalDistanceRef.value = nominalTotalKm * 1000;
        
        // D. Calculate Duration
        const msPerKm = getSettingValue('Visualisation/Lecture/vitesse') || 3730;
        totalDurationAt1xRef.value = nominalTotalKm * msPerKm; 

        // E. Populate control points
        controlPointIndicesRef.value = trackingData
            .map((p, i) => (p.pointDeControl ? i : -1))
            .filter(i => i !== -1);

        // Reset Events (Variants don't imply events yet)
        pauseIncrements.value = [];
        flytoEvents.value = {};
        rangeEvents.value = [];

        await initWeather(circuit, trackingPointsWithDistanceRef.value);



        // E. Generate Slope Segments (Backend)
        const slopeColors = {
                TrancheNegative: getToHexImproved('Visualisation/Profil Altitude/Couleurs/TrancheNegative'),
                Tranche1: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche1'),
                Tranche2: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche2'),
                Tranche3: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche3'),
                Tranche4: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche4'),
                Tranche5: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche5'),
         };
         
         const expression = await invoke('get_variant_slope_expression', { 
            circuitId: props.circuitId, 
            variantId: selectedVariantId.value,
            slopeColors: slopeColors
         });
         slopeExpressionRef.value = expression;

        // F. Generate Colored Segments (Slope-based + Aller/Retour)
        // This replaces the old get_variant_comparison_geojson to allow live slope color updates
        // and identical behavior to the main trace for Aller/Retour overlaps.
        try {
            const geojson = await invoke('get_colored_segments_geojson', { 
                circuitId: props.circuitId, 
                variantId: selectedVariantId.value,
                slopeColors: slopeColors,
                segmentLength: segmentLength.value
            });
            coloredSegmentsGeoJsonRef.value = geojson;
            
            coloredSegmentsGeoJsonRef.value = geojson;
            
            // Extract Abandoned Segments directly from Variant Modifications (Archive)
            const abandonedRanges = [];
            const segLen = segmentLength.value;
            
            if (modifications.value && modifications.value.length > 0) {
                 modifications.value.forEach(modif => {
                     let startIdx = 0;
                     let endIdx = 0;
                     let type = '';
                     
                     // Helper to check modification type keys (Using keys from logs: anchorIndexOnMaster, type)
                     if (modif.type === 'DEPART_DEPORTE') {
                         startIdx = 0;
                         endIdx = modif.anchorIndexOnMaster;
                         type = 'DEPART';
                     } else if (modif.type === 'ARRIVEE_REPORTEE') {
                         startIdx = modif.anchorIndexOnMaster;
                         endIdx = 999999; 
                         type = 'ARRIVEE';
                     } else if (modif.anchorStart && modif.anchorEnd) {
                         startIdx = modif.anchorStart.index;
                         endIdx = modif.anchorEnd.index;
                         type = 'SEGMENT';
                     } else if (modif.type === 'SEGMENT_DEVIATION') {
                         startIdx = modif.anchorStart?.index || 0;
                         endIdx = modif.anchorEnd?.index || 0;
                         type = 'SEGMENT';
                     }
                     
                     if (endIdx > startIdx) {
                         const startM = startIdx * segLen;
                         let endM = endIdx * segLen;
                         if (endM > masterTraceTotalDistance.value) endM = masterTraceTotalDistance.value;
                         abandonedRanges.push({ start: startM, end: endM });
                     }
                 });
            }
            
            abandonedSegmentsRef.value = abandonedRanges;
            
            // Extract Blue Variant Segments
            const blueSegments = [];
            
            if (trackingData && trackingData.length > 0 && modifications.value && modifications.value.length > 0) {
                // 1. Sort modifications by their position on the MASTER trace
                const sortedModifs = [...modifications.value].sort((a, b) => {
                    const getStart = (m) => {
                        if (m.type === 'DEPART_DEPORTE') return 0;
                        if (m.type === 'ARRIVEE_REPORTEE') return 9999999;
                        return m.anchorStart?.index || 0;
                    };
                    return getStart(a) - getStart(b);
                });

                // 2. Track cursors to calculate cumulative distance in VARIANT track
                let masterCursorIdx = 0;
                let variantCursorM = 0;

                sortedModifs.forEach(modif => {
                    let startAnchorIdx = 0;
                    let endAnchorIdx = 0;
                    let type = '';
                    
                    if (modif.type === 'DEPART_DEPORTE') {
                        type = 'DEPART';
                        startAnchorIdx = 0;
                        endAnchorIdx = modif.anchorIndexOnMaster;
                    } else if (modif.type === 'ARRIVEE_REPORTEE') {
                        type = 'ARRIVEE';
                        startAnchorIdx = modif.anchorIndexOnMaster;
                        endAnchorIdx = 9999999; // Will be clamped
                    } else {
                        type = 'SEGMENT';
                        startAnchorIdx = modif.anchorStart?.index || 0;
                        endAnchorIdx = modif.anchorEnd?.index || 0;
                    }

                    // A. The variant tracking contains a "common" section before this modification
                    // Distance of common section = (Start of modif - End of previous modif) * segmentLength
                    const commonLenM = (startAnchorIdx - masterCursorIdx) * segLen;
                    variantCursorM += commonLenM;

                    // B. The modification itself starts here
                    const variantStartM = variantCursorM;
                    const variantLenM = (modif.longueur || 0) * 1000;
                    const variantEndM = variantCursorM + variantLenM;

                    // C. Extract points from trackingData based on calculated distances
                    // Use a small 1m buffer for precision
                    const startIndex = trackingData.findIndex(p => p.distance * 1000 >= (variantStartM - 1));
                    const endIndex = trackingData.findLastIndex(p => p.distance * 1000 <= (variantEndM + 1));

                    if (startIndex !== -1 && endIndex !== -1 && endIndex >= startIndex) {
                        const slice = trackingData.slice(startIndex, endIndex + 1).map(p => ({
                            distance: p.distance,
                            altitude: p.altitude
                        }));
                        
                        blueSegments.push({
                            type,
                            // Junction point on Master is the anchor
                            anchorM: type === 'DEPART' ? endAnchorIdx * segLen : startAnchorIdx * segLen,
                            anchorEndM: type === 'SEGMENT' ? endAnchorIdx * segLen : 0,
                            lengthM: variantLenM,
                            points: slice
                        });
                    }

                    // D. Advance cursors
                    variantCursorM = variantEndM;
                    masterCursorIdx = endAnchorIdx;
                });
            }
            variantBlueSegmentsRef.value = blueSegments;
        } catch(e) {
            console.error("Colored segments load error", e);
            coloredSegmentsGeoJsonRef.value = { type: 'FeatureCollection', features: [] };
            abandonedSegmentsRef.value = [];
            variantBlueSegmentsRef.value = [];
        }

        // 3. Map Layers (Once data is loaded)
        setupTraceLayers({
            traceWidth: traceWidth.value, traceOpacity: traceOpacity.value, traceColor: colorTraceVariant.value, // Use Variant Trace Color
            coloredSegmentsData: coloredSegmentsGeoJsonRef.value,
            masterTraceData: masterTraceGeoJson.value, // Pass Master Trace for 'Main' layer
            cometWidth: cometWidth.value, cometColor: cometColor.value, cometOpacity: cometOpacity.value,
            slopeExpression: slopeExpressionRef.value,
            segmentThickness: segmentThickness.value, segmentOpacity: segmentOpacity.value,
            slopeThickness: slopeThickness.value, slopeOpacityLogic: slopeOpacity.value,
            colorNew: colorNew.value, colorCommon: colorCommon.value, colorAbandoned: colorAbandoned.value
        });

        // Apply Initial Visibility
        // Segments Layers (Main+Variant) controlled by showSegments
        updateLayerVisibility(true, showSegments.value);
        updateLayerVisibility('trace-main-abandoned', showAbandoned.value);
        
        // Slope Layer Mode (Gradient vs Flat Color)
        updateVariantSlopeMode(showSlope.value, { trace: colorTraceVariant.value });
        
        // Watches for live setting updates
        watch(showSegments, (newVal) => {
            updateLayerVisibility(true, newVal);
        });
        
        watch(showAbandoned, (newVal) => {
             updateLayerVisibility('trace-main-abandoned', newVal);
        });
        
        watch(showSlope, (newVal) => {
            updateVariantSlopeMode(newVal, { trace: colorTraceVariant.value });
        });
        
        // Watchers for Style Updates (Thickness, Opacity, Colors)
        watch([segmentThickness, segmentOpacity, colorNew, colorCommon, colorAbandoned, colorTraceVariant, slopeThickness, slopeOpacity], () => {
             updateVariantStyle({
                 thickness: segmentThickness.value,
                 opacity: segmentOpacity.value,
                 colorNew: colorNew.value,
                 colorCommon: colorCommon.value,
                 colorAbandoned: colorAbandoned.value,
                 colorTrace: colorTraceVariant.value,
                 slopeThickness: slopeThickness.value,
                 slopeOpacity: slopeOpacity.value
             });
             
             // Refresh Slope Mode to ensure correct color is applied if mode didn't change but color did
             updateVariantSlopeMode(showSlope.value, { trace: colorTraceVariant.value });
        });
        
        // Initial Overlap Check (to hide loops on start if any)
        checkLayers(0); 
        
        // 3. Animation Sequence (Simplified for Variants: Direct to Start)
        
        // Note: Map is already initialized with `mapStyle` (3D) in useMapEngine.
        // No need to switch from styleLancement.

        animationState.value = 'Vol_Vers_Depart';
        
        // Initial FlyTo directly to Start
        const startPoint = trackingPointsWithDistanceRef.value[0];
        
        // Initialize camera at a high view first for context? Or just direct?
        // User asked for "Directement faire un flyto vers le km 0".
        // Let's start from a reasonable zoomed out view of the start and zoom in.
        
        await flyToPromise({
            center: startPoint.coordonnee,
            zoom: 16, 
            pitch: 45,
            bearing: 0,
            duration: 3000 // A bit faster/smoother direct entry
        });



        animationState.value = 'En_Pause_au_Depart';
        isInitializing.value = false;
        enableInteraction();
        startAnimation(animateLoop); 
        
        if (pauseAuKm0.value > 0) {
             isPaused.value = true;
             await new Promise(resolve => {
                 let timer = setTimeout(() => { stopWatch(); resolve(); }, pauseAuKm0.value);
                 const stopWatch = watch(isPaused, (newVal) => {
                     if (!newVal) { clearTimeout(timer); stopWatch(); resolve(); }
                 });
             });
             if (isPaused.value) isPaused.value = false;
        }

    } catch (error) {
        console.error("Init Visualization Variant Failed:", error);
        showSnackbar("Erreur d'initialisation variante", "error");
    }
};


    
/**
 * Applies smoothing to tracking data received from backend.
 * Essential for variant segments which have generated points with default cap=0.
 * Also smooths the master tracking parts for consistency.
 */
function applySmoothingToTracking(tracking) {
    // 1. First pass: Ensure Raw Bearings exist
    for (let i = 0; i < tracking.length; i++) {
        const pt = tracking[i];
        
        // If cap is 0 (likely generated), calculate it
        if (!pt.cap || pt.cap === 0) {
            let rawBearing = 0;
            const coord = pt.coordonnee; // [lon, lat]
            
            if (i > 0) {
                const prev = tracking[i-1].coordonnee;
                // Avoid NaN from duplicate points
                if (Math.abs(prev[0] - coord[0]) < 1e-9 && Math.abs(prev[1] - coord[1]) < 1e-9) {
                     rawBearing = tracking[i-1].cap || 0;
                } else {
                     rawBearing = turf.bearing(turf.point(prev), turf.point(coord));
                }
            } else if (tracking.length > 1) {
                const next = tracking[i+1].coordonnee;
                rawBearing = turf.bearing(turf.point(coord), turf.point(next));
            }
             
            if (isNaN(rawBearing)) rawBearing = 0;
            pt.cap = rawBearing;
        }
        
        // Ensure defaults if missing
        if (!pt.zoom) pt.zoom = 17; // Closer zoom to see spline effect distinct from arc
        if (!pt.pitch) pt.pitch = 50;
    }

    // 2. Second pass: Smooth Bearings (Vector Averaging)
    const smoothingWindow = 10; 
    
    // We clone to avoid modifying while reading? 
    // Actually we can compute average based on the raw caps we just set.
    
    // Create a temp array for smoothed caps to apply after calculation
    const newCaps = new Array(tracking.length);

    for (let i = 0; i < tracking.length; i++) {
        let sinSum = 0;
        let cosSum = 0;
        let count = 0;

        for (let j = Math.max(0, i - smoothingWindow); j <= Math.min(tracking.length - 1, i + smoothingWindow + 5); j++) {
            const bearingRad = (tracking[j].cap * Math.PI) / 180;
            sinSum += Math.sin(bearingRad);
            cosSum += Math.cos(bearingRad);
            count++;
        }

        if (count > 0) {
            const avgBearingRad = Math.atan2(sinSum / count, cosSum / count);
            let avgBearingDeg = (avgBearingRad * 180) / Math.PI;
            if (avgBearingDeg < 0) avgBearingDeg += 360; 
            
            newCaps[i] = avgBearingDeg;
        } else {
            newCaps[i] = tracking[i].cap;
        }
    }
    
    // Apply new caps
    for(let i=0; i<tracking.length; i++) {
        tracking[i].editedCap = newCaps[i];
        tracking[i].cap = newCaps[i];
    }
    
    // Fix start
    if(tracking.length > 5) {
        const startBearing = tracking[5].editedCap; 
         for(let k=0; k<5; k++) {
             tracking[k].editedCap = startBearing;
             tracking[k].cap = startBearing;
         }
    }

    return tracking;
}

let lastTimestamp = 0;

const animateLoop = (timestamp) => {
    
    // 0. FlyTo Exclusive Mode
    if (isFlytoActive.value) {
        if (map.value) map.value.triggerRepaint();
        requestAnimationFrame(animateLoop);
        return; 
    }

    // 1. Check Pause/State
    if (isInitializing.value || (isPaused.value && !isRewinding.value && !isFlytoActive.value) || isAnimationFinished.value) {
        if (map.value) map.value.triggerRepaint();
        requestAnimationFrame(animateLoop);
        lastTimestamp = timestamp; 
        return;
    }

    // 2. Update Time
    const deltaTime = Math.min(timestamp - lastTimestamp, 100);
    lastTimestamp = timestamp;
    
    // Call controller to update time refs
    // Call controller to update time refs (Controller expects KM)
    const { phase, distanceTraveled } = updateTime(deltaTime, totalDurationAt1xRef.value, totalDistanceRef.value / 1000);
    
    // 3. Comet (use km directly)
    updateComet(distanceTraveled);

    // 4. Overlap Layers (Trace Retour/Aller switch)
    checkLayers(distanceTraveled);

    // 5. Camera Interpolation (Using New Composable) - Expects KM for tracking comparison
    const { bearing: newBearing } = updateCameraPosition(distanceTraveled, trackingPointsWithDistanceRef.value, controlPointIndicesRef.value, {
        dynamicZoomIntensity: dynamicZoomIntensity.value,
        currentSpeed: currentSpeed.value,
        lineStringRef: lineStringRef,
        isMultisegment: false, 
        activeVariantSegments: []
    });
    if (newBearing !== null) {
        currentTraceBearing.value = newBearing;
    }

    // 5b. Update Weather
    if (simulationStartDate.value && weatherForecasts.value?.length > 0) {
        const timeMs = accumulatedTime.value || 0;
        const currentSimDate = new Date(simulationStartDate.value.getTime() + timeMs);
        
        if (isNaN(currentSimDate.getTime())) {
            console.warn("Weather: Invalid date calculation", {
                startDate: simulationStartDate.value,
                accumulatedTime: accumulatedTime.value,
                timeMs
            });
            return;
        }
        
        const newWeather = WeatherService.getCurrentWeather(distanceTraveled, currentSimDate, weatherForecasts.value);
        if (newWeather) {
            currentWeather.value = newWeather;
            // Debug: console.log("Weather updated:", newWeather.temperature, "°C");
        } else {
            // No weather match
        }
    }

    // 6. Check Events (Pause/Flyto)
    // 6. Check Events (Pause/Flyto) - Expects M
    checkEvents(distanceTraveled * 1000);

    // 7. Loop or End
    if (phase < 1 || isRewinding.value) {
        requestAnimationFrame(animateLoop);
    } else {
        isAnimationFinished.value = true;
        isPaused.value = true;
        handleEndSequence();
    }
};

const handleJumpRequest = async (targetDistanceKm) => {
    // 1. Suspension
    const wasPlaying = !isPaused.value;
    isPaused.value = true;
    isFlytoActive.value = true;
    
    const targetDistanceM = targetDistanceKm * 1000;
    
    // 2. Mise à jour État (Instantanée)
    // Attention: totalDistanceRef est en Mètres ici, mais setTimeFromDistance attend des KM pour les deux ?
    // Check setTimeFromDistance: (targetDistanceInKm, totalDistanceKm, totalDurationMs)
    // Ici totalDistanceRef est en Mètres.
    setTimeFromDistance(targetDistanceKm, totalDistanceRef.value / 1000, totalDurationAt1xRef.value);

    
    // Comet Update
    updateComet(targetDistanceKm); 
    
    // Layers Logic (Aller/Retour)
    checkLayers(targetDistanceKm);
    
    // Weather
    if (simulationStartDate.value && weatherForecasts.value?.length > 0) {
         const timeMs = accumulatedTime.value || 0;
         const currentSimDate = new Date(simulationStartDate.value.getTime() + timeMs);
         const newWeather = WeatherService.getCurrentWeather(targetDistanceKm, currentSimDate, weatherForecasts.value);
         if (newWeather) currentWeather.value = newWeather;
    }
    
    // 3. Calcul Cible Caméra
    // Interpolated Bearing
    const { bearing: interpolatedBearing, target: targetCameraParams } = updateCameraPosition(targetDistanceKm, trackingPointsWithDistanceRef.value, controlPointIndicesRef.value, {
        dynamicZoomIntensity: dynamicZoomIntensity.value,
        currentSpeed: currentSpeed.value,
        lineStringRef: lineStringRef,
        isMultisegment: false, 
        activeVariantSegments: [],
        apply: false // IMPORTANT: Do not move map yet
    });
    
    if (interpolatedBearing !== null) currentTraceBearing.value = interpolatedBearing;
    
    let targetCamera = targetCameraParams;
    
    if (!targetCamera) {
        const pts = trackingPointsWithDistanceRef.value;
        let bestPoint = pts[0];
        for (let i = pts.length - 1; i >= 0; i--) {
            if (pts[i].distance <= targetDistanceKm) {
                bestPoint = pts[i];
                break;
            }
        }
        targetCamera = {
            center: bestPoint.coordonnee, 
            zoom: bestPoint.editedZoom ?? bestPoint.zoom,
            pitch: bestPoint.editedPitch ?? bestPoint.pitch,
            bearing: bestPoint.editedCap ?? bestPoint.cap
        };
    }

    try {
        const pointOnLine = turf.along(lineStringRef.value, targetDistanceKm, { units: 'kilometers' });
        targetCamera.center = pointOnLine.geometry.coordinates;
    } catch(e) { }

    // 4. FlyTo Transition
    await flyToPromise({
        ...targetCamera,
        duration: jumpDuration.value * 1000 
    });
    
    // 5. Reprise
    isFlytoActive.value = false;
    
    if (wasPlaying) {
        isPaused.value = false;
    } else {
        if(map.value) map.value.triggerRepaint();
    }
};

// --- Helpers ---

let lastDirection = 'aller';
const checkLayers = (distanceTraveled) => {
     if (segmentMetadata.value?.overlappingZones) {
         const isRetour = segmentMetadata.value.overlappingZones.some(z => distanceTraveled >= z.retourStartKm && distanceTraveled <= z.retourEndKm); 
         const currentDir = isRetour ? 'retour' : 'aller';
         
         if (currentDir !== lastDirection) {
             console.log(`[VisualizeVariant] Switching Layer to: ${currentDir} at distance ${distanceTraveled.toFixed(2)} km`);
             lastDirection = currentDir;
         }

         updateTraceOverlapVisibility(null, currentDir, showSegments.value);
    }
};

const updateComet = (distanceTraveled) => {
     if(!map.value || !lineStringRef.value) return;
     const totalLen = turf.length(lineStringRef.value, { units: 'kilometers' });
     const currentDist = Math.max(0, Math.min(distanceTraveled, totalLen));
     const cometLengthKm = cometLength.value / 1000;
     const startDistance = Math.max(0, currentDist - cometLengthKm);
     
     if (currentDist > startDistance) {
            try {
                const cometSlice = turf.lineSliceAlong(lineStringRef.value, startDistance, currentDist, { units: 'kilometers' });
                if (map.value.getSource('comet-source')) {
                    map.value.getSource('comet-source').setData(cometSlice);
                }
            } catch(e) { /* ignore */ }
     } else {
          if (map.value.getSource('comet-source')) {
              map.value.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
          }
     }
};


const checkEvents = async (distanceTraveled) => {
    // Logic from original VisualizeView:
    // 1. Find Increment
    // We assume robust find from index like in original view
    // Simplified search for refactor:
    // find index where point.distance > distanceTraveled
    
    if(!trackingPointsWithDistanceRef.value || trackingPointsWithDistanceRef.value.length < 2) return;
    
    // Locate current point logic
    let currentPoint = null;
    // Find point <= distanceTraveled
    // Reverse search often faster if continuous
    // But let's use a simple robust search
    const pts = trackingPointsWithDistanceRef.value;
    // Optimization: remember last index in controller? For now, linear search.
    let idx = -1;
    for(let i=pts.length-1; i>=0; i--) {
        if(pts[i].distance <= distanceTraveled) {
            idx = i;
            break;
        }
    }
    if(idx !== -1) currentPoint = pts[idx];

    if(!currentPoint) return;
    const currentIncrement = currentPoint.increment;

    if (currentPoint.commune) currentCommuneName.value = currentPoint.commune;

    // Logic Rewind Reset
    if (isRewinding.value) {
        if (triggeredPauseIncrement.value !== null && currentIncrement < triggeredPauseIncrement.value) triggeredPauseIncrement.value = null;
        if (triggeredFlytoIncrement.value !== null && currentIncrement < triggeredFlytoIncrement.value) triggeredFlytoIncrement.value = null;
    }

    if (currentIncrement !== undefined) {
        // Popups (Range Events)
        if (rangeEvents.value.length > 0) {
              const newVisibleIds = new Set();
              rangeEvents.value.forEach(msg => {
                  if (currentIncrement >= msg.startIncrement && currentIncrement <= msg.endIncrement) {
                      newVisibleIds.add(msg.eventId);
                  }
              });
              // ... update activePopups (standard logic)
              // Simplify for Step 1: Just log or implement fully?
              // Implementing basics:
               const currentVisibleIds = new Set(activePopups.keys());
               currentVisibleIds.forEach(id => {
                   if (!newVisibleIds.has(id)) {
                       activePopups.get(id)?.remove();
                       activePopups.delete(id);
                   }
               });
               newVisibleIds.forEach(id => {
                   if (!currentVisibleIds.has(id)) {
                       const m = rangeEvents.value.find(ev => ev.eventId === id);
                       if(m && m.message) {
                           const content = createMessageSVG(m);
                           const p = new mapboxgl.Popup({ closeButton: false, closeOnClick: false, className: 'map-message-popup' })
                                .setLngLat(m.coord).setHTML(content).addTo(map.value);
                           activePopups.set(id, p);
                       }
                   }
               });
        }

        // Pause
        if (pauseIncrements.value.includes(currentIncrement)) {
            if (triggeredPauseIncrement.value !== currentIncrement) {
                isPaused.value = true;
                triggeredPauseIncrement.value = currentIncrement;
            }
        }
        
        // FlyTo
        const flyData = flytoEvents.value[currentIncrement];
        if (flyData && triggeredFlytoIncrement.value !== currentIncrement) {
             if (isPaused.value) return; // Don't trigger if already paused
             triggeredFlytoIncrement.value = currentIncrement;
             executeFlytoSequence(flyData); 
        }
    }
};

const executeFlytoSequence = async (flytoData) => {
    isFlytoActive.value = true;
    isPaused.value = true;
    animationState.value = 'Survol_Evenementiel';
    // Save state
    saveCameraState();
    preFlytoCameraOptions.value = pausedCameraOptions.value;

    const duration = (flytoData.duree > 100 ? flytoData.duree : flytoData.duree * 1000);
    const durationTarget = Math.max(200, duration / currentSpeed.value);

    await flyToPromise({
        center: flytoData.coord, zoom: flytoData.zoom, pitch: flytoData.pitch, bearing: flytoData.cap, duration: durationTarget
    });

    animationState.value = 'En_Pause';
    // Wait for Play
    await new Promise(resolve => {
        const stop = watch(isPaused, (val) => {
            if(!val) { stop(); resolve(); }
        });
    });

    animationState.value = 'Survol_Evenementiel';
    const durationBack = Math.max(200, duration / currentSpeed.value);
    const preOpts = preFlytoCameraOptions.value;
    await flyToPromise({ 
        center: preOpts.center, 
        zoom: preOpts.zoom, 
        pitch: preOpts.pitch, 
        bearing: preOpts.bearing, 
        duration: durationBack 
    });

    isFlytoActive.value = false;
    isPaused.value = false;
    animationState.value = 'En_Animation';
    lastTimestamp = 0; // Reset timer for smooth resume
    requestAnimationFrame(animateLoop); // Restart loop explicitly
};

const handleEndSequence = async () => {
    if(!map.value) return;

    // Explicitly set state to Finished (needed if triggered manually via button)
    isAnimationFinished.value = true;
    isPaused.value = true;
    
    animationState.value = 'Vol_Final';
    // Hide UI
    isDistanceDisplayVisible.value = false;
    isCommuneWidgetVisible.value = false;
    isAltitudeVisible.value = false; 
    // ...
    // Hide Comet
     if (map.value.getSource('comet-source')) {
          map.value.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
     }
    
    // Switch to launch style if needed
    if (styleLancement.value !== mapStyle.value) {
        map.value.setStyle(styleLancement.value);
        await new Promise(r => map.value.once('style.load', r));
        setupTraceLayers({
            traceWidth: traceWidth.value, traceOpacity: traceOpacity.value, traceColor: traceColor.value,
            lineStringData: lineStringRef.value, cometWidth: cometWidth.value, cometColor: cometColor.value, cometOpacity: cometOpacity.value,
            coloredSegmentsData: coloredSegmentsGeoJsonRef.value
        });
    }

    const traceBbox = turf.bbox(lineStringRef.value);
    /* Safe implementation of Final FlyTo */
    try {
        const camParams = map.value.cameraForBounds(traceBbox, { padding: 40, bearing: 0, pitch: 0 });
        if (camParams) {
             await flyToPromise({
                center: camParams.center,
                zoom: camParams.zoom,
                pitch: 0, 
                bearing: 0, 
                duration: flyToGlobalDuration.value
            });
        }
    } catch (err) {
        console.warn("End sequence flyTo failed", err);
    }
    
    animationState.value = 'Termine';
     // Logic Reprise Auto...
};

const resetAnimation = async () => {
    // Reset Logic
    resetTime();
    isPaused.value = true;
    isAnimationFinished.value = false;
    triggeredPauseIncrement.value = null;
    triggeredFlytoIncrement.value = null;
    isFlytoActive.value = false;
    
    // Restore UI
    isDistanceDisplayVisible.value = true;
    isAltitudeVisible.value = true;
    isCommuneWidgetVisible.value = true;
    isWeatherInfoVisible.value = true;
    isCompassVisible.value = true;

    // Restore Map Style for 3D View if changed
    if (mapStyle.value && map.value.getStyle().name !== mapStyle.value) { // Simple check, might need robust URL check
        // Or assume if we are in End State (standard map) we need to switch back
         map.value.setStyle(mapStyle.value);
         await new Promise(r => map.value.once('style.load', r));
         
         // Re-add Terrain
         if (!map.value.getSource('mapbox-dem')) {
             map.value.addSource('mapbox-dem', {
                'type': 'raster-dem',
                'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
                'tileSize': 512,
                'maxzoom': 14
            });
        }
        map.value.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': terrainExaggeration.value });

        // Re-setup layers
        setupTraceLayers({
            traceWidth: traceWidth.value, traceOpacity: traceOpacity.value, traceColor: traceColor.value,
            lineStringData: lineStringRef.value, cometWidth: cometWidth.value, cometColor: cometColor.value, cometOpacity: cometOpacity.value,
            coloredSegmentsData: coloredSegmentsGeoJsonRef.value
        });
    }

    activePopups.forEach(p => p.remove());
    activePopups.clear();
    
    // Reset Camera
    if(trackingPointsWithDistanceRef.value.length > 0) {
        const start = trackingPointsWithDistanceRef.value[0];
        await flyToPromise({
            center: start.coordonnee, zoom: start.editedZoom??start.zoom, pitch: start.editedPitch??start.pitch, bearing: start.editedCap??start.cap, duration: 2000
        });
    }
    
    animationState.value = 'En_Pause_au_Depart';
    requestAnimationFrame(animateLoop);
};

// --- Weather ---
async function initWeather(circuit, trackingPoints) {
    if (!trackingPoints || trackingPoints.length === 0) {
        console.warn("Tracking Points empty in initWeather");
    }
    let startDate = null;
    if (!circuit || !circuit.dateDepart) {
        startDate = new Date();
        startDate.setDate(startDate.getDate() + 1); // Default to tomorrow
        startDate.setHours(9, 0, 0, 0); // Force 9:00 AM
    } else {
        startDate = new Date(circuit.dateDepart);
        // If date is older than 30 days, fallback to tomorrow (Open-Meteo Forecast limit)
        const now = new Date();
        const diffTime = now - startDate; // positive if past
        const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
        if (diffDays > 30 || diffDays < -14) {
             console.warn("Date trop ancienne/lointaine pour météo, utilisation date du lendemain:", startDate);
             startDate = new Date();
             startDate.setDate(startDate.getDate() + 1);
             startDate.setHours(9, 0, 0, 0); // Force 9:00 AM
        }
    }
    simulationStartDate.value = startDate;
    try {
        const scenarios = await invoke('get_circuit_scenarios', { circuitId: props.circuitId });
        circuitScenarios.value = scenarios;
        weatherForecasts.value = await WeatherService.generateWeatherForecasts({ ...circuit, dateDepart: startDate }, trackingPoints, scenarios, props.circuitId);
    } catch (e) { console.warn("Weather init error", e); }
}

const wasPausedBeforeArrowRight = ref(false);


const handleKeydown = (e) => {
    if (['INPUT', 'TEXTAREA'].includes(e.target.tagName)) return;

    switch(e.key) {
        case ' ': // Espace
            e.preventDefault();
            togglePlayPauseOrReset();
            break;
        case 'h':
        case 'H':
            if (isBackButtonVisibleFinal.value) goBack();
            break;
        case 'ArrowLeft':
            if (!isRewinding.value) isRewinding.value = true;
            break;
        case 'ArrowRight':
            if (isPaused.value) {
                wasPausedBeforeArrowRight.value = true;
                isPaused.value = false;
            }
            break;
        case 'ArrowUp':
             e.preventDefault();
             sliderPosition.value = Math.min(100, sliderPosition.value + 5);
             break;
        case 'ArrowDown':
             e.preventDefault();
             sliderPosition.value = Math.max(0, sliderPosition.value - 5);
             break;
        case '1':
        case '&': // Support AZERTY '1'
             sliderPosition.value = mapSpeedToSlider(1.0); // Reset speed to 1x
             break;
        case 'w':
        case 'W':
             // Toggle ALL widgets (replacing V)
             const allState = !isControlsCardVisible.value;
             isControlsCardVisible.value = allState;
             isDistanceDisplayVisible.value = allState;
             isCommuneWidgetVisible.value = allState;
             isAltitudeVisible.value = allState;
             isWeatherInfoVisible.value = allState;
             isCompassVisible.value = allState;
             break;
        case 'c':
        case 'C':
             isControlsCardVisible.value = !isControlsCardVisible.value;
             break;
        case 'v': // Villes / Communes
        case 'V':
             isCommuneWidgetVisible.value = !isCommuneWidgetVisible.value;
             break;
        case 'd':
        case 'D':
             isDistanceDisplayVisible.value = !isDistanceDisplayVisible.value;
             break;
        case 'a':
        case 'A':
             isAltitudeVisible.value = !isAltitudeVisible.value;
             break;
        case 'm':
        case 'M':
             isWeatherInfoVisible.value = !isWeatherInfoVisible.value;
             break;
        case 'b':
        case 'B':
             isCompassVisible.value = !isCompassVisible.value;
             break;
    }
};

const handleKeyup = (e) => {
    switch(e.key) {
        case 'ArrowLeft':
            isRewinding.value = false;
            break;
        case 'ArrowRight':
            if (wasPausedBeforeArrowRight.value) {
                isPaused.value = true;
                wasPausedBeforeArrowRight.value = false;
            }
            break;
    }
};

// --- Lifecycle ---
onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('keyup', handleKeyup);
    // Do not call init here directly. Wait for settings.
});

// Watch settings changes that mandate restart/update...
onMounted(async () => {
    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('keyup', handleKeyup);
    
    // Ensure DOM is ready
    await nextTick();

    if (mapboxToken.value) {
        initializeVisualization();
    } else {
        // Fallback or wait for settings?
        // Usually settings are loaded. If not, a watch might be needed.
        const unwatch = watch(mapboxToken, (token) => {
            if (token) {
                initializeVisualization();
                unwatch();
            }
        });
    }
});

// Sync isPaused with animationState for UI visibility
watch(isPaused, (newVal) => {
    if (newVal) {
        if (animationState.value === 'En_Animation') {
            animationState.value = 'En_Pause';
        }
    } else {
        if (animationState.value === 'En_Pause' || animationState.value === 'En_Pause_au_Depart') {
            animationState.value = 'En_Animation';
        }
    }
});

// Watch Visualization Layers Toggles
watch([showSegments, showSlope, map], () => {
     updateLayerVisibility('trace-variant-common', showSegments.value);
     updateLayerVisibility('trace-variant-new', showSegments.value);
     updateLayerVisibility('trace-variant-abandoned', showSegments.value);
     updateLayerVisibility('trace-slope', showSlope.value);
});

function generateSlopeSegments(trackingPoints) {
    // Deprecated in favor of Backend expression
    return { type: 'FeatureCollection', features: [] };
}


onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown);
    window.removeEventListener('keyup', handleKeyup);
    cleanupMap();
    if (mapContainer.value) mapContainer.value.remove();
    activePopups.forEach(p => p.remove());
    activePopups.clear();
     unlistenFunctions.forEach(fn => fn());
});

</script>

<style scoped>
#map-container { position: absolute; top: 0; bottom: 0; width: 100%; }
.hide-cursor { cursor: none; }
.back-button { position: absolute !important; top: 20px; left: 20px; z-index: 1; pointer-events: auto; }
.distance-display { pointer-events: auto; width: fit-content; height: 48px; max-height: 60px; overflow: hidden; }
.top-center-container { position: absolute; top: 20px; left: 50%; transform: translateX(-50%); z-index: 1; pointer-events: none; }
.bottom-center-container { position: absolute; bottom: 20px; left: 50%; transform: translateX(-50%); z-index: 1; display: flex; flex-direction: column-reverse; align-items: center; pointer-events: none; }
.bottom-controls, .altitude-svg-container, .commune-display, .controls-card { pointer-events: auto; }
.altitude-svg-container { justify-content: center; background-color: rgba(0, 0, 0, 0.7); border-radius: 5px; margin-top: 8px; max-height: 500px; overflow: hidden; }
.commune-display { position: absolute; top: 20px; left: 80px; width: 250px; height: 48px; max-height: 60px; overflow: hidden; background-color: white; border-width: 4px; border-style: solid; border-radius: 5px; color: black; padding: 4px; z-index: 1; display: flex; align-items: center; justify-content: center; }
.speed-slider { width: 200px; }
.speed-value-display { font-family: monospace; font-size: 0.9em; padding: 0 8px; min-width: 45px; text-align: center; }
</style>
<style>
/* Global style strictly for popups to avoid scoped issues if any - or keep standard */
.map-message-popup .mapboxgl-popup-content { background: none; padding: 0; box-shadow: none; }
.map-message-popup .mapboxgl-popup-tip { display: none; }
.mapboxgl-ctrl-bottom-left, .mapboxgl-ctrl-bottom-right { display: none; }
</style>
