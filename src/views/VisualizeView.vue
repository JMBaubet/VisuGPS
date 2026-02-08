<template>
  <VisualizeMapArea
    ref="visualizeMapAreaRef"
    :is-cursor-hidden="isCursorHidden"
    :is-center-marker-visible="isCenterMarkerVisible"
    :center-marker-color="couleurCroixCentrale"
    :show-back-button="showWidgets && isBackButtonVisibleFinal"
    :is-initializing="isInitializing"
    @go-back="goBack"
    @register-map-container="(el) => { mapContainer = el }"
  />

  <transition name="fade">
    <VisualizeInfoDisplay
        v-if="showWidgets && is3DWidgetsReady"
        :is-visible="true"
        :show-distance="isDistanceDisplayVisible"
        :distance-display="distanceDisplay"
        :total-distance="totalDistanceRef / 1000"
        :show-commune="shouldShowCommuneWidget && isCommuneWidgetVisible"
        :commune-name="currentCommuneName"
        :commune-border-color="communeWidgetBorderColor"
    />
  </transition>

  <div class="top-right-container" style="position: absolute; top: 10px; right: 10px; z-index: 1000; pointer-events: none;">
    <transition name="fade">
        <WeatherWidgetDynamic 
            v-if="showWidgets && is3DWidgetsReady && currentWeather && (isWeatherInfoVisible || isCompassVisible)" 
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
    v-if="showWidgets"
    :is-visible="showWidgets"
    :is-altitude-visible="isAltitudeVisible && is3DWidgetsReady"
    :allow-overlay="is3DContext"
    v-model:is-paused="isPaused"
    :is-animation-finished="isAnimationFinished"
    v-model:is-rewinding="isRewinding"
    v-model:current-speed="currentSpeed"
    :controls-visible="isControlsCardVisible"
    :min-speed="minSpeedValue"
    :max-speed="maxSpeedValue"
    :default-speed="defaultSpeedValue"
    @reset="resetAnimation"
    @trigger-final-view="handleEndSequence(true)"
  >
    <template #altitude-chart v-if="showWidgets && is3DWidgetsReady">
      <transition name="fade" appear>
        <div>
            <AltitudeSVG 
                v-if="isMainTrace"
                :key="`altitude-${props.circuitId}-${totalDistanceRef}`"
                :circuit-id="props.circuitId"
                :current-distance="currentDistanceInMeters"
                :total-distance="totalDistanceRef"
                :tracking-points="trackingPointsWithDistanceRef"
                :is-variant-comparison="false"
                @jump-requested="(distRef) => handleJumpRequest(distRef / 1000)"
            />
            
            <AltitudeVariantSVG 
                v-else
                :key="`altitude-variant-${props.circuitId}`"
                :current-distance="currentDistanceInMeters" 
                :total-main-distance="masterTraceTotalDistance"
                :main-trace-points="masterTrackingPoints"
                :abandoned-segments="abandonedSegmentsRef"
                :variant-blue-segments="variantBlueSegmentsRef"
                @jump-requested="handleJumpRequest"
            />
        </div>
      </transition>
    </template>

    <template #final-action>
        <v-menu v-if="isVariantTrace" location="top center" offset="10" open-on-hover>
            <template v-slot:activator="{ props: menuProps }">
                <v-btn icon="mdi-map-marker-radius-outline" variant="text" size="small" v-bind="menuProps"
                       :disabled="!isPaused || isAnimationFinished || !is3DContext"
                ></v-btn>
            </template>
            <v-list density="compact" class="bg-surface pa-0 elevation-10" style="border-radius: 8px; min-width: 40px;">
                <v-list-item v-for="(item, i) in navigationItems" :key="i" @click="handleNavigationClick(item)"
                             link class="pa-0 justify-center">
                    <div class="d-flex justify-center w-100 py-2">
                        <v-icon :icon="item.icon" :color="item.color" size="small"></v-icon>
                    </div>
                </v-list-item>
            </v-list>
        </v-menu>
    </template>


    <template #extra-controls>
        <!-- Variant Switching & Return -->
        <v-divider vertical class="mx-2"></v-divider>
        
        <!-- Return to Main Trace -->
        <v-btn v-if="isVariantTrace && is3DContext"
            icon="mdi-map-marker-distance" 
            variant="text" 
            color="secondary"
            title="Retour Trace Principale" 
            :disabled="!isPaused"
            @click="returnToMainTrace">
        </v-btn>
    </template>
    
    <template #extra-overlay-actions>
       <!-- Overlay vidé à la demande de l'utilisateur pour une interface strictement minimale -->
    </template>
  </VisualizeControls>

    <!-- Variant Selection Dialog -->
    <v-dialog v-model="showVariantSelection" persistent max-width="500" scrim="black" opacity="0.5">
        <v-card>
            <v-card-title class="text-h5 bg-primary text-white">Choisir une variante</v-card-title>
            <v-list>
                <v-list-item v-for="v in availableVariants" :key="v.id" @click="selectVariant(v.id)" link>
                    <template v-slot:prepend>
                        <v-icon icon="mdi-map-marker-path" color="primary"></v-icon>
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
  variantId: { type: String, default: null }, // Optional
  traceType: { 
    type: String, 
    default: 'variant', // Par défaut variante pour compatibilité
    validator: (v) => ['main', 'variant'].includes(v)
  }
});

const router = useRouter();
const route = useRoute();

const showVariantSelection = ref(false);
const availableVariants = ref([]);
const selectedVariantId = ref(props.variantId || route.query.variantId);

const selectVariant = (id) => {
    selectedVariantId.value = id;
    showVariantSelection.value = false;
    
    // Update URL to reflect the new variant
    router.replace({ 
        name: 'VisualizeVariant', 
        params: { circuitId: props.circuitId, variantId: id },
        query: route.query 
    });
    
    initializeVisualization();
};
const { settings, getSettingValue } = useSettings();
const { showSnackbar } = useSnackbar();
const { interruptUpdate } = useCommunesUpdate();
const { toHex } = useVuetifyColors();
const { createMessageSVG } = useMessageDisplay();
const { isBackButtonVisible, toggleBackButtonVisibility } = useSharedUiState();

// --- Mode Detection ---
const isVariantTrace = computed(() => !!selectedVariantId.value);
const isMainTrace = computed(() => !selectedVariantId.value);
const isDirectStart = computed(() => route.query.directStart === 'true' || isVariantTrace.value);

// --- Initialization + Refs ---
const mapContainer = ref(null);
const isCursorHidden = ref(false); 
let cursorTimeout = null;

