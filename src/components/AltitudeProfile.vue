<template>
  <div class="altitude-profile-container" ref="container" :style="containerStyle">
    <div class="graph-wrapper" :style="{ transform: `translateX(${translateX}px)` }">
      <svg v-if="profileData.length" :width="svgWidth" :height="svgHeight">
        <defs>
          <linearGradient :id="gradientId" x1="0" x2="0" y1="0" y2="1">
            <stop offset="0%" :stop-color="gradientStartColor" stop-opacity="0.5"/>
            <stop offset="100%" :stop-color="gradientEndColor" stop-opacity="0.0"/>
          </linearGradient>
        </defs>
        <path :d="pathData" class="altitude-path" :fill="`url(#${gradientId})`" stroke="rgb(var(--v-theme-primary))" stroke-width="2" />


      </svg>
    </div>
    <div class="cursor-overlay" :style="{ left: `${cursorLeftPosition}px`, backgroundColor: cursorColor }"></div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import * as turf from '@turf/turf';
import { useTheme } from 'vuetify';
import { useSettings } from '@/composables/useSettings';

const props = defineProps({
  lineString: {
    type: Object,
    required: true,
  },
  currentDistanceKm: {
    type: Number,
    required: true,
  },
});

const emit = defineEmits(['update:height']);

const { current: theme } = useTheme();
const { getSettingValue } = useSettings();
const container = ref(null);
const profileData = ref([]);
const minAltitude = ref(0);
const maxAltitude = ref(0);
const totalDistanceKm = ref(0);
const minAltitudeDistanceKm = ref(0);
const viewportWidth = ref(0);
let resizeObserver = null;

const PIXELS_PER_KM = 20;
const PIXELS_PER_10M_ALT = 2;
const CURSOR_OFFSET_KM = 10;
const SVG_PADDING_X = 10; // New constant for horizontal padding inside SVG
const SVG_PADDING_BOTTOM = 5; // New constant for bottom padding inside SVG

// --- Lifecycle Hooks for viewport detection ---
onMounted(() => {
  if (container.value && container.value.parentElement) {
    resizeObserver = new ResizeObserver(entries => {
      if (entries[0]) {
        viewportWidth.value = entries[0].contentRect.width;
      }
    });
    resizeObserver.observe(container.value.parentElement);
  }
});

onUnmounted(() => {
  if (resizeObserver) {
    resizeObserver.disconnect();
  }
});

// --- Computed properties ---
const gradientId = computed(() => `altitude-gradient-${Math.random().toString(36).substring(7)}`);
const gradientStartColor = computed(() => theme.value.colors.primary);
const gradientEndColor = computed(() => theme.value.colors.primary);

const svgWidth = computed(() => totalDistanceKm.value * PIXELS_PER_KM + 2 * SVG_PADDING_X);
const svgHeight = computed(() => {
  const altitudeRange = maxAltitude.value - minAltitude.value;
  if (altitudeRange <= 0) return 50;
  return (altitudeRange / 10) * PIXELS_PER_10M_ALT + 10 + SVG_PADDING_BOTTOM; // +10 for top padding, +SVG_PADDING_BOTTOM
});

const availableWidth = computed(() => viewportWidth.value - 40); // 20px margins
const isScrollable = computed(() => svgWidth.value > availableWidth.value);

const containerStyle = computed(() => {
    const style = { height: `${svgHeight.value}px` };
    if (svgWidth.value > 0 && !isScrollable.value) {
        style.width = `${svgWidth.value}px`;
        style.left = '50%';
        style.right = 'auto';
        style.transform = 'translateX(-50%)';
    }
    return style;
});

const translateX = computed(() => {
  if (!isScrollable.value) return 0;

  const containerWidth = availableWidth.value;
  const contentWidth = svgWidth.value;

  // Phase 1: Cursor moves, graph fixed at left
  if (props.currentDistanceKm < CURSOR_OFFSET_KM) {
    return 0;
  }

  // Phase 2: Cursor fixed, graph scrolls
  const cursorScreenPosition = CURSOR_OFFSET_KM * PIXELS_PER_KM + SVG_PADDING_X;
  const cursorSvgPosition = props.currentDistanceKm * PIXELS_PER_KM + SVG_PADDING_X;
  let calculatedTranslateX = cursorScreenPosition - cursorSvgPosition;

  // Phase 3: End of track - clamp the scroll
  const minTranslateX = containerWidth - contentWidth;
  const maxTranslateX = 0;

  return Math.max(minTranslateX, Math.min(maxTranslateX, calculatedTranslateX));
});

