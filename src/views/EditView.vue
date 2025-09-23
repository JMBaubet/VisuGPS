<template>
  <v-container fluid class="pa-0 fill-height">
    <div id="map-container" ref="mapContainer" class="fill-height"></div>

    <!-- Buttons Container -->
    <div style="position: absolute; top: 10px; left: 10px; z-index: 1000; display: flex;">
      <v-btn icon="mdi-arrow-left" @click="goBack" class="mr-2"></v-btn>
      <v-btn icon="mdi-content-save" @click="saveCameraPosition"></v-btn>
    </div>

    <!-- Switch Container -->
    <div class="control-widget" style="position: absolute; top: 10px; left: 132px; z-index: 1000;">
      <v-switch
        v-model="updateCameraOnNav"
        color="primary"
        label="Sync Cam"
        hide-details
        density="compact"
      ></v-switch>
    </div>

    <CameraInfoWidget
      :bearing="currentBearing"
      :zoom="currentZoom"
      :pitch="currentPitch"
      :defaultZoom="defaultZoom"
      :defaultPitch="defaultPitch"
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

const route = useRoute();
const router = useRouter();
const { showSnackbar } = useSnackbar();
const { getSettingValue } = useSettings();

const circuitId = route.params.circuitId;
const mapContainer = ref(null);
let map = null;
const currentPointIndex = ref(0);
const trackingPoints = ref([]);
const lineStringCoordinates = ref([]); // Pour stocker les coordonnées de la lineString complète
const totalLineLength = ref(0); // Longueur totale de la lineString
const progressPercentage = ref(0); // Pourcentage de progression
const cameraCommandSettings = ref({}); // Pour stocker les paramètres des commandes de la caméra
const updateCameraOnNav = ref(true); // Switch state

// Reactive state for camera parameters display
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
  const bearing = map.getBearing(); // Cap

  // Mapbox GL JS ne fournit pas directement l'altitude de la caméra.
  // Pour l'instant, nous allons utiliser une valeur par défaut ou estimée.
  // Une implémentation plus avancée pourrait nécessiter un calcul basé sur le zoom et le pitch.
  // Estimer l'altitude de la caméra en fonction du zoom
  // C'est une heuristique, Mapbox GL JS ne fournit pas directement l'altitude de la caméra.
  // Plus le zoom est élevé, plus l'altitude est faible.
  // Utilisation d'une formule qui donne des valeurs plus significatives pour l'altitude de la caméra.
  const altitude = Math.round(5000 * Math.pow(0.8, zoom)); // Estimation plus progressive

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