const handleInteraction = () => {
    isCursorHidden.value = false;
    if (cursorTimeout) clearTimeout(cursorTimeout);
    cursorTimeout = setTimeout(() => {
        // Hide cursor only if not paused (optional, but usually desired in full screen)
        // For now, let's keep it simple: hide after 3s of inactivity
        isCursorHidden.value = true;
    }, 3000);
};
const animationState = ref('Initialisation'); // Initialisation, Vol_Vers_Vue_Globale, Vol_Vers_Depart, En_Animation, En_Pause, Termine
const is3DContext = computed(() => {
    // Les widgets "3D" ne s'affichent que lors de la visualisation active (départ, animation, pause)
    // Ils sont masqués pendant l'intro (Standard) et la sortie (FlyTo Global)
    return ['En_Animation', 'En_Pause', 'En_Pause_au_Depart'].includes(animationState.value);
});
const is3DWidgetsReady = computed(() => {
    return is3DContext.value && !!(!isWeatherInfoVisible.value && !isCompassVisible.value || currentWeather.value);
});
const showWidgets = ref(false);
const hasVariants = ref(false);

// --- Settings Computed ---
const mapboxToken = computed(() => getSettingValue('Système/Tokens/mapbox'));
const mapStyle = computed(() => getSettingValue('Visualisation/Vue 3D/Carte/styleVisualisation'));
const styleLancement = computed(() => getSettingValue('Visualisation/Lancement/styleLancement'));
// Use Edition setting as verified source
const terrainExaggeration = computed(() => getSettingValue('Edition/Vue 3D/Carte/exaggeration')); 
const zoomEurope = computed(() => getSettingValue('Visualisation/Lancement/zoomEurope'));
const durationEuropeToTrace = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/durationEuropeToTrace')));
const durationTraceToStart = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/durationTraceToStart')));
const pauseBeforeStart = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/pauseBeforeStart')));
const repriseAutoVueTrace = computed(() => getSettingValue('Visualisation/Lancement/repriseAutoVueTrace'));
const pauseAuKm0 = computed(() => formatDuration(getSettingValue('Visualisation/Lancement/pauseAuKm0'))); 
const repriseAutoKm0 = computed(() => getSettingValue('Visualisation/Lancement/repriseAutoKm0'));
const flyToKm0Duration = computed(() => formatDuration(getSettingValue('Visualisation/Finalisation/flyToKm0Duration'))); // Fixed path
const flyToGlobalDuration = computed(() => formatDuration(getSettingValue('Visualisation/Finalisation/flyToGlobalDuration'))); // Fixed path
const delayAfterAnimationEnd = computed(() => formatDuration(getSettingValue('Visualisation/Finalisation/delayAfterAnimationEnd'))); // Fixed path
const repriseAutomatique = computed(() => getSettingValue('Visualisation/Finalisation/repriseAutomatique')); // Nouveau paramètre
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

// Remote Control Sensitivities
const remoteSensX = computed(() => (getSettingValue('Système/Télécommande/sensibilitePointDeVueX') ?? 200) / 100);
const remoteSensY = computed(() => (getSettingValue('Système/Télécommande/sensibilitePointDeVueY') ?? 200) / 100);
const remoteSensCap = computed(() => (getSettingValue('Système/Télécommande/sensibiliteCap') ?? 50) / 100);
const remoteSensZoom = computed(() => (getSettingValue('Système/Télécommande/sensibiliteZoom') ?? 50) / 100);
const remoteSensTilt = computed(() => (getSettingValue('Système/Télécommande/sensibiliteTilt') ?? 50) / 100);

// --- Helper: Contextual Setting Path Resolver (Phase 6) ---
const getSettingPath = (leafPath) => {
    // 1. Widgets (Common structure suffix)
    if (leafPath.startsWith('Widgets/')) {
        if (isMainTrace.value) {
            return `Visualisation/${leafPath}`;
        } else {
            return `Variante/Visualisation/${leafPath}`;
        }
    }

    // 2. Custom Mappings (Trace, Map, etc.)
    const mappings = {
        'couleurTrace': {
            main: 'Visualisation/Vue 3D/Trace/couleurTrace',
            variant: 'Variante/Visualisation/couleurTrace'
        },
        'epaisseurTrace': {
             main: 'Visualisation/Vue 3D/Trace/epaisseurTrace',
             variant: 'Variante/Visualisation/epaisseurSegments' // Approximate fallback
        }
    };

    if (mappings[leafPath]) {
        return isMainTrace.value ? mappings[leafPath].main : mappings[leafPath].variant;
    }

    return leafPath;
};

// --- Computed Parameters (Unified) ---
const isDistanceDisplayVisible = ref(getSettingValue(getSettingPath('Widgets/distance')) ?? true);
const isControlsCardVisible = ref(getSettingValue(getSettingPath('Widgets/commandes')) ?? false);
const isCommuneWidgetVisible = ref(getSettingValue(getSettingPath('Widgets/communes')) ?? true);
const isAltitudeVisible = ref(getSettingValue(getSettingPath('Widgets/altitude')) ?? true);
const isWeatherInfoVisible = ref(getSettingValue(getSettingPath('Widgets/meteo')) ?? true);
const isCompassVisible = ref(getSettingValue(getSettingPath('Widgets/boussole')) ?? true);

// Unified Trace Color (used by setupTraceLayers in main mode, etc.)
const traceColorComputed = computed(() => toHex(getSettingValue(getSettingPath('couleurTrace'))));

// --- Variant Visualization Settings (Specific) ---
const showSegments = computed(() => getSettingValue('Variante/Visualisation/afficherSegments'));
const showSlope = computed(() => getSettingValue('Variante/Visualisation/afficherPente'));
const segmentThickness = computed(() => getSettingValue('Variante/Visualisation/epaisseurSegments'));
const segmentOpacity = computed(() => getSettingValue('Variante/Visualisation/opaciteSegments'));
const slopeThickness = computed(() => getSettingValue('Variante/Visualisation/epaisseurPente'));
const slopeOpacity = computed(() => getSettingValue('Variante/Visualisation/opacitePente'));
const colorNew = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurNouveau')));
const colorCommon = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurCommun')));
const colorAbandoned = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurAbandonne')));
const colorTraceVariant = computed(() => toHex(getSettingValue('Variante/Visualisation/couleurTrace'))); // Kept for specific variant logic
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
const cameraMoved = ref(false);

const { isPaused, isRewinding, isAnimationFinished, currentSpeed, currentDistanceInMeters, distanceDisplay, currentTraceBearing, startAnimation, pauseAnimation, resetTime, updateTime, accumulatedTime, setTimeFromDistance } = useAnimationController();

// --- Speed Control Logic (Restore Logarithmic) ---
const sliderPosition = ref(25); 
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
    const calculatedSlider = mapSpeedToSlider(newVal);
    if (Math.abs(calculatedSlider - sliderPosition.value) > 1) {
        sliderPosition.value = calculatedSlider;
    }
}, { immediate: true });

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

