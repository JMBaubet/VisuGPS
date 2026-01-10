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
            <p class="font-weight-bold">Layers de trace:</p>
            <v-radio-group v-model="selectedTraceLayer" density="compact">
              <v-radio label="Overlay Aller" value="aller"></v-radio>
              <v-radio label="Overlay Retour" value="retour"></v-radio>
              <v-radio label="Tous (Aller + Retour)" value="all"></v-radio>
            </v-radio-group>

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
const selectedTraceLayer = ref('all');
const layerGradients = ref({
    complete: null,
    aller: null,
    retour: null
});
const coloredSegmentsGeoJson = ref(null);

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


function toHex(value) {
  if (!value) return '#FF0000';
  if (typeof value === 'string') {
    if (value.startsWith('#') || value.startsWith('rgb')) return value;
    // Map basic color names commonly used
    const colors = {
        'red': '#FF0000',
        'blue': '#0000FF',
        'green': '#008000',
        'yellow': '#FFFF00',
        'white': '#FFFFFF',
        'black': '#000000',
        'gray': '#808080',
        'light-blue': '#ADD8E6',
        'orange': '#FFA500'
    };
    return colors[value] || value; // fallback to value if not found (might fail if unknown name)
  }
  return '#FF0000';
}

async function loadNonMapData() {
  const lissageCapValue = await getSettingValue('Importation/Tracking/LissageCap');
  lissageCap.value = lissageCapValue || 15;

  try {
    const data = await invoke('get_debug_data', { circuitId: circuitId.value });
    lineString.value = data.line_string;
    trackingPoints.value = data.tracking_points;

    // Charger les gradients comme dans VisualizeView
    const segmentLength = await getSettingValue('Importation/Tracking/LongueurSegment') || 20;
    try {
        const slopeColors = {
            TrancheNegative: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative')),
            Tranche1: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1')),
            Tranche2: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2')),
            Tranche3: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3')),
            Tranche4: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4')),
            Tranche5: toHex(await getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5')),
        };

        const geoJson = await invoke('get_colored_segments_geojson', {
            circuitId: circuitId.value,
            slopeColors: slopeColors,
            segmentLength: segmentLength,
        });

        if (geoJson) {
            coloredSegmentsGeoJson.value = geoJson;
            console.log('[Debug] Colored Segments GeoJSON loaded');
        }
    } catch (e) {
        console.error("Error getting colored segments:", e);
    }

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
  const initialPointData = trackingPoints.value[0];
  const initialCoords = initialPointData.coordonnee;
  const initialBearing = initialPointData.cap;
  const initialEndRange = Math.min(lissageCap.value, trackingPoints.value.length);
  const initialCalcCoords = trackingPoints.value.slice(0, initialEndRange).map(p => p.coordonnee);
  const initialVectorStart = turf.point(initialCoords);
  const initialVectorEnd = turf.destination(initialVectorStart, 1.4, initialBearing, { units: 'kilometers' });

  const initialNormalCalcCoords = initialCalcCoords.slice(0, -1);
  const initialLastCalcCoord = initialCalcCoords.length > 0 ? initialCalcCoords[initialCalcCoords.length - 1] : null;


  // Source for the colored segments (Detailed FeatureCollection)
  if (coloredSegmentsGeoJson.value && !map.getSource('colored-segments')) {
    map.addSource('colored-segments', { type: 'geojson', data: coloredSegmentsGeoJson.value });

        // Layer 1: Trace complète (SUPPRIME - Simplification Phase 8)
        
        // Layer 2: Overlay Aller (Tout SAUF Retour)
        // Filter: type != 'retour_overlap'
        if (!map.getLayer('gpx-trace-aller')) {
            map.addLayer({
                id: 'gpx-trace-aller',
                type: 'line',
                source: 'colored-segments',
                layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'visible' }, // Visible par défaut pour "all"
                paint: {
                    'line-width': 4,
                    'line-opacity': 1,
                    'line-color': ['get', 'color_raw']
                },
                filter: ['!=', ['get', 'segment_type'], 'retour_overlap']
            });
        }

    // Layer 3: Overlay Retour (Tout SAUF Aller)
    // Filter: type != 'aller_overlap'
    if (!map.getLayer('gpx-trace-retour')) {
        map.addLayer({
            id: 'gpx-trace-retour',
            type: 'line',
            source: 'colored-segments',
            layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' }, // Caché par défaut
            paint: {
                'line-width': 4,
                'line-opacity': 1,
                'line-color': ['get', 'color_raw']
            },
            filter: ['!=', ['get', 'segment_type'], 'aller_overlap']
        });
    }
  }

  // Backup: Source for main lineString if colored segments fail (or unused but safe to keep)
  if (!map.getSource('gpx-trace') && lineString.value) {
     map.addSource('gpx-trace', { type: 'geojson', data: lineString.value });
  }

  updateLayerVisibility();

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

function updateLayerVisibility() {
    if (!map) return;
    
    // 'complete' n'existe plus. 'all' affiche Aller + Retour.
    const showAller = selectedTraceLayer.value === 'all' || selectedTraceLayer.value === 'aller';
    const showRetour = selectedTraceLayer.value === 'all' || selectedTraceLayer.value === 'retour';
    
    // Global toggle
    const globalVisible = showTrace.value;

    if (map.getLayer('gpx-trace-aller')) {
        map.setLayoutProperty('gpx-trace-aller', 'visibility', globalVisible && showAller ? 'visible' : 'none');
    }
    if (map.getLayer('gpx-trace-retour')) {
        map.setLayoutProperty('gpx-trace-retour', 'visibility', globalVisible && showRetour ? 'visible' : 'none');
    }
}

watch(currentIndex, updateMapFeatures);

watch(showTrace, updateLayerVisibility);
watch(selectedTraceLayer, updateLayerVisibility);

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
