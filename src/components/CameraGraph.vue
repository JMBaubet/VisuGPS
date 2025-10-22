<template>
  <div class="graph-container-wrapper" @wheel.prevent>
    <div class="graph-container" :class="{ 'is-scrollable': isScrollable }" ref="scrollContainer">
      <svg :width="svgWidth" :height="svgHeight" @click="handleGraphClick">
        <!-- Range Events (Messages) -->
        <g v-for="(event, index) in processedRangeEvents" :key="`re-${index}`">
          <rect
            :x="event.x"
            :y="event.y"
            :width="event.width"
            :height="event.height"
            :fill="event.fillColor"
            rx="2"
          />
          <line
            :x1="event.anchorX"
            :y1="event.y - 1"
            :x2="event.anchorX"
            :y2="event.y + event.height + 1"
            :stroke="event.strokeColor"
            stroke-width="3"
          />
        </g>

        <!-- Axe X de référence -->
        <line :x1="0" :y1="graphCenterY" :x2="svgWidth" :y2="graphCenterY" class="axis-line" />

        <!-- Repères sur l'axe X -->
        <g v-for="marker in xMarkers" :key="marker.label">
          <line :x1="marker.x" :y1="graphCenterY - 5" :x2="marker.x" :y2="graphCenterY + 5" class="marker-line" />
          <text :x="marker.x" :y="graphCenterY + 20" class="marker-text">{{ marker.label }}</text>
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
            :x1="point.distance * kmToPx"
            :y1="0"
            :x2="point.distance * kmToPx"
            :y2="controlPointLength"
            :stroke="controlPointColor"
            :stroke-width="controlPointThickness"
          />
        </g>

        <!-- Marqueurs de pause -->
        <g v-for="point in pausePoints" :key="`pause-${point.increment}`">
          <line
            :x1="point.distance * kmToPx"
            :y1="0"
            :x2="point.distance * kmToPx"
            :y2="pauseLength"
            :stroke="pauseColor"
            stroke-width="3"
          />
        </g>

        <!-- Marqueurs Flyto -->
        <g v-for="point in flytoPoints" :key="`flyto-${point.increment}`">
          <line
            :x1="point.distance * kmToPx"
            :y1="0"
            :x2="point.distance * kmToPx"
            :y2="flytoLength"
            :stroke="flytoColor"
            stroke-width="3"
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
const progressZoneOpacity = ref(0.1); // New ref for opacity
const controlPointColor = ref('');
const controlPointThickness = ref(3);
const controlPointLength = ref(20);

const pauseColor = ref('white');
const pauseLength = ref(12);

const flytoColor = ref('orange'); // Default, will be loaded from settings
const flytoLength = ref(12); // Default, will be loaded from settings

const editedZoomColor = ref('');
const editedPitchColor = ref('');
const editedBearingDeltaColor = ref('');
const editedBearingTotalDeltaColor = ref('');