// --- Navigation Menu Computed ---
const navigationItems = computed(() => {
    const list = [];
    
    // Sort variant segments by distance
    const sorted = [...variantBlueSegmentsRef.value].sort((a,b) => a.points[0].distance - b.points[0].distance);
    
    sorted.forEach((seg, index) => {
        let icon = 'mdi-map-marker-path';
        let color = 'primary';
        let name = seg.name;
        
        if (seg.type === 'DEPART') {
            icon = 'mdi-ray-start-arrow';
            color = 'success';
            if (!name) name = 'Départ';
        } else if (seg.type === 'ARRIVEE') {
            icon = 'mdi-ray-end-arrow';
            color = 'error';
            if (!name) name = 'Arrivée';
        } else {
             if (!name) name = `Segment ${index + 1}`;
        }
        
        list.push({
            icon,
            color,
            name,
            distanceKm: seg.points[0].distance,
            type: seg.type
        });
    });
    
    // Final View Item
    list.push({
        icon: 'mdi-clock-end',
        color: 'grey-darken-1',
        name: 'Vue Finale',
        distanceKm: totalDistanceRef.value / 1000,
        type: 'FIN',
        isFinalView: true
    });
    
    return list;
});

const handleNavigationClick = (item) => {
    if (item.isFinalView) {
        handleEndSequence();
    } else if (typeof item.distanceKm === 'number') {
        handleJumpRequest(item.distanceKm);
    }
};



// --- Legacy State ---
const isInitializing = ref(true);
const avancementCommunes = ref('Non calculé');
const activePopups = new Map();
let unlistenFunctions = [];

// Widgets State
// Widgets State (Unified moved up)
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

const returnToMainTrace = () => { 
    selectedVariantId.value = null;
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
    router.replace({ 
        name: 'Visualize', 
        params: { circuitId: props.circuitId },
        query: query
    }); 
    initializeVisualization();
};
const goBack = () => { router.push({ name: 'Main' }); };
const goToVariantView = () => { 
    if (availableVariants.value.length > 1) {
        showVariantSelection.value = true;
    } else if (availableVariants.value.length === 1) {
        selectVariant(availableVariants.value[0].id);
    }
};
const getToHexImproved = (n) => toHex(getSettingValue(n));

// --- User Interactions ---
const togglePlayPauseOrReset = () => {
    if (isAnimationFinished.value) {
        resetAnimation();
    } else {
        isPaused.value = !isPaused.value;
    }
};

