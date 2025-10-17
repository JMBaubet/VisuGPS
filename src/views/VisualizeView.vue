<template>
  <div id="map-container" ref="mapContainer"></div>

  <v-btn icon="mdi-arrow-left" class="back-button" @click="goBack" title="Retour à l'accueil"></v-btn>

  <div v-if="shouldShowCommuneWidget" class="commune-display" :style="{ borderColor: communeWidgetBorderColor }">
    <span class="font-weight-bold">{{ currentCommuneName }}</span>
  </div>

              <v-card variant="elevated" class="distance-display">
                      <div class="d-flex align-center justify-center fill-height px-4">
                        <span class="font-weight-bold">Distance :&nbsp;</span>
                        <span :class="['font-weight-bold', `text-${cometColor}`]">{{ distanceDisplay }}</span>
                      </div>  
              </v-card>  <div class="bottom-controls">
    <v-card variant="elevated" class="controls-card">
        <div class="d-flex align-center pa-1">
            <v-btn :icon="isAnimationFinished ? 'mdi-reload' : 'mdi-rewind'" variant="text" size="x-small"
                   @mousedown="isAnimationFinished ? resetAnimation() : isRewinding = true"
                   @mouseup="isRewinding = false" @mouseleave="isRewinding = false"></v-btn>
            <v-btn :icon="isPaused ? 'mdi-play' : 'mdi-pause'" variant="text" @click="isPaused = !isPaused"></v-btn>
            <v-divider vertical class="mx-2"></v-divider>
            <v-btn icon="mdi-minus" variant="text" @click="decreaseSpeed" size="x-small"></v-btn>
            <span class="speed-display-text">x{{ speedSteps[speedIndex] }}</span>
            <v-btn icon="mdi-plus" variant="text" @click="increaseSpeed" size="x-small"></v-btn>
        </div>
    </v-card>
  </div>

  <div v-if="showAltitudeProfile" class="altitude-svg-container">
      <altitude-s-v-g :circuit-id="props.circuitId" :current-distance="currentDistanceInMeters" />
  </div>


</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import mapboxgl from 'mapbox-gl';
import * as turf from '@turf/turf';
import { useTheme } from 'vuetify';
import { useSettings } from '@/composables/useSettings';
import { useSnackbar } from '@/composables/useSnackbar';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import AltitudeSVG from '@/components/AltitudeSVG.vue';

const props = defineProps({
  circuitId: {
    type: String,
    required: true,
  },
});

const router = useRouter();
const { settings, getSettingValue } = useSettings();
const { showSnackbar } = useSnackbar();
const { interruptUpdate } = useCommunesUpdate();
const { current: theme } = useTheme();

const mapContainer = ref(null);
let map = null;
let animationFrameId = null;
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

// --- Commune Widget State ---
const avancementCommunes = ref(0);
const currentCommuneName = ref('');
const shouldShowCommuneWidget = computed(() => avancementCommunes.value > 6);
const communeWidgetBorderColor = computed(() => theme.value.colors['red-darken-3'] || '#C62828');

const triggeredPauseIncrement = ref(null);
const triggeredFlytoIncrement = ref(null);
const isFlytoActive = ref(false);
const preFlytoCameraOptions = ref(null);

async function executeFlytoSequence(flytoData) {
    isFlytoActive.value = true;
    isPaused.value = true;

    preFlytoCameraOptions.value = {
        center: map.getCenter(),
        zoom: map.getZoom(),
        pitch: map.getPitch(),
        bearing: map.getBearing(),
    };

    showSnackbar('Début du survol programmé...', 'info');
    
    await new Promise(resolve => {
        map.once('moveend', resolve);
        map.flyTo({
            center: flytoData.coord,
            zoom: flytoData.zoom,
            pitch: flytoData.pitch,
            bearing: flytoData.cap,
            duration: flytoData.duree,
        });
    });

    await new Promise(resolve => {
        const unwatch = watch(() => isPaused.value, (newVal) => {
            if (newVal === false) {
                unwatch();
                resolve();
            }
        });
    });

    showSnackbar('Retour à la trace...', 'info');
    
    await new Promise(resolve => {
        map.once('moveend', resolve);
        map.flyTo({
            ...preFlytoCameraOptions.value,
            duration: flytoData.duree,
        });
    });

    isFlytoActive.value = false;
}

