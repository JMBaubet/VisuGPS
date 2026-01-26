<template>
  <div class="altitude-graph-wrapper">
    <!-- Y-Axis Labels (Ported from AltitudeSVG) -->
    <div class="y-axis-labels-container" :style="{ height: svgHeight + 'px', width: props.padding.left + 'px' }">
      <div v-for="tick in yTicks" :key="tick.label" class="y-axis-label" :style="{ top: tick.y + 'px' }">
        {{ tick.label }}
      </div>
    </div>

    <!-- Scrolling SVG Container -->
    <div class="svg-container" ref="containerRef" 
         @mousemove="handleMouseMove" 
         @mouseleave="handleMouseLeave"
         @click="handleGraphClick"
         style="cursor: pointer;"
    >
      <div
        v-if="tooltipVisible"
        class="tooltip"
        :style="{ top: tooltipPosition.y + 'px', left: tooltipPosition.x + 'px' }"
      >
        <div v-if="tooltipData.altitude">{{ Math.round(tooltipData.altitude) }} m</div>
        <div>{{ Math.round(tooltipData.distance / 100) / 10 }} km</div>
        <div>{{ tooltipData.type }}</div>
      </div>
      <svg :viewBox="`0 0 ${viewBoxWidth} ${svgHeight}`" :height="svgHeight" :width="viewBoxWidth" preserveAspectRatio="xMinYMin meet">
        <g>
          <!-- Y-Axis Grid Lines -->
          <g class="y-axis-grid">
            <g v-for="tick in yTicks" :key="tick.label">
              <line :x1="0" :y1="tick.y" :x2="viewBoxWidth" :y2="tick.y"></line>
            </g>
          </g>

          <!-- Altitude Area Segments (Common only) -->
          <g class="areas">
            <template v-for="(segment, index) in pathSegments" :key="`area-${index}`">
              <path v-if="!segment.isAbandoned"
                    :d="segment.path" :fill="segment.color" fill-opacity="0.6"></path>
            </template>
            <!-- Variant Specific Areas (Colored by Slope) -->
            <path v-for="(segment, index) in variantAltitudeSegments" :key="`area-var-${index}`"
                  :d="segment.path" :fill="segment.color" fill-opacity="0.6"></path>
          </g>

          <!-- Altitude Line Segments (Common only) -->
          <g class="lines">
              <template v-for="(segment, index) in pathSegments" :key="`line-${index}`">
                <path v-if="!segment.isAbandoned"
                      :d="segment.linePath" :stroke="segment.color" stroke-width="2" fill="none"></path>
              </template>
              <!-- Variant Specific Lines (Colored by Slope) -->
              <path v-for="(segment, index) in variantAltitudeSegments" :key="`line-var-${index}`"
                    :d="segment.linePath" :stroke="segment.color" stroke-width="2" fill="none"></path>
          </g>

          <!-- Altitude Connectors (Dashed Lines) -->
          <g class="altitude-connectors">
            <line v-for="(conn, index) in altitudeConnectors" :key="`conn-${index}`"
                  :x1="conn.x1" :y1="conn.y1" :x2="conn.x2" :y2="conn.y2"
                  :stroke="conn.color" stroke-width="1.5" stroke-dasharray="4 2" />
          </g>
          
          <!-- Abandoned Squares -->
          <g class="abandoned-altitude-markers">
              <template v-for="(segment, index) in pathSegments" :key="`sq-${index}`">
                <rect v-if="segment.isAbandoned"
                      :x="segment.x" :y="segment.y - (pixelScaleX / 2)" :width="pixelScaleX" :height="pixelScaleX"
                      :fill="segment.color" />
              </template>
          </g>

          <!-- Segments Visualization (Under the baseline) -->
          <g class="variant-segments">
            <!-- 1. Background (Common / Green) -->
            <!-- Positioned 2px below the baseline (innerHeight + padding.top) -->
            <rect 
                :x="0" 
                :y="innerHeight + props.padding.top + 2" 
                :width="viewBoxWidth" 
                :height="6" 
                fill="#4CAF50"
                opacity="0.9"
            />
            
            <!-- 2. Abandoned Segments (Grey) -->
            <g v-if="abandonedRects.length > 0">
                <rect v-for="(g, i) in abandonedRects" :key="`abandoned-${i}`"
                    :x="g.x" :y="innerHeight + props.padding.top + 2" :width="g.width" :height="6"
                    fill="#808080" stroke="none" opacity="1.0" />
            </g>

            <!-- 3. Variant Segments (Blue) - Drawn on top of grey/green -->
            <g v-if="blueRects.length > 0">
                <rect v-for="(b, i) in blueRects" :key="`variant-blue-${i}`"
                    :x="b.x" :y="innerHeight + props.padding.top + 2" :width="b.width" :height="6"
                    fill="#2196F3" stroke="none" opacity="1.0" />
            </g>
          </g>

          <!-- Axe X (Grid lines, Labels and Baseline) -->
          <g class="axis x-axis">
            <template v-for="tick in xTicks" :key="tick.value">
              <line :x1="tick.position" :y1="props.padding.top" :x2="tick.position" :y2="innerHeight + props.padding.top"></line>
              <!-- Labels slightly below the segments (innerHeight + padding.top + baseline gap + segments height + label gap) -->
              <text :x="tick.position" :y="innerHeight + props.padding.top + 22">{{ tick.label }}</text>
            </template>

            <!-- Baseline (Drawn last in group to be on top) -->
            <line 
                :x1="0" 
                :y1="innerHeight + props.padding.top" 
                :x2="viewBoxWidth" 
                :y2="innerHeight + props.padding.top" 
                class="axis-baseline"
            ></line>
          </g>

          <!-- Progress Bar / Cursor -->
          <line
              class="progress-bar"
              :x1="progressX"
              y1="0"
              :x2="progressX"
              :y2="svgHeight - 30"
          ></line>
          
          <!-- Hover Line -->
          <line
              v-if="tooltipVisible"
              class="hover-line"
              :x1="hoverLineX"
              y1="0"
              :x2="hoverLineX"
              :y2="svgHeight - 30"
          ></line>
        </g>
      </svg>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, computed, nextTick } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const emits = defineEmits(['jump-requested']);