// --- Initialization Logic ---
const initializeVisualization = async () => {
    // Ensure settings are loaded before anything else
    await useSettings().initSettings();

    isInitializing.value = true;
    showWidgets.value = false;
    
    // Reset Data & Animation State
    trackingPointsWithDistanceRef.value = [];
    variantBlueSegmentsRef.value = [];   // Prevent ghost segments during reset
    abandonedSegmentsRef.value = [];     // Prevent ghost segments
    isAnimationFinished.value = false;

    // Clear existing popups
    activePopups.forEach(p => p.remove());
    activePopups.clear();

    resetAnimation(); 
    
    try {
        // 0.5 Ensure we have a variant ID (only for variant mode)
        if (isVariantTrace.value && !selectedVariantId.value) {
             showSnackbar("Aucune variante spécifiée.", "error");
             setTimeout(() => goBack(), 1000);
             return;
        }

        // 1. Load Circuit Metadata
        const { circuit } = await loadCircuitData(props.circuitId);
        currentCircuitRef.value = circuit;
        if (circuit) avancementCommunes.value = circuit.avancementCommunes;

        // Load variants list
        const variants = await invoke('get_variants', { circuitId: props.circuitId });
        availableVariants.value = variants;
        hasVariants.value = variants.length > 0;

        // 2. Load LineString & Calculate Trace Center
        let lineString;
        let traceCenter;
        
        if (isMainTrace.value) {
            // Mode Trace Principale : charger directement la trace principale
            lineString = await invoke('read_line_string_file', { circuitId: props.circuitId });
            lineStringRef.value = lineString;
            
            // Pas de données master en mode main
            masterTraceGeoJson.value = null;
            masterTraceTotalDistance.value = 0;
            masterTrackingPoints.value = [];
            
            // Calculate trace center
            traceCenter = turf.center(lineString).geometry.coordinates;
        } else {
            // Mode Variante : charger master + variante
            // A. Load MASTER Trace (Source of truth for original segments)
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
            
            // B. Load VARIANT LineString
            lineString = await invoke('read_line_string_file', { 
                circuitId: props.circuitId, 
                filename: `lineString_${selectedVariantId.value}_FULL.json` 
            });
            lineStringRef.value = lineString;
            
            // Calculate trace center from master
            traceCenter = turf.center(masterGeoJson).geometry.coordinates;
        }
        
        // 0. Map Init (Pre-load to avoid black screen) - Now using trace center
        let initialCenter = traceCenter; // Use trace center instead of Paris
        let initialZoom = zoomEurope.value;
        if (route.query.lat && route.query.lng && route.query.zoom) {
             initialCenter = [parseFloat(route.query.lng), parseFloat(route.query.lat)];
             initialZoom = parseFloat(route.query.zoom);
        }
        
        if (!map.value) {
            // Mode INITIAL SETUP
            let finalCenter = initialCenter;
            let finalZoom = initialZoom;
            let finalStyle = null;

            if (!isDirectStart.value) {
                // Séquence Standard : Démarrer sur l'Europe avec le style de lancement
                finalZoom = zoomEurope.value;
                finalStyle = styleLancement.value;
            } else {
                showWidgets.value = true;
            }

            let instance = await initMapEngine(finalCenter, finalZoom, finalStyle);
            if(!instance) throw new Error("Map failed to init");
            
            // Listen for style data changes (which wipe custom layers)
            instance.on('styledata', () => {
                 if (map.value && !map.value.getLayer('trace-main-abandoned') && masterTraceGeoJson.value) {
                     setupTraceLayers({
                        traceWidth: traceWidth.value, traceOpacity: traceOpacity.value, traceColor: colorTraceVariant.value,
                        coloredSegmentsData: coloredSegmentsGeoJsonRef.value,
                        masterTraceData: masterTraceGeoJson.value, 
                        cometWidth: cometWidth.value, cometColor: cometColor.value, cometOpacity: cometOpacity.value,
                        slopeExpression: slopeExpressionRef.value,
                        segmentThickness: segmentThickness.value, segmentOpacity: segmentOpacity.value,
                        slopeThickness: slopeThickness.value, slopeOpacityLogic: slopeOpacity.value,
                        colorNew: colorNew.value, colorCommon: colorCommon.value, colorAbandoned: colorAbandoned.value
                    });
                    
                    // Restore Visibility
                    if (isVariantTrace.value) {
                        updateLayerVisibility(true, showSegments.value);
                        updateLayerVisibility('trace-main-abandoned', showAbandoned.value);
                    }
                 }
            });
            
            // Reveal map as soon as style is ready (Europe view)
            if (!isDirectStart.value) {
                isInitializing.value = false;
            }

            // setMinZoom will be applied AFTER initial sequences to allow zoom 5 at start
            
            // Detect manual camera movement during pause
            instance.on('movestart', (e) => {
                if (isPaused.value && e.originalEvent) {
                    cameraMoved.value = true;
                }
            });

            if (route.query.bearing && route.query.pitch) {
                instance.jumpTo({
                    bearing: parseFloat(route.query.bearing),
                    pitch: parseFloat(route.query.pitch)
                });
            }
            startBearingTracking();
        }
        
        // 3. Load Tracking Data
        let trackingData;
        
        if (isMainTrace.value) {
            // Mode Trace Principale : charger et traiter le tracking standard
            const trackingRaw = await invoke('read_tracking_file', { 
                circuitId: props.circuitId, 
                filename: 'tracking.json' 
            });
            const processed = await invoke('process_tracking_data', { 
                lineStringGeojson: lineString,
                trackingPointsJs: trackingRaw 
            });
            trackingPointsWithDistanceRef.value = processed.processedPoints;
            totalDistanceRef.value = processed.totalDistanceKm * 1000; // En mètres
        } else {
            // Mode Variante
            // A. Load MASTER Tracking Points (for Altitude Profile)
            const masterTrackingRaw = await invoke('read_tracking_file', { 
                circuitId: props.circuitId, 
                filename: 'tracking.json' 
            });
            const masterProcessed = await invoke('process_tracking_data', { 
                lineStringGeojson: masterTraceGeoJson.value,
                trackingPointsJs: masterTrackingRaw 
            });
            masterTrackingPoints.value = masterProcessed.processedPoints;

            // B. Load Variant Details (Metadata + Stats)
            const variantArchive = await invoke('get_variant_details', { 
                circuitId: props.circuitId, 
                variantId: selectedVariantId.value 
            });
            modifications.value = variantArchive.modifications || [];
            variantStats.value = {
                total: (variantArchive.metadata?.stats?.totalDistance || 0),
                current: 0
            };

            // C. Load Overlap Metadata (Aller/Retour)
            try {
                segmentMetadata.value = await invoke('get_variant_overlap_metadata', { 
                    circuitId: props.circuitId, 
                    variantId: selectedVariantId.value 
                });
                if (segmentMetadata.value) {
                    // console.log(`[VisualizeVariant] Metadata loaded: ${segmentMetadata.value.overlappingZones?.length || 0} overlap zones detected.`);
                }
            } catch (e) {
                console.warn("[VisualizeVariant] Could not fetch variant overlap metadata:", e);
                segmentMetadata.value = null;
            }

            // D. Load RECONSTITUTED Tracking (FULL)
            trackingData = await invoke('read_tracking_file', { 
                circuitId: props.circuitId, 
                filename: `tracking_${selectedVariantId.value}_FULL.json` 
            });
            
            trackingPointsWithDistanceRef.value = trackingData;
            
            // Use exact total distance from tracking
            const nominalTotalKm = trackingData[trackingData.length - 1].distance;
            totalDistanceRef.value = nominalTotalKm * 1000;
        } 

        // 4. Calculate Duration
        const nominalTotalKm = totalDistanceRef.value / 1000;
        const msPerKm = getSettingValue('Visualisation/Lecture/vitesse') || 3730;
        totalDurationAt1xRef.value = nominalTotalKm * msPerKm; 

        // 5. Populate control points
        controlPointIndicesRef.value = trackingPointsWithDistanceRef.value
            .map((p, i) => (p.pointDeControl ? i : -1))
            .filter(i => i !== -1);

        // 6. Load Events
        const eventsFilename = isVariantTrace.value ? `${selectedVariantId.value}_FULL` : null;
        try {
            const events = await invoke('get_events', { 
                circuitId: props.circuitId, 
                variantId: eventsFilename 
            });

            if (events && events.pointEvents) {
                pauseIncrements.value = Object.keys(events.pointEvents)
                    .filter(k => events.pointEvents[k].some(e => e.type === 'Pause'))
                    .map(Number);

                const flytos = {};
                Object.keys(events.pointEvents).forEach(k => {
                    const ev = events.pointEvents[k].find(e => e.type === 'Flyto');
                    if (ev) flytos[Number(k)] = ev.data;
                });
                flytoEvents.value = flytos;
            }
            rangeEvents.value = events?.rangeEvents || [];
        } catch (e) {
            console.warn("[Visualize] No events found:", e);
            pauseIncrements.value = [];
            flytoEvents.value = {};
            rangeEvents.value = [];
        }

        // 7. Initialize Weather
        await initWeather(circuit, trackingPointsWithDistanceRef.value);

        // 8. Generate Slope Colors
        const slopeColors = {
                TrancheNegative: getToHexImproved('Visualisation/Profil Altitude/Couleurs/TrancheNegative'),
                Tranche1: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche1'),
                Tranche2: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche2'),
                Tranche3: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche3'),
                Tranche4: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche4'),
                Tranche5: getToHexImproved('Visualisation/Profil Altitude/Couleurs/Tranche5'),
         };
         
        // 9. Load Slope Expression (Variant only)
        if (isVariantTrace.value) {
            const expression = await invoke('get_variant_slope_expression', { 
                circuitId: props.circuitId, 
                variantId: selectedVariantId.value,
                slopeColors: slopeColors
            });
            slopeExpressionRef.value = expression;
        }

        // 10. Generate Colored Segments (Slope-based + Aller/Retour)
        try {
            const geojson = await invoke('get_colored_segments_geojson', { 
                circuitId: props.circuitId, 
                variantId: isVariantTrace.value ? selectedVariantId.value : null,
                slopeColors: slopeColors,
                segmentLength: segmentLength.value
            });
            coloredSegmentsGeoJsonRef.value = geojson;
            
            // 11. Extract Abandoned & Blue Segments (Variant only)
            if (isVariantTrace.value) {
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
                            name: modif.name, // Support custom segment names
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
            } else {
                // Mode trace principale : pas de segments abandonnés ou bleus
                abandonedSegmentsRef.value = [];
                variantBlueSegmentsRef.value = [];
            }
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

        // Variant-specific watchers and initialization
        if (isVariantTrace.value) {
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
        }
        
        // Initial Overlap Check (to hide loops on start if any)
        checkLayers(0); 
        
        // 12. Animation Sequence
        const startPoint = trackingPointsWithDistanceRef.value[0];
        const mapInstance = map.value;
        
        if (isDirectStart.value) {
            // Mode Direct (Variante OU DirectStart pour trace principale)
            animationState.value = 'Vol_Vers_Depart';
            
            // Afficher les widgets (ils seront filtrés par isIntroSequence pour Altitude/Ville/Distance)
            showWidgets.value = true;
            
            // Appliquer le zoom minimum
            mapInstance.setMinZoom(zoomMinimum.value);
            
            // Vol direct vers km 0
            isFlytoActive.value = true;
            
            // Reveal map for direct start (km 0 is usually zoomed in, show it immediately)
            isInitializing.value = false;

            await flyToPromise({
                center: startPoint.coordonnee,
                zoom: startPoint.editedZoom ?? startPoint.zoom ?? 16,
                pitch: startPoint.editedPitch ?? startPoint.pitch ?? 45,
                bearing: startPoint.editedCap ?? startPoint.cap ?? 0,
                duration: 3000
            });
            isFlytoActive.value = false;
            
        } else {
            // Mode Standard (Trace Principale avec séquence complète)
            animationState.value = 'Vol_Vers_Vue_Globale';
            showWidgets.value = true; // Permettre l'affichage des Commandes dès maintenant
            
            // Calculer le zoom cible pour voir toute la trace
            const traceBbox = turf.bbox(lineStringRef.value);
            const globalView = mapInstance.cameraForBounds(traceBbox, { padding: 40, bearing: 0, pitch: 0 });
            
            isFlytoActive.value = true;
            
            // Démarrer le flyTo (carte déjà révélée ou en cours d'apparition)
            const flyToTask = flyToPromise({
                zoom: globalView.zoom,
                bearing: 0,
                pitch: 0
            }, { duration: durationEuropeToTrace.value });
            
            // Déclencher les messages à 50% du vol depuis l'Europe
            setTimeout(() => {
                checkEvents(0);
            }, durationEuropeToTrace.value / 2);

            // Attendre 200ms pour s'assurer que le zoom a bien commencé (optionnel, pour fluidité)
            await new Promise(r => setTimeout(r, 200));
            isInitializing.value = false; // Sécurité si pas déjà fait
            
            // Attendre la fin du flyTo
            await flyToTask;
            isFlytoActive.value = false;
            
            // Appliquer le zoom minimum MAINTENANT que la séquence Europe est terminée
            mapInstance.setMinZoom(zoomMinimum.value);
            
            animationState.value = 'Pause_Observation';
            
            // Pause d'observation
            if (repriseAutoVueTrace.value) {
                await new Promise(r => setTimeout(r, pauseBeforeStart.value));
            } else {
                // Pause manuelle
                isPaused.value = true;
                await new Promise(resolve => {
                    const checkResume = () => {
                        if (!isPaused.value) {
                            resolve();
                        } else {
                            setTimeout(checkResume, 100);
                        }
                    };
                    checkResume();
                });
            }
            
            animationState.value = 'Vol_Vers_Depart';
            
            // Changement de style si nécessaire
            if (mapStyle.value !== styleLancement.value) {
                mapInstance.setStyle(mapStyle.value);
                await new Promise(resolve => mapInstance.once('style.load', resolve));
                
                // IMPORTANT: re-add active popups after style change
                activePopups.forEach(p => {
                    if (map.value) p.addTo(map.value);
                });
                
                // Re-setup layers après changement de style
                if (isMainTrace.value) {
                    setupTraceLayers({
                        traceWidth: traceWidth.value,
                        traceOpacity: traceOpacity.value,
                        traceColor: traceColor.value,
                        lineStringData: lineStringRef.value,
                        cometWidth: cometWidth.value,
                        cometColor: cometColor.value,
                        cometOpacity: cometOpacity.value,
                        coloredSegmentsData: coloredSegmentsGeoJsonRef.value
                    });
                } else {
                    setupTraceLayers({
                        traceWidth: traceWidth.value,
                        traceOpacity: traceOpacity.value,
                        traceColor: colorTraceVariant.value,
                        coloredSegmentsData: coloredSegmentsGeoJsonRef.value,
                        masterTraceData: masterTraceGeoJson.value,
                        cometWidth: cometWidth.value,
                        cometColor: cometColor.value,
                        cometOpacity: cometOpacity.value,
                        slopeExpression: slopeExpressionRef.value,
                        segmentThickness: segmentThickness.value,
                        segmentOpacity: segmentOpacity.value,
                        slopeThickness: slopeThickness.value,
                        slopeOpacityLogic: slopeOpacity.value,
                        colorNew: colorNew.value,
                        colorCommon: colorCommon.value,
                        colorAbandoned: colorAbandoned.value
                    });
                    
                    // Force re-apply visibility as style reset might have confused state or z-index
                    updateLayerVisibility(true, showSegments.value);
                    updateLayerVisibility('trace-main-abandoned', showAbandoned.value);
                }
            }
            
            // Vol vers le km 0
            isFlytoActive.value = true;
            await flyToPromise({
                center: startPoint.coordonnee,
                zoom: startPoint.editedZoom ?? startPoint.zoom,
                pitch: startPoint.editedPitch ?? startPoint.pitch,
                bearing: startPoint.editedCap ?? startPoint.cap,
                duration: durationTraceToStart.value
            });
            isFlytoActive.value = false;
        }



        animationState.value = 'En_Pause_au_Depart';
        isInitializing.value = false;
        enableInteraction();
        
        // S'assurer que les widgets sont affichés (au cas où, bien qu'ils le soient déjà)
        showWidgets.value = true;

        // Ensure KM 0 messages are displayed during the initial pause
        await checkEvents(0);

        startAnimation(animateLoop); 
        
        if (pauseAuKm0.value > 0 || !repriseAutoKm0.value) {
             isPaused.value = true;
             
             // Attendre selon la configuration (NON-BLOQUANT)
             if (repriseAutoKm0.value) {
                 // Auto-resume after delay
                 setTimeout(() => {
                     if (isPaused.value && animationState.value === 'En_Pause_au_Depart') {
                         isPaused.value = false;
                     }
                 }, pauseAuKm0.value);
             }
             // Si pas de reprise auto, on reste simplement en pause (isPaused=true).
             // L'utilisateur devra cliquer sur Play (Espace ou Remote) pour débloquer.
             // On ne bloque pas l'exécution ici, sinon setupRemoteControl n'est jamais appelé !
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


    // 4. FlyTo Transition
    await flyToPromise({
        ...targetCamera,
        duration: jumpDuration.value * 1000 
    });
    
    // 5. Reprise
    isFlytoActive.value = false;
    
    // Force remote update
    await nextTick();
    updateRemoteViewState();
    
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
             // console.log(`[VisualizeVariant] Switching Layer to: ${currentDir} at distance ${distanceTraveled.toFixed(2)} km`);
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
    
    // Allow interaction during pause
    isFlytoActive.value = false;
    updateRemoteViewState();
    
    // Wait for Play
    await new Promise(resolve => {
        const stop = watch(isPaused, (val) => {
            if(!val) { stop(); resolve(); }
        });
    });

    isFlytoActive.value = true; // Lock again for return flight
    updateRemoteViewState();

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
    
    // Force remote update to clear "Repositionnement..." overlay
    await nextTick();
    updateRemoteViewState();

    lastTimestamp = 0; // Reset timer for smooth resume
    requestAnimationFrame(animateLoop); // Restart loop explicitly
};

const handleEndSequence = async (skipDelay = false) => {
    if(!map.value) return;

    // Pause à l'arrivée
    if (!skipDelay && delayAfterAnimationEnd.value > 0) {
        await new Promise(r => setTimeout(r, delayAfterAnimationEnd.value));
    }

    // Explicitly set state to Finished (needed if triggered manually via button)
    isAnimationFinished.value = true;
    isPaused.value = true;
    
    animationState.value = 'Vol_Final';
    // Hide UI elements contextually via animationState (is3DContext)
    // We no longer overwrite the user preferences (refs) here to avoid losing them for the next loop.
    
    // Hide Comet
     if (map.value.getSource('comet-source')) {
          map.value.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
     }
    
    // Force visibility of master trace layers for comparison
    updateLayerVisibility('trace-main-abandoned', true);
    
    // Force remote update to clear segments
    updateRemoteViewState();
    
    // Switch to launch style if needed
    if (styleLancement.value !== mapStyle.value) {
        map.value.setStyle(styleLancement.value);
        await new Promise(r => map.value.once('style.load', r));
        
        // IMPORTANT: re-add active popups after style change (they might be removed by Mapbox)
        activePopups.forEach(p => {
             if (map.value) p.addTo(map.value);
        });

        setupTraceLayers({
            traceWidth: traceWidth.value, traceOpacity: traceOpacity.value, traceColor: traceColor.value,
            lineStringData: lineStringRef.value, 
            masterTraceData: masterTraceGeoJson.value,
            coloredSegmentsData: coloredSegmentsGeoJsonRef.value,
            cometWidth: cometWidth.value, cometColor: cometColor.value, cometOpacity: cometOpacity.value,
            segmentThickness: segmentThickness.value, segmentOpacity: segmentOpacity.value,
            colorNew: colorNew.value, colorCommon: colorCommon.value, colorAbandoned: colorAbandoned.value
        });
    }

    // Combined BBox for both traces
    let combinedBbox = null;
    try {
        const variantFeature = { type: 'Feature', geometry: lineStringRef.value.geometry || lineStringRef.value, properties: {} };
        const features = [variantFeature];
        
        if (masterTraceGeoJson.value) {
            const masterFeature = { type: 'Feature', geometry: masterTraceGeoJson.value.geometry || masterTraceGeoJson.value, properties: {} };
            features.push(masterFeature);
        }
        
        combinedBbox = turf.bbox({
            type: 'FeatureCollection',
            features: features
        });
    } catch (e) {
        console.warn("BBox calculation failed, fallback to variant only", e);
        if (lineStringRef.value) {
             combinedBbox = turf.bbox(lineStringRef.value);
        }
    }

    /* Safe implementation of Final FlyTo */
    try {
        const camParams = map.value.cameraForBounds(combinedBbox, { padding: 80, bearing: 0, pitch: 0 });
        if (camParams) {
             isFlytoActive.value = true;
             await flyToPromise({
                center: camParams.center,
                zoom: camParams.zoom,
                pitch: 0, 
                bearing: 0, 
                duration: flyToGlobalDuration.value
            });
            isFlytoActive.value = false;
        }
    } catch (err) {
        console.warn("End sequence flyTo failed", err);
    }
    
    animationState.value = 'Termine';
    
    // Logic Reprise Auto
    if (repriseAutomatique.value) {
        // Optionnel : un petit délai supplémentaire avant de repartir ?
        // Pour l'instant, on lance le reset et le start
        setTimeout(async () => {
            if (animationState.value === 'Termine') {
                await resetAnimation();
                isPaused.value = false; // Relancer l'animation
            }
        }, 1000); // 1s de pause sur la vue finale avant de repartir
    }
};

const resetAnimation = async () => {
    // Reset Logic
    resetTime();
    isPaused.value = true;
    isAnimationFinished.value = false;
    triggeredPauseIncrement.value = null;
    triggeredFlytoIncrement.value = null;
    isFlytoActive.value = false;
    
    // Restore UI (Note: we no longer reset visibility here to persist user toggles across session restarts)
    // isDistanceDisplayVisible.value = getSettingValue('Variante/Visualisation/Widgets/distance') ?? true;
    // ... logic moved to initialization and persisted during session
    
    // Restore Map Style for 3D View if changed
    if (map.value && mapStyle.value && map.value.getStyle()?.name !== mapStyle.value) { 
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

    if (map.value) {
        activePopups.forEach(p => p.remove());
        activePopups.clear();
    }
    
    // Reset Camera
    if(trackingPointsWithDistanceRef.value.length > 0) {
        const start = trackingPointsWithDistanceRef.value[0];
        isFlytoActive.value = true;
        await flyToPromise({
            center: start.coordonnee, zoom: start.editedZoom??start.zoom, pitch: start.editedPitch??start.pitch, bearing: start.editedCap??start.cap, duration: 2000
        });
        isFlytoActive.value = false;
    }
    
    animationState.value = 'En_Pause_au_Depart';
    
    // Force Pause state before checking auto-resume logic
    // (This fixes the issue where isPaused might have flipped to false during the async flyTo)
    isPaused.value = true;

    // Handle Auto-Resume Logic (Same as initialization)
    if (pauseAuKm0.value > 0 || !repriseAutoKm0.value) {
         // Keep paused (isPaused is already true)
         if (repriseAutoKm0.value) {
             // Auto-resume after delay
             setTimeout(() => {
                 if (isPaused.value && animationState.value === 'En_Pause_au_Depart') {
                     isPaused.value = false;
                 }
             }, pauseAuKm0.value);
         }
    } else {
        // No pause required, start immediately
        isPaused.value = false;
    }

    requestAnimationFrame(animateLoop);
};

// --- Remote Control Logic ---
const setupRemoteControl = async () => {
    // Notify starting view based on current mode
    await invoke('update_current_view', { newView: remoteViewName.value });
    
    // Send initial state including segments
    await updateRemoteViewState();

    // 1. Listeners for Remote Commands
    const listeners = [
        // Play/Pause
        await listen('remote_command::toggle_play', () => togglePlayPauseOrReset()),
        
        // Restart
        await listen('remote_command::restart_animation', () => resetAnimation()),
        
        // Final View
        await listen('remote_command::trigger_final_view', () => handleEndSequence(true)),

        // Return to Main Trace
        await listen('remote_command::return_to_main_trace', () => returnToMainTrace()),

        // Home
        await listen('remote_command::go_home', () => goBack()),

        // Rewind
        await listen('remote_command::start_rewind', () => { isRewinding.value = true; }),
        await listen('remote_command::stop_rewind', () => { isRewinding.value = false; }),
        
        // Speed
        await listen('remote_command::increase_speed', () => {
             sliderPosition.value = Math.min(100, sliderPosition.value + 5);
        }),
        
        // Segment Navigation
        await listen('remote_command::jump_to_segment', (event) => {
            const index = event.payload?.index;
            if (typeof index === 'number') {
                const segments = buildFullSegmentList();
                if (segments[index]) {
                    // Use computed start for jump
                    handleJumpRequest(segments[index].computedStart);
                }
            }
        }),
        await listen('remote_command::decrease_speed', () => {
             sliderPosition.value = Math.max(0, sliderPosition.value - 5);
        }),
        await listen('remote_command::update_speed', (event) => {
             if (event.payload !== undefined) {
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
        
        // Camera Updates
        await listen('remote_command::update_camera', (event) => {
            const { type, dx, dy } = event.payload;

            if (isPaused.value) cameraMoved.value = true;

            // Map common events to local logic
            switch(type) {
                case 'pan':
                    if (map.value) map.value.panBy([-parseFloat(dx) * remoteSensX.value, -parseFloat(dy) * remoteSensY.value], { duration: 0 });
                    break;
                case 'tilt':
                    if (map.value) map.value.setPitch(map.value.getPitch() - (parseFloat(dy) * remoteSensTilt.value));
                    break;
                case 'zoom':
                    // Sensitivity: base 0.03 * user factor
                    if (map.value) map.value.setZoom(map.value.getZoom() - (parseFloat(dy) * 0.03 * remoteSensZoom.value));
                    break;
                case 'bearing':
                    if (map.value) map.value.setBearing(map.value.getBearing() + (parseFloat(dx) * remoteSensCap.value));
                    break;
            }
        }),

        // Widget Toggles
        await listen('remote_command::toggle_altitude_profile', () => { isAltitudeVisible.value = !isAltitudeVisible.value; }),
        await listen('remote_command::toggle_commands_widget', () => { isControlsCardVisible.value = !isControlsCardVisible.value; }),
        await listen('remote_command::toggle_distance_display', () => { isDistanceDisplayVisible.value = !isDistanceDisplayVisible.value; }),
        await listen('remote_command::toggle_weather_dynamic', () => { isCompassVisible.value = !isCompassVisible.value; }),
        await listen('remote_command::toggle_weather_static', () => { isWeatherInfoVisible.value = !isWeatherInfoVisible.value; }),
        await listen('remote_command::toggle_communes_display', () => { isCommuneWidgetVisible.value = !isCommuneWidgetVisible.value; }),
        
        
        // Final View jump
        await listen('remote_command::trigger_final_view', () => handleEndSequence()),
        
        // Return to main trace
        await listen('remote_command::return_to_main_trace', () => returnToMainTrace()),

        // Select another variant
        await listen('remote_command::trigger_variant_selection', () => { showVariantSelection.value = true; }),
        await listen('remote_command::select_variant', (event) => {
            if (event.payload && event.payload.variantId) {
                selectVariant(event.payload.variantId);
            }
        }),
    ];

    unlistenFunctions.push(...listeners);
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

// --- Lifecycle ---
onMounted(async () => {
    window.addEventListener('keydown', handleInteraction);
    window.addEventListener('keydown', handleKeydown);
    window.addEventListener('keyup', handleKeyup);
    window.addEventListener('mousemove', handleInteraction);
    
    // Initial cursor hide timer
    handleInteraction();

    // Ensure DOM is ready
    await nextTick();

    if (mapboxToken.value) {
        await initializeVisualization();
        setupRemoteControl();
    } else {
        const unwatch = watch(mapboxToken, (token) => {
            if (token) {
                initializeVisualization();
                setupRemoteControl();
                unwatch();
            }
        });
    }
});

// Sync isPaused with animationState for UI visibility
watch(isPaused, async (newVal, oldVal) => {
    if (newVal) {
        cameraMoved.value = false;
        if (animationState.value === 'En_Animation') {
            animationState.value = 'En_Pause';
        }
    } else {
        if (oldVal === true && (animationState.value === 'En_Pause' || animationState.value === 'En_Pause_au_Depart')) {
             // Fix: Smooth Resume if user moved the camera during pause
            if (cameraMoved.value && map.value && trackingPointsWithDistanceRef.value.length > 0) {
                const currentDistKm = (currentDistanceInMeters.value || 0) / 1000;
                
                // Find the theoretical camera position on track
                const { target } = updateCameraPosition(currentDistKm, trackingPointsWithDistanceRef.value, controlPointIndicesRef.value, {
                    dynamicZoomIntensity: dynamicZoomIntensity.value,
                    currentSpeed: currentSpeed.value,
                    lineStringRef: lineStringRef,
                    isMultisegment: false, 
                    activeVariantSegments: [],
                    apply: false
                });

                if (target) {
                    isFlytoActive.value = true;
                    await flyToPromise({
                        center: target.center, 
                        zoom: target.zoom, 
                        pitch: target.pitch, 
                        bearing: target.bearing,
                        duration: 1200 // Smooth transition
                    });
                    isFlytoActive.value = false;
                }
            }
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

// --- Remote Sync ---
watch([
    isControlsCardVisible,
    isAltitudeVisible,
    isCommuneWidgetVisible,
    isDistanceDisplayVisible,
    isWeatherInfoVisible,
    isCompassVisible,
    hasVariants,
    availableVariants,
    animationState,
    isFlytoActive
], () => {
    invoke('update_animation_state', { newState: animationState.value });
    updateRemoteViewState();
});

function generateSlopeSegments(trackingPoints) {
    // Deprecated in favor of Backend expression
    return { type: 'FeatureCollection', features: [] };
}


onUnmounted(() => {
    window.removeEventListener('keydown', handleInteraction);
    window.removeEventListener('keydown', handleKeydown);
    window.removeEventListener('keyup', handleKeyup);
    window.removeEventListener('mousemove', handleInteraction);
    if (cursorTimeout) clearTimeout(cursorTimeout);

    cleanupMap();
    if (mapContainer.value) mapContainer.value.remove();
    activePopups.forEach(p => p.remove());
    activePopups.clear();
     unlistenFunctions.forEach(fn => fn());
});

// --- Remote Control Sync ---
const lastSentSegmentIndex = ref(-1);

function buildFullSegmentList() {
    const rawSegments = [...variantBlueSegmentsRef.value].sort((a,b) => a.points[0].distance - b.points[0].distance);
    const fullList = [];
    const totalDistKm = totalDistanceRef.value / 1000;

    // 1. Add COMMON at start if needed
    if (rawSegments.length > 0 && rawSegments[0].points[0].distance > 0.01) {
        fullList.push({
            name: "Tronçon Commun",
            type: "COMMON",
            computedStart: 0,
            computedEnd: rawSegments[0].points[0].distance,
            isGap: true,
            id: `gap_start`
        });
    }
    
    for (let i = 0; i < rawSegments.length; i++) {
        const seg = rawSegments[i];
        const segStart = seg.points[0].distance;
        
        let segEnd = seg.lengthM ? (segStart + seg.lengthM/1000) : null;
        if (seg.points.length > 1) {
             const lastPtDist = seg.points[seg.points.length - 1].distance;
             if (!segEnd || lastPtDist > segEnd) {
                 segEnd = lastPtDist;
             }
        }
        if (!segEnd) segEnd = segStart + 0.1;

        fullList.push({
            ...seg,
            id: `seg_${i}`,
            computedStart: segStart,
            computedEnd: segEnd,
            isGap: false,
            originalIndex: i
        });
        
        if (i < rawSegments.length - 1) {
            const nextSeg = rawSegments[i+1];
            const nextStart = nextSeg.points[0].distance;
            
            if (segEnd && nextStart > segEnd + 0.01) { 
                fullList.push({
                    name: "Tronçon Commun",
                    type: "COMMON",
                    computedStart: segEnd,
                    computedEnd: nextStart,
                    isGap: true,
                    id: `gap_${i}`
                });
            } else if (segEnd && nextStart > segEnd) {
                fullList[fullList.length - 1].computedEnd = nextStart;
            }
        } else {
            // 2. Add COMMON at end if needed
            if (segEnd && totalDistKm > segEnd + 0.01) {
                fullList.push({
                    name: "Tronçon Commun",
                    type: "COMMON",
                    computedStart: segEnd,
                    computedEnd: totalDistKm,
                    isGap: true,
                    id: `gap_end`
                });
            }
        }
    }

    // Special case: empty rawSegments (unlikely in this view but for safety)
    if (rawSegments.length === 0 && totalDistKm > 0) {
        fullList.push({
            name: "Tronçon Commun",
            type: "COMMON",
            computedStart: 0,
            computedEnd: totalDistKm,
            isGap: true,
            id: `gap_full`
        });
    }

    return fullList;
};

// Compute current segment index based on distance (using FULL list)
const currentSegmentIndex = computed(() => {
    const segments = buildFullSegmentList();
    if (segments.length === 0) return -1;
    
    const curDistKm = currentDistanceInMeters.value / 1000;
    
    // Check each segment
    for (let i = 0; i < segments.length; i++) {
        const seg = segments[i];
        const end = seg.computedEnd !== null ? seg.computedEnd : Infinity;
        
        if (curDistKm >= seg.computedStart && curDistKm <= end) {
            return i;
        }
    }
    
    // If before first segment
    if (curDistKm < segments[0].computedStart) return 0;
    // If after last segment
    if (curDistKm > segments[segments.length - 1].computedEnd) return segments.length - 1;
    
    return -1;
});

const lastNotifiedDistance = ref(-1);

// Watch for widget visibility changes to update remote
watch([isControlsCardVisible, isAltitudeVisible, isCommuneWidgetVisible, isDistanceDisplayVisible, isWeatherInfoVisible, isCompassVisible], () => {
    updateRemoteViewState();
});

// Watch for distance changes to update remote progress (every 100m)
watch(currentDistanceInMeters, (newDist) => {
    // Only notify if distance change >= 100m
    if (Math.abs(newDist - lastNotifiedDistance.value) >= 100) {
        invoke('notify_animation_progress', { 
            currentDistance: newDist / 1000, 
            currentSegmentIndex: currentSegmentIndex.value !== -1 ? currentSegmentIndex.value : null
        }).catch(err => console.error("Failed to notify progress:", err));
        lastNotifiedDistance.value = newDist;
    }
});

// Watch for index change to update remote (Full state remains useful for metadata)
watch(currentSegmentIndex, (newIndex) => {
    if (newIndex !== lastSentSegmentIndex.value) {
        lastSentSegmentIndex.value = newIndex;
        // Trigger update to send new index to remote (and potentially updated metadata)
        updateRemoteViewState(); 
        
        // Also send immediate progress update for the new index
        invoke('notify_animation_progress', { 
            currentDistance: currentDistanceInMeters.value / 1000, 
            currentSegmentIndex: newIndex !== -1 ? newIndex : null
        }).catch(err => console.error("Failed to notify index change:", err));
    }
});

async function updateRemoteViewState() {
    // Transform segments for remote (using FULL list)
    // IMPORTANT: Clear segments if in Final View OR not in Variant mode (prevent fallback "Tronçon Commun")
    const segmentsSource = (animationState.value === 'Vol_Final' || animationState.value === 'Termine' || !isVariantTrace.value) ? [] : buildFullSegmentList();
    
    const segments = segmentsSource.map((seg, idx) => ({
        id: String(idx), // Ensure ID is a String for Rust compatibility
        name: seg.type === 'DEPART' ? 'Départ' : (seg.type === 'ARRIVEE' ? 'Arrivée' : (seg.name || seg.filename || `Segment ${idx + 1}`)),
        segmentType: seg.type,
        startDistance: seg.computedStart,
        endDistance: seg.computedEnd
    }));

    const viewState = {
        isControlsCardVisible: isControlsCardVisible.value,
        isAltitudeVisible: isAltitudeVisible.value,
        isCommuneWidgetVisible: isCommuneWidgetVisible.value,
        isDistanceDisplayVisible: isDistanceDisplayVisible.value,
        isStaticWeatherVisible: isWeatherInfoVisible.value,
        isDynamicWeatherVisible: isCompassVisible.value,
        currentSpeed: currentSpeed.value,
        animationState: animationState.value,
        isFlytoActive: isFlytoActive.value,
        hasVariants: hasVariants.value,
        isVariantTrace: isVariantTrace.value,
        variantCount: availableVariants.value.length,
        variants: availableVariants.value.map(v => ({ id: v.id, name: v.name })),
        segments: segments,
        currentSegmentIndex: currentSegmentIndex.value !== -1 ? currentSegmentIndex.value : null
    };

    invoke('update_visualize_view_state', { state: viewState })
        .catch(err => console.error("[Remote] Failed to update view state:", err));
};

// --- Remote Initial Sync ---
const remoteViewName = computed(() => isVariantTrace.value ? 'VisualizeVariant' : 'Visualize');

watch(remoteViewName, (newName) => {
    invoke('update_current_view', { newView: newName })
        .catch(err => console.error("[Remote] Failed to update current view:", err));
    updateRemoteViewState();
}, { immediate: true });

watch(selectedVariantId, () => {
    // Force immediate remote update when changing variant
    updateRemoteViewState();
});



watch(isFlytoActive, () => {
    // Safety net for any other flyto triggers
    updateRemoteViewState();
});

watch(variantBlueSegmentsRef, () => {
    // Ensure remote gets updated when segments data is loaded/calculated
    updateRemoteViewState();
}, { deep: true });

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
#map-visualization-container-inner { 
    transition: opacity 1s ease-in-out;
}
.initializing { opacity: 0; }
</style>
<style>
/* Global style strictly for popups to avoid scoped issues if any - or keep standard */
.map-message-popup .mapboxgl-popup-content { background: none; padding: 0; box-shadow: none; }
.map-message-popup .mapboxgl-popup-tip { display: none; }
.mapboxgl-ctrl-bottom-left, .mapboxgl-ctrl-bottom-right { display: none; }
</style>
