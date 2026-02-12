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
      <v-col cols="9" class="fill-height pa-0" style="position: relative;">
        <div id="map" class="fill-height"></div>
        
        <!-- Cross in the middle of the map -->
        <div class="map-center-cross"></div>
        
        <!-- Widget de données sur la carte -->
        <v-card v-if="currentPointData && showPointData" class="debug-overlay-card elevation-4" border>
          <v-card-title class="text-subtitle-2 bg-primary text-white py-1 d-flex justify-space-between">
            <span>{{ currentTraceName }}</span>
            <span class="text-caption">Index: {{ currentIndex }}</span>
          </v-card-title>
          <v-card-text class="pa-1">
            <div v-for="field in orderedPointData" :key="field.key" class="d-flex debug-data-row">
              <span class="font-weight-bold mr-1">{{ field.key }}:</span>
              <span :style="getFieldStyle(field.key, field.value, currentPointData)" class="text-truncate">
                {{ formatValue(field.value, field.key) }}
              </span>
            </div>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="3">
        <v-card>
          <v-card-title>Contrôles de débogage</v-card-title>
          <v-card-text>
            <p>Circuit ID: {{ circuitId }}</p>
            <v-switch v-model="showTrace" label="Afficher la trace GPX" density="compact" hide-details></v-switch>
            <v-switch v-model="showCaps" label="Afficher les caps" density="compact" hide-details></v-switch>
            <v-switch v-model="showPointData" label="Afficher données point" density="compact" hide-details></v-switch>
            
            <v-divider class="my-4"></v-divider>
            <p class="font-weight-bold">Élément à déboguer:</p>
            <v-select
              v-model="selectedVariantId"
              :items="variantOptions"
              label="Sélectionner une trace"
              density="compact"
              variant="outlined"
              class="mt-2"
            ></v-select>
            
            <v-divider class="my-4"></v-divider>
            <p class="font-weight-bold">Layers de trace:</p>
            <v-radio-group v-model="selectedTraceLayer" density="compact">
              <v-radio label="Overlay Aller" value="aller"></v-radio>
              <v-radio label="Overlay Retour" value="retour"></v-radio>
              <v-radio label="Tous (Pentes)" value="all"></v-radio>
              <v-radio label="Statut Variante" value="status"></v-radio>
            </v-radio-group>
            
            <v-divider class="my-4"></v-divider>
            <p class="font-weight-bold">Zones de chevauchement:</p>
            <v-select
              v-model="selectedZoneId"
              :items="zoneOptions"
              label="Visualiser une zone"
              density="compact"
              variant="outlined"
              class="mt-2"
              hide-details
            ></v-select>
            <div v-if="selectedZoneId" class="caption mt-2">
              <v-chip size="x-small" color="#D2B48C" variant="flat" class="mr-1">Start Aller</v-chip>
              <v-chip size="x-small" color="#5D4037" variant="flat" class="mr-1">Stop Aller</v-chip>
              <v-chip size="x-small" color="#E1BEE7" variant="flat" class="mr-1">Start Retour</v-chip>
              <v-chip size="x-small" color="#7B1FA2" variant="flat" class="mr-1">Stop Retour</v-chip>
            </div>

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

<style scoped>
.debug-data-row {
  border-bottom: 1px solid #eee;
  font-size: 0.75rem;
  line-height: 1.2;
  padding: 2px 0;
}
.debug-data-row:last-child {
  border-bottom: none;
}
.debug-overlay-card {
  position: absolute;
  top: 10px;
  left: 10px;
  z-index: 5;
  width: 280px;
  max-height: 80vh;
  overflow-y: auto;
  opacity: 0.8;
  pointer-events: auto;
}

.map-center-cross {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 20px;
  height: 20px;
  transform: translate(-50%, -50%);
  z-index: 10;
  pointer-events: none;
}
.map-center-cross::before,
.map-center-cross::after {
  content: '';
  position: absolute;
  background-color: black;
}
.map-center-cross::before {
  top: 50%;
  left: 0;
  width: 100%;
  height: 2px;
  transform: translateY(-50%);
}
.map-center-cross::after {
  top: 0;
  left: 50%;
  width: 2px;
  height: 100%;
  transform: translateX(-50%);
}
</style>

