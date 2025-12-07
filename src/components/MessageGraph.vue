<template>
  <div v-if="tooltipVisible" class="custom-tooltip" :style="{ left: tooltipX + 'px', top: tooltipY + 'px' }">
    {{ tooltipText }}
  </div>
  <div class="graph-container-wrapper" @wheel.prevent :style="{ height: dynamicSvgHeight + 'px' }">
    <div class="graph-container" :class="{ 'is-scrollable': isScrollable }" ref="scrollContainer" :style="{ height: dynamicSvgHeight + 'px' }">
      <svg :width="svgWidth" :height="dynamicSvgHeight" @click="handleGraphClick">
        <!-- Axe X de référence -->
        <line :x1="0" :y1="xAxisY" :x2="svgWidth" :y2="xAxisY" class="axis-line" />

        <!-- Repères sur l'axe X -->
        <g v-for="marker in xMarkers" :key="marker.label">
          <line :x1="marker.x" :y1="xAxisY - 5" :x2="marker.x" :y2="xAxisY + 5" class="marker-line" />
          <text :x="marker.x" :y="xAxisY + 20" class="marker-text" :style="{ textAnchor: marker.anchor }">{{ marker.label }}</text>
        </g>

        <!-- Zone d'avancement -->
        <rect x="0" y="0" :width="progressIndicatorX + 2" :height="xAxisY" :fill="progressZoneColor" :opacity="progressZoneOpacity" />

        <!-- Range Events (Messages) -->
        <g v-for="(event, index) in processedRangeEvents" :key="`re-${index}`">
          <rect
            @mouseenter="showCustomTooltip(event, $event)"
            @mouseleave="hideCustomTooltip()"
            :x="event.x"
            :y="event.y"
            :width="event.width"
            :height="event.height"
            :fill="event.fillColor"
            :stroke="event.strokeColor"
            :stroke-width="1"
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

        <!-- Indicateur de progression actuel -->
        <line
          :x1="progressIndicatorX + 2"
          :y1="0"
          :x2="progressIndicatorX + 2"
          :y2="dynamicSvgHeight"
          stroke="currentColor"
          stroke-width="2"
        />
      </svg>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import { useTheme } from 'vuetify';

const emit = defineEmits(['seek-distance']);

const { getSettingValue } = useSettings();
const { toHex } = useVuetifyColors();
const theme = useTheme();

const tooltipVisible = ref(false);
const tooltipText = ref('');
const tooltipX = ref(0);
const tooltipY = ref(0);
const currentMaxLanes = ref(1); // New ref to track the maximum number of lanes needed, initialized to 1

const showCustomTooltip = (eventData, mouseEvent) => {
  tooltipText.value = eventData.messageText;
  tooltipX.value = mouseEvent.clientX + 10; // Offset by 10px from mouse cursor
  tooltipY.value = mouseEvent.clientY - 10;
  tooltipVisible.value = true;
};

const hideCustomTooltip = () => {
  tooltipVisible.value = false;
};

const progressZoneColor = ref('');
const progressZoneOpacity = ref(0.1);

onMounted(async () => {
  progressZoneColor.value = toHex(await getSettingValue('Edition/Graphe/couleurAvancementZone'));
  progressZoneOpacity.value = await getSettingValue('Edition/Graphe/opaciteAvancementZone');
});

const props = defineProps({
  trackingPoints: { type: Array, required: true },
  totalLength: { type: Number, required: true },
  currentDistance: { type: Number, required: true },
  rangeEvents: { type: Array, default: () => [] },
  messageGraphHeight: { type: Number, default: 10 },
  messageLibrary: { type: Array, default: () => [] },
});

const handleGraphClick = (event) => {
  const svgRect = event.currentTarget.getBoundingClientRect();
  const x = event.clientX - svgRect.left;
  
  const clickedKm = (x - startOffsetPx) / kmToPx;
  emit('seek-distance', clickedKm);
};

const scrollContainer = ref(null);
const xAxisMargin = 30; // Margin from bottom for X-axis
const dynamicSvgHeight = computed(() => {
  const minContentHeight = 100; // Minimum height needed for messages
  const messageAreaHeight = currentMaxLanes.value * (props.messageGraphHeight + verticalGap);
  const totalHeight = messageAreaHeight + xAxisMargin + 20; // +20 for markers labels
  return Math.min(svgContainerHeight, Math.max(minContentHeight, totalHeight)); // Capped by container height
});

const xAxisY = computed(() => dynamicSvgHeight.value - xAxisMargin); // Position X-axis relative to dynamicSvgHeight