const props = defineProps({
    currentDistance: { type: Number, required: true },
    totalMainDistance: { type: Number, required: true }, // In Meters
    mainTracePoints: { type: Array, default: () => [] }, // Points from tracking.json
    abandonedSegments: { type: Array, default: () => [] }, // list of { start: m, end: m }
    variantBlueSegments: { type: Array, default: () => [] }, // list of { type, anchorM, anchorEndM, lengthM }
    padding: {
        type: Object,
        default: () => ({ top: 10, right: 10, bottom: 40, left: 45 }) // Back to 40
    }
});

// --- Refs ---
const containerRef = ref(null);

const viewBoxWidth = ref(1000);
const svgHeight = ref(210); // Standard height from AltitudeSVG
const progressX = ref(0);
const pixelScaleX = ref(2); // Pixels per 100m
const pathSegments = ref([]);
const abandonedRects = ref([]); 
const blueRects = ref([]);
const variantAltitudeSegments = ref([]);
const altitudeConnectors = ref([]);
const xTicks = ref([]);
const yTicks = ref([]);
const minAltitude = ref(0);
const maxAltitude = ref(0);

const innerWidth = computed(() => viewBoxWidth.value - props.padding.left - props.padding.right);
const innerHeight = computed(() => svgHeight.value - props.padding.top - props.padding.bottom);

// --- Tooltip Refs ---
const tooltipVisible = ref(false);
const tooltipData = ref({ distance: 0, altitude: 0, type: '' });
const tooltipPosition = ref({ x: 0, y: 0 });
const hoverLineX = ref(0);

// --- Composables ---
const { getSettingValue } = useSettings();
const { toHex } = useVuetifyColors();

onMounted(() => {
    processData();
});

