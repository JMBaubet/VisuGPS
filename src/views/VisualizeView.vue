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
    :total-distance="totalDistanceRef"
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
       <altitude-s-v-g 
            :key="`altitude-${props.circuitId}-${totalDistanceRef}`"
            :circuit-id="props.circuitId" 
            :current-distance="currentDistanceInMeters" 
            :total-distance="totalDistanceRef > 0 ? totalDistanceRef * 1000 : 1"
            :tracking-points="trackingPointsWithDistanceRef"
            :is-variant-comparison="false"
            :main-trace-points="null"
            :variant-segments="[]"
            :current-segment-index="null"
            @jump-requested="(distRef) => handleJumpRequest(distRef / 1000)"
          />
    </template>

    <template #extra-overlay-actions>
       <v-btn v-if="isAnimationFinished && hasVariants"
             color="primary"
             @click="goToVariantView"
             size="x-large"
             rounded
             prepend-icon="mdi-source-branch"
             title="Voir les variantes"
      >
        Variantes
      </v-btn>
    </template>

    <template #extra-controls>
        <template v-if="hasVariants">
            <v-divider vertical class="mx-2"></v-divider>
            <v-btn icon="mdi-source-branch" variant="text" title="Mode Variants" @click="goToVariantView" :disabled="!isPaused && !isAnimationFinished"></v-btn>
        </template>
    </template>
  </VisualizeControls>

    <!-- Variant Selection Dialog -->
    <v-dialog v-model="showVariantSelection" max-width="500">
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
                <v-btn text @click="showVariantSelection = false">Annuler</v-btn>
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
});

const router = useRouter();
const route = useRoute();
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

const formatDuration = (val) => (val > 100 ? val : val * 1000);

// --- Using New Composables ---
// 1. Map Engine
const { map, isMapLoaded, initializeMap: initMapEngine, flyToPromise, cleanupMap } = useMapEngine(mapContainer, mapboxToken, styleLancement, terrainExaggeration);

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
const { setupTraceLayers, updateLayerVisibility, updateTraceOverlapVisibility, coloredSegmentsGeoJsonRef } = useTraceLayers(map);

// 5. Animation Controller
// Note: accumulatedTime can be manipulated directly via composable exposed ref if needed
const { isPaused, isRewinding, isAnimationFinished, currentSpeed, currentDistanceInMeters, distanceDisplay, currentTraceBearing, startAnimation, pauseAnimation, resetTime, updateTime, accumulatedTime, setTimeFromDistance } = useAnimationController();

// --- Speed Control Logic (Restore Logarithmic) ---
const sliderPosition = ref(25); // Default start pos ~ 1.0x if min=0.1 max=100
const minSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/min_value') || 0.1);
const maxSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/max_value') || 100.0);
const defaultSpeedValue = computed(() => getSettingValue('Visualisation/Lecture/Vitesse/default_value') || 25.0);

function mapSliderToSpeed(sliderValue) {
    const min = minSpeedValue.value;
    const max = maxSpeedValue.value;
    if (sliderValue <= 0) return min;
    if (sliderValue >= 100) return max;
    const minLog = Math.log(min);
    const maxLog = Math.log(max);
    const logVal = minLog + (maxLog - minLog) * (sliderValue / 100);
    return Math.exp(logVal);
}

function mapSpeedToSlider(speed) {
    const min = minSpeedValue.value;
    const max = maxSpeedValue.value;
    if (speed <= min) return 0;
    if (speed >= max) return 100;
    const minLog = Math.log(min);
    const maxLog = Math.log(max);
    const val = (Math.log(speed) - minLog) / (maxLog - minLog);
    return val * 100;
}

// Sync Slider -> Speed
watch(sliderPosition, (newVal) => {
    currentSpeed.value = mapSliderToSpeed(newVal);
});

// Sync Speed -> Slider (Init)
watch(currentSpeed, (newVal) => {
    // Avoid feedback loop if diff is small
    const calculatedSlider = mapSpeedToSlider(newVal);
    if (Math.abs(calculatedSlider - sliderPosition.value) > 1) {
        sliderPosition.value = calculatedSlider;
    }
}, { immediate: true });

