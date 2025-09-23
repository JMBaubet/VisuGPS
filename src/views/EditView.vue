<template>
  <v-container fluid class="pa-0 fill-height">
    <div id="map-container" ref="mapContainer" class="fill-height"></div>

    <!-- Buttons Container -->
    <div style="position: absolute; top: 10px; left: 10px; z-index: 1000; display: flex;">
      <v-btn icon="mdi-arrow-left" @click="goBack" class="mr-2"></v-btn>
      <v-btn icon="mdi-content-save" @click="saveCameraPosition"></v-btn>
    </div>

    <!-- Switch Container -->
    <div style="position: absolute; top: 10px; left: 132px; z-index: 1000; display: flex; align-items: center;">
      <v-sheet
        class="control-widget"
        height="48"
        rounded="pill"
      >
        <v-switch
          v-model="updateCameraOnNav"
          color="primary"
          label="Sync Cam"
          hide-details
          density="compact"
        ></v-switch>
      </v-sheet>

      <v-btn
        v-if="!updateCameraOnNav"
        icon="mdi-camera-retake"
        @click="forceUpdateCamera"
        class="ml-2"
        color="info"
        size="small"
      ></v-btn>
    </div>

    <!-- Graph Controls Container -->
    <v-sheet
      class="control-widget"
      style="position: absolute; bottom: 10px; right: 10px; z-index: 1000; display: flex; flex-direction: column; align-items: flex-start; gap: 4px; padding: 8px;"
      rounded="lg"
    >
      <v-switch
        v-model="showGraph"
        color="primary"
        label="Graph"
        hide-details
        density="compact"
        style="margin-left: 8px; margin-right: 8px;"
      ></v-switch>

      <div v-if="showGraph" style="display: flex; flex-direction: column; gap: 4px;">
        <v-switch v-model="showBearingDelta" label="Δ Bearing" :color="graphBearingDeltaColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
        <v-switch v-model="showBearingTotalDelta" label="ΣΔ Bearing" :color="graphBearingTotalDeltaColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
        <v-switch v-model="showZoom" label="Zoom" :color="graphZoomColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
        <v-switch v-model="showPitch" label="Pitch" :color="graphPitchColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
      </div>
    </v-sheet>

    <TrackProgressWidget
      :totalLength="totalLineLength"
      :currentDistance="currentProgressDistance"
      :progressColor="couleurAvancement"
      :lineStringColor="traceColor"
    />

    <CameraInfoWidget
      :bearing="currentBearing"
      :zoom="currentZoom"
      :pitch="currentPitch"
      :defaultZoom="defaultZoom"
      :defaultPitch="defaultPitch"
    />

 

    <CameraGraph 
      v-if="trackingPoints.length > 0 && showGraph"
      :trackingPoints="trackingPoints"
      :totalLength="totalLineLength"
      :currentDistance="currentProgressDistance"
      :showZoom="showZoom"
      :showPitch="showPitch"
      :showBearingDelta="showBearingDelta"
      :showBearingTotalDelta="showBearingTotalDelta"
      @seek-distance="handleSeekDistance"
    />
  </v-container>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors'; // New import
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';
import * as turf from '@turf/turf';
import CameraInfoWidget from '@/components/CameraInfoWidget.vue';
import TrackProgressWidget from '@/components/TrackProgressWidget.vue';
import CameraGraph from '@/components/CameraGraph.vue';

const route = useRoute();
const router = useRouter();
const { showSnackbar } = useSnackbar();
const { getSettingValue } = useSettings();

const circuitId = route.params.circuitId;
const mapContainer = ref(null);
let map = null;
const currentPointIndex = ref(0);
const trackingPoints = ref([]);
const lineStringCoordinates = ref([]);
const totalLineLength = ref(0);
const progressPercentage = ref(0);
const currentProgressDistance = ref(0);
const cameraCommandSettings = ref({});
const updateCameraOnNav = ref(true);
const showGraph = ref(true);
const couleurAvancement = ref(''); // Declare as ref
const traceColor = ref('#FFA726'); // Initialize traceColor with a default hex color (e.g., orange)
const mapboxAvancementColorHex = ref(''); // New ref for Mapbox hex color

const showZoom = ref(false);
const showPitch = ref(false);
const showBearingDelta = ref(true);
const showBearingTotalDelta = ref(true);

// Graph curve colors
const graphZoomColor = ref('green'); // Default to green
const graphPitchColor = ref('blue'); // Default to blue
const graphBearingDeltaColor = ref('amber'); // Default to amber
const graphBearingTotalDeltaColor = ref('pink'); // Default to pink

const { toHex } = useVuetifyColors(); // New line

const currentZoom = ref(0);
const currentPitch = ref(0);
const currentBearing = ref(0);
const defaultZoom = ref(0);
const defaultPitch = ref(0);