const animate = (timestamp) => {
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
      accumulatedTime = Math.max(0, accumulatedTime - (deltaTime * 2 * speedMultiplier.value));
  } else {
      accumulatedTime += deltaTime * speedMultiplier.value;
  }

  const phase = Math.min(accumulatedTime / totalDurationAt1xRef.value, 1);
  const distanceTraveled = totalDistanceRef.value * phase;
  distanceDisplay.value = `${distanceTraveled.toFixed(2)} km`;
  currentDistanceInMeters.value = distanceTraveled * 1000;

  const cometLengthKm = cometLength.value / 1000;
  const startDistance = Math.max(0, distanceTraveled - cometLengthKm);
  if (distanceTraveled > startDistance) {
      const cometSlice = turf.lineSliceAlong(lineStringRef.value, startDistance, distanceTraveled, { units: 'kilometers' });
      map.getSource('comet-source').setData(cometSlice);
  } else {
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

  // 2. Handle events using the accurate increment
  if (currentIncrement !== undefined) {
      // Popups
      if (map && rangeEvents.value.length > 0) {
          const newVisibleIds = new Set();
          for (const msg of rangeEvents.value) {
              if (currentIncrement >= msg.startIncrement && currentIncrement <= msg.endIncrement) {
                  newVisibleIds.add(msg.id);
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
                  const newMsg = rangeEvents.value.find(m => m.id === id);
                  if (newMsg) {
                      const popup = new mapboxgl.Popup({ closeButton: false, closeOnClick: false, anchor: 'bottom', className: 'map-message-popup' })
                          .setLngLat(newMsg.coord)
                          .setHTML(`<div style="background-color: ${newMsg.backgroundColor || 'white'}; color: black; font-weight: bold; font-size: 16px; border-color: ${newMsg.borderColor || 'black'}; border-width: ${newMsg.borderWidth != null ? newMsg.borderWidth + 'px' : '1px'}; border-radius: ${newMsg.borderRadius != null ? newMsg.borderRadius + 'px' : '4px'}; padding: 5px 10px; border-style: solid;">${newMsg.text}</div>`)
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
              showSnackbar('Pause programmée atteinte.', 'info');
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

    const zoom = lerp(prevZoom, nextZoom, progressInSegment);
    const pitch = lerp(prevPitch, nextPitch, progressInSegment);
    const bearing = lerpAngle(prevCap, nextCap, progressInSegment);
    
    map.setZoom(zoom);
    map.setPitch(pitch);
    map.setBearing(bearing);
    map.setCenter([lookAtPointLng, lookAtPointLat]);
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

            const zoom = lerp(prevZoom, nextZoom, progressInSegment);
            const pitch = lerp(prevPitch, nextPitch, progressInSegment);
            const bearing = lerpAngle(prevCap, nextCap, progressInSegment);
            
            map.setZoom(zoom);
            map.setPitch(pitch);
            map.setBearing(bearing);
            map.setCenter([lookAtPointLng, lookAtPointLat]);
        } else {
            // At the very last point, just set the camera to its values
            const zoom = currentPoint.editedZoom ?? currentPoint.zoom;
            const pitch = currentPoint.editedPitch ?? currentPoint.pitch;
            const bearing = currentPoint.editedCap ?? currentPoint.cap;
            const center = currentPoint.coordonnee;

            map.setZoom(zoom);
            map.setPitch(pitch);
            map.setBearing(bearing);
            map.setCenter(center);
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
  }
};

const isPaused = ref(true);
const isRewinding = ref(false);
const isAnimationFinished = ref(false);
const distanceDisplay = ref('0.00 km');

const speedSteps = [0.5, 1, 2, 3, 5, 7, 10];
const speedIndex = ref(1); // Default to 1x speed (index 1)
const speedMultiplier = computed(() => speedSteps[speedIndex.value]);

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

// --- Computed settings ---
const mapboxToken = computed(() => getSettingValue('Système/Tokens/mapbox'));
const mapStyle = computed(() => getSettingValue('Visualisation/Mapbox/styleVisualisation'));
const terrainExaggeration = computed(() => getSettingValue('Edition/Mapbox/Relief/exaggeration'));
const traceColor = computed(() => getSettingValue('Visualisation/Mapbox/Traces/couleurTrace'));
const traceWidth = computed(() => getSettingValue('Visualisation/Mapbox/Traces/epaisseurTrace'));
const traceOpacity = computed(() => getSettingValue('Visualisation/Mapbox/Traces/opaciteTrace'));
const cometColor = computed(() => getSettingValue('Visualisation/Mapbox/Traces/couleurComete'));
const cometLength = computed(() => getSettingValue('Visualisation/Mapbox/Traces/longueurComete'));
const animationSpeed = computed(() => getSettingValue('Visualisation/Animation/vitesse'));
const showAltitudeProfile = computed(() => {
    const value = getSettingValue('Altitude/Visualisation/Affichage');
    console.log('showAltitudeProfile computed value:', value);
    return value;
});

const goBack = () => {
  router.push({ name: 'Main' });
};

const resetAnimation = () => {
    accumulatedTime = 0;
    isPaused.value = true;
    isAnimationFinished.value = false;
    warningShown = false;
    triggeredPauseIncrement.value = null;
    triggeredFlytoIncrement.value = null;
    isFlytoActive.value = false;
    preFlytoCameraOptions.value = null;

    activePopups.forEach(popup => popup.remove());
    activePopups.clear();

    // Reset comet to start
    map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
    // Restart animation loop if it was stopped
    if (!animationFrameId) {
        animationFrameId = requestAnimationFrame(animate);
    }
};

const decreaseSpeed = () => {
    if (speedIndex.value > 0) {
        speedIndex.value--;
    }
};

const increaseSpeed = () => {
    if (speedIndex.value < speedSteps.length - 1) {
        speedIndex.value++;
    }
};

const handleKeyDown = (e) => {
    if (e.key === 'p' || e.key === 'P') {
        isPaused.value = !isPaused.value;
    } else if (e.key === 'ArrowLeft') {
        isRewinding.value = true;
    } else if (e.key === 'ArrowDown') {
        decreaseSpeed();
    } else if (e.key === 'ArrowUp') {
        increaseSpeed();
    }
};

const handleKeyUp = (e) => {
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
        rangeEvents.value = fetchedEvents.rangeEvents || [];
    }

    if (!fetchedLineString || !fetchedTrackingData || fetchedTrackingData.length < 2) {
      console.error("Failed to load valid circuit data.");
      return;
    }

    lineStringRef.value = fetchedLineString;
    trackingDataRef.value = fetchedTrackingData; // Garder pour l'initialisation de Mapbox center

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

    // Vérifier les données initiales pour Mapbox
    if (!trackingDataRef.value[0]) {
        console.error("Initial tracking data point is undefined.");
        return;
    }

    map = new mapboxgl.Map({
      container: mapContainer.value,
      style: mapStyle.value,
      center: trackingDataRef.value[0].coordonnee,
      zoom: trackingDataRef.value[0].zoom,
      pitch: trackingDataRef.value[0].pitch,
      bearing: trackingDataRef.value[0].cap,
      interactive: false,
    });

    map.on('style.load', () => {
      map.addSource('mapbox-dem', {
        'type': 'raster-dem',
        'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
        'tileSize': 512,
        'maxzoom': 14
      });
      map.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': terrainExaggeration.value });
      map.setFog({});
    });

    map.on('load', () => {
      map.addSource('trace', { type: 'geojson', data: lineStringRef.value });
      map.addLayer({
        id: 'trace-background-layer',
        type: 'line',
        source: 'trace',
        paint: { 'line-width': traceWidth.value, 'line-color': traceColor.value, 'line-opacity': traceOpacity.value }
      });

      map.addSource('comet-source', { type: 'geojson', data: { type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} } });
      map.addLayer({ id: 'comet-layer', type: 'line', source: 'comet-source', paint: { 'line-width': traceWidth.value, 'line-color': cometColor.value, 'line-opacity': traceOpacity.value } });

      animationFrameId = requestAnimationFrame(animate);
    });

  } catch (error) {
    console.error("Error during visualization setup:", error);
  }
};

onMounted(() => {
  interruptUpdate(); // Interrupt commune update task
  window.addEventListener('keydown', handleKeyDown);
  window.addEventListener('keyup', handleKeyUp);
  watch(settings, (newSettings) => {
    if (newSettings && mapContainer.value && !isMapInitialized) {
      isMapInitialized = true;
      initializeMap();
    }
  }, { immediate: true });
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
  window.removeEventListener('keyup', handleKeyUp);
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

.back-button {
  position: absolute;
  top: 20px;
  left: 20px;
  z-index: 1;
  pointer-events: auto;
}

.distance-display {
  position: absolute;
  top: -20px; /* User preference */
  left: 50%;
  transform: translateX(-50%);
  z-index: 1;
  pointer-events: auto;
  width: fit-content;
  height: 48px; /* Force height to match button */
}

.bottom-controls {
  position: absolute;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1;
  pointer-events: auto;
}

.altitude-svg-container {
    position: absolute;
    bottom: 80px;
    left: 50%;
    transform: translateX(-50%);
    justify-content: center; /* Center the child if it's smaller */
    z-index: 1;
    pointer-events: auto;
    background-color: rgba(0, 0, 0, 0.7);
    border-radius: 5px;
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
  right: 20px;
  width: 250px;
  height: 40px;
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
</style>