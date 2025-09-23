<template>
  <div class="graph-container-wrapper" @wheel.prevent>
    <div class="graph-container" :class="{ 'is-scrollable': isScrollable }" ref="scrollContainer">
      <svg :width="svgWidth" :height="svgHeight" @click="handleGraphClick">
        <!-- Axe X de référence -->
        <line :x1="0" :y1="graphCenterY" :x2="svgWidth" :y2="graphCenterY" class="axis-line" />

        <!-- Repères sur l'axe X -->
        <g v-for="marker in xMarkers" :key="marker.label">
          <line :x1="marker.x" :y1="graphCenterY - 5" :x2="marker.x" :y2="graphCenterY + 5" class="marker-line" />
          <text :x="marker.x" :y="graphCenterY + 20" class="marker-text">{{ marker.label }}</text>
        </g>

        <!-- Graphique du Zoom -->
        <path v-if="props.showZoom" :d="zoomPath" :style="{ stroke: zoomColor }" />

        <!-- Graphique du Pitch -->
        <path v-if="props.showPitch" :d="pitchPath" :style="{ stroke: pitchColor }" />

        <!-- Graphique du Bearing (Delta vs Précédent) -->
        <path v-if="props.showBearingDelta" :d="bearingDeltaPath" :style="{ stroke: bearingDeltaColor }" />

        <!-- Graphique du Bearing (Delta vs Début) -->
        <path v-if="props.showBearingTotalDelta" :d="bearingTotalDeltaPath" :style="{ stroke: bearingTotalDeltaColor }" />

        <!-- Zone d'avancement -->
        <rect x="0" y="0" :width="progressIndicatorX + 2" :height="svgHeight" :fill="progressZoneColor" opacity="0.1" />
      </svg>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const emit = defineEmits(['seek-distance']);

const { getSettingValue } = useSettings();
const { toHex } = useVuetifyColors();

const zoomColor = ref('');
const pitchColor = ref('');
const bearingDeltaColor = ref('');
const bearingTotalDeltaColor = ref('');
const progressZoneColor = ref(''); // New ref

onMounted(async () => {
  zoomColor.value = toHex(await getSettingValue('Edition/Graphe/couleurZoom'));
  pitchColor.value = toHex(await getSettingValue('Edition/Graphe/couleurPitch'));
  bearingDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/couleurBearingDelta'));
  bearingTotalDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/couleurBearingTotalDelta'));
  progressZoneColor.value = toHex(await getSettingValue('Edition/Mapbox/Trace/couleurAvancement')); // Load existing color
});

const props = defineProps({
  trackingPoints: { type: Array, required: true },
  totalLength: { type: Number, required: true },
  currentDistance: { type: Number, required: true },
  showZoom: { type: Boolean, default: true },
  showPitch: { type: Boolean, default: true },
  showBearingDelta: { type: Boolean, default: true },
  showBearingTotalDelta: { type: Boolean, default: true },
});

const handleGraphClick = (event) => {
  const svgRect = event.currentTarget.getBoundingClientRect();
  const x = event.clientX - svgRect.left;
  
  const clickedKm = x / kmToPx;
  emit('seek-distance', clickedKm);
};

const scrollContainer = ref(null);
const svgHeight = 400;
const graphCenterY = svgHeight / 2;

// --- Échelles ---
const kmToPx = 30;
const zoomToPx = 1 / 0.1;
const pitchToPx = 1;
const bearingDeltaToPx = 3;
const bearingTotalDeltaToPx = 1;

const svgWidth = computed(() => (props.totalLength * kmToPx) + 10);

const progressIndicatorX = computed(() => (props.currentDistance * kmToPx) - 1.5);

const isScrollable = computed(() => {
  const contentWidth = props.totalLength * kmToPx;
  const containerWidth = 1500;
  return contentWidth > containerWidth;
});