const goBack = () => {
  router.push({ name: 'Main' });
};

const saveCameraPosition = async () => {
  if (!map) {
    showSnackbar('Carte non initialisée.', 'error');
    return;
  }

  const center = map.getCenter();
  const zoom = map.getZoom();
  const pitch = map.getPitch();
  const bearing = map.getBearing();

  const altitude = Math.round(5000 * Math.pow(0.8, zoom));

  try {
    await invoke('update_camera_position', {
      circuitId: circuitId,
      longitude: center.lng,
      latitude: center.lat,
      altitude: altitude,
      zoom: zoom,
      pitch: pitch,
      bearing: bearing,
    });
    showSnackbar('Position de la caméra sauvegardée avec succès.', 'success');
  } catch (error) {
    console.error('Erreur lors de la sauvegarde de la position de la caméra:', error);
    showSnackbar(`Erreur lors de la sauvegarde: ${error.message || error}`, 'error');
  }
};

const forceUpdateCamera = () => {
  if (!updateCameraOnNav.value) { // Only allow if sync is off
    const point = trackingPoints.value[currentPointIndex.value];
    if (point) {
      currentZoom.value = point.zoom;
      currentPitch.value = point.pitch;
      currentBearing.value = point.cap;

      map.flyTo({
        center: point.coordonnee, // Always update center
        zoom: currentZoom.value,
        pitch: currentPitch.value,
        bearing: currentBearing.value,
        essential: true,
        duration: 1000 // Add duration for smoother animation (1 second)
      });
    }
  }
};

const handleSeekDistance = (distanceInKm) => {
  if (!trackingPoints.value || trackingPoints.value.length === 0) return;

  // Find the closest point in trackingPoints to the clicked distance
  const closest = trackingPoints.value.reduce((prev, curr) => {
    return (Math.abs(curr.distance - distanceInKm) < Math.abs(prev.distance - distanceInKm) ? curr : prev);
  });

  // Find the index of that closest point
  const closestIndex = trackingPoints.value.findIndex(p => p.increment === closest.increment);

  if (closestIndex !== -1) {
    currentPointIndex.value = closestIndex;
    updateCameraPosition(closestIndex);
  }
};

const updateCameraPosition = (index) => {
  if (!map || !trackingPoints.value.length) return;

  const point = trackingPoints.value[index];

  // Always update the map center
  const flyToOptions = {
    center: point.coordonnee,
    essential: true
  };

  if (updateCameraOnNav.value) {
    // If Sync Cam is ON, update zoom, pitch, bearing automatically
    currentZoom.value = point.zoom;
    currentPitch.value = point.pitch;
    currentBearing.value = point.cap;
    flyToOptions.zoom = currentZoom.value;
    flyToOptions.pitch = currentPitch.value;
    flyToOptions.bearing = currentBearing.value;
  }
  // If Sync Cam is OFF, zoom, pitch, bearing are NOT updated automatically here.
  // They will only be updated when forceUpdateCamera is called.

  map.flyTo(flyToOptions);

  if (totalLineLength.value > 0) {
    currentProgressDistance.value = point.distance;
    progressPercentage.value = (point.distance / totalLineLength.value) * 100;

    const line = turf.lineString(lineStringCoordinates.value);
    const slicedLine = turf.lineSliceAlong(line, 0, point.distance, { units: 'kilometers' });

    if (map.getSource('progress-line')) {
      map.getSource('progress-line').setData(slicedLine);
    }
  } else {
    progressPercentage.value = 0;
    currentProgressDistance.value = 0;
  }
};

