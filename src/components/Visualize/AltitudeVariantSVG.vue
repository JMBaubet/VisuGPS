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
         @mouseleave="tooltipVisible = false"
         @click="handleGraphClick"
         style="cursor: pointer;"
    >
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
              stroke="#D32F2F"
              stroke-width="2"
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
import { getSlopeColor, buildSlopeColorsMap } from '@/composables/useSlopeColors';

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
const minVisualM = ref(0);
const visualSpanM = ref(1);

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

// --- Hover Refs ---
const tooltipVisible = ref(false);
const hoverLineX = ref(0);

// --- Composables ---
const { getSettingValue } = useSettings();
const { toHex } = useVuetifyColors();

// --- Computed Mapping Structure ---
const sortedModifications = computed(() => {
    return props.variantBlueSegments.map(b => {
        const isDepart = b.type === 'DEPART';
        const isArrivee = b.type === 'ARRIVEE';
        const widthM = b.lengthM; 
        
        let mMin = 0; // Blue Start in Master Scale
        if (isDepart) mMin = b.anchorM - widthM;
        else if (isArrivee) mMin = b.anchorM;
        else {
            const mCenter = (b.anchorM + b.anchorEndM) / 2;
            mMin = mCenter - widthM / 2;
        }
        const mMax = mMin + widthM; // Blue End in Master Scale
        
        return {
            ...b,
            mStart: isDepart ? 0 : b.anchorM,
            mEnd: isArrivee ? props.totalMainDistance : (b.anchorEndM || b.anchorM),
            mMin,
            mMax,
            vDistStart: (b.points?.[0]?.distance || 0) * 1000,
            vDistEnd: (b.points?.[b.points.length - 1]?.distance || 0) * 1000
        };
    }).sort((a,b) => a.mStart - b.mStart);
});

// Helper: Visual Master Meters (X-axis distance) -> Variant Journey Meters
const visualMasterMetersToVariantMeters = (dm) => {
    let dv = 0;
    let lastMasterM = 0;

    for (const m of sortedModifications.value) {
        // 1. Common section before this modification
        if (dm < m.mStart) {
            return Math.max(0, dv + (dm - lastMasterM));
        }
        dv += (m.mStart - lastMasterM);
        
        // 2. We are inside the master abandoned range [mStart, mEnd]
        if (dm <= m.mEnd) {
            if (dm < m.mMin) {
                // Gray zone before blue: jump to start of variant
                return m.vDistStart;
            } else if (dm <= m.mMax) {
                // Blue zone: linear interpolation
                const ratio = (dm - m.mMin) / (m.mMax - m.mMin || 1);
                return m.vDistStart + ratio * (m.vDistEnd - m.vDistStart);
            } else {
                // Gray zone after blue: jump to end of variant (resuming master)
                return m.vDistEnd;
            }
        }
        
        // Past this modification
        dv = m.vDistEnd;
        lastMasterM = m.mEnd;
    }
    // 3. Final common section
    return Math.max(0, dv + (dm - lastMasterM));
};

// Helper: Variant Journey Meters -> Visual Master Meters (X-axis distance)
const variantMetersToVisualMasterMeters = (vDist) => {
    let cursorV = 0;
    let cursorM = 0;
    const EPS = 0.01; // Small epsilon for float comparison
    
    for (const m of sortedModifications.value) {
        // 1. Common section before this modification
        const commonLen = m.mStart - cursorM;
        // Optimization: if commonLen is 0 (direct start of variant), we skip common check
        // Also use a negative buffer (-EPS) so that if vDist is exactly at the boundary, 
        // we fall into the modification check instead of the preceding common check.
        if (commonLen > 0 && vDist <= (cursorV + commonLen - EPS)) {
            return cursorM + Math.max(0, vDist - cursorV);
        }
        cursorV += commonLen;
        cursorM = m.mStart;
        
        // 2. Variant section (Blue)
        const variantLen = m.vDistEnd - m.vDistStart;
        if (vDist <= (cursorV + variantLen + EPS)) {
            const ratio = Math.min(1, Math.max(0, (vDist - cursorV) / (variantLen || 1)));
            return m.mMin + ratio * (m.mMax - m.mMin);
        }
        cursorV += variantLen;
        cursorM = m.mEnd; 
    }
    // 3. Final common section
    return cursorM + Math.max(0, vDist - cursorV);
};