// --- Logic ---
async function processData() {
    if (!props.mainTracePoints || props.mainTracePoints.length === 0) return;

    const contextTotalDistMeters = props.totalMainDistance;
    
    // Scaling Factors
    pixelScaleX.value = getSettingValue('Visualisation/Profil Altitude/Graphe/Abscisse') || 2;
    viewBoxWidth.value = (contextTotalDistMeters / 100) * pixelScaleX.value;
    const getX = (distFromStartMeters) => (distFromStartMeters / contextTotalDistMeters) * viewBoxWidth.value;

    // Altitude Range & Y Scale
    const pixelsFor10Meters = getSettingValue('Visualisation/Profil Altitude/Graphe/Ordonnee') || 10;
    const pixelsPerMeter = pixelsFor10Meters / 10;
    
    const allAltitudes = props.mainTracePoints.map(p => p.altitude).filter(a => typeof a === 'number' && !isNaN(a));
    minAltitude.value = Math.min(...allAltitudes);
    maxAltitude.value = Math.max(...allAltitudes);
    
    const altitudeTickInterval = getSettingValue('Visualisation/Profil Altitude/Graphe/RepereAltitude') || 200;
    const graphMinY = Math.floor(minAltitude.value / altitudeTickInterval) * altitudeTickInterval;
    const graphMaxY = Math.ceil(maxAltitude.value / altitudeTickInterval) * altitudeTickInterval;
    
    const effectiveMinAltitude = graphMinY;
    const effectiveAltitudeSpan = graphMaxY - graphMinY === 0 ? 1 : graphMaxY - graphMinY;
    const graphDrawingHeight = effectiveAltitudeSpan * pixelsPerMeter;
    
    // Total SVG Height = Curve Height + Padding
    svgHeight.value = graphDrawingHeight + props.padding.top + props.padding.bottom;

    const yScale = (alt) => graphDrawingHeight - ((alt - effectiveMinAltitude) / effectiveAltitudeSpan) * graphDrawingHeight + props.padding.top;

    const getSlopeColor = (slope) => {
        if (slope <= 0) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative') || 'light-blue');
        if (slope < 3) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1') || 'green');
        if (slope < 6) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2') || 'yellow');
        if (slope < 9) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3') || 'orange');
        if (slope < 12) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4') || 'red');
        return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5') || 'purple');
    };

    // Generate Path Segments
    const segs = [];
    const baselineY = innerHeight.value + props.padding.top; 
    for (let i = 1; i < props.mainTracePoints.length; i++) {
        const p1 = props.mainTracePoints[i - 1];
        const p2 = props.mainTracePoints[i];
        
        const distDiff = (p2.distance - p1.distance) * 1000;
        const altDiff = p2.altitude - p1.altitude;
        const slope = distDiff > 0 ? (altDiff / distDiff) * 100 : 0;
        
        const x1 = getX(p1.distance * 1000);
        const x2 = getX(p2.distance * 1000);
        const y1 = yScale(p1.altitude);
        const y2 = yScale(p2.altitude);

        // Check if this segment is abandoned
        const midDist = (p1.distance + p2.distance) * 500; // in meters ( (p1+p2)/2 * 1000 )
        const isAbandoned = props.abandonedSegments.some(s => midDist >= s.start && midDist <= s.end);

        segs.push({
            path: `M ${x1},${y1} L ${x2},${y2} L ${x2},${baselineY} L ${x1},${baselineY} Z`,
            linePath: `M ${x1},${y1} L ${x2},${y2}`,
            color: getSlopeColor(slope),
            altitude: p2.altitude,
            isAbandoned,
            x: x1,
            y: y1
        });
    }
    pathSegments.value = segs;

    // Generate Abandoned Rects (now use innerHeight for base pos)
    abandonedRects.value = props.abandonedSegments.map(seg => {
        const x1 = getX(seg.start);
        const x2 = getX(seg.end);
        return {
            x: x1,
            width: Math.max(1, x2 - x1),
            range: [seg.start, seg.end]
        };
    });

    // Generate Blue Rects (Variant Segments indicators)
    blueRects.value = props.variantBlueSegments.map(seg => {
        const widthPx = (seg.lengthM / 100) * pixelScaleX.value;
        let x = 0;
        if (seg.type === 'DEPART') x = getX(seg.anchorM) - widthPx;
        else if (seg.type === 'ARRIVEE') x = getX(seg.anchorM);
        else if (seg.type === 'SEGMENT') {
            const center = (getX(seg.anchorM) + getX(seg.anchorEndM)) / 2;
            x = center - (widthPx / 2);
        }
        return { x, width: Math.max(pixelScaleX.value, widthPx) };
    });

    // Generate Variant Altitude Profile (Blue)
    const varAltSegs = [];
    props.variantBlueSegments.forEach(seg => {
        if (!seg.points || seg.points.length < 2) return;
        
        const widthPx = (seg.lengthM / 100) * pixelScaleX.value;
        let startX = 0;
        if (seg.type === 'DEPART') startX = getX(seg.anchorM) - widthPx;
        else if (seg.type === 'ARRIVEE') startX = getX(seg.anchorM);
        else if (seg.type === 'SEGMENT') {
             const center = (getX(seg.anchorM) + getX(seg.anchorEndM)) / 2;
             startX = center - (widthPx / 2);
        }

        const firstPtDist = seg.points[0].distance;
        
        for (let i = 1; i < seg.points.length; i++) {
            const p1 = seg.points[i - 1];
            const p2 = seg.points[i];
            
            const distDiff = (p2.distance - p1.distance) * 1000;
            const altDiff = p2.altitude - p1.altitude;
            const slope = distDiff > 0 ? (altDiff / distDiff) * 100 : 0;
            
            const localDist1 = (p1.distance - firstPtDist) * 1000;
            const localDist2 = (p2.distance - firstPtDist) * 1000;
            const x1 = startX + (localDist1 / 100) * pixelScaleX.value;
            const x2 = startX + (localDist2 / 100) * pixelScaleX.value;
            
            const y1 = yScale(p1.altitude);
            const y2 = yScale(p2.altitude);

            varAltSegs.push({
                path: `M ${x1},${y1} L ${x2},${y2} L ${x2},${baselineY} L ${x1},${baselineY} Z`,
                linePath: `M ${x1},${y1} L ${x2},${y2}`,
                color: getSlopeColor(slope), // Use slope color instead of solid blue
                altitude: p2.altitude
            });
        }
    });
    variantAltitudeSegments.value = varAltSegs;

    // Generate Altitude Connectors (Dashed Lines)
    const connectors = [];
    const findMasterPoint = (distM) => {
        const distKm = distM / 1000;
        return props.mainTracePoints.reduce((prev, curr) => {
            return (Math.abs(curr.distance - distKm) < Math.abs(prev.distance - distKm)) ? curr : prev;
        }, props.mainTracePoints[0]);
    };

    props.variantBlueSegments.forEach(seg => {
        if (!seg.points || seg.points.length < 2) return;

        const widthPx = (seg.lengthM / 100) * pixelScaleX.value;
        let startX = 0;
        if (seg.type === 'DEPART') startX = getX(seg.anchorM) - widthPx;
        else if (seg.type === 'ARRIVEE') startX = getX(seg.anchorM);
        else if (seg.type === 'SEGMENT') {
             startX = (getX(seg.anchorM) + getX(seg.anchorEndM)) / 2 - (widthPx / 2);
        }

        const firstPtDist = seg.points[0].distance;

        // 1. Entry Connector (Anchor -> Start of Variant)
        // Only for SEGMENT or ARRIVEE (Depart starts at 0 or free)
        // Wait, the prompt says "1er point d'ancrage du segment au 1er point du segment"
        // and "dernier point du segment au second point d'ancrage".
        // This applies mostly to SEGMENT type deviations.
        
        if (seg.type === 'SEGMENT' || seg.type === 'ARRIVEE') {
            const anchorPt = findMasterPoint(seg.anchorM);
            const varStartPt = seg.points[0];
            
            // X/Y for Anchor
            const ax = getX(seg.anchorM);
            const ay = yScale(anchorPt.altitude);
            
            // X/Y for Variant Start
            const vx = startX;
            const vy = yScale(varStartPt.altitude);

            // Color: Slope of 1st Variant Slice
            const p2 = seg.points[1];
            const slope = ((p2.altitude - varStartPt.altitude) / ((p2.distance - varStartPt.distance) * 1000)) * 100;
            
            connectors.push({ x1: ax, y1: ay, x2: vx, y2: vy, color: getSlopeColor(slope) });
        }

        // 2. Exit Connector (End of Variant -> Anchor 2)
        if (seg.type === 'SEGMENT' || seg.type === 'DEPART') {
            const targetAnchorM = (seg.type === 'DEPART') ? seg.anchorM : seg.anchorEndM;
            const anchorPt = findMasterPoint(targetAnchorM);
            const varEndPt = seg.points[seg.points.length - 1];
            
            // X/Y for Variant End
            const vx = startX + ((varEndPt.distance - firstPtDist) * 1000 / 100) * pixelScaleX.value;
            const vy = yScale(varEndPt.altitude);
            
            // X/Y for Anchor
            const ax = getX(targetAnchorM);
            const ay = yScale(anchorPt.altitude);

            // Color: Slope of Master at Anchor
            // Find slope in props.mainTracePoints around targetAnchorM
            const distKm = targetAnchorM / 1000;
            const idx = props.mainTracePoints.findIndex(p => p.distance >= (distKm - 0.0001));
            let masterSlope = 0;
            if (idx > 0) {
                const mp1 = props.mainTracePoints[idx - 1];
                const mp2 = props.mainTracePoints[idx];
                masterSlope = ((mp2.altitude - mp1.altitude) / ((mp2.distance - mp1.distance) * 1000)) * 100;
            }
            
            connectors.push({ x1: vx, y1: vy, x2: ax, y2: ay, color: getSlopeColor(masterSlope) });
        }
    });
    altitudeConnectors.value = connectors;

    // 4. Ticks
    xTicks.value = [];
    const tickInterval = (getSettingValue('Visualisation/Profil Altitude/Graphe/RepereDistance') || 10) * 1000; 
    
    if (tickInterval > 0) {
        for (let d = 0; d <= contextTotalDistMeters; d += tickInterval) {
             let labelValKm = d / 1000;
             xTicks.value.push({ value: d, position: getX(d), label: `${labelValKm.toFixed(1)}km` });
        }
    }

    yTicks.value = [];
    for (let alt = graphMinY; alt <= graphMaxY; alt += altitudeTickInterval) {
        yTicks.value.push({ y: yScale(alt), label: `${alt}m` });
    }
}

