<template>
  <div class="altitude-graph-wrapper">
    <!-- Y-Axis Labels (Fixed) -->
    <div class="y-axis-labels-container" :style="{ height: svgHeight + 'px', width: props.padding.left + 'px' }">
      <div v-for="tick in yTicks" :key="tick.label" class="y-axis-label" :style="{ top: tick.y + 'px' }">
        {{ tick.label }}
      </div>
    </div>

    <!-- Scrolling SVG Container -->
    <div class="svg-container" ref="containerRef" @mousemove="handleMouseMove" @mouseleave="handleMouseLeave">
      <div
        v-if="tooltipVisible"
        class="tooltip"
        :style="{ top: tooltipPosition.y + 'px', left: tooltipPosition.x + 'px' }"
      >
        <div>{{ Math.round(tooltipData.distance / 100) / 10 }} km</div>
        <div>{{ Math.round(tooltipData.altitude) }} m</div>
        <div>{{ tooltipData.slope.toFixed(1) }} %</div>
      </div>
      <svg :viewBox="`0 0 ${viewBoxWidth} ${svgHeight}`" :height="svgHeight" :width="viewBoxWidth" preserveAspectRatio="xMinYMin meet">
        <g>
          <!-- Y-Axis Grid Lines -->
          <g class="y-axis-grid">
            <g v-for="tick in yTicks" :key="tick.label">
              <line :x1="0" :y1="tick.y" :x2="viewBoxWidth" :y2="tick.y"></line>
            </g>
          </g>

          <!-- Axe X (Grid lines and Labels) -->
          <g class="axis x-axis">
            <g v-for="tick in xTicks" :key="tick.value">
              <line :x1="tick.position" :y1="props.padding.top" :x2="tick.position" :y2="innerHeight + props.padding.top"></line>
              <text :x="tick.position" :y="svgHeight - 15">{{ tick.label }}</text>
            </g>
          </g>

          <!-- Area Segments -->
          <g class="areas">
            <path v-for="(segment, index) in pathSegments" :key="`area-${index}`" :d="segment.path" :fill="segment.color" fill-opacity="0.6"></path>
          </g>

          <!-- Line Segments -->
          <g class="lines">
              <path v-for="(segment, index) in pathSegments" :key="`line-${index}`" :d="segment.linePath" :stroke="segment.color" stroke-width="2" fill="none"></path>
          </g>

          <!-- Progress Bar -->
          <line class="progress-bar" :x1="progressX" y1="0" :x2="progressX" :y2="svgHeight - 20"></line>

          <!-- Hover Line -->
          <line
              v-if="tooltipVisible"
              class="hover-line"
              :x1="hoverLineX"
              y1="0"
              :x2="hoverLineX"
              :y2="svgHeight - 20"
          ></line>
        </g>
      </svg>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, computed } from 'vue';

import { invoke } from '@tauri-apps/api/core';

import { useSettings } from '@/composables/useSettings';

import { useVuetifyColors } from '@/composables/useVuetifyColors';

const props = defineProps({
    circuitId: { type: String, required: true },
    currentDistance: { type: Number, required: true },
    padding: {
        type: Object,
        default: () => ({ top: 10, right: 10, bottom: 30, left: 45 })
    }
});

// --- Refs ---
const containerRef = ref(null);
const viewBoxWidth = ref(1000);
const svgHeight = ref(210);
const progressX = ref(0);
const pathSegments = ref([]);
const xTicks = ref([]);
const yTicks = ref([]);
const totalDistance = ref(0);
let lastUpdatedDistance = 0;
const minAltitude = ref(0);
const maxAltitude = ref(0);
const zeroAltitudeY = ref(0);

const innerWidth = computed(() => viewBoxWidth.value - props.padding.left - props.padding.right);
const innerHeight = computed(() => svgHeight.value - props.padding.top - props.padding.bottom);

// --- Tooltip Refs ---
const tooltipVisible = ref(false);
const tooltipData = ref({ distance: 0, altitude: 0, slope: 0 });
const tooltipPosition = ref({ x: 0, y: 0 });
const hoverLineX = ref(0);
const dataPointsForTooltip = ref([]);

// --- Composables ---
const { getSettingValue } = useSettings();
const { toHex } = useVuetifyColors();