onMounted(() => {
    processData();
});

// --- Logic ---
async function processData() {
    if (!props.mainTracePoints || props.mainTracePoints.length === 0) return;

    pixelScaleX.value = getSettingValue('Visualisation/Profil Altitude/Graphe/Abscisse') || 2;
    
    // 1. Calculate the Visual Bound of the graph
    let minX = 0;
    let maxX = props.totalMainDistance;
    sortedModifications.value.forEach(m => {
        if (m.mMin < minX) minX = m.mMin;
        if (m.mMax > maxX) maxX = m.mMax;
    });
    minVisualM.value = minX;
    visualSpanM.value = Math.max(1, maxX - minX);

    viewBoxWidth.value = (visualSpanM.value / 100) * pixelScaleX.value;
    const getX = (distFromStartMeters) => ((distFromStartMeters - minVisualM.value) / visualSpanM.value) * viewBoxWidth.value;

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

    const slopeColorMap = await buildSlopeColorsMap(getSettingValue, toHex);

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
            color: getSlopeColor(slope, slopeColorMap),
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
                color: getSlopeColor(slope, slopeColorMap), // Use slope color instead of solid blue
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
            
            connectors.push({ x1: ax, y1: ay, x2: vx, y2: vy, color: getSlopeColor(slope, slopeColorMap) });
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
            
            connectors.push({ x1: vx, y1: vy, x2: ax, y2: ay, color: getSlopeColor(masterSlope, slopeColorMap) });
        }
    });
    altitudeConnectors.value = connectors;

    // 4. Ticks (with Mapping Master -> Variant Distance)
    xTicks.value = [];
    const tickInterval = (getSettingValue('Visualisation/Profil Altitude/Graphe/RepereDistance') || 10) * 1000; 
    
    // Helper to calculate variant distance at a specific master distance
    const getVarDistAtMasterM = (dm) => {
        // Prepare list of modifications with horizontal bounds and variant distance ranges
        const sortedModifs = props.variantBlueSegments.map(b => {
            const isDepart = b.type === 'DEPART';
            const isArrivee = b.type === 'ARRIVEE';
            const widthM = b.lengthM; 
            let mMin = 0; // Start of blue segment on x-axis
            let mMax = 0; // End of blue segment on x-axis
            
            // Master abandoned range [mStart, mEnd]
            let mStart = 0;
            let mEnd = 0;

            if (isDepart) {
                mEnd = b.anchorM;
                mStart = 0;
                mMin = b.anchorM - widthM;
                mMax = b.anchorM;
            } else if (isArrivee) {
                mStart = b.anchorM;
                mEnd = props.totalMainDistance;
                mMin = b.anchorM;
                mMax = b.anchorM + widthM;
            } else {
                mStart = b.anchorM;
                mEnd = b.anchorEndM;
                const mCenter = (b.anchorM + b.anchorEndM) / 2;
                mMin = mCenter - widthM / 2;
                mMax = mMin + widthM;
            }
            
            return {
                ...b,
                mStart,
                mEnd,
                mMin,
                mMax,
                vDistStart: b.points?.[0]?.distance || 0,
                vDistEnd: b.points?.[b.points.length - 1]?.distance || 0
            };
        }).sort((a,b) => a.mStart - b.mStart);

        let dv = 0;
        let lastMasterM = 0;

        for (const m of sortedModifs) {
            // 1. Common section before this modification
            if (dm < m.mStart) {
                return Math.max(0, dv + (dm - lastMasterM));
            }
            dv += (m.mStart - lastMasterM);
            
            // 2. We are inside the master abandoned range [mStart, mEnd]
            if (dm <= m.mEnd) {
                if (dm < m.mMin) {
                   // Gray zone before blue: distance is fixed at start of variant
                   return Math.max(0, m.vDistStart * 1000);
                } else if (dm <= m.mMax) {
                   // Blue zone: distance increases
                   const ratio = (dm - m.mMin) / (m.mMax - m.mMin || 1);
                   return Math.max(0, (m.vDistStart + ratio * (m.vDistEnd - m.vDistStart)) * 1000);
                } else {
                   // Gray zone after blue: distance is fixed at end of variant
                   return Math.max(0, m.vDistEnd * 1000);
                }
            }
            
            // We are past this modification
            dv = m.vDistEnd * 1000;
            lastMasterM = m.mEnd;
        }

        // 3. Final common section
        return Math.max(0, dv + (dm - lastMasterM));
    };

    if (tickInterval > 0) {
        for (let d = 0; d <= props.totalMainDistance; d += tickInterval) {
             const variantDistM = visualMasterMetersToVariantMeters(d);
             let labelValKm = variantDistM / 1000;
             xTicks.value.push({ value: d, position: getX(d), label: `${labelValKm.toFixed(1)}km` });
        }
    }
    
    // Define updateProgressPosition inside processData to access getX and local scope
    // But assign it to a shared ref or export it? 
    // Easier: just expose getX or use a computed for progressX?
    // Let's use a computed property or a watcher that re-calculates.
    // But `getX` uses pixelScaleX etc.
    
    // Solution: Assign the update function to a module-level variable or ref that the watcher can call?
    // Or simply define a standalone getX helper at script setup level?
    // Let's stick to defining it inside for now, but we need to solve the watch.
    
    // Actually, let's just make the watch effective *inside* processData? No.
    // Let's refactor getX to be available. 
    
    // WAIT. I can just copy the getX logic. 
    // getX = (d) => (d / contextTotalDistMeters) * (viewBoxWidth.value - props.padding.left - props.padding.right); (roughly)
    
    // Let's try to find getX first to be sure.

    yTicks.value = [];
    for (let alt = graphMinY; alt <= graphMaxY; alt += altitudeTickInterval) {
        yTicks.value.push({ y: yScale(alt), label: `${alt}m` });
    }

    // Reset scroll when data reloaded
    nextTick(() => {
        if (containerRef.value) containerRef.value.scrollLeft = 0;
    });
}