watch([() => props.totalMainDistance, () => props.abandonedSegments, () => props.mainTracePoints, () => props.variantBlueSegments], () => {
    processData();
});

function handleMouseMove(event) {
    if (!containerRef.value || props.totalMainDistance === 0) return;

    const rect = containerRef.value.getBoundingClientRect();
    const hitX = event.clientX - rect.left; 
    const scrollLeft = containerRef.value.scrollLeft;
    const clickXInSvg = hitX + scrollLeft;
    
    let targetDistanceMeters = (clickXInSvg * props.totalMainDistance) / viewBoxWidth.value;
    targetDistanceMeters = Math.max(0, Math.min(targetDistanceMeters, props.totalMainDistance));
    
    hoverLineX.value = clickXInSvg;
    tooltipPosition.value = { x: hitX + 10, y: event.clientY - rect.top + 10 };
    
    const isAbandoned = props.abandonedSegments.some(s => targetDistanceMeters >= s.start && targetDistanceMeters <= s.end);
    
    // Find closest altitude
    // This is expensive in mousemove, but ok for small data
    let foundAlt = 0;
    const dKm = targetDistanceMeters / 1000;
    // Binary search or just find closest
    const closestP = props.mainTracePoints.reduce((prev, curr) => {
        return (Math.abs(curr.distance - dKm) < Math.abs(prev.distance - dKm)) ? curr : prev;
    }, props.mainTracePoints[0]);
    if (closestP) foundAlt = closestP.altitude;

    tooltipData.value = {
        distance: targetDistanceMeters,
        altitude: foundAlt,
        type: isAbandoned ? 'Abandonné (Gris)' : 'Commun (Vert)'
    };
    tooltipVisible.value = true;
}