// --- Data Loading and Processing ---
async function processData() {
    if (!props.circuitId) return;
    try {
        const trackingData = await invoke('read_tracking_file', { circuitId: props.circuitId });
        if (trackingData.length < 2) return;

        // Filter trackingData to ensure valid altitudes
        const filteredTrackingData = trackingData.filter(p => typeof p.altitude === 'number' && !isNaN(p.altitude));

        if (filteredTrackingData.length < 2) {
            console.warn("AltitudeSVG: Not enough valid data points after filtering for altitude profile.");
            pathSegments.value = [];
            return;
        }

        const segmentLength = getSettingValue('Importation/Tracking/LongueurSegment') || 100;
        
        minAltitude.value = Math.min(...filteredTrackingData.map(p => p.altitude));
        maxAltitude.value = Math.max(...filteredTrackingData.map(p => p.altitude));
        
        const altitudeTickInterval = getSettingValue('Altitude/Visualisation/RepereAltitude') || 200;
        const graphMinY = Math.floor(minAltitude.value / altitudeTickInterval) * altitudeTickInterval;
        const graphMaxY = Math.ceil(maxAltitude.value / altitudeTickInterval) * altitudeTickInterval;
        
        const effectiveMinAltitude = graphMinY;
        const effectiveAltitudeSpan = graphMaxY - graphMinY === 0 ? 1 : graphMaxY - graphMinY;
        
        const pixelsFor10Meters = getSettingValue('Altitude/Visualisation/Ordonnee') || 10; // Default to 10 pixels for 10 meters
        const pixelsPerMeter = pixelsFor10Meters / 10;
        const graphDrawingHeight = effectiveAltitudeSpan * pixelsPerMeter;
        svgHeight.value = graphDrawingHeight + props.padding.top + props.padding.bottom;
        
        const dataPoints = filteredTrackingData.map((point, index) => {
            let slope = 0;
            if (index > 0) {
                const prevPoint = filteredTrackingData[index - 1]; // Use filtered data for prevPoint
                const altitudeChange = point.altitude - prevPoint.altitude;
                slope = (altitudeChange / segmentLength) * 100;
            }
            const distance = index * segmentLength;
            return { distance, altitude: point.altitude, slope };
        });
        
        totalDistance.value = dataPoints[dataPoints.length - 1].distance;
        dataPointsForTooltip.value = dataPoints;
        const scaleX = getSettingValue('Altitude/Visualisation/Abscisse') || 2;
        viewBoxWidth.value = (totalDistance.value / 100) * scaleX;
        
        const yScale = (alt) => graphDrawingHeight - ((alt - effectiveMinAltitude) / effectiveAltitudeSpan) * graphDrawingHeight + props.padding.top;
        
        zeroAltitudeY.value = yScale(0);
        
        const getSlopeColor = (slope) => {
            if (slope <= 0) return toHex(getSettingValue('Altitude/Couleurs/TrancheNegative') || 'light-blue');
            if (slope < 3) return toHex(getSettingValue('Altitude/Couleurs/Tranche1') || 'green');
            if (slope < 6) return toHex(getSettingValue('Altitude/Couleurs/Tranche2') || 'yellow');
            if (slope < 9) return toHex(getSettingValue('Altitude/Couleurs/Tranche3') || 'orange');
            if (slope < 12) return toHex(getSettingValue('Altitude/Couleurs/Tranche4') || 'red');
            return toHex(getSettingValue('Altitude/Couleurs/Tranche5') || 'purple');
        };
        
        const segments = [];
        const graphBottomY = svgHeight.value - props.padding.bottom;
        for (let i = 1; i < dataPoints.length; i++) {
            const p1 = dataPoints[i - 1];
            const p2 = dataPoints[i];
            const x1 = (p1.distance / totalDistance.value) * viewBoxWidth.value;
            const x2 = (p2.distance / totalDistance.value) * viewBoxWidth.value;
            const y1 = yScale(p1.altitude);
            const y2 = yScale(p2.altitude);

            segments.push({
                path: `M ${x1},${y1} L ${x2},${y2} L ${x2},${graphBottomY} L ${x1},${graphBottomY} Z`,
                linePath: `M ${x1},${y1} L ${x2},${y2}`,
                color: getSlopeColor(p2.slope)
            });
        }
        pathSegments.value = segments;
        
        yTicks.value = []; // Clear previous ticks
        for (let alt = graphMinY; alt <= graphMaxY; alt += altitudeTickInterval) {
            const y = yScale(alt);
            yTicks.value.push({ y, label: `${alt}m` });
        }

        const distanceTicks = [];
        const tickInterval = (getSettingValue('Altitude/Visualisation/RepereDistance') || 10) * 1000;
        for (let d = 0; d <= totalDistance.value; d += tickInterval) {
            distanceTicks.push({ value: d, position: (d / totalDistance.value) * viewBoxWidth.value, label: `${d / 1000}km` });
        }
        xTicks.value = distanceTicks;

    } catch (error) {
        console.error("AltitudeSVG: Error processing data:", error);
    }
}

onMounted(() => {
    processData();
});

