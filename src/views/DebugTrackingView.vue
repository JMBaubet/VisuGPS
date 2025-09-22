<template>
  <v-container fluid class="fill-height">
    <v-btn
      icon="mdi-close"
      color="red"
      variant="text"
      @click="goHome"
      style="position: absolute; top: 16px; right: 16px; z-index: 10;"
    ></v-btn>
    <v-row class="fill-height">
      <v-col cols="9" class="fill-height pa-0">
        <div id="map" class="fill-height"></div>
      </v-col>
      <v-col cols="3">
        <v-card>
          <v-card-title>Contrôles de débogage</v-card-title>
          <v-card-text>
            <p>Circuit ID: {{ circuitId }}</p>
            <v-switch v-model="showTrace" label="Afficher la trace GPX"></v-switch>
            <v-switch v-model="showCaps" label="Afficher les caps"></v-switch>
            <v-divider class="my-4"></v-divider>
            <p>Point: {{ currentIndex + 1 }} / {{ trackingPoints.length }}</p>
            <div class="caption mt-4">
              <p class="font-weight-bold mb-1">Navigation:</p>
              <ul>
                <li><b>m</b> : Point suivant</li>
                <li><b>l</b> : Point précédent</li>
                <li><b>Shift</b> + touche : Sauter par 10</li>
                <li><b>Ctrl</b> + touche : Sauter par 100</li>
              </ul>
            </div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';
import * as turf from '@turf/turf';
import { useSettings } from '@/composables/useSettings';
import { useEnvironment } from '@/composables/useEnvironment';

const route = useRoute();
const router = useRouter();
const { getSettingValue } = useSettings();
const { mapboxToken } = useEnvironment();

const goHome = () => {
  router.push('/');
};

const circuitId = ref(route.params.circuitId);
const lineString = ref(null);
const trackingPoints = ref([]);
const currentIndex = ref(0);
const lissageCap = ref(15);

const showTrace = ref(true);
const showCaps = ref(true);

let map = null;

onMounted(async () => {
  await loadNonMapData();
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  if (map) {
    map.remove();
  }
  window.removeEventListener('keydown', handleKeyDown);
});

watch(mapboxToken, (newToken) => {
  if (newToken && !map && lineString.value && trackingPoints.value.length > 0) {
    mapboxgl.accessToken = newToken;
    initializeMap();
  }
}, { immediate: true });

async function loadNonMapData() {
  const lissageCapValue = await getSettingValue('data.groupes.Importation.groupes.Tracking.parametres.LissageCap');
  lissageCap.value = lissageCapValue || 15;

  try {
    const data = await invoke('get_debug_data', { circuitId: circuitId.value });
    lineString.value = data.line_string;
    trackingPoints.value = data.tracking_points;

    if (mapboxToken.value && !map) {
      mapboxgl.accessToken = mapboxToken.value;
      initializeMap();
    }
  } catch (error) {
    console.error('Failed to load debug data:', error);
  }
}

function initializeMap() {
  if (!lineString.value || !trackingPoints.value.length) return;

  const startPoint = trackingPoints.value[0].coordonnee;

  map = new mapboxgl.Map({
    container: 'map',
    style: 'mapbox://styles/mapbox/streets-v11',
    center: startPoint,
    zoom: 14,
    pitch: 0,
  });

  map.on('load', () => {
    setupLayers();
  });
}