// 6. Camera Interpolator (NEW)
const { updateCameraPosition } = useCameraInterpolator(map);


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

// Speed Slider Constants (Removed duplicates)

// --- Methods ---

const showVariantSelection = ref(false);
const availableVariants = ref([]);

const goToVariantView = async () => {
    try {
        const variants = await invoke('get_variants', { circuitId: props.circuitId });
        if (!variants || variants.length === 0) {
             showSnackbar("Aucune variante disponible.", "warning");
             return;
        }

        if (variants.length === 1) {
            navigateToVariant(variants[0].id);
        } else {
            availableVariants.value = variants;
            showVariantSelection.value = true;
        }
    } catch (e) {
        console.error("Erreur chargement variants:", e);
        showSnackbar("Erreur chargement variants", "error");
    }
};

const selectVariant = (variantId) => {
    showVariantSelection.value = false;
    navigateToVariant(variantId);
};

const navigateToVariant = (variantId) => {
    let query = {};
    if (map.value) {
        const center = map.value.getCenter();
        query = {
            lat: center.lat,
            lng: center.lng,
            zoom: map.value.getZoom(),
            bearing: map.value.getBearing(),
            pitch: map.value.getPitch()
        };
    }
    router.push({ 
        name: 'VisualizeVariant', 
        params: { circuitId: props.circuitId }, 
        query: { ...query, variantId: variantId } // Pass variantId in query or params? Ideally params but route is /visualize-variant/:circuitId
        // Let's pass it in query if the route doesn't support it, OR update router to support /:variantId
        // Actually, route definition is important.
    });
};
const goBack = () => { router.push({ name: 'Main' }); };
const getToHexImproved = (n) => toHex(getSettingValue(n));

// --- User Interactions ---
const togglePlayPauseOrReset = () => {
    if (animationState.value === 'Termine' || animationState.value === 'Vol_Final' || isAnimationFinished.value) {
        resetAnimation();
    } else {
        if (isPaused.value) {
            startAnimation();
            animationState.value = 'En_Animation';
        } else {
            pauseAnimation();
            animationState.value = 'En_Pause';
        }
    }
};