watch([() => props.totalMainDistance, () => props.abandonedSegments, () => props.mainTracePoints, () => props.variantBlueSegments], () => {
    processData();
});

const updateProgressPosition = () => {
    if (props.totalMainDistance === 0 || visualSpanM.value <= 0) return;
    const getX = (d) => ((d - minVisualM.value) / visualSpanM.value) * viewBoxWidth.value;
    
    const visualMasterM = variantMetersToVisualMasterMeters(props.currentDistance);
    progressX.value = getX(visualMasterM);
};

// Horizontal Scroll Management (Moved to a separate watcher to avoid early return issues)
watch(progressX, (newX) => {
    if (containerRef.value) {
        const containerWidth = containerRef.value.clientWidth;
        if (viewBoxWidth.value > containerWidth) {
            // Scroll to center the cursor if it passes the middle
            let targetScroll = newX - (containerWidth / 2);
            // Clamp scroll
            targetScroll = Math.max(0, Math.min(targetScroll, viewBoxWidth.value - containerWidth));
            containerRef.value.scrollLeft = targetScroll;
        }
    }
});

watch(() => props.currentDistance, updateProgressPosition);

function handleMouseMove(event) {
    if (!containerRef.value || props.totalMainDistance === 0) return;

    const rect = containerRef.value.getBoundingClientRect();
    const hitX = event.clientX - rect.left; 
    const scrollLeft = containerRef.value.scrollLeft;
    const clickXInSvg = hitX + scrollLeft;
    
    hoverLineX.value = clickXInSvg;
    tooltipVisible.value = true;
}

function handleGraphClick(event) {
     if (!containerRef.value || props.totalMainDistance === 0) return;
     const rect = containerRef.value.getBoundingClientRect();
     const scrollLeft = containerRef.value.scrollLeft;
     const clickXInSvg = (event.clientX - rect.left) + scrollLeft;
     
     let targetVisualMasterMeters = ((clickXInSvg / viewBoxWidth.value) * visualSpanM.value) + minVisualM.value;
     targetVisualMasterMeters = Math.max(minVisualM.value, Math.min(targetVisualMasterMeters, minVisualM.value + visualSpanM.value));
     
     const variantDistM = visualMasterMetersToVariantMeters(targetVisualMasterMeters);
     const variantDistKm = variantDistM / 1000;
     
     emits('jump-requested', variantDistKm);
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
</style>
