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

          <!-- Gap Rects (Skipped Zones) -->
          <g class="gaps" v-if="gapRects.length > 0">
              <rect v-for="(g, i) in gapRects" :key="`gap-${i}`"
                  :x="g.x" :y="g.y" :width="g.width" :height="g.height"
                  fill="#e0e0e0" stroke="none" opacity="0.5" />
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

          <!-- Variant Reference Points (Single Dots) -->
          <g class="variant-ref" v-if="variantReferencePoints.length > 0">
              <circle v-for="(p, i) in variantReferencePoints" :key="`vref-${i}`"
                  :cx="p.cx" :cy="p.cy" :r="p.r" :fill="p.fill" stroke="white" stroke-width="0.5" />
          </g>

          <!-- Comparison Connectors -->
          <g class="comparison-connectors" v-if="comparisonConnectors.length > 0">
              <line v-for="(l, i) in comparisonConnectors" :key="`conn-${i}`"
                  :x1="l.x1" :y1="l.y1" :x2="l.x2" :y2="l.y2"
                  :stroke="l.color || '#00ffff'" stroke-width="1.5" stroke-dasharray="4 2" />
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
import { ref, onMounted, watch, computed, nextTick } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const props = defineProps({
    circuitId: { type: String, required: true },
    currentDistance: { type: Number, required: true },
    totalDistance: { type: Number, required: true }, // In Meters
    trackingPoints: { type: Array, required: true },
    // COMPARISON PROPS
    isVariantComparison: { type: Boolean, default: false },
    mainTracePoints: { type: Array, default: () => [] },
    variantSegments: { type: Array, default: () => [] }, // New Multi-Segment Prop
    currentSegmentIndex: { type: Number, default: null }, // Which segment is currently selected
    padding: {
        type: Object,
        default: () => ({ top: 10, right: 10, bottom: 30, left: 45 })
    }
});

// --- Refs ---
const containerRef = ref(null);

onMounted(() => {
    nextTick(() => {
        updateProgressX(props.currentDistance);
    });
});

const viewBoxWidth = ref(1000);
const svgHeight = ref(210);
const progressX = ref(0);
const pathSegments = ref([]);
const gapRects = ref([]); // New: Skipped Zones
const comparisonConnectors = ref([]);
const mainTraceReferencePoints = ref([]); 
const variantReferencePoints = ref([]); // Restore Ref
const xTicks = ref([]);
const yTicks = ref([]);
const totalDrawableDistance = ref(0);
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
        if (typeof val === 'string') val = val.replace(',', '.');
        const num = parseFloat(val);
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
    
    // Calculate edges ensuring non-negative width
    const leftEdge = px - cw;
    const rightEdge = px;
    
    const visibleLeft = Math.max(0, leftEdge);
    const visibleRight = Math.max(0, rightEdge);
    
    const width = Math.max(0, visibleRight - visibleLeft);
    
    return { x: visibleLeft, width };
});

