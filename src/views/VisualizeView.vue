<template>
  <div id="map-container" ref="mapContainer"></div>

  <v-btn icon="mdi-arrow-left" class="back-button" @click="goBack" title="Retour à l'accueil"></v-btn>

  <v-card variant="elevated" class="distance-display">
          <div class="d-flex align-center justify-center fill-height px-4">
            <span class="font-weight-bold font-monospace">Distance :&nbsp;</span>
            <span :class="['font-weight-bold', 'font-monospace', `text-${cometColor}`]">{{ distanceDisplay }}</span>
          </div>  </v-card>

  <div class="bottom-controls">
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
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import mapboxgl from 'mapbox-gl';
import * as turf from '@turf/turf';
import { useSettings } from '@/composables/useSettings';
import { useSnackbar } from '@/composables/useSnackbar';

const props = defineProps({
  circuitId: {
    type: String,
    required: true,
  },
});

const router = useRouter();
const { settings, getSettingValue } = useSettings();
const { showSnackbar } = useSnackbar();

const mapContainer = ref(null);
let map = null;
let animationFrameId = null;
let isMapInitialized = false;
let warningShown = false;
let accumulatedTime = 0;
let lastTimestamp = 0;

const lineStringRef = ref(null);
const trackingDataRef = ref(null);
const totalDistanceRef = ref(0);
const totalDurationAt1xRef = ref(0);
const trackingPointsWithDistanceRef = ref([]);
const controlPointIndicesRef = ref([]);

const animate = (timestamp) => {
  if (lastTimestamp === 0) lastTimestamp = timestamp;
  const deltaTime = timestamp - lastTimestamp;
  lastTimestamp = timestamp;

  if (isRewinding.value) {
      accumulatedTime = Math.max(0, accumulatedTime - (deltaTime * 2 * speedMultiplier.value));
  } else if (!isPaused.value) {
      accumulatedTime += deltaTime * speedMultiplier.value;
  }

  const phase = Math.min(accumulatedTime / totalDurationAt1xRef.value, 1);
  const distanceTraveled = totalDistanceRef.value * phase;
  distanceDisplay.value = `${distanceTraveled.toFixed(2)} km`;

  // Debugging logs for animate function
  // console.log("Animate - accumulatedTime:", accumulatedTime);
  // console.log("Animate - totalDurationAt1xRef:", totalDurationAt1xRef.value);
  // console.log("Animate - phase:", phase);
  // console.log("Animate - distanceTraveled:", distanceTraveled);
  // console.log("Animate - isPaused:", isPaused.value);
  // console.log("Animate - isRewinding:", isRewinding.value);

  const cometLengthKm = cometLength.value / 1000;
  const startDistance = Math.max(0, distanceTraveled - cometLengthKm);
  if (distanceTraveled > startDistance) {
      const cometSlice = turf.lineSliceAlong(lineStringRef.value, startDistance, distanceTraveled, { units: 'kilometers' });
      map.getSource('comet-source').setData(cometSlice);
  } else {
      map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
  }

  let prevKeyframe, nextKeyframe;

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
              prevKeyframe = controlPoint;
              nextKeyframe = trackingPointsWithDistanceRef.value[nextCpIndex];
          } 
      }
  }

  if (!prevKeyframe || !nextKeyframe) {
      const isLastControlPoint = controlPointIndicesRef.value.indexOf(lastPassedControlPointIndex) === controlPointIndicesRef.value.length - 1;

      if (lastPassedControlPointIndex !== -1 && !isLastControlPoint && !warningShown) {
          showSnackbar("Le tracking n'est pas complétement validé !", 'warning');
          warningShown = true;
      }
      let currentPointIndex = trackingPointsWithDistanceRef.value.findIndex((p, i) => {
          const nextPoint = trackingPointsWithDistanceRef.value[i + 1];
          return nextPoint && distanceTraveled >= p.distance && distanceTraveled < nextPoint.distance;
      });
      if (currentPointIndex === -1) currentPointIndex = trackingPointsWithDistanceRef.value.length - 2;
      if (currentPointIndex < 0) currentPointIndex = 0;
      prevKeyframe = trackingPointsWithDistanceRef.value[currentPointIndex];
      nextKeyframe = trackingPointsWithDistanceRef.value[currentPointIndex + 1];
  }

  const prevKeyframeDist = prevKeyframe.distance;
  const nextKeyframeDist = nextKeyframe.distance;
  const segmentDist = nextKeyframeDist - prevKeyframeDist;
  const progressInSegment = segmentDist > 0 ? (distanceTraveled - prevKeyframeDist) / segmentDist : 0;

  const prevZoom = prevKeyframe.editedZoom ?? prevKeyframe.zoom;
  const nextZoom = nextKeyframe.editedZoom ?? nextKeyframe.zoom;

  const prevPitch = prevKeyframe.editedPitch ?? prevKeyframe.pitch;
  const nextPitch = nextKeyframe.editedPitch ?? nextKeyframe.pitch;

  const prevCap = prevKeyframe.editedCap ?? prevKeyframe.cap;
  const nextCap = nextKeyframe.editedCap ?? nextKeyframe.cap;

  const zoom = lerp(prevZoom, nextZoom, progressInSegment);
  const pitch = lerp(prevPitch, nextPitch, progressInSegment);
  const bearing = lerpAngle(prevCap, nextCap, progressInSegment);
  
  const lookAtPointLng = lerp(prevKeyframe.coordonnee[0], nextKeyframe.coordonnee[0], progressInSegment);
  const lookAtPointLat = lerp(prevKeyframe.coordonnee[1], nextKeyframe.coordonnee[1], progressInSegment);

  // console.log("Animate - Zoom:", zoom, "Pitch:", pitch, "Bearing:", bearing, "Center:", [lookAtPointLng, lookAtPointLat]);

  map.setZoom(zoom);
  map.setPitch(pitch);
  map.setBearing(bearing);
  map.setCenter([lookAtPointLng, lookAtPointLat]);

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