// --- Génération des repères --- 
const xMarkers = computed(() => {
  const markers = [];
  const intervalKm = 10;
  if (props.totalLength < intervalKm) return [];

  const markerCount = Math.floor(props.totalLength / intervalKm);
  for (let i = 1; i <= markerCount; i++) {
    const distance = i * intervalKm;
    markers.push({
      x: distance * kmToPx,
      label: `${distance}km`
    });
  }
  return markers;
});

// Auto-scroll logic
watch(() => props.currentDistance, (newDistance) => {
  if (!scrollContainer.value) return;

  const indicatorMarginKm = 10;
  const endBufferKm = 40;

  const scrollWidth = scrollContainer.value.scrollWidth;
  const clientWidth = scrollContainer.value.clientWidth;
  const maxScrollLeft = scrollWidth - clientWidth;

  let targetScrollLeft = (newDistance - indicatorMarginKm) * kmToPx;

  if (targetScrollLeft < 0) {
    targetScrollLeft = 0;
  }
  if (targetScrollLeft > maxScrollLeft) {
    targetScrollLeft = maxScrollLeft;
  }

  if (newDistance <= indicatorMarginKm) {
    targetScrollLeft = 0;
  } else if (newDistance >= props.totalLength - endBufferKm) {
    // Let clamped value take over
  }

  scrollContainer.value.scrollTo({
    left: targetScrollLeft,
    behavior: 'smooth',
  });
});

// --- Calcul des chemins SVG ---
const zoomPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultZoom = props.trackingPoints[0]?.zoom || 16;
  return props.trackingPoints.map((p, i) => {
    const x = p.distance * kmToPx;
    const y = graphCenterY - ((p.zoom - defaultZoom) * zoomToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const pitchPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultPitch = props.trackingPoints[0]?.pitch || 60;
  return props.trackingPoints.map((p, i) => {
    const x = p.distance * kmToPx;
    const y = graphCenterY - ((p.pitch - defaultPitch) * pitchToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const bearingDeltaPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  let lastBearing = props.trackingPoints[0]?.cap || 0;
  return props.trackingPoints.map((p, i) => {
    const x = p.distance * kmToPx;
    let delta = p.cap - lastBearing;
    if (delta > 180) delta -= 360;
    if (delta < -180) delta += 360;
    
    const y = graphCenterY - (delta * bearingDeltaToPx);
    lastBearing = p.cap;
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const bearingTotalDeltaPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const initialBearing = props.trackingPoints[0]?.cap || 0;
  return props.trackingPoints.map((p, i) => {
    const x = p.distance * kmToPx;
    let delta = p.cap - initialBearing;
    if (delta > 180) delta -= 360;
    if (delta < -180) delta += 360;

    const y = graphCenterY - (delta * bearingTotalDeltaToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

</script>

<style scoped>
.graph-container-wrapper {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 1500px;
  max-width: 95vw; /* Safety for smaller screens */
  height: 400px;
  background-color: rgba(var(--v-theme-surface), 0.8);
  backdrop-filter: blur(4px);
  border-top: 1px solid rgba(var(--v-theme-on-surface), 0.2);
  z-index: 1000;
  border-top-left-radius: 8px;
  border-top-right-radius: 8px;
}

.graph-container {
  width: 100%;
  height: 100%;
  overflow-x: hidden; /* Hide by default */
  overflow-y: hidden; /* Prevent vertical scroll caused by horizontal scrollbar */
}

.graph-container.is-scrollable {
  overflow-x: auto; /* Show only when needed */
}

.axis-line {
  stroke: rgba(var(--v-theme-on-surface), 0.3);
  stroke-width: 1;
  stroke-dasharray: 4 2;
}

.marker-line {
  stroke: rgba(var(--v-theme-on-surface), 0.2);
  stroke-width: 1;
}

.marker-text {
  fill: rgba(var(--v-theme-on-surface), 0.5);
  font-size: 0.75rem;
  text-anchor: middle;
}

path {
  fill: none;
  stroke-width: 1.5;
}

 

 
</style>
