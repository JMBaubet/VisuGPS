<template>
  <div id="map-container" ref="mapContainer"></div>
  <v-btn icon="mdi-arrow-left" class="back-button" @click="goBack" title="Retour à l'accueil"></v-btn>
  <div class="distance-display">{{ distanceDisplay }}</div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import mapboxgl from 'mapbox-gl';
import * as turf from '@turf/turf';
import { useSettings } from '@/composables/useSettings';
import { useSnackbar } from '@/composables/useSnackbar';

const props = defineProps({
  circuitId: {
    type: String,
    required: true,
  },
});

const router = useRouter();
const { settings, getSettingValue } = useSettings();
const { showSnackbar } = useSnackbar();

const mapContainer = ref(null);
let map = null;
let animationFrameId = null;
let isMapInitialized = false;
let warningShown = false;

const isPaused = ref(false);
const distanceDisplay = ref('0.00 km');

// --- Helper Functions ---
const lerp = (start, end, t) => start * (1 - t) + end * t;

const lerpAngle = (start, end, t) => {
    let delta = end - start;
    if (delta > 180) delta -= 360;
    else if (delta < -180) delta += 360;
    let result = start + delta * t;
    result = result % 360;
    if (result < 0) result += 360;
    return result;
};

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

const handleKeyPress = (e) => {
    if (e.key === 'p' || e.key === 'P') {
        isPaused.value = !isPaused.value;
    }
};

const initializeMap = async () => {
  if (!settings.value || !mapboxToken.value) {
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

    const controlPointIndices = trackingPointsWithDistance.reduce((acc, p, index) => {
        if (p.pointDeControl) acc.push(index);
        return acc;
    }, []);

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
      map.addSource('trace', { type: 'geojson', data: lineString });
      map.addLayer({
        id: 'trace-background-layer',
        type: 'line',
        source: 'trace',
        paint: { 'line-width': traceWidth.value, 'line-color': traceColor.value, 'line-opacity': traceOpacity.value * 0.4 }
      });

      map.addSource('comet-source', { type: 'geojson', data: { type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} } });
      map.addLayer({ id: 'comet-layer', type: 'line', source: 'comet-source', paint: { 'line-width': traceWidth.value, 'line-color': cometColor.value, 'line-opacity': traceOpacity.value } });

      let accumulatedTime = 0;
      let lastTimestamp = 0;

      const animate = (timestamp) => {
        if (lastTimestamp === 0) lastTimestamp = timestamp;
        const deltaTime = timestamp - lastTimestamp;
        lastTimestamp = timestamp;

        if (!isPaused.value) {
            accumulatedTime += deltaTime;
        }

        const phase = Math.min(accumulatedTime / totalDuration, 1);
        const distanceTraveled = totalDistance * phase;
        distanceDisplay.value = `${distanceTraveled.toFixed(2)} km`;

        if (!isPaused.value) {
            const cometLengthKm = cometLength.value / 1000;
            const startDistance = Math.max(0, distanceTraveled - cometLengthKm);
            if (distanceTraveled > startDistance) {
                const cometSlice = turf.lineSliceAlong(lineString, startDistance, distanceTraveled, { units: 'kilometers' });
                map.getSource('comet-source').setData(cometSlice);
            } else {
                map.getSource('comet-source').setData({ type: 'Feature', geometry: { type: 'LineString', coordinates: [] }, properties: {} });
            }

            let prevKeyframe, nextKeyframe;

            let lastPassedControlPointIndex = -1;
            for (let i = controlPointIndices.length - 1; i >= 0; i--) {
                const cpIndex = controlPointIndices[i];
                if (trackingPointsWithDistance[cpIndex].distance <= distanceTraveled) {
                    lastPassedControlPointIndex = cpIndex;
                    break;
                }
            }

            if (lastPassedControlPointIndex !== -1) {
                const controlPoint = trackingPointsWithDistance[lastPassedControlPointIndex];
                if (controlPoint.nbrSegment > 0) {
                    const nextCpIndex = lastPassedControlPointIndex + controlPoint.nbrSegment;
                    if (nextCpIndex < trackingPointsWithDistance.length) {
                        prevKeyframe = controlPoint;
                        nextKeyframe = trackingPointsWithDistance[nextCpIndex];
                    } 
                }
            }

            if (!prevKeyframe || !nextKeyframe) {
                if (lastPassedControlPointIndex !== -1 && !warningShown) {
                    showSnackbar("Le tracking n'est pas complétement validé !", 'warning');
                    warningShown = true;
                }
                let currentPointIndex = trackingPointsWithDistance.findIndex((p, i) => {
                    const nextPoint = trackingPointsWithDistance[i + 1];
                    return nextPoint && distanceTraveled >= p.distance && distanceTraveled < nextPoint.distance;
                });
                if (currentPointIndex === -1) currentPointIndex = trackingPointsWithDistance.length - 2;
                if (currentPointIndex < 0) currentPointIndex = 0;
                prevKeyframe = trackingPointsWithDistance[currentPointIndex];
                nextKeyframe = trackingPointsWithDistance[currentPointIndex + 1];
            }

            const prevKeyframeDist = prevKeyframe.distance;
            const nextKeyframeDist = nextKeyframe.distance;
            const segmentDist = nextKeyframeDist - prevKeyframeDist;
            const progressInSegment = segmentDist > 0 ? (distanceTraveled - prevKeyframeDist) / segmentDist : 0;

                        const prevZoom = prevKeyframe.editedZoom ?? prevKeyframe.zoom;
                        const nextZoom = nextKeyframe.editedZoom ?? nextKeyframe.zoom;
            
                        const prevPitch = prevKeyframe.editedPitch ?? prevKeyframe.pitch;
                        const nextPitch = nextKeyframe.editedPitch ?? nextKeyframe.pitch;
            
                        const prevCap = prevKeyframe.editedCap ?? prevKeyframe.cap;
                        const nextCap = nextKeyframe.editedCap ?? nextKeyframe.cap;
            
                        const zoom = lerp(prevZoom, nextZoom, progressInSegment);
                        const pitch = lerp(prevPitch, nextPitch, progressInSegment);
                        const bearing = lerpAngle(prevCap, nextCap, progressInSegment);
                        
                        const lookAtPointLng = lerp(prevKeyframe.coordonnee[0], nextKeyframe.coordonnee[0], progressInSegment);
                        const lookAtPointLat = lerp(prevKeyframe.coordonnee[1], nextKeyframe.coordonnee[1], progressInSegment);
            
                        map.setZoom(zoom);
                        map.setPitch(pitch);
                        map.setBearing(bearing);
                        map.setCenter([lookAtPointLng, lookAtPointLat]);        }

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
  window.addEventListener('keydown', handleKeyPress);
  watch(settings, (newSettings) => {
    if (newSettings && mapContainer.value && !isMapInitialized) {
      isMapInitialized = true;
      initializeMap();
    }
  }, { immediate: true });
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyPress);
  if (animationFrameId) cancelAnimationFrame(animationFrameId);
  if (map) map.remove();
  map = null;
  isMapInitialized = false;
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
.distance-display {
    position: absolute;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
    background-color: rgba(0, 0, 0, 0.5);
    color: white;
    padding: 5px 15px;
    border-radius: 10px;
    font-family: monospace;
    font-size: 1.2em;
    z-index: 1;
}
/* Hide mapbox logo/attribution for cleaner view, but ensure it's compliant with Mapbox terms */
.mapboxgl-ctrl-bottom-left, .mapboxgl-ctrl-bottom-right {
  display: none;
}
</style>