function setupLayers() {
  // Initial data for point 0
  const initialPointData = trackingPoints.value[0];
  const initialCoords = initialPointData.coordonnee;
  const initialBearing = initialPointData.cap;
  const initialEndRange = Math.min(lissageCap.value, trackingPoints.value.length);
  const initialCalcCoords = trackingPoints.value.slice(0, initialEndRange).map(p => p.coordonnee);
  const initialVectorStart = turf.point(initialCoords);
  const initialVectorEnd = turf.destination(initialVectorStart, 1.4, initialBearing, { units: 'kilometers' });

  const initialNormalCalcCoords = initialCalcCoords.slice(0, -1);
  const initialLastCalcCoord = initialCalcCoords.length > 0 ? initialCalcCoords[initialCalcCoords.length - 1] : null;


  // Source and Layer for the main GPX trace
  map.addSource('gpx-trace', { type: 'geojson', data: lineString.value });
  map.addLayer({
    id: 'gpx-trace-layer',
    type: 'line',
    source: 'gpx-trace',
    layout: { 'line-join': 'round', 'line-cap': 'round' },
    paint: { 'line-color': '#0000FF', 'line-width': 4, 'line-opacity': 0.5 },
  });

  // Source and Layer for the bearing vector (added first to be underneath points)
  map.addSource('bearing-vector', { type: 'geojson', data: turf.lineString([initialVectorStart.geometry.coordinates, initialVectorEnd.geometry.coordinates]) });
  map.addLayer({
    id: 'bearing-vector-layer',
    type: 'line',
    source: 'bearing-vector',
    paint: { 'line-color': '#FF0000', 'line-width': 5 },
  });

  // Source and Layer for calculation (yellow) points
  map.addSource('calc-points', { type: 'geojson', data: turf.multiPoint(initialNormalCalcCoords) });
  map.addLayer({
    id: 'calc-points-layer',
    type: 'circle',
    source: 'calc-points',
    paint: { 'circle-radius': 3, 'circle-color': '#FFFF00' },
  });

  // Source and Layer for the LAST calculation point (larger)
  map.addSource('last-calc-point', { type: 'geojson', data: initialLastCalcCoord ? turf.point(initialLastCalcCoord) : turf.point([]) });
  map.addLayer({
    id: 'last-calc-point-layer',
    type: 'circle',
    source: 'last-calc-point',
    paint: { 'circle-radius': 6, 'circle-color': '#FFFF00' },
  });

  // Source and Layer for the current (red) point (added last to be on top)
  map.addSource('current-point', { type: 'geojson', data: turf.point(initialCoords) });
  map.addLayer({
    id: 'current-point-layer',
    type: 'circle',
    source: 'current-point',
    paint: { 'circle-radius': 5, 'circle-color': '#FF0000', 'circle-stroke-width': 1, 'circle-stroke-color': '#FFFFFF' },
  });
}

function updateMapFeatures() {
  if (!map || !trackingPoints.value.length) return;

  const currentPointData = trackingPoints.value[currentIndex.value];
  const currentCoords = currentPointData.coordonnee;

  // Update current point
  map.getSource('current-point').setData(turf.point(currentCoords));

  // Update calculation points
  const endRange = Math.min(currentIndex.value + lissageCap.value, trackingPoints.value.length);
  const calcPointsCoords = trackingPoints.value.slice(currentIndex.value, endRange).map(p => p.coordonnee);
  
  let normalCalcPoints = [];
  let lastCalcPoint = null;
  if (calcPointsCoords.length > 0) {
    lastCalcPoint = calcPointsCoords[calcPointsCoords.length - 1];
    normalCalcPoints = calcPointsCoords.slice(0, -1);
  }

  map.getSource('calc-points').setData(turf.multiPoint(normalCalcPoints));
  map.getSource('last-calc-point').setData(lastCalcPoint ? turf.point(lastCalcPoint) : turf.point([]));


  // Update bearing vector
  const bearing = currentPointData.cap;
  const vectorStart = turf.point(currentCoords);
  const vectorEnd = turf.destination(vectorStart, 1.4, bearing, { units: 'kilometers' }); // 1400m
  map.getSource('bearing-vector').setData(turf.lineString([vectorStart.geometry.coordinates, vectorEnd.geometry.coordinates]));

  // Center map
  map.flyTo({
    center: currentCoords,
    speed: 0.7
  });
}

watch(currentIndex, updateMapFeatures);

watch(showTrace, (visible) => {
  map.setLayoutProperty('gpx-trace-layer', 'visibility', visible ? 'visible' : 'none');
});

watch(showCaps, (visible) => {
    const visibility = visible ? 'visible' : 'none';
    map.setLayoutProperty('current-point-layer', 'visibility', visibility);
    map.setLayoutProperty('calc-points-layer', 'visibility', visibility);
    map.setLayoutProperty('last-calc-point-layer', 'visibility', visibility);
    map.setLayoutProperty('bearing-vector-layer', 'visibility', visibility);
});


function handleKeyDown(event) {
  let step = 1;
  if (event.ctrlKey) {
    step = 100;
  } else if (event.shiftKey) {
    step = 10;
  }

  if (event.key === 'm' || event.key === 'M') { // Next
    event.preventDefault();
    currentIndex.value = Math.min(currentIndex.value + step, trackingPoints.value.length - 1);
  } else if (event.key === 'l' || event.key === 'L') { // Previous
    event.preventDefault();
    currentIndex.value = Math.max(currentIndex.value - step, 0);
  }
}
</script>

<style>
#map {
  width: 100%;
  height: 100%;
}
.mapboxgl-canvas {
    outline: none;
}
</style>