// --- Logic ---
async function processData() {
    if (!props.trackingPoints) {
        pathSegments.value = [];
        return;
    }
    progressX.value = 0;
    
    // Check comparison validity
    const isValidComparison = props.isVariantComparison && 
                              props.mainTracePoints && 
                              props.mainTracePoints.length > 0 &&
                              props.variantSegments && 
                              props.variantSegments.length > 0;

    // Standard mode data (Variant or Single Trace)
    const filteredTrackingData = props.trackingPoints.filter(p => typeof p.altitude === 'number' && !isNaN(p.altitude));
    if (filteredTrackingData.length < 2 && !isValidComparison) {
        pathSegments.value = [];
        return;
    }

    // --- 1. Determine Context (X-Axis World) & Altitude Range ---
    let contextStartKm = 0;
    let contextEndKm = props.totalDistance / 1000;
    
    // If comparing, World = Main Trace
    if (isValidComparison) {
        const lastPt = props.mainTracePoints[props.mainTracePoints.length - 1];
        contextEndKm = lastPt ? lastPt.distance : contextEndKm;
    }
    
    const contextTotalDistMeters = (contextEndKm - contextStartKm) * 1000;
    totalDrawableDistance.value = contextTotalDistMeters;

    // Scaling Factors
    const pixelsFor10Meters = getSettingValue('Visualisation/Profil Altitude/Graphe/Ordonnee') || 10;
    const pixelsPerMeter = pixelsFor10Meters / 10;
    
    // Altitude Range
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

    const getX = (distFromStartMeters) => (distFromStartMeters / contextTotalDistMeters) * viewBoxWidth.value;

    const getSlopeColor = (slope) => {
        if (slope <= 0) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/TrancheNegative') || 'light-blue');
        if (slope < 3) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche1') || 'green');
        if (slope < 6) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche2') || 'yellow');
        if (slope < 9) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche3') || 'orange');
        if (slope < 12) return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche4') || 'red');
        return toHex(getSettingValue('Visualisation/Profil Altitude/Couleurs/Tranche5') || 'purple');
    };

    const generateSegments = (points, xOffsetMeters = 0) => {
        const segs = [];
        const graphBottomY = svgHeight.value - props.padding.bottom;
        for (let i = 1; i < points.length; i++) {
            const p1 = points[i - 1];
            const p2 = points[i];
            const distDiff = (p2.distance - p1.distance) * 1000;
            const altDiff = p2.altitude - p1.altitude;
            const slope = distDiff > 0 ? (altDiff / distDiff) * 100 : 0;
            
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
    gapRects.value = [];
    comparisonConnectors.value = [];
    mainTraceReferencePoints.value = [];
    variantReferencePoints.value = []; // Clear Ref
    
    if (isValidComparison) {
         // --- Multi-Segment Comparison Logic ---
         
         // 1. Define Skipped Zones
         const skippedZones = [];
         
         props.variantSegments.forEach(seg => {
             // Access anchors populated in VisualizeView
             const sKm = seg.mainStartDistKm || 0;
             const eKm = seg.mainEndDistKm || 0;
             
             // Define zones based on segment type
             if (seg.type === 'DEPART_DEPORTE') {
                 // Skip from 0 to EndAnchor
                 skippedZones.push({ start: 0, end: eKm });
             } else if (seg.type === 'ARRIVEE_REPORTEE') {
                 // Skip from StartAnchor to TotalMain
                 skippedZones.push({ start: sKm, end: contextEndKm }); // Approx
             } else {
                 // Skip from StartAnchor to EndAnchor
                 skippedZones.push({ start: sKm, end: eKm });
             }
         });
         
         // Merge zones? For now assume they are disjoint or sequential.
         // Better to sort by start.
         // Better to sort by start.
         skippedZones.sort((a,b) => a.start - b.start);
         
         // 2. Render Main Trace (Valid = Path, Bypassed = Dots)
         const mainPoints = props.mainTracePoints;
         let lastValidEnd = 0;
         
         skippedZones.forEach(zone => {
             // A. VALID CHUNK (Before Gap)
             if (zone.start > lastValidEnd) {
                 const chunk = mainPoints.filter(p => p.distance >= lastValidEnd && p.distance <= zone.start);
                 if (chunk.length > 1) {
                     pathSegments.value.push(...generateSegments(chunk));
                 }
             }
             
             // B. BYPASSED CHUNK (Inside Gap) -> Dots
             const chunkInGap = mainPoints.filter(p => p.distance >= zone.start && p.distance <= zone.end);
             chunkInGap.forEach((p, i) => {
                 // Revert to simple grey dots as requested
                 mainTraceReferencePoints.value.push({
                     cx: getX(p.distance * 1000),
                     cy: yScale(p.altitude),
                     r: 1,
                     fill: '#B0B0B0' // Light grey for subtle display
                 });
             });
             
             lastValidEnd = zone.end;
         });
         
         // Final Valid Chunk
         if (lastValidEnd < contextEndKm) {
             const chunk = mainPoints.filter(p => p.distance >= lastValidEnd);
             if (chunk.length > 1) {
                 pathSegments.value.push(...generateSegments(chunk));
             }
         }
         
         // 3. Render Variant Segments
         props.variantSegments.forEach(seg => {
             if (seg.points && seg.points.length > 0) {
                 const sKm = seg.mainStartDistKm || 0;
                 const eKm = seg.mainEndDistKm || 0;
                 const gapLenM = (eKm - sKm) * 1000;
                 const variantLenM = seg.lengthKm * 1000;
                 
                 let offsetM = 0;
                 
                  // DEPART: Variant End aligns with Main Anchor End
                 if (seg.type === 'DEPART_DEPORTE') {
                     const anchorEndM = eKm * 1000;
                     const variantStartM = anchorEndM - variantLenM;
                     offsetM = variantStartM;
                 }
                 // ARRIVEE: Variant Start aligns with Main Anchor Start
                 else if (seg.type === 'ARRIVEE_REPORTEE') {
                     const anchorStartM = sKm * 1000;
                     offsetM = anchorStartM;
                 }
                 // SEGMENT: Center in gap
                 else {
                     const anchorStartM = sKm * 1000;
                     const centerDelta = (gapLenM - variantLenM) / 2;
                     offsetM = anchorStartM + centerDelta;
                 }

                 // Points in seg.points have CUMULATIVE distance relative to Variant Start.
                 // Example: Segment 2 might start at 5km. We want to place it at [OffsetM].
                 // If we pass OffsetM to generateSegments, result X = (PointDist + OffsetM).
                 // X = 5000 + OffsetM. WRONG.
                 // We want X = OffsetM. So we need to shift by (OffsetM - PointDistStart).
                 
                 const segmentStartDistM = (seg.startDistKm || 0) * 1000;
                 const xOffsetMeters = offsetM - segmentStartDistM;
                 
                 
                 // Generate Variant Path
                 let vSegs = [];
                 if (seg.points.length >= 2) {
                     vSegs = generateSegments(seg.points, xOffsetMeters);
                     pathSegments.value.push(...vSegs);
                 } else if (seg.points.length === 1) {
                      variantReferencePoints.value.push({
                         cx: getX((seg.points[0].distance*1000) + xOffsetMeters),
                         cy: yScale(seg.points[0].altitude),
                         r: 3,
                         fill: seg.type === 'DEPART_DEPORTE' ? '#4CAF50' : '#F44336'
                     });
                 }
                 
                 // 4. Connectors
                 // Variant Points: p[0] is start, p[last] is end.
                 // Main Points: At sKm and eKm.
                 
                 const vStartP = seg.points[0];
                 const vEndP = seg.points[seg.points.length-1];
                 const vStartX = getX((vStartP.distance*1000) + xOffsetMeters);
                 const vStartY = yScale(vStartP.altitude);
                 const vEndX = getX((vEndP.distance*1000) + xOffsetMeters);
                 const vEndY = yScale(vEndP.altitude);
                 
                 // Main Anchors Y
                 // Find main point close to sKm/eKm
                 const mStartP = mainPoints.find(p => Math.abs(p.distance - sKm) < 0.01);
                 const mEndP = mainPoints.find(p => Math.abs(p.distance - eKm) < 0.01);
                 
                 if (mStartP && seg.type !== 'DEPART_DEPORTE') {
                     // Connect Main Start to Variant Start (Cyan colored line replaced by Slope Color)
                     // "connecté par une ligne de la couleur de la pente du variant" -> First slope color
                     const slopeColor = (vSegs.length > 0) ? (vSegs[0]?.color || '#00ffff') : '#00ffff';
                     comparisonConnectors.value.push({
                         x1: getX(sKm*1000), y1: yScale(mStartP.altitude),
                         x2: vStartX, y2: vStartY,
                         color: slopeColor
                     });
                 }
                 
                 if (mEndP && seg.type !== 'ARRIVEE_REPORTEE') {
                     // Connect Variant End to Main End
                     const slopeColor = (vSegs.length > 0) ? (vSegs[vSegs.length-1]?.color || '#00ffff') : '#00ffff';
                     comparisonConnectors.value.push({
                         x1: vEndX, y1: vEndY,
                         x2: getX(eKm*1000), y2: yScale(mEndP.altitude),
                         color: slopeColor
                     });
                 }
             }
         });
         
    } else {
        pathSegments.value = generateSegments(filteredTrackingData);
    }
    
    // --- Ticks ... (Standard) ---
    yTicks.value = [];
    for (let alt = graphMinY; alt <= graphMaxY; alt += altitudeTickInterval) {
        yTicks.value.push({ y: yScale(alt), label: `${alt}m` });
    }
    xTicks.value = [];
    const tickInterval = (getSettingValue('Visualisation/Profil Altitude/Graphe/RepereDistance') || 10) * 1000; 
    if (tickInterval > 0) {
        for (let d = 0; d <= contextTotalDistMeters; d += tickInterval) {
            let labelValKm = (contextStartKm * 1000 + d) / 1000;
             xTicks.value.push({ value: d, position: getX(d), label: `${labelValKm.toFixed(1)}km` });
        }
    }
}

watch(() => [props.trackingPoints, props.totalDistance, props.isVariantComparison, props.mainTracePoints, props.variantSegments], 
      processData, { deep: true, immediate: true });

function updateProgressX(newDistance) {
    if (!containerRef.value || totalDrawableDistance.value === 0) return;
    
    let xPos = 0;
    if (props.isVariantComparison && props.variantSegments.length > 0) {
        
        let found = false;
        const dKm = newDistance / 1000;
        
        for (let i = 0; i < props.variantSegments.length; i++) {
            const seg = props.variantSegments[i];
            const isLast = i === props.variantSegments.length - 1;
            const isZeroLength = Math.abs(seg.endDistKm - seg.startDistKm) < 0.001;
            
            let matches = false;
            
            // If currentSegmentIndex is provided and this segment's index matches,
            // AND the distance is approximately correct, force this segment
            if (props.currentSegmentIndex !== null && seg.index === props.currentSegmentIndex) {
                const distanceMatches = Math.abs(dKm - seg.startDistKm) < 0.05; // 50m tolerance
                if (distanceMatches) {
                    matches = true;
                } else {
                    // Current segment but distance is way off - continue search
                    matches = false;
                }
            } else {
                // Normal matching logic
                if (isZeroLength) {
                    const exactMatch = Math.abs(dKm - seg.startDistKm) < 0.001;
                    if (exactMatch) {
                        // Check if there's a normal segment starting at the same position
                        const hasNormalSegmentHere = props.variantSegments.some((s, idx) => {
                            const isNormalLength = Math.abs(s.endDistKm - s.startDistKm) >= 0.001;
                            const startsSamePlace = Math.abs(s.startDistKm - seg.startDistKm) < 0.001;
                            return isNormalLength && startsSamePlace;
                        });
                        
                        // Only match if there's no normal segment here
                        matches = !hasNormalSegmentHere;
                    }
                } else {
                    // Logic to favor 'Next Segment' at boundary (Start of Seg 2 vs End of Seg 1)
                    // If dKm is exactly at boundary, we want Seg 2.
                    const matchesStart = dKm >= seg.startDistKm - 0.001;
                    const matchesEnd = isLast ? (dKm <= seg.endDistKm + 0.001) : (dKm < seg.endDistKm - 0.001);
                    matches = matchesStart && matchesEnd;
                }
            }
            
            if (matches) {
                const localDistInSegM = (dKm - seg.startDistKm) * 1000;
                
                // Calculate visual offset of this segment
                const sKm = seg.mainStartDistKm || 0;
                const eKm = seg.mainEndDistKm || 0;
                const gapLenM = (eKm - sKm) * 1000;
                const variantLenM = seg.lengthKm * 1000;
                
                let offsetM = 0;
                // Correct Offset Logic (Must match processData)
                if (seg.type === 'DEPART_DEPORTE') {
                     // offsetM = (eKm*1000) - variantLenM; 
                     // Wait, processData uses xOffsetMeters calculation.
                     // The visual position X = getX(offsetM + localDist).
                     // In processData: cx: getX((seg.points[0].distance*1000) + xOffsetMeters)
                     // xOffsetMeters = offsetM - seg.startDistKm*1000
                     
                     // Here we want GlobalVisualX.
                     // GlobalVisualX = xOffsetMeters + (dKm * 1000) ?? No.
                     
                     // Let's use the same offsetM logic as processData
                     offsetM = (eKm*1000) - variantLenM;
                }
                else if (seg.type === 'ARRIVEE_REPORTEE') offsetM = sKm*1000;
                else offsetM = (sKm*1000) + (gapLenM - variantLenM)/2;
                
                // processData uses: xOffsetMeters = offsetM - segmentStartDistM
                // Point X = getX(PointDist + xOffsetMeters)
                // PointDist = dKm * 1000 (roughly, strictly it's dist inside LineString)
                // But dKm here IS the cumulative dist inside LineString (stitched).
                
                const segmentStartDistM = seg.startDistKm * 1000;
                const xOffsetMeters = offsetM - segmentStartDistM;
                
                const visualDistM = (dKm * 1000) + xOffsetMeters;
                
                xPos = (visualDistM / totalDrawableDistance.value) * viewBoxWidth.value;
                found = true;
                break;
            }
        }
        if (!found) {
             console.warn('[AltitudeSVG] No segment found for dKm:', dKm);
             xPos = 0; 
        }
        
    } else {
        xPos = (newDistance / totalDrawableDistance.value) * viewBoxWidth.value;
    }
    
    if (isNaN(xPos)) xPos = 0;
    progressX.value = xPos;
    
    // Scroll Logic
    const containerWidth = containerRef.value.clientWidth;
    const stuckPositionKm = getSettingValue('Visualisation/Profil Altitude/Graphe/CurseurPositionKm') || 10;
    const stuckPositionPx = (stuckPositionKm * 1000 / totalDrawableDistance.value) * viewBoxWidth.value;
    let scrollLeft = 0;
    const transitionPoint = viewBoxWidth.value - containerWidth + stuckPositionPx;
    if (progressX.value < stuckPositionPx) scrollLeft = 0;
    else if (progressX.value >= transitionPoint) scrollLeft = viewBoxWidth.value - containerWidth;
    else scrollLeft = progressX.value - stuckPositionPx;
    
    if (containerRef.value.scrollLeft !== scrollLeft) {
        containerRef.value.scrollLeft = scrollLeft;
    }
}

watch(() => props.currentDistance, (newDistance) => {
    lastUpdatedDistance = newDistance;
    updateProgressX(newDistance);
});

// Watch segment index changes too (e.g., when switching between DEPART and SEGMENT at same distance)
watch(() => props.currentSegmentIndex, () => {
    if (props.currentDistance !== undefined) {
        updateProgressX(props.currentDistance);
    }
});

function handleMouseMove(event) {
    // Tooltip logic (needs update for comparison?)
    // For now, keep standard logic or disable tooltip on gaps?
    // Standard tooltip logic maps MouseX -> GlobalDist -> Index.
    // In comparison mode, GlobalDist might be a gap.
    // Ideally we map MouseX back to the visible segment. 
    // This is complex. Keeping standard logic might show Main Trace info (since it's the context).
    // Let's postpone complex tooltip refactor.
}

function handleMouseLeave() {
    tooltipVisible.value = false;
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
.axis text { font-family: sans-serif; font-size: 10px; fill: #888; text-anchor: middle; }
.progress-bar { stroke: white; stroke-width: 1.5; }
rect { fill: white; }
.connector-line { stroke-width: 1.5; stroke-dasharray: 4 2; opacity: 0.8; }
.hover-line { stroke: rgba(255, 255, 255, 0.7); stroke-width: 1; stroke-dasharray: 4 2; }
.tooltip { position: absolute; background: rgba(255,255,255,0.75); padding: 8px; border-radius: 5px; font-size: 12px; pointer-events: none; z-index: 9999; }
</style>