// --- Initialization Logic ---
const initializeVisualization = async () => {
    resetTime(); // Ensure animation time starts at 0
    try {
        const { circuit, trackingData } = await loadCircuitData(props.circuitId);
        currentCircuitRef.value = circuit;
        if (circuit) avancementCommunes.value = circuit.avancementCommunes;
        // Debug logs removed
         
         try {
            const variants = await invoke('get_variants', { circuitId: props.circuitId });
            hasVariants.value = variants && variants.length > 0;
        } catch(e) { console.warn("Check variants failed", e); }

        const processed = await processTrackingData(lineStringRef.value, trackingData);
        totalDistanceRef.value = processed.totalDistanceKm;
        
        const msPerKm = getSettingValue('Visualisation/Lecture/vitesse') || 3730;
        totalDurationAt1xRef.value = totalDistanceRef.value * msPerKm; 

        controlPointIndicesRef.value = trackingPointsWithDistanceRef.value.reduce((acc, p, index) => {
            if (p.pointDeControl) acc.push(index);
            return acc;
        }, []);

        const events = eventsRef.value;
        if (events && events.pointEvents) {
             pauseIncrements.value = Object.keys(events.pointEvents).filter(k => events.pointEvents[k].some(e => e.type === 'Pause')).map(Number);
             const flytos = {};
             Object.keys(events.pointEvents).forEach(k => {
                 const ev = events.pointEvents[k].find(e => e.type === 'Flyto');
                 if(ev) flytos[Number(k)] = ev.data;
             });
             flytoEvents.value = flytos;
        }
        rangeEvents.value = events?.rangeEvents || [];

        await initWeather(circuit, trackingPointsWithDistanceRef.value);

        if (colorTraceBySlope.value) {
             const slopeColors = {
                TrancheNegative: getToHexImproved('Visualisation/Profil Altitude/Couleurs/TrancheNegative'),
                Tranche1: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche1'),
                Tranche2: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche2'),
                Tranche3: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche3'),
                Tranche4: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche4'),
                Tranche5: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche5'),
            };
            try {
                const geojson = await invoke('get_colored_segments_geojson', { 
                    circuitId: props.circuitId, 
                    slopeColors, 
                    segmentLength: segmentLength.value,
                    variantId: null 
                });
                coloredSegmentsGeoJsonRef.value = geojson;
            } catch(e) { console.error("Colored segments error", e); }
        }

        const startPoint = trackingPointsWithDistanceRef.value[0]; 
        
        // Determine Initial Map State
        let initialCenter = centerEurope.value;
        let initialZoom = zoomEurope.value;
        const isDirectStart = route.query.directStart === 'true';
        const hasCameraParams = route.query.lat && route.query.lng && route.query.zoom;

        if (hasCameraParams) {
             initialCenter = [parseFloat(route.query.lng), parseFloat(route.query.lat)];
             initialZoom = parseFloat(route.query.zoom);
        } else if (isDirectStart && startPoint) {
            initialCenter = startPoint.coordonnee;
            initialZoom = startPoint.editedZoom ?? startPoint.zoom ?? 14;
        }

        // Init Map with correct style immediately
        // If DirectStart, use mapStyle (3D), otherwise use default
        const styleToUse = isDirectStart ? mapStyle.value : null; 
        
        const mapInstance = await initMapEngine(initialCenter, initialZoom, styleToUse);
        if(!mapInstance) throw new Error("Map failed to init");
        
        mapInstance.setMinZoom(zoomMinimum.value);

        // Immediate Positioning (JumpTo)
        if (hasCameraParams) {
             mapInstance.jumpTo({
                 center: initialCenter,
                 zoom: initialZoom,
                 pitch: parseFloat(route.query.pitch || 0),
                 bearing: parseFloat(route.query.bearing || 0)
             });
        } else if (isDirectStart && startPoint) {
             mapInstance.jumpTo({
                 center: initialCenter,
                 zoom: initialZoom,
                 pitch: startPoint.editedPitch ?? startPoint.pitch ?? 0,
                 bearing: startPoint.editedCap ?? startPoint.cap ?? 0
             });
        }

        startBearingTracking();

        setupTraceLayers({
            traceWidth: traceWidth.value, traceOpacity: traceOpacity.value, traceColor: traceColor.value,
            lineStringData: lineStringRef.value, cometWidth: cometWidth.value, cometColor: cometColor.value, cometOpacity: cometOpacity.value,
            coloredSegmentsData: coloredSegmentsGeoJsonRef.value
        });
        
        if (isDirectStart) {
             // Direct Start: Skip Global View logic
             animationState.value = 'Vol_Vers_Depart';
             
             // No need to switch style, we initialized with it.
        } else {
            // Normal Sequence
            animationState.value = 'Vol_Vers_Vue_Globale';
            const traceBbox = turf.bbox(lineStringRef.value);
            const globalView = mapInstance.cameraForBounds(traceBbox, { padding: 40, bearing: 0, pitch: 0 });
            
            await flyToPromise(globalView, { duration: durationEuropeToTrace.value });

            animationState.value = 'Pause_Observation';
            await new Promise(r => setTimeout(r, pauseBeforeStart.value));
            
            animationState.value = 'Vol_Vers_Depart';
            if (mapStyle.value !== styleLancement.value) {
                mapInstance.setStyle(mapStyle.value);
                await new Promise(resolve => mapInstance.once('style.load', resolve));
                setupTraceLayers({
                    traceWidth: traceWidth.value, traceOpacity: traceOpacity.value, traceColor: traceColor.value,
                    lineStringData: lineStringRef.value, cometWidth: cometWidth.value, cometColor: cometColor.value, cometOpacity: cometOpacity.value,
                    coloredSegmentsData: coloredSegmentsGeoJsonRef.value
                }); 
            }
        }

        // Final Positioning at Start (FlyTo)
        // Execute if:
        // 1. Standard Start (!isDirectStart)
        // 2. OR Direct Start but coming from somewhere else (hasCameraParams) -> Fly from there to start.
        if (!isDirectStart || hasCameraParams) {
            await flyToPromise({
                center: startPoint.coordonnee,
                zoom: startPoint.editedZoom ?? startPoint.zoom,
                pitch: startPoint.editedPitch ?? startPoint.pitch,
                bearing: startPoint.editedCap ?? startPoint.cap,
                duration: durationTraceToStart.value
            });
        }

        animationState.value = 'En_Pause_au_Depart';
        isInitializing.value = false;
        enableInteraction();
        startAnimation(animateLoop); 
        
        if (pauseAuKm0.value > 0) {
             isPaused.value = true;
             // Wait for delay OR user interaction (isPaused becoming false)
             await new Promise(resolve => {
                 let timer = setTimeout(() => {
                     stopWatch();
                     resolve();
                 }, pauseAuKm0.value);
                 
                 const stopWatch = watch(isPaused, (newVal) => {
                     if (!newVal) { // User clicked Play
                         clearTimeout(timer);
                         stopWatch();
                         resolve();
                     }
                 });
             });
             // Ensure paused is false if timeout expired naturally
             if (isPaused.value) isPaused.value = false;
        }

    } catch (error) {
        console.error("Init Visualization Failed:", error);
        showSnackbar("Erreur d'initialisation", "error");
    }
};

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
    const { phase, distanceTraveled } = updateTime(deltaTime, totalDurationAt1xRef.value, totalDistanceRef.value);
    
    // 3. Comet
    updateComet(distanceTraveled);

    // 4. Overlap Layers (Trace Retour/Aller switch)
    checkLayers(distanceTraveled);

    // 5. Camera Interpolation (Using New Composable)
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
    checkEvents(distanceTraveled);

    // 7. Loop or End
    if (phase < 1 || isRewinding.value) {
        requestAnimationFrame(animateLoop);
    } else {
        isAnimationFinished.value = true;
        isPaused.value = true;
        handleEndSequence();
    }
};