const handleKeydown = (event) => {
  if (!map) return;

  const {
    zoomInKey, zoomOutKey, pitchUpKey, pitchDownKey, bearingLeftKey, bearingRightKey,
    zoomIncrement, pitchIncrement, bearingIncrement,
    shiftZoomIncrement, shiftPitchIncrement, shiftBearingIncrement
  } = cameraCommandSettings.value;

  const isShiftPressed = event.shiftKey;
  let handled = false;

  switch (event.key) {
    case zoomInKey: {
      const newZoom = currentZoom.value + (isShiftPressed ? shiftZoomIncrement : zoomIncrement);
      currentZoom.value = parseFloat(Math.max(0, Math.min(22, newZoom)).toFixed(1));
      map.easeTo({ zoom: currentZoom.value, duration: 50 });
      handled = true;
      break;
    }
    case zoomOutKey: {
      const newZoom = currentZoom.value - (isShiftPressed ? shiftZoomIncrement : zoomIncrement);
      currentZoom.value = parseFloat(Math.max(0, Math.min(22, newZoom)).toFixed(1));
      map.easeTo({ zoom: currentZoom.value, duration: 50 });
      handled = true;
      break;
    }
    case pitchUpKey: {
      const newPitch = currentPitch.value + (isShiftPressed ? shiftPitchIncrement : pitchIncrement);
      currentPitch.value = Math.round(Math.max(0, Math.min(85, newPitch)));
      map.easeTo({ pitch: currentPitch.value, duration: 50 });
      handled = true;
      break;
    }
    case pitchDownKey: {
      const newPitch = currentPitch.value - (isShiftPressed ? shiftPitchIncrement : pitchIncrement);
      currentPitch.value = Math.round(Math.max(0, Math.min(85, newPitch)));
      map.easeTo({ pitch: currentPitch.value, duration: 50 });
      handled = true;
      break;
    }
    case bearingLeftKey: {
      let newBearing = currentBearing.value - (isShiftPressed ? shiftBearingIncrement : bearingIncrement);
      newBearing = (newBearing % 360 + 360) % 360;
      currentBearing.value = Math.round(newBearing);
      map.easeTo({ bearing: currentBearing.value, duration: 50 });
      handled = true;
      break;
    }
    case bearingRightKey: {
      let newBearing = currentBearing.value + (isShiftPressed ? shiftBearingIncrement : bearingIncrement);
      newBearing = (newBearing % 360 + 360) % 360;
      currentBearing.value = Math.round(newBearing);
      map.easeTo({ bearing: currentBearing.value, duration: 50 });
      handled = true;
      break;
    }
  }

  let newIndex = currentPointIndex.value;
  let step = 1;
  if (event.ctrlKey) {
    step = 100;
  } else if (event.shiftKey && !handled) {
    step = 10;
  }

  if (event.key === 'm' || event.key === 'M') {
    newIndex = Math.min(currentPointIndex.value + step, trackingPoints.value.length - 1);
    handled = true;
  } else if (event.key === 'l' || event.key === 'L') {
    newIndex = Math.max(currentPointIndex.value - step, 0);
    handled = true;
  }

  if (newIndex !== currentPointIndex.value) {
    currentPointIndex.value = newIndex;
    updateCameraPosition(currentPointIndex.value);
  }

  if (handled) {
    event.preventDefault();
  }
};

