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

        <!-- Zone d'avancement -->
        <rect x="0" y="0" :width="progressIndicatorX + 2" :height="svgHeight" :fill="progressZoneColor" :opacity="progressZoneOpacity" />

        <!-- Indicateur de progression actuel -->
        <line
          :x1="progressIndicatorX + 2"
          :y1="0"
          :x2="progressIndicatorX + 2"
          :y2="svgHeight"
          stroke="currentColor"
          stroke-width="2"
        />

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

const progressZoneColor = ref('');
const progressZoneOpacity = ref(0.1);
const pauseColor = ref('white');
const pauseLength = ref(12);
const flytoColor = ref('orange');
const flytoLength = ref(12);

onMounted(async () => {
  progressZoneColor.value = toHex(await getSettingValue('Edition/Graphe/couleurAvancementZone'));
  progressZoneOpacity.value = await getSettingValue('Edition/Graphe/opaciteAvancementZone');

  pauseColor.value = toHex(await getSettingValue('Edition/Evenements/couleurPause'));
  pauseLength.value = await getSettingValue('Edition/Evenements/longueurPause');

  flytoColor.value = toHex(await getSettingValue('Edition/Evenements/Flyto/couleur'));
  flytoLength.value = await getSettingValue('Edition/Evenements/Flyto/longueur');
});

const props = defineProps({
  trackingPoints: { type: Array, required: true },
  totalLength: { type: Number, required: true },
  currentDistance: { type: Number, required: true },
  pauseEvents: { type: Array, default: () => [] },
  flytoEvents: { type: Array, default: () => [] },
});

const handleGraphClick = (event) => {
  const svgRect = event.currentTarget.getBoundingClientRect();
  const x = event.clientX - svgRect.left;
  
  const clickedKm = x / kmToPx;
  emit('seek-distance', clickedKm);
};

const scrollContainer = ref(null);
const svgHeight = 400; // Fixed height for the graph container
const graphCenterY = svgHeight / 2; // Central Y-axis for markers, not directly used by messages

const kmToPx = 30; // Scale: 30 pixels per kilometer

const svgWidth = computed(() => (props.totalLength * kmToPx) + 10);

const progressIndicatorX = computed(() => (props.currentDistance * kmToPx) - 1.5);

const isScrollable = computed(() => {
  const contentWidth = props.totalLength * kmToPx;
  const containerWidth = 1500; // Assuming a fixed container width for now, will be dynamic based on parent
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

</script>

<style scoped>
.graph-container-wrapper {
  flex-grow: 1;
  width: 0;
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
  overflow-x: hidden;
  overflow-y: hidden;
}

.graph-container.is-scrollable {
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
</style>