onMounted(async () => {
  zoomColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurZoom'));
  pitchColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurPitch'));
  bearingDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurBearingDelta'));
  bearingTotalDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurBearingTotalDelta'));
  progressZoneColor.value = toHex(await getSettingValue('Edition/Graphe/couleurAvancementZone'));
  progressZoneOpacity.value = await getSettingValue('Edition/Graphe/opaciteAvancementZone');
  controlPointColor.value = toHex(await getSettingValue('Edition/Graphe/couleurPointDeControle'));
  controlPointThickness.value = await getSettingValue('Edition/Graphe/epaisseurPointDeControle');
  controlPointLength.value = await getSettingValue('Edition/Graphe/longueurPointDeControle');

  pauseColor.value = toHex(await getSettingValue('Edition/Evenements/couleurPause'));
  pauseLength.value = await getSettingValue('Edition/Evenements/longueurPause');

  flytoColor.value = toHex(await getSettingValue('Edition/Evenements/Flyto/couleur'));
  flytoLength.value = await getSettingValue('Edition/Evenements/Flyto/longueur');

  editedZoomColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedZoom'));
  editedPitchColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedPitch'));
  editedBearingDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedBearingDelta'));
  editedBearingTotalDeltaColor.value = toHex(await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedBearingTotalDelta'));
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
  currentCameraBearing: { type: Number, default: 0 }, // New prop for current camera bearing
  initialCameraBearing: { type: Number, default: 0 }, // New prop for initial bearing (km 0)
  currentCameraZoom: { type: Number, default: 0 },
  defaultCameraZoom: { type: Number, default: 0 },
  currentCameraPitch: { type: Number, default: 0 },
  defaultCameraPitch: { type: Number, default: 0 },
  pauseEvents: { type: Array, default: () => [] },
  flytoEvents: { type: Array, default: () => [] },
  rangeEvents: { type: Array, default: () => [] },
});

const controlPoints = computed(() => {
  return props.trackingPoints.filter(p => p.pointDeControl);
});

const pausePoints = computed(() => {
  if (!props.pauseEvents || props.pauseEvents.length === 0) return [];
  const pauseIncrements = new Set(props.pauseEvents);
  return props.trackingPoints.filter(p => pauseIncrements.has(p.increment));
});

const flytoPoints = computed(() => {
  if (!props.flytoEvents || props.flytoEvents.length === 0) return [];
  const flytoIncrements = new Set(props.flytoEvents);
  return props.trackingPoints.filter(p => flytoIncrements.has(p.increment));
});

const processedRangeEvents = computed(() => {
  if (!props.rangeEvents || props.rangeEvents.length === 0 || props.trackingPoints.length === 0) {
    return [];
  }

  const trackingMap = new Map(props.trackingPoints.map(p => [p.increment, p.distance]));

  const eventsWithCoords = props.rangeEvents.map(event => {
    // Use the start/end increments provided directly by the event data
    const startIncrement = event.startIncrement;
    const endIncrement = event.endIncrement;
    const anchorIncrement = event.anchorIncrement;

    const startDistance = trackingMap.get(startIncrement);
    const endDistance = trackingMap.get(endIncrement);
    const anchorDistance = trackingMap.get(anchorIncrement);

    if (startDistance === undefined || endDistance === undefined || anchorDistance === undefined) {
      return null; // Event is out of bounds or invalid
    }

    return {
      ...event,
      startX: startDistance * kmToPx,
      endX: endDistance * kmToPx,
      anchorX: anchorDistance * kmToPx,
    };
  }).filter(Boolean);

  // Sort events by startX
  eventsWithCoords.sort((a, b) => a.startX - b.startX);

  const lanes = []; // Each lane stores the endX of the last event
  const rectangleHeight = 3;
  const verticalGap = 3;
  const baseOffsetY = 5; // Offset from the bottom of the graph

  return eventsWithCoords.map(event => {
    let assignedLane = -1;

    // Find an available lane
    for (let i = 0; i < lanes.length; i++) {
      if (event.startX >= lanes[i]) {
        assignedLane = i;
        lanes[i] = event.endX;
        break;
      }
    }

    // If no lane was found, create a new one
    if (assignedLane === -1) {
      assignedLane = lanes.length;
      lanes.push(event.endX);
    }

    return {
      ...event,
      x: event.startX,
      y: svgHeight - baseOffsetY - (assignedLane * (rectangleHeight + verticalGap)),
      width: event.endX - event.startX,
      height: rectangleHeight,
      fillColor: toHex(event.backgroundColor),
      strokeColor: toHex(event.borderColor),
    };
  });
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

// --- Calcul des chemins SVG pour les données éditées ---
const editedZoomPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultZoom = props.trackingPoints[0]?.zoom || 16;
  return props.trackingPoints.map((p, i) => {
    const x = p.distance * kmToPx;
    const y = graphCenterY - ((p.editedZoom - defaultZoom) * zoomToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const editedPitchPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  const defaultPitch = props.trackingPoints[0]?.pitch || 60;
  return props.trackingPoints.map((p, i) => {
    const x = p.distance * kmToPx;
    const y = graphCenterY - ((p.editedPitch - defaultPitch) * pitchToPx);
    return (i === 0 ? 'M' : 'L') + `${x},${y}`;
  }).join(' ');
});

const editedBearingDeltaPath = computed(() => {
  if (props.trackingPoints.length < 2) return '';
  let lastBearing = props.trackingPoints[0]?.editedCap || 0;
  return props.trackingPoints.map((p, i) => {
    const x = p.distance * kmToPx;
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
    const x = p.distance * kmToPx;
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