function handleMouseLeave() {
    tooltipVisible.value = false;
}

function handleGraphClick(event) {
     if (!containerRef.value || props.totalMainDistance === 0) return;
     const rect = containerRef.value.getBoundingClientRect();
     const scrollLeft = containerRef.value.scrollLeft;
     const clickXInSvg = (event.clientX - rect.left) + scrollLeft;
     let targetDistanceMeters = (clickXInSvg * props.totalMainDistance) / viewBoxWidth.value;
     targetDistanceMeters = Math.max(0, Math.min(targetDistanceMeters, props.totalMainDistance));
     
     // Note: This emits Main Trace Distance. VisualizeVariantView might need conversion
     // but the user just asked for visualized altitude for now.
}
</script>

<style scoped>
.altitude-graph-wrapper {
  display: flex;
  max-width: 90vw;
  background-color: rgba(0,0,0,0.3);
  transform: translateZ(0);
}
.svg-container { position: relative; overflow-x: auto; }
.y-axis-labels-container { position: relative; flex-shrink: 0; pointer-events: none; z-index: 1; text-align: right; padding-right: 5px; }
.y-axis-label { position: absolute; transform: translateY(-50%); color: #888; font-size: 10px; white-space: nowrap; right: 5px; }
svg { display: block; }
.axis line, .y-axis-grid line { stroke: #444; stroke-width: 1; }
.y-axis-grid line { stroke-dasharray: 2 4; }
.axis-baseline { stroke: #FFF; stroke-width: 2; }
.axis line { stroke: #666; stroke-width: 1; }
.axis text { font-family: sans-serif; font-size: 11px; fill: #FFF; text-anchor: middle; font-weight: bold; }
.progress-bar { stroke: white; stroke-width: 1.5; }
rect { shape-rendering: crispEdges; }
.hover-line { stroke: rgba(255, 255, 255, 0.7); stroke-width: 1; stroke-dasharray: 4 2; }
.tooltip { position: absolute; background: rgba(255,255,255,0.75); padding: 8px; border-radius: 5px; font-size: 12px; pointer-events: none; z-index: 9999; }
</style>
