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

const route = useRoute();
const router = useRouter();
const { showSnackbar } = useSnackbar();
const { getSettingValue } = useSettings();

const circuitId = route.params.circuitId;
const mapContainer = ref(null);
let map = null;

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

    // Récupérer les données de tracking et lineString
    const trackingData = await invoke('read_tracking_file', { circuitId: circuitId });
    const lineStringData = await invoke('read_line_string_file', { circuitId: circuitId });

    if (!trackingData || trackingData.length === 0) {
      showSnackbar('Données de tracking introuvables ou vides.', 'error');
      router.push({ name: 'Main' });
      return;
    }

    const firstPoint = trackingData[0];
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
      map.addSource('circuit-line', {
        type: 'geojson',
        data: {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'LineString',
            coordinates: lineStringData.coordinates,
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

onUnmounted(() => {
  if (map) {
    map.remove();
    map = null;
  }
});
</script>

<style scoped>
#map-container {
  width: 100%;
  height: 100%;
}
</style>