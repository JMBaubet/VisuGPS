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
              <span class="font-weight-bold">{{ distanceDisplay }}</span> <span class="font-weight-bold">&nbsp;/ {{ totalDistanceRef.toFixed(2) }} km</span>
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
          <altitude-s-v-g :circuit-id="props.circuitId" :current-distance="currentDistanceInMeters" />
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
                            <v-btn :icon="isAnimationFinished ? 'mdi-reload' : 'mdi-rewind'" variant="text" size="x-small"
                                   @mousedown="isAnimationFinished ? resetAnimation() : isRewinding = true"
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
                            </v-slider>          </div>
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
import AltitudeSVG from '@/components/AltitudeSVG.vue';
import CenterMarker from '@/components/CenterMarker.vue';

import WeatherWidgetDynamic from '@/components/WeatherWidgetDynamic.vue';
import WeatherWidgetStatic from '@/components/WeatherWidgetStatic.vue';
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
const controlPointIndicesRef = ref([]);
const pauseIncrements = ref([]);
const flytoEvents = ref({});
const rangeEvents = ref([]);
const currentDistanceInMeters = ref(0);

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
    const durationInMs = flytoData.duree > 100 ? flytoData.duree : flytoData.duree * 1000;
    const durationToTarget = Math.max(200, durationInMs / currentSpeed.value);

    await flyToPromise(map, {
        center: flytoData.coord,
        zoom: flytoData.zoom,
        pitch: flytoData.pitch,
        bearing: flytoData.cap,
        duration: durationToTarget,
    });

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
  if (isResuming.value) {
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
  if (isPaused.value) {
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

  const phase = Math.min(accumulatedTime / totalDurationAt1xRef.value, 1);
  const distanceTraveled = totalDistanceRef.value * phase;
  distanceDisplay.value = distanceTraveled.toFixed(2);
  currentDistanceInMeters.value = distanceTraveled * 1000;

  const cometLengthKm = cometLength.value / 1000;
  const startDistance = Math.max(0, distanceTraveled - cometLengthKm);
  if (distanceTraveled > startDistance) {
      const cometSlice = turf.lineSliceAlong(lineStringRef.value, startDistance, distanceTraveled, { units: 'kilometers' });
      map.getSource('comet-source').setData(cometSlice);
  }
  else {
      map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
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
          triggeredFlytoIncrement.value = currentIncrement;
          executeFlytoSequence(flytoData);
          animationFrameId = requestAnimationFrame(animate);
          return;
      }

      // Pause
      if (pauseIncrements.value.includes(currentIncrement)) {
          if (triggeredPauseIncrement.value !== currentIncrement) {
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
    if (trackingPointsWithDistanceRef.value[cpIndex].distance <= distanceTraveled) {
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
            if (lineStringRef.value) {
                try {
                     const p1 = turf.along(lineStringRef.value, distanceTraveled, {units: 'kilometers'});
                     // Look ahead slightly (e.g. 10 meters) to smooth out jitter
                     const p2 = turf.along(lineStringRef.value, distanceTraveled + 0.01, {units: 'kilometers'});
                     currentTraceBearing.value = turf.bearing(p1, p2);
                } catch (e) {
                    console.warn("Error calculating track bearing:", e);
                }
            }
        } else {
            // At the very last point, just set the camera to its values
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
      // Calculate current simulation time
      // accumulatedTime is in ms
      const currentSimTime = new Date(simulationStartDate.value.getTime() + accumulatedTime);
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



    // Start finalization sequence
    setTimeout(async () => {
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
    if (isPaused.value) {
        cameraMovedDuringPause.value = true;
        // Une fois détecté, on peut supprimer les écouteurs pour optimiser
        if (map) {
            map.off('move', onMapInteraction);
            map.off('zoom', onMapInteraction);
            map.off('pitch', onMapInteraction);
            map.off('rotate', onMapInteraction);
        }
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
        if (!isFlytoActive.value) {
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
        if (mapStyle.value !== map.getStyle().style) {
            map.setStyle(mapStyle.value);
            await new Promise(resolve => map.once('style.load', resolve));
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
    } else if (e.key === 'M') {
        showWeatherTable.value = !showWeatherTable.value;
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
    totalDurationAt1xRef.value = totalDistanceRef.value * animationSpeed.value;

    controlPointIndicesRef.value = trackingPointsWithDistanceRef.value.reduce((acc, p, index) => {
        if (p.pointDeControl) acc.push(index);
        return acc;
    }, []);

    if (!trackingDataRef.value[0]) {
        console.error("Initial tracking data point is undefined.");
        return;
    }

    // Initialize Weather
    await initWeather(currentCircuit, trackingPointsWithDistanceRef.value);

    // --- Trace Color Logic ---
    let finalTraceColor = toHex(traceColor.value);
    if (colorTraceBySlope.value) {
        try {
            const slopeColors = {
                TrancheNegative: toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative')),
                Tranche1: toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1')),
                Tranche2: toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2')),
                Tranche3: toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3')),
                Tranche4: toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4')),
                Tranche5: toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5')),
            };

            const colorExpression = await invoke('get_slope_color_expression', {
                circuitId: props.circuitId,
                slopeColors: slopeColors,
                segmentLength: segmentLength.value,
            });
            
            if (colorExpression && Array.isArray(colorExpression)) {
                finalTraceColor = colorExpression;
            } else {
                console.warn("Failed to generate slope color expression, falling back to single color.");
            }
        } catch (e) {
            console.error("Error getting slope color expression:", e);
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

    const setupMapLayersAndSources = async () => {
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

      const paintProps = {
        'line-width': traceWidth.value,
        'line-opacity': traceOpacity.value
      };

      if (colorTraceBySlope.value && Array.isArray(finalTraceColor)) {
        paintProps['line-gradient'] = finalTraceColor;
      } else {
        paintProps['line-color'] = finalTraceColor;
      }

      if (!map.getLayer('trace-background-layer')) {
        map.addLayer({
          id: 'trace-background-layer',
          type: 'line',
          source: 'trace',
          paint: paintProps
        });
      }

      if (!map.getSource('comet-source')) {
        map.addSource('comet-source', { type: 'geojson', data: { type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} } });
      }
      if (!map.getLayer('comet-layer')) {
        map.addLayer({ id: 'comet-layer', type: 'line', source: 'comet-source', paint: { 'line-width': cometWidth.value, 'line-color': cometColor.value, 'line-opacity': cometOpacity.value } });
      }
    };

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
