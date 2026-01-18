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

          <!-- Main Trace Reference (Dots) -->
          <g class="main-trace-ref" v-if="mainTraceReferencePoints.length > 0">
              <circle v-for="(p, i) in mainTraceReferencePoints" :key="`ref-${i}`" 
                  :cx="p.cx" :cy="p.cy" :r="p.r" :fill="p.fill" />
          </g>

          <!-- Comparison Connectors (Dashed Lines) -->
          <g class="comparison-connectors" v-if="comparisonConnectors.length > 0">
              <line v-for="(l, i) in comparisonConnectors" :key="`conn-${i}`"
                  :x1="l.x1" :y1="l.y1" :x2="l.x2" :y2="l.y2"
                  class="connector-line" />
          </g>

          <!-- Progress Bar / Cursor -->
          <line
              v-if="!isCometLinked"
              class="progress-bar"
              :x1="progressX"
              y1="0"
              :x2="progressX"
              :y2="svgHeight - 20"
          ></line>
          
          <rect
              v-else
              :x="rectDimensions.x"
              y="0"
              :width="rectDimensions.width"
              :height="svgHeight - 20"
              :fill="cursorColor"
              :style="{ opacity: cursorOpacity }"
          ></rect>

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

// invoke removed


import { useSettings } from '@/composables/useSettings';

import { useVuetifyColors } from '@/composables/useVuetifyColors';

