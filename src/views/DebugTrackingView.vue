<template>
  <v-container fluid class="fill-height">
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
             <p class="caption">Utilisez les flèches (←, →) pour naviguer.</p>
            <p class="caption">Maintenez Ctrl pour sauter par 10.</p>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue';
import { useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import mapboxgl from 'mapbox-gl';
import * as turf from '@turf/turf';
import { useSettings } from '@/composables/useSettings';

const route = useRoute();
const { getSettingValue } = useSettings();

const circuitId = ref(route.params.circuitId);
const lineString = ref(null);
const trackingPoints = ref([]);
const currentIndex = ref(0);
const lissageCap = ref(15);

const showTrace = ref(true);
const showCaps = ref(true);

let map = null;

const MAPBOX_TOKEN = ref('');

onMounted(async () => {
  await loadSettingsAndData();
  if (MAPBOX_TOKEN.value) {
    mapboxgl.accessToken = MAPBOX_TOKEN.value;
    initializeMap();
    window.addEventListener('keydown', handleKeyDown);
  } else {
    console.error('Mapbox token not available.');
  }
});

onUnmounted(() => {
  if (map) {
    map.remove();
  }
  window.removeEventListener('keydown', handleKeyDown);
});

async function loadSettingsAndData() {
  const tokenValue = await getSettingValue('data.groupes.Système.groupes.Tokens.parametres.mapbox');
  MAPBOX_TOKEN.value = tokenValue;

  const lissageCapValue = await getSettingValue('data.groupes.Importation.groupes.Tracking.parametres.LissageCap');
  lissageCap.value = lissageCapValue || 15;

  try {
    const data = await invoke('get_debug_data', { circuitId: circuitId.value });
    lineString.value = data.line_string;
    trackingPoints.value = data.tracking_points;
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
    pitch: 45,
  });

  map.on('load', () => {
    setupLayers();
    updateMapFeatures();
  });
}

function setupLayers() {
  // Source and Layer for the main GPX trace
  map.addSource('gpx-trace', {
    type: 'geojson',
    data: lineString.value,
  });
  map.addLayer({
    id: 'gpx-trace-layer',
    type: 'line',
    source: 'gpx-trace',
    layout: {
      'line-join': 'round',
      'line-cap': 'round',
    },
    paint: {
      'line-color': '#0000FF',
      'line-width': 4,
      'line-opacity': 0.5,
    },
  });

  // Source and Layer for the current (red) point
  map.addSource('current-point', { type: 'geojson', data: turf.point([0, 0]) });
  map.addLayer({
    id: 'current-point-layer',
    type: 'circle',
    source: 'current-point',
    paint: {
      'circle-radius': 5,
      'circle-color': '#FF0000',
      'circle-stroke-width': 1,
      'circle-stroke-color': '#FFFFFF',
    },
  });

  // Source and Layer for calculation (yellow) points
  map.addSource('calc-points', { type: 'geojson', data: turf.multiPoint([]) });
  map.addLayer({
    id: 'calc-points-layer',
    type: 'circle',
    source: 'calc-points',
    paint: {
      'circle-radius': 3,
      'circle-color': '#FFFF00',
    },
  });

  // Source and Layer for the bearing vector
  map.addSource('bearing-vector', { type: 'geojson', data: turf.lineString([]) });
  map.addLayer({
    id: 'bearing-vector-layer',
    type: 'line',
    source: 'bearing-vector',
    paint: {
      'line-color': '#FFFF00',
      'line-width': 3,
    },
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
  map.getSource('calc-points').setData(turf.multiPoint(calcPointsCoords));

  // Update bearing vector
  const bearing = currentPointData.cap;
  const vectorStart = turf.point(currentCoords);
  const vectorEnd = turf.destination(vectorStart, 0.1, bearing, { units: 'kilometers' }); // 100m
  map.getSource('bearing-vector').setData(turf.lineString([vectorStart.geometry.coordinates, vectorEnd.geometry.coordinates]));

  // Center map
  map.panTo(currentCoords);
}

watch(currentIndex, updateMapFeatures);

watch(showTrace, (visible) => {
  map.setLayoutProperty('gpx-trace-layer', 'visibility', visible ? 'visible' : 'none');
});

watch(showCaps, (visible) => {
    const visibility = visible ? 'visible' : 'none';
    map.setLayoutProperty('current-point-layer', 'visibility', visibility);
    map.setLayoutProperty('calc-points-layer', 'visibility', visibility);
    map.setLayoutProperty('bearing-vector-layer', 'visibility', visibility);
});


function handleKeyDown(event) {
  const step = event.ctrlKey ? 10 : 1;
  if (event.key === 'ArrowRight') {
    currentIndex.value = Math.min(currentIndex.value + step, trackingPoints.value.length - 1);
  } else if (event.key === 'ArrowLeft') {
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
