<template>
  <div class="svg-container" ref="containerRef">
    <div class="y-axis-labels-container" :style="{ height: svgHeight + 'px', width: props.padding.left + 'px' }">
      <div v-for="tick in yTicks" :key="tick.label" class="y-axis-label" :style="{ top: tick.y + 'px' }">
        {{ tick.label }}
      </div>
    </div>
    <svg :viewBox="`0 0 ${viewBoxWidth} ${svgHeight}`" :height="svgHeight" :width="viewBoxWidth" preserveAspectRatio="xMinYMin meet">
      <g>

        <!-- Axe X (Grid lines and Labels) -->

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
      </g>
    </svg>
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
        default: () => ({ top: 10, right: 10, bottom: 30, left: 100 })
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

        const effectiveMinAltitude = minAltitude.value > 0 ? 0 : minAltitude.value;
        const effectiveAltitudeSpan = maxAltitude.value - effectiveMinAltitude === 0 ? 1 : maxAltitude.value - effectiveMinAltitude;

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
        const scaleX = getSettingValue('Altitude/Visualisation/Abscisse') || 2;
        viewBoxWidth.value = (totalDistance.value / 100) * scaleX;

        console.log("AltitudeSVG Debug:", {
            minAltitude: minAltitude.value,
            maxAltitude: maxAltitude.value,
            effectiveMinAltitude: effectiveMinAltitude,
            effectiveAltitudeSpan: effectiveAltitudeSpan,
            innerHeight: innerHeight.value,
            propsPaddingTop: props.padding.top
        });

        const yScale = (alt) => graphDrawingHeight - ((alt - effectiveMinAltitude) / effectiveAltitudeSpan) * graphDrawingHeight + props.padding.top;

        zeroAltitudeY.value = yScale(0);

        const negativeSlopeFactor = getSettingValue('Altitude/Couleurs/NegativeSlopeFactor');
        const tranche1ColorName = getSettingValue('Altitude/Couleurs/Tranche1');
        const negativeSlopeColorName = `${tranche1ColorName}-${negativeSlopeFactor}`;
        const getSlopeColor = (slope) => {
            if (slope <= 0) return toHex(negativeSlopeColorName);
            if (slope <= 2) return toHex(getSettingValue('Altitude/Couleurs/Tranche2'));
            if (slope <= 4) return toHex(getSettingValue('Altitude/Couleurs/Tranche3'));
            if (slope <= 7) return toHex(getSettingValue('Altitude/Couleurs/Tranche4'));
            if (slope <= 10) return toHex(getSettingValue('Altitude/Couleurs/Tranche5'));
            if (slope <= 12) return toHex(getSettingValue('Altitude/Couleurs/Tranche6'));
            if (slope <= 15) return toHex(getSettingValue('Altitude/Couleurs/Tranche7'));
            return toHex(getSettingValue('Altitude/Couleurs/Tranche8'));
        };

        const segments = [];
        for (let i = 1; i < dataPoints.length; i++) {
            const p1 = dataPoints[i - 1];
            const p2 = dataPoints[i];
            const x1 = (p1.distance / totalDistance.value) * viewBoxWidth.value;
            const x2 = (p2.distance / totalDistance.value) * viewBoxWidth.value;
            const y1 = yScale(p1.altitude);
            const y2 = yScale(p2.altitude);

            segments.push({
                path: `M ${x1},${y1} L ${x2},${y2} L ${x2},${zeroAltitudeY.value} L ${x1},${zeroAltitudeY.value} Z`,
                linePath: `M ${x1},${y1} L ${x2},${y2}`,
                color: getSlopeColor(p2.slope)
            });
        }
        pathSegments.value = segments;

        const ticks = [];
        const altitudeTickInterval = getSettingValue('Altitude/Visualisation/RepereAltitude') || 200; // Default to 200m
        for (let alt = Math.floor(effectiveMinAltitude / altitudeTickInterval) * altitudeTickInterval; alt <= maxAltitude.value; alt += altitudeTickInterval) {
            if (alt < effectiveMinAltitude) continue; // Ensure ticks are not below effectiveMinAltitude
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

watch(() => props.currentDistance, (newDistance) => {
    if (Math.abs(newDistance - lastUpdatedDistance) < 100) return;
    lastUpdatedDistance = newDistance;

    if (!containerRef.value || totalDistance.value === 0) return;

    progressX.value = (newDistance / totalDistance.value) * viewBoxWidth.value;

    const containerWidth = containerRef.value.clientWidth;
    const zoomWindowKm = getSettingValue('Altitude/Visualisation/FenetreZoomKm') || 50;
    const stuckPositionKm = getSettingValue('Altitude/Visualisation/CurseurPositionKm') || 10;

    const zoomWindowMeters = zoomWindowKm * 1000;
    const stuckPositionMeters = stuckPositionKm * 1000;

    const zoomWindowPx = (zoomWindowMeters / totalDistance.value) * viewBoxWidth.value;
    const stuckPositionPx = (stuckPositionMeters / totalDistance.value) * viewBoxWidth.value;

    let scrollLeft = 0;
    const transitionPoint = viewBoxWidth.value - zoomWindowPx + stuckPositionPx;

    if (progressX.value < stuckPositionPx) {
        scrollLeft = 0;
    }
    else if (progressX.value >= transitionPoint) {
        scrollLeft = viewBoxWidth.value - containerWidth;
    }
    else {
        scrollLeft = progressX.value - stuckPositionPx;
    }

    containerRef.value.scrollLeft = scrollLeft;
});

</script>

<style scoped>
.svg-container {
    position: relative; /* Needed for absolute positioning of children */
    width: 100%;
    overflow-x: auto; /* Use native browser scrolling */
    background-color: rgba(0,0,0,0.3);
}

.y-axis-labels-container {
    position: absolute;
    top: 0;
    left: 0;
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
}

svg {
    display: block;
}
.axis line {
    stroke: #444;
    stroke-width: 1;
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
</style>