const cursorLeftPosition = computed(() => {
  if (!isScrollable.value) {
    return props.currentDistanceKm * PIXELS_PER_KM + SVG_PADDING_X;
  }

  const containerWidth = availableWidth.value;
  const contentWidth = svgWidth.value;
  const currentTranslateX = translateX.value; // Use the clamped translateX

  if (currentTranslateX === containerWidth - contentWidth) {
    return props.currentDistanceKm * PIXELS_PER_KM + SVG_PADDING_X + currentTranslateX;
  }

  if (props.currentDistanceKm < CURSOR_OFFSET_KM) {
    return props.currentDistanceKm * PIXELS_PER_KM + SVG_PADDING_X;
  }
  return CURSOR_OFFSET_KM * PIXELS_PER_KM + SVG_PADDING_X;
});

const pathData = computed(() => {
  if (!profileData.value.length) return '';
  
  const pathParts = profileData.value.map((point, index) => {
    const x = point.distance * PIXELS_PER_KM + SVG_PADDING_X;
    const y = svgHeight.value - ((point.altitude - minAltitude.value) / 10) * PIXELS_PER_10M_ALT - 1 - SVG_PADDING_BOTTOM;
    return `${index === 0 ? 'M' : 'L'} ${x.toFixed(2)},${y.toFixed(2)}`;
  });

  const lastPointX = totalDistanceKm.value * PIXELS_PER_KM + SVG_PADDING_X;
  const bottomY = svgHeight.value - 1 - SVG_PADDING_BOTTOM;
  return `${pathParts.join(' ')} L ${lastPointX},${bottomY} L ${SVG_PADDING_X},${bottomY} Z`;
});

const cursorColor = computed(() => getSettingValue('Visualisation/Mapbox/Traces/couleurComete'));

// --- Watchers ---
watch(svgHeight, (newHeight) => {
  emit('update:height', newHeight);
});

watch(() => props.lineString, (newLineString) => {
  if (newLineString && newLineString.coordinates && newLineString.coordinates.length > 1) {
    let cumulativeDistance = 0;
    const data = [];
    let minAlt = Infinity;
    let maxAlt = -Infinity;
    let minAltDistance = 0;

    const points = newLineString.coordinates.map(coord => turf.point(coord));

    for (let i = 0; i < points.length; i++) {
      const altitude = points[i].geometry.coordinates[2] || 0;
      
      if (i > 0) {
        cumulativeDistance += turf.distance(points[i-1], points[i], { units: 'kilometers' });
      }

      if (altitude < minAlt) {
        minAlt = altitude;
        minAltDistance = cumulativeDistance;
      }
      maxAlt = Math.max(maxAlt, altitude);

      data.push({ distance: cumulativeDistance, altitude });
    }
    
    console.log(`Point le plus bas trouvé à ${minAltDistance.toFixed(3)} km | Altitude: ${minAlt.toFixed(2)} m`);

    minAltitudeDistanceKm.value = minAltDistance;
    minAltitude.value = minAlt;
    maxAltitude.value = maxAlt;
    totalDistanceKm.value = cumulativeDistance;
    profileData.value = data;
  }
}, { immediate: true, deep: true });


</script><style scoped>
.altitude-profile-container {
  position: absolute;
  bottom: 5px;
  left: 20px;
  right: 20px;
  background-color: rgba(var(--v-theme-surface), 0.85);
  backdrop-filter: blur(4px);
  /* Decorative styles */
  border: 1px solid rgba(var(--v-theme-on-surface), 0.2); /* Restored original border */
  border-bottom: none;
  border-top-left-radius: 8px;
  border-top-right-radius: 8px;
  overflow: hidden; /* Ensures content respects the rounded corners */

  z-index: 2;
  transition: height 0.3s ease, bottom 0.3s ease, left 0.3s ease, right 0.3s ease;
}

.graph-wrapper {
  height: 100%;
  transition: transform 100ms linear;
}

.cursor-overlay {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 1px;
  /* background-color: red; */ /* Removed, now dynamic */
  z-index: 2;
  transition: left 100ms linear;
}

.altitude-path {
  stroke: rgb(var(--v-theme-primary));
}
</style>
