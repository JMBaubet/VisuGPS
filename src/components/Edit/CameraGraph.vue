<template>
  <div class="graph-container-wrapper" @wheel.prevent>
    <div class="graph-container" :class="{ 'is-scrollable': isScrollable }" ref="scrollContainer">
      <svg :width="svgWidth" :height="svgHeight" @click="handleGraphClick">
        <!-- Axe X de référence -->
        <line :x1="0" :y1="graphCenterY" :x2="svgWidth" :y2="graphCenterY" class="axis-line" />

        <!-- Repères sur l'axe X -->
        <g v-for="marker in xMarkers" :key="marker.label">
          <line :x1="marker.x" :y1="graphCenterY - 5" :x2="marker.x" :y2="graphCenterY + 5" class="marker-line" />
          <text :x="marker.x" :y="graphCenterY + 20" class="marker-text" :style="{ textAnchor: marker.anchor }">{{ marker.label }}</text>
        </g>

        <!-- Zone d'avancement -->
        <rect x="0" y="0" :width="progressIndicatorX + 2" :height="svgHeight" :fill="progressZoneColor" :opacity="progressZoneOpacity" />

        <!-- Graphiques des données éditées (drawn first, to be in the background) -->
        <path v-if="props.showEditedZoom" :d="editedZoomPath" :style="{ stroke: editedZoomColor }" />
        <path v-if="props.showEditedPitch" :d="editedPitchPath" :style="{ stroke: editedPitchColor }" />
        <path v-if="props.showEditedBearingDelta" :d="editedBearingDeltaPath" :style="{ stroke: editedBearingDeltaColor }" />
        <path v-if="props.showEditedBearingTotalDelta" :d="editedBearingTotalDeltaPath" :style="{ stroke: editedBearingTotalDeltaColor }" />



        <!-- Indicateur de pitch actuel -->
        <line
          :x1="progressIndicatorX + 2 - 13"
          :y1="currentPitchDeltaY"
          :x2="progressIndicatorX + 2 + 13"
          :y2="currentPitchDeltaY"
          :stroke="pitchColor"
          stroke-width="2"
        />

        <!-- Indicateur de zoom actuel -->
        <line
          :x1="progressIndicatorX + 2 - 9"
          :y1="currentZoomDeltaY"
          :x2="progressIndicatorX + 2 + 9"
          :y2="currentZoomDeltaY"
          :stroke="zoomColor"
          stroke-width="2"
        />

        <!-- Graphique du Bearing (Delta vs Précédent) -->
        <path v-if="props.showBearingDelta" :d="bearingDeltaPath" :style="{ stroke: bearingDeltaColor }" />

        <!-- Graphique du Bearing (Delta vs Début) -->
        <path v-if="props.showBearingTotalDelta" :d="bearingTotalDeltaPath" :style="{ stroke: bearingTotalDeltaColor }" />

        <!-- Indicateur de bearing actuel -->
        <line
          :x1="progressIndicatorX + 2 - 5"
          :y1="currentBearingDeltaY"
          :x2="progressIndicatorX + 2 + 5"
          :y2="currentBearingDeltaY"
          :stroke="bearingTotalDeltaColor"
          stroke-width="2"
        />

        <!-- Points de contrôle -->
        <g v-for="point in controlPoints" :key="`cp-${point.increment}`">
          <line
            :x1="(point.distance * kmToPx) + startOffsetPx"
            :y1="0"
            :x2="(point.distance * kmToPx) + startOffsetPx"
            :y2="controlPointLength"
            :stroke="controlPointColor"
            :stroke-width="controlPointThickness"
          />
        </g>

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
const progressZoneColor = ref('');
const progressZoneOpacity = ref(0.1);
const controlPointColor = ref('');
const controlPointThickness = ref(3);
const controlPointLength = ref(20);

const editedZoomColor = ref('');
const editedPitchColor = ref('');
const editedBearingDeltaColor = ref('');
const editedBearingTotalDeltaColor = ref('');

// Graph scaling constants
const kmToPx = 30;
const startOffsetKm = 0.2;
const startOffsetPx = startOffsetKm * kmToPx;
const zoomToPx = 1 / 0.1;
const pitchToPx = 1;
const bearingDeltaToPx = 3;
const bearingTotalDeltaToPx = 1;

onMounted(async () => {
  zoomColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurZoom'));
  pitchColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurPitch'));
  bearingDeltaColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurBearingDelta'));
  bearingTotalDeltaColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurBearingTotalDelta'));
  progressZoneColor.value = toHex(await getSettingValue('Edition/Avancement dans les graphes/couleurAvancementZone'));
  progressZoneOpacity.value = await getSettingValue('Edition/Avancement dans les graphes/opaciteAvancementZone');
  controlPointColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/couleurPointDeControle'));
  controlPointThickness.value = await getSettingValue('Edition/Camera/Graphe caméra/epaisseurPointDeControle');
  controlPointLength.value = await getSettingValue('Edition/Camera/Graphe caméra/longueurPointDeControle');

  editedZoomColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurEditedZoom'));
  editedPitchColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurEditedPitch'));
  editedBearingDeltaColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurEditedBearingDelta'));
  editedBearingTotalDeltaColor.value = toHex(await getSettingValue('Edition/Camera/Graphe caméra/Couleur courbes/couleurEditedBearingTotalDelta'));
});

