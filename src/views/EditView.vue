<template>
  <v-container fluid class="pa-0 fill-height">
    <div id="map-container" ref="mapContainer" class="fill-height"></div>

    <!-- Buttons Container -->
    <div style="position: absolute; top: 10px; left: 10px; z-index: 1000; display: flex;">
      <v-btn icon="mdi-arrow-left" @click="goBack" class="mr-2"></v-btn>
      <v-btn icon="mdi-content-save" @click="saveCameraPosition"></v-btn>
    </div>

    <!-- Switch Container -->
    <v-sheet
      class="control-widget"
      style="position: absolute; top: 10px; left: 132px; z-index: 1000;"
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

    <!-- Graph Toggle Switch -->
    <v-sheet
      class="control-widget"
      style="position: absolute; bottom: 30px; right: 10px; z-index: 1000;"
      height="48"
      rounded="pill"
    >
        <v-switch
          v-model="showGraph"
          color="primary"
          label="Graph"
          hide-details
          density="compact"
        ></v-switch>
    </v-sheet>

    <TrackProgressWidget
      :totalLength="totalLineLength"
      :currentDistance="currentProgressDistance"
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

  if (updateCameraOnNav.value) {
    currentZoom.value = point.zoom;
    currentPitch.value = point.pitch;
    currentBearing.value = point.cap;

    map.flyTo({
      center: point.coordonnee,
      zoom: currentZoom.value,
      pitch: currentPitch.value,
      bearing: currentBearing.value,
      essential: true
    });
  } else {
    map.flyTo({
      center: point.coordonnee,
      essential: true
    });
  }

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
    let traceColor = await getSettingValue('Edition/Mapbox/Trace/couleur');
    if (traceColor && !traceColor.startsWith('#')) {
      traceColor = await invoke('convert_vuetify_color', { colorName: traceColor });
    }
    const traceWidth = await getSettingValue('Edition/Mapbox/Trace/epaisseur');
    const exaggeration = await getSettingValue('Edition/Mapbox/Relief/exaggeration');
    let couleurAvancement = await getSettingValue('Edition/Mapbox/Trace/couleurAvancement');
    if (couleurAvancement && !couleurAvancement.startsWith('#')) {
      couleurAvancement = await invoke('convert_vuetify_color', { colorName: couleurAvancement });
    }
    const epaisseurAvancement = await getSettingValue('Edition/Mapbox/Trace/epaisseurAvancement');

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
          'line-color': traceColor,
          'line-width': traceWidth,
        },
      });

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
          'line-color': couleurAvancement,
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