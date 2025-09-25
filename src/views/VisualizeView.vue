<template>
  <div id="map-container" ref="mapContainer"></div>
  <v-btn icon="mdi-arrow-left" class="back-button" @click="goBack" title="Retour à l'accueil"></v-btn>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import mapboxgl from 'mapbox-gl';
import * as turf from '@turf/turf';
import { useSettings } from '@/composables/useSettings';

const props = defineProps({
  circuitId: {
    type: String,
    required: true,
  },
});

const router = useRouter();
const { settings, getSettingValue } = useSettings();

const mapContainer = ref(null);
let map = null;
let animationFrameId = null;
let isMapInitialized = false; // Guard flag

// --- Helper Functions ---
const lerp = (start, end, t) => start * (1 - t) + end * t;

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

const initializeMap = async () => {
  if (!settings.value || !mapboxToken.value) {
    console.error("Settings or Mapbox token are not available.");
    return;
  }
  mapboxgl.accessToken = mapboxToken.value;

  try {
    const [lineString, trackingData] = await Promise.all([
      invoke('read_line_string_file', { circuitId: props.circuitId }),
      invoke('read_tracking_file', { circuitId: props.circuitId })
    ]);

    if (!lineString || !trackingData || trackingData.length < 2) {
      console.error("Failed to load valid circuit data.");
      return;
    }

    const totalDistance = turf.length(lineString, { units: 'kilometers' });
    const totalDuration = totalDistance * animationSpeed.value;

    const trackingPointsWithDistance = trackingData.map(p => {
        const pointOnLine = turf.point(p.coordonnee);
        const snapped = turf.nearestPointOnLine(lineString, pointOnLine, {units: 'kilometers'});
        return { ...p, distance: snapped.properties.location };
    });

    map = new mapboxgl.Map({
      container: mapContainer.value,
      style: mapStyle.value,
      center: trackingData[0].coordonnee,
      zoom: trackingData[0].zoom,
      pitch: trackingData[0].pitch,
      bearing: trackingData[0].cap,
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
      map.addSource('trace', {
        type: 'geojson',
        data: lineString,
      });
      map.addLayer({
        id: 'trace-background-layer',
        type: 'line',
        source: 'trace',
        paint: {
          'line-width': traceWidth.value,
          'line-color': traceColor.value,
          'line-opacity': traceOpacity.value * 0.4 
        }
      });

      map.addSource('comet-source', {
          type: 'geojson',
          data: { type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} }
      });
      map.addLayer({
          id: 'comet-layer',
          type: 'line',
          source: 'comet-source',
          paint: {
              'line-width': traceWidth.value,
              'line-color': cometColor.value,
              'line-opacity': traceOpacity.value
          }
      });

      let startTime = null;

      const animate = (timestamp) => {
        if (!startTime) startTime = timestamp;
        const elapsedTime = timestamp - startTime;
        const phase = Math.min(elapsedTime / totalDuration, 1);
        const distanceTraveled = totalDistance * phase;

        const cometLengthKm = cometLength.value / 1000;
        const startDistance = Math.max(0, distanceTraveled - cometLengthKm);
        if (distanceTraveled > startDistance) {
            const cometSlice = turf.lineSliceAlong(lineString, startDistance, distanceTraveled, { units: 'kilometers' });
            map.getSource('comet-source').setData(cometSlice);
        } else {
            map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
        }

        let currentPointIndex = trackingPointsWithDistance.findIndex((p, i) => {
            const nextPoint = trackingPointsWithDistance[i + 1];
            if (!nextPoint) return false;
            return distanceTraveled >= p.distance && distanceTraveled < nextPoint.distance;
        });

        if (currentPointIndex === -1) {
            currentPointIndex = distanceTraveled < trackingPointsWithDistance[0].distance ? 0 : trackingPointsWithDistance.length - 2;
        }
        if (currentPointIndex < 0) currentPointIndex = 0;

        const prevKeyframe = trackingPointsWithDistance[currentPointIndex];
        const nextKeyframe = trackingPointsWithDistance[currentPointIndex + 1];

        const prevKeyframeDist = prevKeyframe.distance;
        const nextKeyframeDist = nextKeyframe.distance;
        const segmentDist = nextKeyframeDist - prevKeyframeDist;
        const progressInSegment = segmentDist > 0 ? (distanceTraveled - prevKeyframeDist) / segmentDist : 0;

        const lookAtPoint = turf.along(lineString, distanceTraveled, { units: 'kilometers' });

        if (prevKeyframe.coordonneeCamera && prevKeyframe.coordonneeCamera.length > 0 && nextKeyframe.coordonneeCamera && nextKeyframe.coordonneeCamera.length > 0) {
            const cameraLng = lerp(prevKeyframe.coordonneeCamera[0], nextKeyframe.coordonneeCamera[0], progressInSegment);
            const cameraLat = lerp(prevKeyframe.coordonneeCamera[1], nextKeyframe.coordonneeCamera[1], progressInSegment);
            const cameraAltitude = lerp(prevKeyframe.altitudeCamera, nextKeyframe.altitudeCamera, progressInSegment);

            const cameraOptions = {
                position: mapboxgl.MercatorCoordinate.fromLngLat({ lng: cameraLng, lat: cameraLat }, cameraAltitude),
                lookAtPoint: { lng: lookAtPoint.geometry.coordinates[0], lat: lookAtPoint.geometry.coordinates[1] },
            };
            map.setFreeCameraOptions(cameraOptions);
        } else {
            const zoom = lerp(prevKeyframe.zoom, nextKeyframe.zoom, progressInSegment);
            const pitch = lerp(prevKeyframe.pitch, nextKeyframe.pitch, progressInSegment);
            const bearing = lerp(prevKeyframe.cap, nextKeyframe.cap, progressInSegment);

            map.setZoom(zoom);
            map.setPitch(pitch);
            map.setBearing(bearing);
            map.setCenter(lookAtPoint.geometry.coordinates);
        }

        if (phase < 1) {
          animationFrameId = requestAnimationFrame(animate);
        }
      };

      animationFrameId = requestAnimationFrame(animate);
    });

  } catch (error) {
    console.error("Error during visualization setup:", error);
  }
};

onMounted(() => {
  watch(settings, (newSettings) => {
    if (newSettings && mapContainer.value && !isMapInitialized) {
      isMapInitialized = true;
      initializeMap();
    }
  }, { immediate: true });
});

onUnmounted(() => {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
  }
  if (map) {
    map.remove();
    map = null;
  }
  isMapInitialized = false; // Reset flag
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
}
/* Hide mapbox logo/attribution for cleaner view, but ensure it's compliant with Mapbox terms */
.mapboxgl-ctrl-bottom-left, .mapboxgl-ctrl-bottom-right {
  display: none;
}
</style>