<script setup>
import { ref, onMounted, onUnmounted, watch, computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';
import * as turf from '@turf/turf';
import { useSettings } from '@/composables/useSettings';
import { useEnvironment } from '@/composables/useEnvironment';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import { buildSlopeColorsMap } from '@/composables/useSlopeColors';

const route = useRoute();
const router = useRouter();
const { getSettingValue } = useSettings();
const { mapboxToken } = useEnvironment();
const { toHex } = useVuetifyColors();

const goHome = () => {
  router.push('/');
};

const circuitId = ref(route.params.circuitId);
const lineString = ref(null);
const trackingPoints = ref([]);
const currentIndex = ref(0);
const lissageCap = ref(15);
const showPointData = ref(true);

const showTrace = ref(true);
const showCaps = ref(true);
const selectedTraceLayer = ref('all');
const layerGradients = ref({
    complete: null,
    aller: null,
    retour: null
});
const coloredSegmentsGeoJson = ref(null);
const variants = ref([]);
const selectedVariantId = ref(null); // null = Trace Principale

const variantOptions = computed(() => {
    const options = [{ title: 'Trace Principale', value: null }];
    variants.value.forEach(v => {
        options.push({ title: `Variant: ${v.name || v.id}`, value: v.id });
    });
    return options;
});
const currentPointData = computed(() => {
    if (trackingPoints.value.length > 0 && currentIndex.value < trackingPoints.value.length) {
        return trackingPoints.value[currentIndex.value];
    }
    return null;
});

function formatValue(val, key) {
    if (key === 'isAnchorPoint') {
        return val === true ? 'TRUE' : '';
    }
    if (val === null || val === undefined) return 'null';
    if (Array.isArray(val)) {
        return `[${val.map(v => typeof v === 'number' ? v.toFixed(6) : v).join(', ')}]`;
    }
    if (typeof val === 'number') {
        return Number.isInteger(val) ? val.toString() : val.toFixed(4);
    }
    return val.toString();
}

function getFieldStyle(key, value, data) {
    if (key === 'pointDeControl') {
        return { color: value ? '#4CAF50' : '#9E9E9E', fontWeight: value ? 'bold' : 'normal' };
    }
    if (key === 'nbrSegment') {
        return { color: data.pointDeControl ? '#4CAF50' : '#9E9E9E', fontWeight: data.pointDeControl ? 'bold' : 'normal' };
    }
    if (key === 'altitude' && value === 0) {
        return { color: '#F44336', fontWeight: 'bold' };
    }
    if (key === 'isAnchorPoint' && value === true) {
        return { color: '#4CAF50', fontWeight: 'bold' };
    }
    if (key === 'typeTroncon') {
        if (value === 'Commun') return { color: '#ECEFF1', textShadow: '0.5px 0.5px 1px #000' }; // Blanc nacré
        if (value === 'Segment') return { color: '#2196F3', fontWeight: 'bold' };
        if (value === 'Départ') return { color: '#4CAF50', fontWeight: 'bold' };
        if (value === 'Arrivée') return { color: '#F44336', fontWeight: 'bold' };
    }
    if (key === 'editedZoom' && value !== null && value !== undefined && value !== 16) {
        return { color: '#F44336', fontWeight: 'bold' };
    }
    if (key === 'editedPitch' && value !== null && value !== undefined && value !== 60) {
        return { color: '#F44336', fontWeight: 'bold' };
    }
    return {};
}

const currentTraceName = computed(() => {
    if (!selectedVariantId.value) return 'Trace Principale';
    const variant = variants.value.find(v => v.id === selectedVariantId.value);
    return variant ? (variant.name || variant.id) : 'Variant';
});

const orderedPointData = computed(() => {
    if (!currentPointData.value) return [];
    
    const data = currentPointData.value;
    const order = [
        'increment',
        'pointDeControl',
        'nbrSegment',
        'altitude',
        'isAnchorPoint',
        'typeTroncon',
        'editedCap',
        'editedZoom',
        'editedPitch',
        'commune',
        'altitudeCamera',
        'cap',
        'zoom',
        'pitch',
        'isRegularSegment',
        'actualSegmentLength',
        'coordonnee',
        'coordonneeCamera'
    ];
    
    const result = [];
    order.forEach(key => {
        // Pour isAnchorPoint, on l'affiche même s'il n'est pas dans les données (valeur nulle)
        if (key in data || key === 'isAnchorPoint') {
            result.push({ key, value: data[key] });
        }
    });

    // Ajouter les éventuels champs restants non listés dans l'ordre
    Object.keys(data).forEach(key => {
        if (!order.includes(key)) {
            result.push({ key, value: data[key] });
        }
    });
    
    return result;
});

const segmentMetadata = ref(null);
const selectedZoneId = ref(null);
const zoneOptions = computed(() => {
    if (!segmentMetadata.value || !segmentMetadata.value.overlappingZones || segmentMetadata.value.overlappingZones.length === 0) {
        return [{ title: 'Aucune zone détectée', value: null }];
    }
    const options = [{ title: 'Masquer les zones', value: null }];
    segmentMetadata.value.overlappingZones.forEach(z => {
        options.push({ title: `Zone ${z.zoneId}`, value: z.zoneId });
    });
    return options;
});

let map = null;

onMounted(async () => {
  await fetchVariants();
  await loadNonMapData();
  window.addEventListener('keydown', handleKeyDown);
});

async function fetchVariants() {
    try {
        const fetched = await invoke('get_variants', { circuitId: circuitId.value });
        variants.value = fetched || [];
    } catch (e) {
        console.error("Error fetching variants:", e);
    }
}

watch(selectedVariantId, async () => {
    selectedZoneId.value = null; // Reset zone selection when changing trace
    await loadNonMapData();
    if (map) {
        // Force refresh of sources
        if (map.getSource('colored-segments')) {
            map.getSource('colored-segments').setData(coloredSegmentsGeoJson.value || { type: 'FeatureCollection', features: [] });
        }
        if (map.getSource('gpx-trace') && lineString.value) {
            map.getSource('gpx-trace').setData(lineString.value);
        }
        updateMapFeatures();
        updateZoneMarkers();
    }
});

watch(selectedZoneId, () => {
    updateZoneMarkers();
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


// La fonction toHex locale a été supprimée au profit de useVuetifyColors

async function loadNonMapData() {
  const lissageCapValue = await getSettingValue('Importation/Tracking/LissageCap');
  lissageCap.value = lissageCapValue || 15;

  try {
    const data = await invoke('get_debug_data', { 
        circuitId: circuitId.value, 
        variantId: selectedVariantId.value 
    });
    lineString.value = data.line_string;
    trackingPoints.value = data.tracking_points;
    segmentMetadata.value = data.segment_metadata;

    // Charger les gradients comme dans VisualizeView
    const segmentLength = await getSettingValue('Importation/Tracking/LongueurSegment') || 20;
    try {
        // Utilisation de la logique partagée pour construire la map de couleurs (incluant les pentes négatives)
        const slopeColors = await buildSlopeColorsMap(getSettingValue, toHex);

        const geoJson = await invoke('get_colored_segments_geojson', {
            circuitId: circuitId.value,
            slopeColors: slopeColors,
            segmentLength: segmentLength,
            variantId: selectedVariantId.value
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

    // Layers Variantes (Separes pour z-index)
    // Background: Abandoned (Black)
    if (!map.getLayer('trace-variant-abandoned')) {
        map.addLayer({
            id: 'trace-variant-abandoned',
            type: 'line',
            source: 'colored-segments',
            layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' },
            paint: { 'line-width': 4, 'line-opacity': 1, 'line-color': '#000000' },
            filter: ['==', ['get', 'status'], 'ABANDONED']
        });
    }

    // Middle: Common (Green)
    if (!map.getLayer('trace-variant-common')) {
        map.addLayer({
            id: 'trace-variant-common',
            type: 'line',
            source: 'colored-segments',
            layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' },
            paint: { 'line-width': 4, 'line-opacity': 1, 'line-color': '#4CAF50' },
            filter: ['==', ['get', 'status'], 'COMMON']
        });
    }

    // Top: New (Blue)
    if (!map.getLayer('trace-variant-new')) {
        map.addLayer({
            id: 'trace-variant-new',
            type: 'line',
            source: 'colored-segments',
            layout: { 'line-join': 'round', 'line-cap': 'round', 'visibility': 'none' },
            paint: { 'line-width': 4, 'line-opacity': 1, 'line-color': '#2196F3' },
            filter: ['==', ['get', 'status'], 'NEW']
        });
    }
  }

  // Backup: Source for main lineString if colored segments fail (or unused but safe to keep)
  if (!map.getSource('gpx-trace') && lineString.value) {
     map.addSource('gpx-trace', { type: 'geojson', data: lineString.value });
  }

  // Source for zone markers
  if (!map.getSource('zone-markers')) {
      map.addSource('zone-markers', { type: 'geojson', data: turf.featureCollection([]) });
      
      map.addLayer({
          id: 'zone-markers-layer',
          type: 'circle',
          source: 'zone-markers',
          paint: {
              'circle-radius': ['case', ['get', 'isRetour'], 5, 8],
              'circle-color': ['get', 'color'],
              'circle-stroke-width': ['case', ['get', 'isRetour'], 0, 2],
              'circle-stroke-color': '#FFFFFF'
          }
      });
      
      map.addLayer({
          id: 'zone-labels-layer',
          type: 'symbol',
          source: 'zone-markers',
          layout: {
              'text-field': ['get', 'title'],
              'text-font': ['Open Sans Bold', 'Arial Unicode MS Bold'],
              'text-size': 11,
              'text-offset': [0, 1.5],
              'text-anchor': 'top'
          },
          paint: {
              'text-color': '#000000',
              'text-halo-color': '#FFFFFF',
              'text-halo-width': 1
          }
      });
  }

  updateLayerVisibility();
  updateZoneMarkers();

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
    
    const showAller = selectedTraceLayer.value === 'all' || selectedTraceLayer.value === 'aller';
    const showRetour = selectedTraceLayer.value === 'all' || selectedTraceLayer.value === 'retour';
    const showStatus = selectedTraceLayer.value === 'status';
    
    // Global toggle
    const globalVisible = showTrace.value;

    if (map.getLayer('gpx-trace-aller')) {
        map.setLayoutProperty('gpx-trace-aller', 'visibility', globalVisible && showAller && !showStatus ? 'visible' : 'none');
    }
    if (map.getLayer('gpx-trace-retour')) {
        map.setLayoutProperty('gpx-trace-retour', 'visibility', globalVisible && showRetour && !showStatus ? 'visible' : 'none');
    }

    if (map.getLayer('trace-variant-abandoned')) {
        map.setLayoutProperty('trace-variant-abandoned', 'visibility', globalVisible && showStatus ? 'visible' : 'none');
    }
    if (map.getLayer('trace-variant-common')) {
        map.setLayoutProperty('trace-variant-common', 'visibility', globalVisible && showStatus ? 'visible' : 'none');
    }
    if (map.getLayer('trace-variant-new')) {
        map.setLayoutProperty('trace-variant-new', 'visibility', globalVisible && showStatus ? 'visible' : 'none');
    }
}

function updateZoneMarkers() {
    if (!map || !map.getSource('zone-markers')) return;
    
    if (selectedZoneId.value === null || !segmentMetadata.value) {
        map.getSource('zone-markers').setData(turf.featureCollection([]));
        return;
    }
    
    const zone = segmentMetadata.value.overlappingZones.find(z => z.zoneId === selectedZoneId.value);
    if (!zone) {
        map.getSource('zone-markers').setData(turf.featureCollection([]));
        return;
    }
    
    const features = [];
    
    // 🔴 UNIFICATION INDEXATION
    // Désormais, tant pour la trace principale que pour les variants, 
    // les index stockés dans metadata se réfèrent au fichier lineString (ou lineString_FULL).
    let coordinatesSource = [];
    if (lineString.value && lineString.value.coordinates) {
        coordinatesSource = lineString.value.coordinates;
    } else if (lineString.value && lineString.value.geometry && lineString.value.geometry.coordinates) {
        coordinatesSource = lineString.value.geometry.coordinates;
    }
    
    // Helper to get point safely
    const getPt = (idx) => coordinatesSource[idx];
    
    // Aller Start (Marron clair)
    const allerStartIdx = zone.allerStartIndex;
    const p1 = getPt(allerStartIdx);
    if (p1) {
        features.push(turf.point(p1, { 
            color: '#D2B48C', 
            title: `Start Aller (idx:${allerStartIdx})`,
            isRetour: false
        }));
    }
    // Aller Stop (Marron foncé)
    const allerEndIdx = zone.allerEndIndex;
    const p2 = getPt(allerEndIdx);
    if (p2) {
        features.push(turf.point(p2, { 
            color: '#5D4037', 
            title: `Stop Aller (idx:${allerEndIdx})`,
            isRetour: false
        }));
    }
    // Retour Start (Mauve clair)
    const retourStartIdx = zone.retourStartIndex;
    const p3 = getPt(retourStartIdx);
    if (p3) {
        features.push(turf.point(p3, { 
            color: '#E1BEE7', 
            title: `Start Retour (idx:${retourStartIdx})`,
            isRetour: true
        }));
    }
    // Retour Stop (Mauve foncé)
    const retourEndIdx = zone.retourEndIndex;
    const p4 = getPt(retourEndIdx);
    if (p4) {
        features.push(turf.point(p4, { 
            color: '#7B1FA2', 
            title: `Stop Retour (idx:${retourEndIdx})`,
            isRetour: true
        }));
    }
    
    map.getSource('zone-markers').setData(turf.featureCollection(features));
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
