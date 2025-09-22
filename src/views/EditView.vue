<template>
  <v-container fluid class="pa-0 fill-height">
    <div id="map-container" ref="mapContainer" class="fill-height"></div>

    <v-btn icon="mdi-arrow-left" @click="goBack" class="ma-2" style="position: absolute; top: 0; left: 0; z-index: 1000;"></v-btn>
    <v-btn icon="mdi-content-save" @click="saveCameraPosition" class="ma-2" style="position: absolute; top: 0; left: 60px; z-index: 1000;"></v-btn>
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
      map.addSource('mapbox-dem', {
        'type': 'raster-dem',
        'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
        'tileSize': 512
      });
      map.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': exaggeration });
    });

    map.on('load', () => {
      // Ajouter la source et la couche pour la lineString
      console.log('lineStringCoordinates.value:', lineStringCoordinates.value);
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
  map.flyTo({
    center: point.coordonnee,
    zoom: point.zoom,
    pitch: point.pitch,
    bearing: point.cap,
    essential: true // This ensures the animation is smooth
  });

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
    if (map.getSource('progress-line')) {
      map.getSource('progress-line').setData({
        type: 'Feature',
        properties: {},
        geometry: {
          type: 'LineString',
          coordinates: [],
        },
      });
    }
  }
  console.log('Progression:', progressPercentage.value.toFixed(2) + '%');
};

const handleKeydown = (event) => {
  console.log('Keydown event:', event.key, 'Shift:', event.shiftKey, 'Ctrl:', event.ctrlKey);
  let newIndex = currentPointIndex.value;
  let step = 1;
  if (event.ctrlKey) {
    step = 100;
  } else if (event.shiftKey) {
    step = 10;
  }
  console.log('Calculated step:', step);

  if (event.key === 'm' || event.key === 'M') { // Avancer
    newIndex = Math.min(currentPointIndex.value + step, trackingPoints.value.length - 1);
  } else if (event.key === 'l' || event.key === 'L') { // Reculer
    newIndex = Math.max(currentPointIndex.value - step, 0);
  }

  if (newIndex !== currentPointIndex.value) {
    currentPointIndex.value = newIndex;
    updateCameraPosition(currentPointIndex.value);
    // Optionnel: Mettre à jour le premier point du tracking.json avec la nouvelle position de la caméra
    // Cela pourrait être fait ici en appelant la commande Tauri 'update_camera_position'
    // avec les données du point actuel.
    // Pour l'instant, nous nous contentons de déplacer la caméra.
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
</style>