<template>
  <div class="graph-container-wrapper">
    <div class="graph-container" ref="scrollContainer">
      <svg :width="svgWidth" :height="svgHeight">
        <!-- Axe X de référence -->
        <line :x1="0" :y1="graphCenterY" :x2="svgWidth" :y2="graphCenterY" class="axis-line" />

        <!-- Graphique du Zoom -->
        <path :d="zoomPath" class="zoom-path" />

        <!-- Graphique du Pitch -->
        <path :d="pitchPath" class="pitch-path" />

        <!-- Graphique du Bearing (Delta vs Précédent) -->
        <path :d="bearingDeltaPath" class="bearing-delta-path" />

        <!-- Graphique du Bearing (Delta vs Début) -->
        <path :d="bearingTotalDeltaPath" class="bearing-total-delta-path" />

        <!-- Indicateur de progression -->
        <rect :x="progressIndicatorX" y="0" width="3" :height="svgHeight" class="progress-indicator" />
      </svg>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const props = defineProps({
  trackingPoints: {
    type: Array,
    required: true,
  },
  totalLength: {
    type: Number,
    required: true,
  },
  currentDistance: {
    type: Number,
    required: true,
  },
});

const scrollContainer = ref(null);
const svgHeight = 400; // Hauteur fixe pour le widget
const graphCenterY = svgHeight / 2;

// --- Échelles ---
const kmToPx = 30; // 30px pour 1 km
const zoomToPx = 1 / 0.1; // 1px pour 0.1 de zoom
const pitchToPx = 1; // 1px pour 1 degré
const bearingDeltaToPx = 3; // 3px pour 1 degré de delta
const bearingTotalDeltaToPx = 1; // 1px pour 1 degré de delta total

const svgWidth = computed(() => props.totalLength * kmToPx);

const progressIndicatorX = computed(() => (props.currentDistance * kmToPx) - 1.5);

// Auto-scroll logic
watch(() => props.currentDistance, (newDistance) => {
  if (!scrollContainer.value) return;

  const indicatorMarginKm = 10; // Keep indicator 10km from the left edge
  const endBufferKm = 40; // Stop auto-scrolling when 40km from the end

  const scrollWidth = scrollContainer.value.scrollWidth;
  const clientWidth = scrollContainer.value.clientWidth;
  const maxScrollLeft = scrollWidth - clientWidth;

  let targetScrollLeft = (newDistance - indicatorMarginKm) * kmToPx;

  // Clamp the scroll position
  if (targetScrollLeft < 0) {
    targetScrollLeft = 0;
  }
  if (targetScrollLeft > maxScrollLeft) {
    targetScrollLeft = maxScrollLeft;
  }

  // Override if near the start or end based on user logic
  if (newDistance <= indicatorMarginKm) {
    targetScrollLeft = 0;
  } else if (newDistance >= props.totalLength - endBufferKm) {
    // When near the end, we let the clamped value take over, 
    // which will be maxScrollLeft, so no special action needed.
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
    // Handle circular wrap-around for shortest angle
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
    // Handle circular wrap-around for shortest angle
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
  overflow-x: auto;
}

.axis-line {
  stroke: rgba(var(--v-theme-on-surface), 0.3);
  stroke-width: 1;
  stroke-dasharray: 4 2;
}

path {
  fill: none;
  stroke-width: 1.5;
}

.zoom-path {
  stroke: #4CAF50; /* Green */
}

.pitch-path {
  stroke: #2196F3; /* Blue */
}

.bearing-delta-path {
  stroke: #FFC107; /* Amber */
}

.bearing-total-delta-path {
  stroke: #E91E63; /* Pink */
}

.progress-indicator {
  fill: rgba(var(--v-theme-primary), 0.4);
}
</style>