const kmToPx = 30; // Scale: 30 pixels per kilometer
const startOffsetKm = 0.2;
const startOffsetPx = startOffsetKm * kmToPx;
const verticalGap = 3; // Gap between message lanes
const svgContainerHeight = 400; // Fixed height for the graph container wrapper

const svgWidth = computed(() => (props.totalLength * kmToPx) + startOffsetPx + 10);

const progressIndicatorX = computed(() => ((props.currentDistance * kmToPx) + startOffsetPx) - 1.5);

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

const processedRangeEvents = computed(() => {
  if (!props.rangeEvents || props.rangeEvents.length === 0 || props.trackingPoints.length === 0) {
    return [];
  }

  const trackingMap = new Map(props.trackingPoints.map(p => [p.increment, p.distance]));

  // --- Initial processing to add coordinates to events ---
  const validEventsWithCoords = props.rangeEvents.map(event => {
    const startIncrement = event.startIncrement;
    const endIncrement = event.endIncrement;
    const anchorIncrement = event.anchorIncrement;

    if (!trackingMap.has(startIncrement) || !trackingMap.has(endIncrement) || !trackingMap.has(anchorIncrement)) {
      console.warn(`[MessageGraph] Event with missing increment in trackingPoints. Skipping event:`, event);
      return null;
    }

    let message = null;
    if (event.message) {
      message = event.message;
    } else {
      message = props.messageLibrary.find(msg => msg.id === event.messageId);
    }
    
    // If no message data can be found, skip the event
    if (!message) {
        console.warn(`[MessageGraph] No message data found for event. Skipping:`, event);
        return null;
    }

    const startDistance = trackingMap.get(startIncrement);
    const endDistance = trackingMap.get(endIncrement);
    const anchorDistance = trackingMap.get(anchorIncrement);

    return {
      ...event,
      message, // Embed the resolved message
      startX: (startDistance * kmToPx) + startOffsetPx,
      endX: (endDistance * kmToPx) + startOffsetPx,
      anchorX: (anchorDistance * kmToPx) + startOffsetPx,
    };
  }).filter(Boolean);
  
  validEventsWithCoords.sort((a, b) => a.startX - b.startX);

  // --- Partition events into KM and Standard ---
  const kmEvents = validEventsWithCoords.filter(e => e.message?.source?.startsWith('distance_markers'));
  const standardEvents = validEventsWithCoords.filter(e => !e.message?.source?.startsWith('distance_markers'));

  // --- Lane assignment function ---
  const assignLanes = (events) => {
    const lanes = []; // Stores the end coordinate of the last event in a lane
    const processed = events.map(event => {
      let assignedLane = -1;
      for (let i = 0; i < lanes.length; i++) {
        if (event.startX >= lanes[i]) {
          assignedLane = i;
          lanes[i] = event.endX;
          break;
        }
      }
      if (assignedLane === -1) {
        assignedLane = lanes.length;
        lanes.push(event.endX);
      }
      return { ...event, lane: assignedLane };
    });
    return { processed, laneCount: lanes.length };
  };
  
  // --- Process each group to get lanes ---
  const { processed: processedKms, laneCount: kmLaneCount } = assignLanes(kmEvents);
  const { processed: processedStandards, laneCount: standardLaneCount } = assignLanes(standardEvents);

  currentMaxLanes.value = kmLaneCount + standardLaneCount;

  // --- Combine and calculate final positions ---
  const finalStandardEvents = processedStandards.map(e => ({ ...e, lane: e.lane + kmLaneCount }));
  const combinedEvents = [...processedKms, ...finalStandardEvents];

  return combinedEvents.map(event => {
    const fillColor = event.message?.style ? toHex(event.message.style.backgroundColor) : 'grey';
    const strokeColor = event.message?.style ? event.message.style.textColor : (theme.global.current.value.dark ? '#FFFFFF' : '#000000');
    
    const finalY = xAxisY.value - ((event.lane + 1) * props.messageGraphHeight) - (event.lane * verticalGap) - 3;

    return {
      ...event,
      messageText: event.message ? event.message.text : (event.messageId || ''),
      x: event.startX,
      y: finalY,
      width: Math.max(1, event.endX - event.startX), // Ensure width is at least 1px
      height: props.messageGraphHeight,
      fillColor: fillColor,
      strokeColor: strokeColor,
    };
  });
});
</script>

<style scoped>
.graph-container-wrapper {
  flex-grow: 1;
  width: 0;
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
  background-color: rgba(0, 0, 0, 0.8);
  color: white;
  border-radius: 4px;
  font-size: 0.85rem;
  pointer-events: none; /* Allows mouse events to pass through */
  z-index: 10000; /* Ensure it's on top */
  transform: translate(-50%, -100%); /* Position above cursor */
  white-space: nowrap; /* Prevent text wrapping */
}

</style>