// --- Helpers ---

const checkLayers = (distanceTraveled) => {
     if (segmentMetadata.value?.overlappingZones) {
         const isRetour = segmentMetadata.value.overlappingZones.some(z => distanceTraveled >= z.retourStartKm && distanceTraveled <= z.retourEndKm); 
         if (isRetour) {
             updateTraceOverlapVisibility(null, 'retour');
         } else {
             updateTraceOverlapVisibility(null, 'aller');
         }
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
                       if(m && m.message && map.value) {
                           const content = createMessageSVG(m);
                           const anchor = m.orientation === 'Gauche' ? 'bottom-right' : 'bottom-left';
                            const p = new mapboxgl.Popup({ 
                                closeButton: false, 
                                closeOnClick: false, 
                                className: 'map-message-popup',
                                anchor: anchor
                            })
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

const handleJumpRequest = async (targetDistanceKm) => {
    // 1. Suspension
    const wasPlaying = !isPaused.value;
    isPaused.value = true;
    isFlytoActive.value = true;
    
    // 2. Mise à jour État (Instantanée) de la Comète et des Widgets
    setTimeFromDistance(targetDistanceKm, totalDistanceRef.value, totalDurationAt1xRef.value);
    const targetDistanceM = targetDistanceKm * 1000;
    
    updateComet(targetDistanceKm); // Comet uses KM
    checkLayers(targetDistanceKm);
    if (simulationStartDate.value && weatherForecasts.value?.length > 0) {
         // Force update weather
         const timeMs = accumulatedTime.value || 0;
         const currentSimDate = new Date(simulationStartDate.value.getTime() + timeMs);
         const newWeather = WeatherService.getCurrentWeather(targetDistanceKm, currentSimDate, weatherForecasts.value);
         if (newWeather) currentWeather.value = newWeather;
    }
    
    // 3. Calcul Cible Caméra
    // Recherche point proche pour fallback
    // Calculate interpolated bearing AND target camera
    const { bearing: interpolatedBearing, target: targetCameraParams } = updateCameraPosition(targetDistanceKm, trackingPointsWithDistanceRef.value, controlPointIndicesRef.value, {
        dynamicZoomIntensity: dynamicZoomIntensity.value,
        currentSpeed: currentSpeed.value,
        lineStringRef: lineStringRef,
        isMultisegment: false, 
        activeVariantSegments: [],
        apply: false // IMPORTANT: Do not move map yet
    });
    
    if (interpolatedBearing !== null) currentTraceBearing.value = interpolatedBearing;
    
    // Target Camera State
    let targetCamera = targetCameraParams;
    
    if (!targetCamera) {
        // Fallback robust (si hors range ou erreur)
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

    // 4. FlyTo Transition
    await flyToPromise({
        ...targetCamera,
        duration: jumpDuration.value * 1000 // Convert seconds to ms
    });
    
    // 5. Reprise
    isFlytoActive.value = false;
    
    // On ne reprend QUE si on était en lecture
    if (wasPlaying) {
        isPaused.value = false;
        // On restart la loop si besoin (normalement elle tourne tjs si initialized)
        // Mais checkLayers/etc seront appelés au prochain frame
    } else {
        // Force repaint one last time to be sure
        if(map.value) map.value.triggerRepaint();
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
    await flyToPromise({ ...preFlytoCameraOptions.value, duration: durationBack });

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
    await flyToPromise({
        pitch: 0, bearing: 0, duration: flyToGlobalDuration.value,
        ...(map.value.cameraForBounds(traceBbox, { padding: 40, bearing: 0, pitch: 0 }))
    });
    
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
             sliderPosition.value = mapSpeedToSlider(1.0);
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



// --- Remote Control Logic ---
const setupRemoteControl = async () => {
    // 1. Listeners for Remote Commands
    const listeners = [
        // Play/Pause
        await listen('remote_command::toggle_play', () => togglePlayPauseOrReset()),
        
        // Restart
        await listen('remote_command::restart_animation', () => resetAnimation()),
        
        // Rewind
        await listen('remote_command::start_rewind', () => { isRewinding.value = true; }),
        await listen('remote_command::stop_rewind', () => { isRewinding.value = false; }),
        
        // Speed
        await listen('remote_command::increase_speed', () => {
             sliderPosition.value = Math.min(100, sliderPosition.value + 5);
        }),
        await listen('remote_command::decrease_speed', () => {
             sliderPosition.value = Math.max(0, sliderPosition.value - 5);
        }),
        await listen('remote_command::update_speed', (event) => {
             if (event.payload !== undefined) {
                 // Handle both raw value and { speed: val } object
                 let newSpeed;
                 if (typeof event.payload === 'object' && event.payload !== null) {
                     newSpeed = parseFloat(event.payload.speed);
                 } else {
                     newSpeed = parseFloat(event.payload);
                 }
                 
                 if (!isNaN(newSpeed)) {
                     sliderPosition.value = mapSpeedToSlider(newSpeed);
                 }
             }
        }),
        await listen('remote_command::set_speed_to_1x', () => {
             sliderPosition.value = mapSpeedToSlider(1.0);
        }),
        
         // Widget Toggles
        await listen('remote_command::toggle_commands_widget', () => { isControlsCardVisible.value = !isControlsCardVisible.value; }),
        await listen('remote_command::toggle_altitude_profile', () => { isAltitudeVisible.value = !isAltitudeVisible.value; }),
        await listen('remote_command::toggle_communes_display', () => { isCommuneWidgetVisible.value = !isCommuneWidgetVisible.value; }),
        await listen('remote_command::toggle_distance_display', () => { isDistanceDisplayVisible.value = !isDistanceDisplayVisible.value; }),

        // MAPPING FIXES based on Remote client HTML labels:
        // "Boussole" (Compass) uses ID "toggle-weather-dynamic" -> toggle_weather_dynamic
        await listen('remote_command::toggle_weather_dynamic', () => { isCompassVisible.value = !isCompassVisible.value; }),
        
        // "Météo" (Weather) uses ID "toggle-weather-static" -> toggle_weather_static
        await listen('remote_command::toggle_weather_static', () => { isWeatherInfoVisible.value = !isWeatherInfoVisible.value; }),
        
        // Camera Control
        await listen('remote_command::update_camera', (event) => {
            if (!map.value || !event.payload) return;
            const { type, dx, dy } = event.payload;
            
            // Convert to numbers explicitly
            const fDx = parseFloat(dx) || 0;
            const fDy = parseFloat(dy) || 0;

            switch(type) {
                case 'pan':
                    // Invert deltas for natural panning (dragging moves map under camera)
                    map.value.panBy([-fDx, -fDy], { animate: false });
                    break;
                case 'zoom':
                    // Sensitivity: 100px = 1 zoom level
                    const currentZoom = map.value.getZoom();
                    map.value.setZoom(currentZoom - (fDy * 0.015)); 
                    break;
                case 'bearing':
                    // Sensitivity: 1px = 0.5 degree
                    const currentBearing = map.value.getBearing();
                    map.value.setBearing(currentBearing + (fDx * 0.5));
                    break;
                case 'tilt':
                    // Sensitivity: 1px = 0.5 degree
                    const currentPitch = map.value.getPitch();
                    map.value.setPitch(currentPitch - (fDy * 0.5));
                    break;
            }
            // Force map repaint to reflect immediate changes
            map.value.triggerRepaint();
        }),
        
        // Keep these if original intent persists, but map is now specific
        // ...
    ];
    unlistenFunctions.push(...listeners);
    
    // 2. Initial State Sync
    sendVisualizeStateUpdate();
    invoke('update_animation_speed', { speed: currentSpeed.value });
};

const sendVisualizeStateUpdate = () => {
    const state = {
        isControlsCardVisible: isControlsCardVisible.value,
        isAltitudeVisible: isAltitudeVisible.value,
        isCommuneWidgetVisible: isCommuneWidgetVisible.value,
        isDistanceDisplayVisible: isDistanceDisplayVisible.value,
        
        // Map State -> Remote IDs
        isStaticWeatherVisible: isWeatherInfoVisible.value,
        isDynamicWeatherVisible: isCompassVisible.value,
        
        currentSpeed: currentSpeed.value,
        animationState: animationState.value
    };
    invoke('update_visualize_view_state', { state });
};

// Watchers for State Sync
watch([
    isControlsCardVisible, 
    isAltitudeVisible, 
    isCommuneWidgetVisible, 
    isDistanceDisplayVisible, 
    isWeatherInfoVisible,
    isCompassVisible,
    animationState
], () => {
    sendVisualizeStateUpdate();
});

// Watch Speed separately
watch(currentSpeed, (newSpeed) => {
    invoke('update_animation_speed', { speed: newSpeed });
    sendVisualizeStateUpdate();
});

// --- Lifecycle ---
onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('keyup', handleKeyup);
    setupRemoteControl();
    // Do not call init here directly. Wait for settings.
});

// Watch settings changes that mandate restart/update...
const unwatchSettings = watch(settings, (newSettings) => {
    if (newSettings && mapboxToken.value && !isMapLoaded.value) {
         initializeVisualization();
    }
}, { immediate: true, deep: true });

// Sync isPaused with animationState for UI visibility
watch(isPaused, async (newVal, oldVal) => {
    if (newVal) {
        if (animationState.value === 'En_Animation') {
            animationState.value = 'En_Pause';
        }
    } else {
        if (oldVal === true && (animationState.value === 'En_Pause' || animationState.value === 'En_Pause_au_Depart')) {
            // Fix: Smooth Resume if user moved the camera during pause
            if (map.value && trackingPointsWithDistanceRef.value.length > 0) {
                const currentDistKm = (currentDistanceInMeters.value || 0) / 1000;
                
                // Find the theoretical camera position on track
                // We use the same logic as updateCameraPosition but just to get the target
                const { target } = updateCameraPosition(currentDistKm, trackingPointsWithDistanceRef.value, controlPointIndicesRef.value, {
                    dynamicZoomIntensity: dynamicZoomIntensity.value,
                    currentSpeed: currentSpeed.value,
                    lineStringRef: lineStringRef,
                    isMultisegment: false,
                    activeVariantSegments: [],
                    apply: false // Correct parameter name from useCameraInterpolator
                });

                if (target) {
                    isFlytoActive.value = true;
                    await flyToPromise({
                        center: target.center, zoom: target.zoom, pitch: target.pitch, bearing: target.bearing,
                        duration: 1200 // Smooth transition
                    });
                    isFlytoActive.value = false;
                }
            }
            animationState.value = 'En_Animation';
        }
    }
});

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