const props = defineProps({
    circuitId: { type: String, required: true },
    currentDistance: { type: Number, required: true },
    // NEW PROPS
    totalDistance: { type: Number, required: true }, // In Meters
    trackingPoints: { type: Array, required: true },
    // COMPARISON PROPS
    isVariantComparison: { type: Boolean, default: false },
    mainTracePoints: { type: Array, default: () => [] },
    mainTraceStartKm: { type: Number, default: 0 },
    mainTraceEndKm: { type: Number, default: 0 },
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
const comparisonConnectors = ref([]); // New: Lines connecting main to variant
const mainTraceReferencePoints = ref([]); // New: Main trace dots
const xTicks = ref([]);
const yTicks = ref([]);
const totalDrawableDistance = ref(0); // The distance spanning the X axis (either totalDistance or MainTraceSpan)
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

// --- Computed for Cursor ---
const isCometLinked = computed(() => getSettingValue('Visualisation/Profil Altitude/Graphe/aspectCurseurLieComete'));

const cursorColor = computed(() => {
    if (isCometLinked.value) {
        const colorName = getSettingValue('Visualisation/Vue 3D/Trace/couleurComete');
       return toHex(colorName) || 'white';
    }
    return 'white';
});

const cursorOpacity = computed(() => {
    if (isCometLinked.value) {
        let val = getSettingValue('Visualisation/Vue 3D/Trace/opaciteComete');
        if (typeof val === 'string') {
            val = val.replace(',', '.');
        }
        const num = parseFloat(val);
        // Ensure strictly normalized value
        return !isNaN(num) ? Math.max(0, Math.min(1, num)) : 1;
    }
    return 1;
});

const cursorWidth = computed(() => {
    if (isCometLinked.value && totalDrawableDistance.value > 0) {
        const cometLengthMeters = getSettingValue('Visualisation/Vue 3D/Trace/longueurComete') || 50;
        return (cometLengthMeters / totalDrawableDistance.value) * viewBoxWidth.value;
    }
    return 1.5; 
});

const rectDimensions = computed(() => {
    const px = progressX.value;
    const cw = cursorWidth.value;
    const x = Math.max(0, px - cw);
    const width = px - x; // Ensures width is exactly what's needed to reach px from x
    return { x, width };
});




// --- Data Loading and Processing ---
async function processData() {
    // Safety check
    if (!props.trackingPoints) {
        pathSegments.value = [];
        return;
    }
    
    // Check if we are in valid comparison mode
    // We basically need mainTracePoints to define the "World"
    const isValidComparison = props.isVariantComparison && 
                              props.mainTracePoints && 
                              props.mainTracePoints.length > 0;
                              // Length check is handled by parent logic

    // Standard mode data (Variant or Single Trace)
    const filteredTrackingData = props.trackingPoints.filter(p => typeof p.altitude === 'number' && !isNaN(p.altitude));
    if (filteredTrackingData.length < 2) {
        pathSegments.value = [];
        return;
    }

    // --- 1. Determine Context (X-Axis World) & Altitude Range ---
    let contextStartKm = 0;
    let contextEndKm = props.totalDistance / 1000;
    
    if (isValidComparison) {
        // If comparing, the World is the Main Trace
        // Assumes mainTracePoints are sorted by distance
        const lastPt = props.mainTracePoints[props.mainTracePoints.length - 1];
        contextEndKm = lastPt ? lastPt.distance : contextEndKm;
    }
    
    const contextTotalDistMeters = (contextEndKm - contextStartKm) * 1000;
    totalDrawableDistance.value = contextTotalDistMeters; // Used for X scaling & Scroll

    // Scaling Factors
    const pixelsFor10Meters = getSettingValue('Visualisation/Profil Altitude/Graphe/Ordonnee') || 10;
    const pixelsPerMeter = pixelsFor10Meters / 10;
    
    // Altitude Range
    // Include Main Trace points in range calculation if comparing
    let allAltitudes = filteredTrackingData.map(p => p.altitude);
    if (isValidComparison) {
         props.mainTracePoints.forEach(p => {
             if (typeof p.altitude === 'number' && !isNaN(p.altitude)) allAltitudes.push(p.altitude);
         });
    }
    
    minAltitude.value = Math.min(...allAltitudes);
    maxAltitude.value = Math.max(...allAltitudes);
    
    const altitudeTickInterval = getSettingValue('Visualisation/Profil Altitude/Graphe/RepereAltitude') || 200;
    const graphMinY = Math.floor(minAltitude.value / altitudeTickInterval) * altitudeTickInterval;
    const graphMaxY = Math.ceil(maxAltitude.value / altitudeTickInterval) * altitudeTickInterval;
    
    const effectiveMinAltitude = graphMinY;
    const effectiveAltitudeSpan = graphMaxY - graphMinY === 0 ? 1 : graphMaxY - graphMinY;
    const graphDrawingHeight = effectiveAltitudeSpan * pixelsPerMeter;
    svgHeight.value = graphDrawingHeight + props.padding.top + props.padding.bottom;

    const yScale = (alt) => graphDrawingHeight - ((alt - effectiveMinAltitude) / effectiveAltitudeSpan) * graphDrawingHeight + props.padding.top;
    zeroAltitudeY.value = yScale(0);

    const scaleX = getSettingValue('Visualisation/Profil Altitude/Graphe/Abscisse') || 2;
    viewBoxWidth.value = (contextTotalDistMeters / 100) * scaleX;

    // Helper: Global X Mapper
    const getX = (distFromStartMeters) => {
        return (distFromStartMeters / contextTotalDistMeters) * viewBoxWidth.value;
    };
    
    // Helper: Slope Color
    const getSlopeColor = (slope) => {
        if (slope <= 0) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative') || 'light-blue');
        if (slope < 3) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1') || 'green');
        if (slope < 6) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2') || 'yellow');
        if (slope < 9) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3') || 'orange');
        if (slope < 12) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4') || 'red');
        return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5') || 'purple');
    };

    // Helper: Segment Generator
    const generateSegments = (points, xOffsetMeters = 0) => {
        const segs = [];
        const graphBottomY = svgHeight.value - props.padding.bottom;
        
        for (let i = 1; i < points.length; i++) {
            const p1 = points[i - 1];
            const p2 = points[i];
            
            // Calc slope locally
            const distDiff = (p2.distance - p1.distance) * 1000;
            const altDiff = p2.altitude - p1.altitude;
            const slope = distDiff > 0 ? (altDiff / distDiff) * 100 : 0;
            
            // Calc X based on Global Distance (p.distance) + Offset
            // Note: points[].distance is in KM.
            const x1 = getX((p1.distance * 1000) + xOffsetMeters);
            const x2 = getX((p2.distance * 1000) + xOffsetMeters);
            const y1 = yScale(p1.altitude);
            const y2 = yScale(p2.altitude);

            segs.push({
                path: `M ${x1},${y1} L ${x2},${y2} L ${x2},${graphBottomY} L ${x1},${graphBottomY} Z`,
                linePath: `M ${x1},${y1} L ${x2},${y2}`,
                color: getSlopeColor(slope)
            });
        }
        return segs;
    };


    pathSegments.value = [];
    mainTraceReferencePoints.value = [];
    comparisonConnectors.value = [];

    if (isValidComparison) {
        // --- Comparison Mode Visualization ---
        const startKm = props.mainTraceStartKm;
        const endKm = props.mainTraceEndKm;
        
        // 1. Split Main Trace
        const mainBefore = props.mainTracePoints.filter(p => p.distance <= startKm);
        const mainAfter = props.mainTracePoints.filter(p => p.distance >= endKm);
        const mainGap = props.mainTracePoints.filter(p => p.distance >= startKm && p.distance <= endKm);
        
        // 2. Render Main Sections
        const segmentsBefore = generateSegments(mainBefore);
        const segmentsAfter = generateSegments(mainAfter);
        pathSegments.value = [...segmentsBefore, ...segmentsAfter];
        
        // 3. Render Variant (Centered in Gap)
        const gapLenMeters = (endKm - startKm) * 1000;
        const variantLenMeters = props.totalDistance;
        const localOffsetMeters = (gapLenMeters - variantLenMeters) / 2;
        
        // Variant points distance is 0..TotalVar. Need to be shifted to startKm + localOffset
        // But generateSegments expects points.distance in KM.
        // We can cheat by passing xOffsetMeters = (startKm * 1000) + localOffsetMeters
        // AND ensuring variant points are treated as starting at 0. (They are)
        
        const globalOffsetForVariant = (startKm * 1000) + localOffsetMeters;
        const segmentsVariant = generateSegments(filteredTrackingData, globalOffsetForVariant);
        pathSegments.value.push(...segmentsVariant);
        
        // 4. Render Reference Dots (The Replaced Section)
        mainTraceReferencePoints.value = mainGap.map(p => ({
             cx: getX(p.distance * 1000),
             cy: yScale(p.altitude),
             r: 1.5,
             fill: '#666'
        }));
        
        // 5. Connectors
        if (filteredTrackingData.length > 0) {
             // Main End Before Gap -> Variant Start
             // Main Before ends at startKm.
             // Variant starts at startKm + localOffset.
             
             let yMain1 = yScale(mainBefore[mainBefore.length-1]?.altitude || 0); // approx
             let yVar1 = yScale(filteredTrackingData[0].altitude);
             let xMain1 = getX(startKm * 1000);
             let xVar1 = getX(globalOffsetForVariant);
             
             comparisonConnectors.value.push({ x1: xMain1, y1: yMain1, x2: xVar1, y2: yVar1 });
             
             // Variant End -> Main Start After Gap
             let yVar2 = yScale(filteredTrackingData[filteredTrackingData.length-1].altitude);
             let yMain2 = yScale(mainAfter[0]?.altitude || 0);
             let xVar2 = getX(globalOffsetForVariant + variantLenMeters);
             let xMain2 = getX(endKm * 1000);
             
             comparisonConnectors.value.push({ x1: xVar2, y1: yVar2, x2: xMain2, y2: yMain2 });
        }

    } else {
        // --- Standard Mode ---
        // Just render the tracking points (Variant or Main)
        pathSegments.value = generateSegments(filteredTrackingData);
    }
    
    // --- ticks ---
    yTicks.value = [];
    for (let alt = graphMinY; alt <= graphMaxY; alt += altitudeTickInterval) {
        const y = yScale(alt);
        yTicks.value.push({ y, label: `${alt}m` });
    }

    const distanceTicks = [];
    const tickInterval = (getSettingValue('Visualisation/Profil Altitude/Graphe/RepereDistance') || 10) * 1000; 
    if (tickInterval > 0) {
        for (let d = 0; d <= contextTotalDistMeters; d += tickInterval) {
            let labelValKm = (contextStartKm * 1000 + d) / 1000;
            distanceTicks.push({ 
                value: d, 
                position: getX(d), 
                label: `${labelValKm.toFixed(1)}km` 
            });
        }
    }
    xTicks.value = distanceTicks;
}

// Watch for changes in data props to re-process graph
watch(() => [props.trackingPoints, props.totalDistance, props.isVariantComparison, props.mainTracePoints], () => {
    processData();
}, { deep: true, immediate: true });

// onMounted removed, handled by immediate watch


function handleMouseMove(event) {
    if (!containerRef.value || dataPointsForTooltip.value.length === 0) return;

    const svg = containerRef.value.querySelector('svg');
    if (!svg) return;

    const svgRect = svg.getBoundingClientRect();
    const mouseX = event.clientX - svgRect.left;

    const clampedMouseX = Math.max(0, Math.min(mouseX, viewBoxWidth.value));

    const hoveredDistance = (clampedMouseX / viewBoxWidth.value) * totalDrawableDistance.value;

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

    if (!containerRef.value || totalDrawableDistance.value === 0) return;

    // Calculate Progress X based on context
    // If Comparison: x = getX(current + offset)
    // If Standard: x = (current / total) * width
    
    let xPos = 0;
    if (props.isVariantComparison && props.mainTracePoints.length > 0) {
         // Calculation logic mirrors processData
         const contextLenM = totalDrawableDistance.value;
         
         const startGapM = props.mainTraceStartKm * 1000;
         const endGapM = props.mainTraceEndKm * 1000;
         const gapLenM = endGapM - startGapM;
         const variantLenM = props.totalDistance;
         const localOffsetM = Math.max(0, (gapLenM - variantLenM) / 2);
         
         const globalPosM = startGapM + localOffsetM + newDistance;
         
         xPos = (globalPosM / contextLenM) * viewBoxWidth.value;
    } else {
         xPos = (newDistance / totalDrawableDistance.value) * viewBoxWidth.value;
    }
    progressX.value = xPos;

    const containerWidth = containerRef.value.clientWidth;
    const stuckPositionKm = getSettingValue('Visualisation/Profil Altitude/Graphe/CurseurPositionKm') || 10;
    const stuckPositionMeters = stuckPositionKm * 1000;
    const stuckPositionPx = (stuckPositionMeters / totalDrawableDistance.value) * viewBoxWidth.value;

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
.connector-line {
    stroke: #00ffff; /* Cyan color for visibility */
    stroke-width: 1;
    stroke-dasharray: 4 2;
    opacity: 0.7;
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