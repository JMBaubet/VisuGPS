<template>
  <div class="graph-container-wrapper">
    <div class="graph-controls">
      <v-checkbox v-model="showZoom" label="Zoom" color="#4CAF50" density="compact" hide-details></v-checkbox>
      <v-checkbox v-model="showPitch" label="Pitch" color="#2196F3" density="compact" hide-details></v-checkbox>
      <v-checkbox v-model="showBearingDelta" label="Δ Bearing (prev)" color="#FFC107" density="compact" hide-details></v-checkbox>
      <v-checkbox v-model="showBearingTotalDelta" label="Δ Bearing (total)" color="#E91E63" density="compact" hide-details></v-checkbox>
    </div>

    <div class="graph-container" ref="scrollContainer">
      <svg :width="svgWidth" :height="svgHeight" @click="handleGraphClick">
        <!-- Axe X de référence -->
        <line :x1="0" :y1="graphCenterY" :x2="svgWidth" :y2="graphCenterY" class="axis-line" />

        <!-- Repères sur l'axe X -->
        <g v-for="marker in xMarkers" :key="marker.label">
          <line :x1="marker.x" :y1="graphCenterY - 5" :x2="marker.x" :y2="graphCenterY + 5" class="marker-line" />
          <text :x="marker.x" :y="graphCenterY + 20" class="marker-text">{{ marker.label }}</text>
        </g>

        <!-- Graphique du Zoom -->
        <path v-if="showZoom" :d="zoomPath" class="zoom-path" />

        <!-- Graphique du Pitch -->
        <path v-if="showPitch" :d="pitchPath" class="pitch-path" />

        <!-- Graphique du Bearing (Delta vs Précédent) -->
        <path v-if="showBearingDelta" :d="bearingDeltaPath" class="bearing-delta-path" />

        <!-- Graphique du Bearing (Delta vs Début) -->
        <path v-if="showBearingTotalDelta" :d="bearingTotalDeltaPath" class="bearing-total-delta-path" />

        <!-- Indicateur de progression -->
        <rect :x="progressIndicatorX" y="0" width="3" :height="svgHeight" class="progress-indicator" />
      </svg>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const emit = defineEmits(['seek-distance']);

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

const handleGraphClick = (event) => {
  // Get click position relative to the SVG element itself
  const svgRect = event.currentTarget.getBoundingClientRect();
  const x = event.clientX - svgRect.left;
  
  const clickedKm = x / kmToPx;
  emit('seek-distance', clickedKm);
};

// Visibility states
const showZoom = ref(true);
const showPitch = ref(true);
const showBearingDelta = ref(true);
const showBearingTotalDelta = ref(true);

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

.graph-controls {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 1010; /* Above the graph svg */
  background-color: rgba(var(--v-theme-surface), 0.7);
  backdrop-filter: blur(2px);
  border-radius: 6px;
  padding: 4px 12px;
}

.graph-controls .v-checkbox {
  font-size: 0.8rem;
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