function handleMouseMove(event) {
    if (!containerRef.value || dataPointsForTooltip.value.length === 0) return;

    const svg = containerRef.value.querySelector('svg');
    if (!svg) return;

    const svgRect = svg.getBoundingClientRect();
    const mouseX = event.clientX - svgRect.left;

    const clampedMouseX = Math.max(0, Math.min(mouseX, viewBoxWidth.value));

    const hoveredDistance = (clampedMouseX / viewBoxWidth.value) * totalDistance.value;

    const segmentLength = getSettingValue('Importation/Tracking/LongueurSegment') || 100;
    const index = Math.round(hoveredDistance / segmentLength);

    if (index >= 0 && index < dataPointsForTooltip.value.length) {
        const point = dataPointsForTooltip.value[index];
        tooltipData.value = {
            distance: point.distance,
            altitude: point.altitude,
            slope: point.slope,
        };
        tooltipVisible.value = true;
        hoverLineX.value = clampedMouseX;
        
        // --- Tooltip positioning logic ---
        // Vertical position: Stick to the bottom edge if cursor is too low
        const tooltipHeight = 80; // Estimated tooltip height in pixels
        const bottomMargin = 5;
        const maxTop = svgHeight.value - tooltipHeight - bottomMargin;
        const desiredY = event.offsetY;
        const finalY = Math.min(desiredY, maxTop);

        // Horizontal position: Stick to the right edge if cursor is too close
        const tooltipWidth = 80; // from CSS
        const rightMargin = 20; // Total threshold of 100px (80px width + 20px margin)
        const scrollLeft = containerRef.value.scrollLeft;
        const containerVisibleWidth = containerRef.value.clientWidth;
        const maxLeft = scrollLeft + containerVisibleWidth - tooltipWidth - rightMargin;
        const desiredX = event.offsetX;
        const finalX = Math.min(desiredX, maxLeft);

        tooltipPosition.value = {
          x: finalX,
          y: finalY
        };
    } else {
        tooltipVisible.value = false;
    }
}

function handleMouseLeave() {
    tooltipVisible.value = false;
}

watch(() => props.currentDistance, (newDistance) => {
    if (Math.abs(newDistance - lastUpdatedDistance) < 100) return;
    lastUpdatedDistance = newDistance;

    if (!containerRef.value || totalDistance.value === 0) return;

    progressX.value = (newDistance / totalDistance.value) * viewBoxWidth.value;

    const containerWidth = containerRef.value.clientWidth;
    const stuckPositionKm = getSettingValue('Altitude/Visualisation/CurseurPositionKm') || 10;
    const stuckPositionMeters = stuckPositionKm * 1000;
    const stuckPositionPx = (stuckPositionMeters / totalDistance.value) * viewBoxWidth.value;

    let scrollLeft = 0;
    // Le point de transition est le moment où le curseur, s'il continuait, 
    // ferait défiler le scroll au-delà de sa position maximale.
    const transitionPoint = viewBoxWidth.value - containerWidth + stuckPositionPx;

    if (progressX.value < stuckPositionPx) {
        // Phase 1: Le curseur se déplace au début, le graphe est fixe.
        scrollLeft = 0;
    }
    else if (progressX.value >= transitionPoint) {
        // Phase 3: Le graphe est calé à la fin, le curseur termine sa course.
        scrollLeft = viewBoxWidth.value - containerWidth;
    }
    else {
        // Phase 2: Le curseur est fixe sur l'écran, le graphe défile.
        scrollLeft = progressX.value - stuckPositionPx;
    }

    // Applique le défilement calculé au conteneur.
    if (containerRef.value.scrollLeft !== scrollLeft) {
        containerRef.value.scrollLeft = scrollLeft;
    }
});
</script>

<style scoped>
.altitude-graph-wrapper {
  display: flex;
  max-width: 90vw; /* Max width is 80% of the viewport width */
  background-color: rgba(0,0,0,0.3);
  transform: translateZ(0);
}

.svg-container {
    position: relative; /* Needed for absolute positioning of children */
    overflow-x: auto; /* Use native browser scrolling */
}

.y-axis-labels-container {
    position: relative; /* For child label positioning */
    flex-shrink: 0;
    pointer-events: none; /* Allow interaction with elements behind */
    z-index: 1; /* Ensure it's above the SVG */
    text-align: right;
    padding-right: 5px; /* Small padding from the graph */
}

.y-axis-label {
    position: absolute;
    transform: translateY(-50%); /* Center vertically */
    color: #888;
    font-size: 10px;
    white-space: nowrap;
    right: 5px; /* Align text to the right */
}

svg {
    display: block;
}
.axis line, .y-axis-grid line {
    stroke: #444;
    stroke-width: 1;
}

.y-axis-grid line {
    stroke-dasharray: 2 4;
}
.axis text {
    font-family: sans-serif;
    font-size: 10px;
    fill: #888;
    text-anchor: middle;
}
.progress-bar {
    stroke: white;
    stroke-width: 1.5;
}
.hover-line {
    stroke: rgba(255, 255, 255, 0.7);
    stroke-width: 1;
    stroke-dasharray: 4 2;
}
.tooltip {
    position: absolute; /* Position is relative to the .svg-container */
    --translateX: 15px;
    --translateY: 15px;
    transform: translate(var(--translateX), var(--translateY));
    background-color: rgba(255, 255, 255, 0.75); /* Light background */
    color: #212121; /* Dark text */
    box-shadow: 0 2px 8px rgba(0,0,0,0.15); /* Subtle shadow */
    padding: 8px 12px;
    border-radius: 5px;
    font-size: 12px;
    font-weight: bold;
    pointer-events: none;
    z-index: 9999; /* Keep high z-index just in case */
    white-space: nowrap;
    transition: transform 0.1s ease-out; /* Smooth transition */
    width: 80px; /* Fixed width */
    text-align: center; /* Center content */
}

</style>