const props = defineProps({
  trackingPoints: { type: Array, required: true },
  totalLength: { type: Number, required: true },
  currentDistance: { type: Number, required: true },
  showBearingDelta: { type: Boolean, default: true },
  showBearingTotalDelta: { type: Boolean, default: true },
  showEditedZoom: { type: Boolean, default: false },
  showEditedPitch: { type: Boolean, default: false },
  showEditedBearingDelta: { type: Boolean, default: false },
  showEditedBearingTotalDelta: { type: Boolean, default: false },
  currentCameraBearing: { type: Number, default: 0 },
  initialCameraBearing: { type: Number, default: 0 },
  currentCameraZoom: { type: Number, default: 0 },
  defaultCameraZoom: { type: Number, default: 0 },
  currentCameraPitch: { type: Number, default: 0 },
  defaultCameraPitch: { type: Number, default: 0 },
});

const controlPoints = computed(() => {
  return props.trackingPoints.filter(p => p.pointDeControl);
});

const handleGraphClick = (event) => {
  const svgRect = event.currentTarget.getBoundingClientRect();
  const x = event.clientX - svgRect.left;
  
  const clickedKm = (x - startOffsetPx) / kmToPx;
  emit('seek-distance', clickedKm);
};

const scrollContainer = ref(null);
const svgHeight = 400;
const graphCenterY = svgHeight / 2;

const svgWidth = computed(() => (props.totalLength * kmToPx) + startOffsetPx + 10);

const progressIndicatorX = computed(() => ((props.currentDistance * kmToPx) + startOffsetPx) - 1.5);

const currentBearingDeltaY = computed(() => {
  let delta = props.currentCameraBearing - props.initialCameraBearing;
  if (delta > 180) delta -= 360;
  if (delta < -180) delta += 360;
  return graphCenterY - (delta * bearingTotalDeltaToPx);
});

const currentZoomDeltaY = computed(() => {
  return graphCenterY - ((props.currentCameraZoom - props.defaultCameraZoom) * zoomToPx);
});

const currentPitchDeltaY = computed(() => {
  return graphCenterY - ((props.currentCameraPitch - props.defaultCameraPitch) * pitchToPx);
});

const isScrollable = computed(() => {
  const contentWidth = props.totalLength * kmToPx;
  const containerWidth = 1500;
  return contentWidth > containerWidth;
});

const xMarkers = computed(() => {
  const markers = [];
  const intervalKm = 10;
  if (props.totalLength < intervalKm) return [];

  const markerCount = Math.floor(props.totalLength / intervalKm);
  for (let i = 0; i <= markerCount; i++) {
    const distance = i * intervalKm;
    markers.push({
      x: (distance * kmToPx) + startOffsetPx,
      label: `${distance}km`,
      anchor: distance === 0 ? 'start' : 'middle'
    });
  }
  return markers;
});

watch(() => props.currentDistance, (newDistance) => {
  if (!scrollContainer.value) return;

  const indicatorMarginKm = 10;
  const endBufferKm = 40;

  const scrollWidth = scrollContainer.value.scrollWidth;
  const clientWidth = scrollContainer.value.clientWidth;
  const maxScrollLeft = scrollWidth - clientWidth;

  let targetScrollLeft = ((newDistance - indicatorMarginKm) * kmToPx) + startOffsetPx;

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

const zoomPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultZoom = props.trackingPoints[0]?.zoom || 16;
  return props.trackingPoints.map((p, i) => {
    const x = (p.distance * kmToPx) + startOffsetPx;
    const y = graphCenterY - ((p.zoom - defaultZoom) * zoomToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const pitchPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultPitch = props.trackingPoints[0]?.pitch || 60;
  return props.trackingPoints.map((p, i) => {
    const x = (p.distance * kmToPx) + startOffsetPx;
    const y = graphCenterY - ((p.pitch - defaultPitch) * pitchToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const bearingDeltaPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  let lastBearing = props.trackingPoints[0]?.cap || 0;
  return props.trackingPoints.map((p, i) => {
    const x = (p.distance * kmToPx) + startOffsetPx;
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
    const x = (p.distance * kmToPx) + startOffsetPx;
    let delta = p.cap - initialBearing;
    if (delta > 180) delta -= 360;
    if (delta < -180) delta += 360;

    const y = graphCenterY - (delta * bearingTotalDeltaToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const editedZoomPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultZoom = props.trackingPoints[0]?.zoom || 16;
  return props.trackingPoints.map((p, i) => {
    const x = (p.distance * kmToPx) + startOffsetPx;
    const y = graphCenterY - ((p.editedZoom - defaultZoom) * zoomToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const editedPitchPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultPitch = props.trackingPoints[0]?.pitch || 60;
  return props.trackingPoints.map((p, i) => {
    const x = (p.distance * kmToPx) + startOffsetPx;
    const y = graphCenterY - ((p.editedPitch - defaultPitch) * pitchToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const editedBearingDeltaPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  let lastBearing = props.trackingPoints[0]?.editedCap || 0;
  return props.trackingPoints.map((p, i) => {
    const x = (p.distance * kmToPx) + startOffsetPx;
    let delta = p.editedCap - lastBearing;
    if (delta > 180) delta -= 360;
    if (delta < -180) delta += 360;
    
    const y = graphCenterY - (delta * bearingDeltaToPx);
    lastBearing = p.editedCap;
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const editedBearingTotalDeltaPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const initialBearing = props.trackingPoints[0]?.editedCap || 0;
  return props.trackingPoints.map((p, i) => {
    const x = (p.distance * kmToPx) + startOffsetPx;
    let delta = p.editedCap - initialBearing;
    if (delta > 180) delta -= 360;
    if (delta < -180) delta += 360;

    const y = graphCenterY - (delta * bearingTotalDeltaToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

</script>

<style scoped>
.graph-container-wrapper {
  flex-grow: 1;
  width: 0; /* Allow the container to shrink and not base its size on its content */
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