onMounted(async () => {
  if (!circuitId) {
    showSnackbar('ID du circuit manquant pour l\'édition.', 'error');
    router.push({ name: 'Main' });
    return;
  }

  try {
    // Récupérer le token Mapbox
    const mapboxToken = await getSettingValue('Système/Tokens/mapbox');
    if (!mapboxToken) {
      showSnackbar('Token Mapbox non configuré.', 'error');
      router.push({ name: 'Main' });
      return;
    }
    mapboxgl.accessToken = mapboxToken;

    // Récupérer les paramètres de la carte depuis settings.json
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

    // Récupérer les paramètres des commandes de la caméra
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

    // Récupérer les données de tracking et lineString
    const rawTrackingData = await invoke('read_tracking_file', { circuitId: circuitId });
    const rawLineStringData = await invoke('read_line_string_file', { circuitId: circuitId });
    lineStringCoordinates.value = rawLineStringData.coordinates;

    // Calculer la longueur totale de la lineString
    const line = turf.lineString(lineStringCoordinates.value);
    totalLineLength.value = turf.length(line, { units: 'kilometers' });

    if (!rawTrackingData || rawTrackingData.length === 0) {
      showSnackbar('Données de tracking introuvables ou vides.', 'error');
      router.push({ name: 'Main' });
      return;
    }
    trackingPoints.value = rawTrackingData;
    currentPointIndex.value = 0; // Toujours commencer au premier point

    const firstPoint = trackingPoints.value[currentPointIndex.value];
    const initialCenter = firstPoint.coordonnee;
    const initialZoom = firstPoint.zoom;
    const initialPitch = firstPoint.pitch;
    const initialBearing = firstPoint.cap; // Cap est la direction (bearing) en Mapbox

    // Initialize default and current values for the widget
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

    // Activer la 3D du terrain
    map.on('load', () => {
      // Désactiver toutes les interactions de la souris
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
    });

    map.on('load', () => {
      // Ajouter la source et la couche pour la lineString
      map.addSource('circuit-line', {
        type: 'geojson',
        data: {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'LineString',
            coordinates: lineStringCoordinates.value, // Utiliser lineStringCoordinates.value
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

      // Ajouter la source et la couche pour la ligne d'avancement
      map.addSource('progress-line', {
        type: 'geojson',
        data: {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'LineString',
            coordinates: [], // Initialement vide
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
    router.push({ name: 'Main' });
  }
});

const updateCameraPosition = (index) => {
  if (!map || !trackingPoints.value.length) return;

  const point = trackingPoints.value[index];

  if (updateCameraOnNav.value) {
    // Update the reactive state for the widget and fly to the point's settings
    currentZoom.value = point.zoom;
    currentPitch.value = point.pitch;
    currentBearing.value = point.cap;

    map.flyTo({
      center: point.coordonnee,
      zoom: currentZoom.value,
      pitch: currentPitch.value,
      bearing: currentBearing.value,
      essential: true // This ensures the animation is smooth
    });
  } else {
    // Only move the center, keep current camera settings
    map.flyTo({
      center: point.coordonnee,
      essential: true
    });
  }

  // Mettre à jour la ligne d'avancement et calculer le pourcentage de progression
  if (totalLineLength.value > 0 && lineStringCoordinates.value.length > 1) {
    const line = turf.lineString(lineStringCoordinates.value);
    const cameraPoint = turf.point(point.coordonnee);

    // Trouver le point sur la ligne le plus proche de la position de la caméra
    const nearestPoint = turf.nearestPointOnLine(line, cameraPoint, { units: 'kilometers' });

    // Obtenir la sous-ligne du début jusqu'à ce point
    const currentProgressLine = turf.lineSlice(line.geometry.coordinates[0], nearestPoint.geometry.coordinates, line);
    const currentProgressLength = turf.length(currentProgressLine, { units: 'kilometers' });

    progressPercentage.value = (currentProgressLength / totalLineLength.value) * 100;

    if (map.getSource('progress-line')) {
      map.getSource('progress-line').setData({
        type: 'Feature',
        properties: {},
        geometry: currentProgressLine.geometry,
      });
    }
  } else {
  progressPercentage.value = 0;
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
      newBearing = (newBearing % 360 + 360) % 360; // Ensure wrap around
      currentBearing.value = Math.round(newBearing);
      map.easeTo({ bearing: currentBearing.value, duration: 50 });
      handled = true;
      break;
    }
    case bearingRightKey: {
      let newBearing = currentBearing.value + (isShiftPressed ? shiftBearingIncrement : bearingIncrement);
      newBearing = (newBearing % 360 + 360) % 360; // Ensure wrap around
      currentBearing.value = Math.round(newBearing);
      map.easeTo({ bearing: currentBearing.value, duration: 50 });
      handled = true;
      break;
    }
  }

  // Keep the existing logic for 'm' and 'l' keys for tracking point navigation
  let newIndex = currentPointIndex.value;
  let step = 1;
  if (event.ctrlKey) {
    step = 100;
  } else if (event.shiftKey && !handled) { // only use shift for step if not used for camera
    step = 10;
  }

  if (event.key === 'm' || event.key === 'M') { // Avancer
    newIndex = Math.min(currentPointIndex.value + step, trackingPoints.value.length - 1);
    handled = true;
  } else if (event.key === 'l' || event.key === 'L') { // Reculer
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
  // ... (code existant) ...
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  if (map) {
    map.remove();
    map = null;
  }
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped>
#map-container {
  width: 100%;
  height: 100%;
}

.control-widget {
  background-color: rgba(var(--v-theme-surface), 0.8);
  backdrop-filter: blur(4px);
  border-radius: 28px; /* Pill shape */
  padding: 0 16px 0 8px; /* Adjust padding for switch */
  display: flex;
  align-items: center;
  height: 40px; /* Match button height */
}
</style>