onMounted(async () => {
  if (!circuitId) {
    showSnackbar('ID du circuit manquant pour l\'édition.', 'error');
    router.push({ name: 'Main' });
    return;
  }

  try {
    const mapboxToken = await getSettingValue('Système/Tokens/mapbox');
    if (!mapboxToken) {
      showSnackbar('Token Mapbox non configuré.', 'error');
      router.push({ name: 'Main' });
      return;
    }
    mapboxgl.accessToken = mapboxToken;

    const styleVisualisation = await getSettingValue('Edition/Mapbox/styleVisualisation');
    let rawTraceColor = await getSettingValue('Edition/Mapbox/Trace/couleur');
    if (rawTraceColor && !rawTraceColor.startsWith('#')) {
      traceColor.value = await invoke('convert_vuetify_color', { colorName: rawTraceColor });
    } else {
      traceColor.value = rawTraceColor;
    }
    const traceWidth = await getSettingValue('Edition/Mapbox/Trace/epaisseur');
    const exaggeration = await getSettingValue('Edition/Mapbox/Relief/exaggeration');
    let rawCouleurAvancement = await getSettingValue('Edition/Mapbox/Trace/couleurAvancement');
    // Extract the base color name (e.g., "yellow" from "yellow-darken-4")
    if (rawCouleurAvancement) {
      const parts = rawCouleurAvancement.split('-');
      couleurAvancement.value = parts[0]; // Take the first part as the base color name
      mapboxAvancementColorHex.value = toHex(couleurAvancement.value); // Convert to hex for Mapbox
    } else {
      couleurAvancement.value = 'primary'; // Fallback if not found
      mapboxAvancementColorHex.value = toHex('primary'); // Convert fallback to hex
    }
    const epaisseurAvancement = await getSettingValue('Edition/Mapbox/Trace/epaisseurAvancement');

    // Load graph curve colors
    graphZoomColor.value = toHex(await getSettingValue('Edition/Graphe/couleurZoom'));
    graphPitchColor.value = toHex(await getSettingValue('Edition/Graphe/couleurPitch'));
    graphBearingDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/couleurBearingDelta'));
    graphBearingTotalDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/couleurBearingTotalDelta'));

    console.log('couleurAvancement before passing to widget:', couleurAvancement); // Debug log
    console.log('traceColor before passing to widget:', traceColor); // Debug log

    cameraCommandSettings.value = {
      zoomInKey: await getSettingValue('Edition/Mapbox/Commandes Caméra/ZoomInKey'),
      zoomOutKey: await getSettingValue('Edition/Mapbox/Commandes Caméra/ZoomOutKey'),
      pitchUpKey: await getSettingValue('Edition/Mapbox/Commandes Caméra/PitchUpKey'),
      pitchDownKey: await getSettingValue('Edition/Mapbox/Commandes Caméra/PitchDownKey'),
      bearingLeftKey: await getSettingValue('Edition/Mapbox/Commandes Caméra/BearingLeftKey'),
      bearingRightKey: await getSettingValue('Edition/Mapbox/Commandes Caméra/BearingRightKey'),
      zoomIncrement: await getSettingValue('Edition/Mapbox/Commandes Caméra/ZoomIncrement'),
      pitchIncrement: await getSettingValue('Edition/Mapbox/Commandes Caméra/PitchIncrement'),
      bearingIncrement: await getSettingValue('Edition/Mapbox/Commandes Caméra/BearingIncrement'),
      shiftZoomIncrement: await getSettingValue('Edition/Mapbox/Commandes Caméra/ShiftZoomIncrement'),
      shiftPitchIncrement: await getSettingValue('Edition/Mapbox/Commandes Caméra/ShiftPitchIncrement'),
      shiftBearingIncrement: await getSettingValue('Edition/Mapbox/Commandes Caméra/ShiftBearingIncrement'),
    };

    const rawTrackingData = await invoke('read_tracking_file', { circuitId: circuitId });
    const rawLineStringData = await invoke('read_line_string_file', { circuitId: circuitId });
    lineStringCoordinates.value = rawLineStringData.coordinates;

    const line = turf.lineString(lineStringCoordinates.value);
    totalLineLength.value = turf.length(line, { units: 'kilometers' });

    if (!rawTrackingData || rawTrackingData.length === 0) {
      showSnackbar('Données de tracking introuvables ou vides.', 'error');
      router.push({ name: 'Main' });
      return;
    }

    // Process tracking data to add cumulative distance (performant method)
    const segmentLengthKm = 0.1; // Each tracking point represents a 100m segment
    const processedTrackingPoints = rawTrackingData.map((point, index) => ({
      ...point,
      distance: index * segmentLengthKm,
    }));
    trackingPoints.value = processedTrackingPoints;

    currentPointIndex.value = 0;

    const firstPoint = trackingPoints.value[currentPointIndex.value];
    const initialCenter = firstPoint.coordonnee;
    const initialZoom = firstPoint.zoom;
    const initialPitch = firstPoint.pitch;
    const initialBearing = firstPoint.cap;

    defaultZoom.value = initialZoom;
    defaultPitch.value = initialPitch;
    currentZoom.value = initialZoom;
    currentPitch.value = initialPitch;
    currentBearing.value = initialBearing;

    map = new mapboxgl.Map({
      container: mapContainer.value,
      style: styleVisualisation,
      center: initialCenter,
      zoom: initialZoom,
      pitch: initialPitch,
      bearing: initialBearing,
      antialias: true,
    });

    map.on('load', () => {
      map.keyboard.disable();
      map.boxZoom.disable();
      map.doubleClickZoom.disable();
      map.dragPan.disable();
      map.dragRotate.disable();
      map.scrollZoom.disable();
      map.touchZoomRotate.disable();

      map.addSource('mapbox-dem', {
        'type': 'raster-dem',
        'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
        'tileSize': 512
      });
      map.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': exaggeration });

      map.addSource('circuit-line', {
        type: 'geojson',
        data: {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'LineString',
            coordinates: lineStringCoordinates.value,
          },
        },
      });

      map.addLayer({
        id: 'circuit-line',
        type: 'line',
        source: 'circuit-line',
        layout: {
          'line-join': 'round',
          'line-cap': 'round',
        },
                  paint: {
                    'line-color': traceColor.value, // Explicitly use .value
                    'line-width': traceWidth,
                  },      });

      map.addSource('progress-line', {
        type: 'geojson',
        data: {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'LineString',
            coordinates: [],
          },
        },
      });

      map.addLayer({
        id: 'progress-line',
        type: 'line',
        source: 'progress-line',
        layout: {
          'line-join': 'round',
          'line-cap': 'round',
        },
        paint: {
          'line-color': mapboxAvancementColorHex.value, // Use hex color for Mapbox
          'line-width': epaisseurAvancement,
        },
      });
    });

    map.on('error', (e) => {
      console.error('Mapbox error:', e.error);
      showSnackbar(`Erreur Mapbox: ${e.error.message}`, 'error');
    });

  } catch (error) {
    console.error('Erreur lors de l\'initialisation de la vue d\'édition:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }

  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  if (map) {
    map.remove();
    map = null;
  }
  window.removeEventListener('keydown', handleKeydown);
});</script>

<style scoped>
#map-container {
  width: 100%;
  height: 100%;
}

.control-widget {
  padding: 0 16px 0 8px; /* Adjust padding for switch */
  display: flex;
  align-items: center;
}
</style>