const goBack = () => {
  router.push({ name: 'Main' });
};

const resetAnimation = () => {
    accumulatedTime = 0;
    isPaused.value = true;
    isAnimationFinished.value = false;
    warningShown = false;
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
    const [fetchedLineString, fetchedTrackingData] = await Promise.all([
      invoke('read_line_string_file', { circuitId: props.circuitId }),
      invoke('read_tracking_file', { circuitId: props.circuitId })
    ]);

    if (!fetchedLineString || !fetchedTrackingData || fetchedTrackingData.length < 2) {
      console.error("Failed to load valid circuit data.");
      return;
    }

    console.log("Fetched LineString:", fetchedLineString);
    console.log("Fetched TrackingData (raw):", fetchedTrackingData);

    lineStringRef.value = fetchedLineString;
    trackingDataRef.value = fetchedTrackingData; // Garder pour l'initialisation de Mapbox center

    const processedData = await invoke('process_tracking_data', {
        lineStringGeojson: fetchedLineString,
        trackingPointsJs: fetchedTrackingData
    });

    console.log("Processed Data from Rust:", processedData);

    trackingPointsWithDistanceRef.value = processedData.processedPoints;
    totalDistanceRef.value = processedData.totalDistanceKm;
    totalDurationAt1xRef.value = totalDistanceRef.value * animationSpeed.value;

    console.log("Tracking Points with Distance:", trackingPointsWithDistanceRef.value);
    console.log("Total Distance (km):", totalDistanceRef.value);
    console.log("Total Duration at 1x (ms):", totalDurationAt1xRef.value);

    controlPointIndicesRef.value = trackingPointsWithDistanceRef.value.reduce((acc, p, index) => {
        if (p.pointDeControl) acc.push(index);
        return acc;
    }, []);

    console.log("Control Point Indices:", controlPointIndicesRef.value);

    // Vérifier les données initiales pour Mapbox
    if (!trackingDataRef.value[0]) {
        console.error("Initial tracking data point is undefined.");
        return;
    }
    console.log("Initial Mapbox Center:", trackingDataRef.value[0].coordonnee);
    console.log("Initial Mapbox Zoom:", trackingDataRef.value[0].zoom);
    console.log("Initial Mapbox Pitch:", trackingDataRef.value[0].pitch);
    console.log("Initial Mapbox Bearing:", trackingDataRef.value[0].cap);

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
        paint: { 'line-width': traceWidth.value, 'line-color': traceColor.value, 'line-opacity': traceOpacity.value * 0.4 }
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

/* Hide mapbox logo/attribution for cleaner view, but ensure it's compliant with Mapbox terms */
.mapboxgl-ctrl-bottom-left, .mapboxgl-ctrl-bottom-right {
  display: none;
}
</style>