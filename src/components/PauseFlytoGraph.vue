<template>
  <div v-if="tooltipVisible" class="custom-tooltip" :style="{ left: tooltipX + 'px', top: tooltipY + 'px' }">
    <div v-html="tooltipText"></div>
  </div>
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
        <g v-for="(point, index) in processedPauseMarkers" :key="`pause-${index}`">
          <line
            :x1="point.x"
            :y1="graphCenterY - 3"
            :x2="point.x"
            :y2="graphCenterY - 3 - pauseLength"
            :stroke="pauseColor"
            stroke-width="3"
          />
        </g>

        <!-- Marqueurs Flyto -->
        <g v-for="(point, index) in processedFlytoMarkers" :key="`flyto-${index}`" 
           @click.stop="emit('verify-flyto', point.eventData)" 
           style="cursor: pointer">
          <line
            @mouseenter="showCustomTooltip(point, $event)"
            @mouseleave="hideCustomTooltip()"
            :x1="point.x"
            :y1="graphCenterY - 3" 
            :x2="point.x"
            :y2="graphCenterY - 3 - flytoLength"
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
import * as turf from '@turf/turf';

const emit = defineEmits(['seek-distance', 'verify-flyto']);

const { getSettingValue } = useSettings();
const { toHex } = useVuetifyColors();

const progressZoneColor = ref('');
const progressZoneOpacity = ref(0.1);
const pauseColor = ref('purple');
const pauseLength = ref(12);
const flytoColor = ref('orange');
const flytoLength = ref(20);

// Tooltip state
const tooltipVisible = ref(false);
const tooltipText = ref('');
const tooltipX = ref(0);
const tooltipY = ref(0);

const showCustomTooltip = (pointData, mouseEvent) => {
  tooltipText.value = pointData.tooltipContent;
  tooltipX.value = mouseEvent.clientX + 10;
  tooltipY.value = mouseEvent.clientY - 10;
  tooltipVisible.value = true;
};

const hideCustomTooltip = () => {
  tooltipVisible.value = false;
};

onMounted(async () => {
  try {
    const pColor = await getSettingValue('Edition/Pause et Survol/Graphe Pause et Survol/couleurPause');
    progressZoneColor.value = toHex(await getSettingValue('Edition/Avancement dans les graphes/couleurAvancementZone'));
    progressZoneOpacity.value = await getSettingValue('Edition/Avancement dans les graphes/opaciteAvancementZone');

    pauseColor.value = toHex(pColor || 'purple');
    pauseLength.value = await getSettingValue('Edition/Pause et Survol/Graphe Pause et Survol/longueur') || 12;

    flytoColor.value = toHex(await getSettingValue('Edition/Pause et Survol/Graphe Pause et Survol/couleurFlyTo') || 'orange');
    flytoLength.value = await getSettingValue('Edition/Pause et Survol/Graphe Pause et Survol/longueur') || 20;
  } catch (e) {
    console.error('Error in PauseFlytoGraph onMounted:', e);
  }
});

const props = defineProps({
  trackingPoints: { type: Array, required: true },
  totalLength: { type: Number, required: true },
  currentDistance: { type: Number, required: true },
  pointEventsData: { type: Object, default: () => ({}) }, // Changed prop
});

const handleGraphClick = (event) => {
  const svgRect = event.currentTarget.getBoundingClientRect();
  const x = event.clientX - svgRect.left;
  
  const clickedKm = (x - startOffsetPx) / kmToPx;
  emit('seek-distance', clickedKm);
};

const scrollContainer = ref(null);
const svgHeight = 50; 
const graphCenterY = 25;

const kmToPx = 30; // Scale: 30 pixels per kilometer
const startOffsetKm = 0.2;
const startOffsetPx = startOffsetKm * kmToPx;

const svgWidth = computed(() => (props.totalLength * kmToPx) + startOffsetPx + 10);

const progressIndicatorX = computed(() => ((props.currentDistance * kmToPx) + startOffsetPx) - 1.5);

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

// Auto-scroll logic
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

const processedPauseMarkers = computed(() => {
  if (!props.pointEventsData || !props.trackingPoints.length) return [];
  
  const markers = [];
  for (const [incrementStr, events] of Object.entries(props.pointEventsData)) {
    const increment = parseInt(incrementStr);
    if (events.some(e => e.type === 'Pause')) {
      const point = props.trackingPoints.find(p => p.increment === increment);
      if (point) {
        markers.push({
          x: (point.distance * kmToPx) + startOffsetPx,
        });
      } else {
         console.warn(`[PauseFlytoGraph] Pause event at increment ${increment} but no matching tracking point found.`);
      }
    }
  }
  return markers;
});

const processedFlytoMarkers = computed(() => {
  if (!props.pointEventsData || !props.trackingPoints.length) return [];

  const markers = [];
  for (const [incrementStr, events] of Object.entries(props.pointEventsData)) {
    const increment = parseInt(incrementStr);
    const flytoEvent = events.find(e => e.type === 'Flyto');
    
    if (flytoEvent) {
      const point = props.trackingPoints.find(p => p.increment === increment);
      if (point) {
        // Validate coordinates
        const pLon = Number(point.coordonnee?.[0]);
        const pLat = Number(point.coordonnee?.[1]);
        const tLon = flytoEvent.data?.coord?.[0];
        const tLat = flytoEvent.data?.coord?.[1];

        if (isNaN(pLon) || isNaN(pLat) || isNaN(Number(tLon)) || isNaN(Number(tLat))) {
            console.warn(`[PauseFlytoGraph] Invalid coordinates for Flyto at increment ${increment}`);
            continue;
        }

        // Calculate distance and bearing
        const from = turf.point([pLon, pLat]);
        const to = turf.point([Number(tLon), Number(tLat)]);
        
        const distance = turf.distance(from, to, { units: 'kilometers' });
        const bearing = turf.bearing(from, to);
        
        // Format tooltip content
        const tooltipContent = `
          <div><strong>Distance:</strong> ${distance.toFixed(2)} km</div>
          <div><strong>Cap:</strong> ${bearing.toFixed(1)}°</div>
          <div><strong>Zoom:</strong> ${flytoEvent.data.zoom}</div>
          <div><strong>Pitch:</strong> ${flytoEvent.data.pitch}°</div>
        `;

        markers.push({
          x: (point.distance * kmToPx) + startOffsetPx,
          tooltipContent: tooltipContent,
          eventData: flytoEvent // Store the full event data
        });
      }
    }
  }
  return markers;
});

</script>

<style scoped>
.graph-container-wrapper {
  flex-grow: 1;
  width: 0;
  height: 50px; /* Match svgHeight */
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

.custom-tooltip {
  position: fixed;
  padding: 8px 12px;
  background-color: rgba(0, 0, 0, 0.9);
  color: white;
  border-radius: 4px;
  font-size: 0.85rem;
  pointer-events: none;
  z-index: 10000;
  transform: translate(0, -100%); /* Position above cursor */
  white-space: nowrap;
  box-shadow: 0 2px 4px rgba(0,0,0,0.2);
  border: 1px solid rgba(255,255,255,0.1);
}
</style>