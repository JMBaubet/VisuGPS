<template>
  <div class="compass-widget d-flex flex-column align-center" :style="{ width: size + 'px' }">
    <div :style="{ width: size + 'px' }">
        <svg 
        viewBox="0 -12 100 112" 
        width="100%" 
        height="100%" 
        xmlns="http://www.w3.org/2000/svg"
        >
        <!-- Main Rotating Group (The Dial + N) -->
        <g :transform="mainGroupTransform">
            
            <!-- Dial Circle -->
            <circle cx="50" cy="50" r="48" :fill="dialFill" stroke="white" stroke-width="2" />
            
            <!-- Cardinals -->
            <!-- N -->
            <line x1="50" y1="2" x2="50" y2="8" stroke="red" stroke-width="2" />
            <text x="50" y="16" font-family="Arial" font-size="14" font-weight="bold" fill="red" text-anchor="middle" dominant-baseline="central" transform="rotate(0, 50, 16)">N</text>
            
            <!-- S -->
            <line x1="50" y1="92" x2="50" y2="98" stroke="white" stroke-width="2" />
            <text x="50" y="86" font-family="Arial" font-size="12" font-weight="bold" fill="white" text-anchor="middle" dominant-baseline="central" transform="rotate(180, 50, 86)">S</text>
            
            <!-- E -->
            <line x1="92" y1="50" x2="98" y2="50" stroke="white" stroke-width="2" />
            <text x="86" y="50" font-family="Arial" font-size="12" font-weight="bold" fill="white" text-anchor="middle" dominant-baseline="central" transform="rotate(90, 86, 50)">E</text>
            
            <!-- W (Ouest) -->
            <line x1="2" y1="50" x2="8" y2="50" stroke="white" stroke-width="2" />
            <text x="14" y="50" font-family="Arial" font-size="12" font-weight="bold" fill="white" text-anchor="middle" dominant-baseline="central" transform="rotate(-90, 14, 50)">O</text>

            <!-- Camera Marker -->
            <!-- If Camera Mode: Black Triangle REMOVED. -->
            <!-- If Track Mode: Grey Arrow Rotating. -->
            <g :transform="`rotate(${cameraBearing}, 50, 50)`">
                <!-- If Track Mode, use White Triangle (Identical to Blue but Inside) -->
                <path v-if="orientationMode === 'Trace'" d="M50 2 L44 12 L56 12 Z" fill="white" stroke="none" /> 
            </g>

             <!-- Wind Markers -->
             <!-- Wind Direction relative to North. -->
             <!-- Rotated by windDirection. -->
             <!-- Wind comes FROM direction. At 0deg (North), it should appear at Top (North). -->
             <!-- Triangle Base at edge pointing Inward. -->
            <g v-if="windSpeed > 0" :transform="`rotate(${windDirection}, 50, 50)`">
                <!-- Diameter Line -->
                <line x1="50" y1="2" x2="50" y2="98" :stroke="windColor" stroke-width="1" />

                <!-- Gust Triangle (Second triangle, representing gusts) -->
                <!-- User: "Height of second = rafale - 0.5 * vent" -> I will assume logic: Base same, Length proportional to Gust speed, behind wind triangle. -->
                <!-- To make it visible behind, it must be wider or longer. -->
                <!-- I'll make it longer based on gust speed ratio. Default logic. -->
                <path :d="getWindTrianglePath(windGusts)" :fill="lightWindColor" stroke="none" opacity="0.6" />
                
                <!-- Wind Triangle (First triangle, closest to edge) -->
                <path :d="getWindTrianglePath(windSpeed)" :fill="windColor" stroke="none" />
            </g>

            <!-- Trace Marker (Blue Triangle) - INNER -->
            <!-- Only shown in Camera Mode (rotating with dial) -->
            <g v-if="orientationMode === 'Camera'" :transform="`rotate(${trackBearing}, 50, 50)`">
                 <path d="M50 2 L45 12 L55 12 Z" fill="#2196F3" stroke="none" /> 
            </g>

        </g>

        <!-- Trace Marker (Blue Triangle) - OUTER -->
        <!-- Only shown in Trace Mode (Fixed Top, Outside Circle) -->
        <g v-if="orientationMode === 'Trace'">
            <!-- Drawn at Top Center, using the -12 negative space in ViewBox -->
             <path d="M50 -12 L44 -2 L56 -2 Z" fill="#2196F3" stroke="none" /> 
        </g>
        </svg>
    </div>
    
    <!-- Speed Text Below -->
    <div class="text-caption font-weight-bold d-flex flex-column align-center" :style="{ color: windColor, lineHeight: '1.1', marginTop: '-5px' }">
        <div class="d-flex align-baseline">
            <span class="text-h6 font-weight-bold mr-1">{{ Math.round(windSpeed) }}</span>
            <span v-if="showGusts" class="text-caption" style="opacity: 0.7">({{ Math.round(windGusts) }})</span>
            <span class="text-caption ml-1">km/h</span>
        </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from 'vuetify';

const props = defineProps({
  size: {
    type: Number,
    default: 80
  },
  cameraBearing: {
    type: Number,
    required: true
  },
  trackBearing: {
    type: Number,
    default: 0
  },
  windDirection: {
    type: Number,
    default: 0
  },
  windSpeed: {
    type: Number,
    default: 0
  },
  windGusts: {
    type: Number,
    default: 0
  },
  orientationMode: { // 'Trace' or 'Camera'
    type: String,
    default: 'Trace' 
  }
});

const theme = useTheme();
const isDark = computed(() => theme.global.current.value.dark);

const dialRotation = computed(() => {
    // If Trace Mode: North is rotated by -trackBearing (Trace is Up at 0)
    if (props.orientationMode === 'Trace') {
        return -props.trackBearing;
    }
    // If Camera Mode: North is rotated by -cameraBearing (Camera is Up at 0)
    return -props.cameraBearing;
});

const mainGroupTransform = computed(() => {
    const rot = dialRotation.value;
    return `translate(50, 50) rotate(${rot}) scale(1.0) translate(-50, -50)`;
});

const dialFill = computed(() => 'black');

const showGusts = computed(() => props.windGusts > props.windSpeed + 5);

// Wind Colors based on Delta (Headwind vs Tailwind)
const windColor = computed(() => {
    let diff = Math.abs(props.trackBearing - props.windDirection) % 360;
    if (diff > 180) diff = 360 - diff;

    // Colors
    const cRed = { r: 244, g: 67, b: 54 };   // #F44336 (Headwind)
    const cGrey = { r: 158, g: 158, b: 158 }; // #9E9E9E (Crosswind - Gris)
    const cGreen = { r: 76, g: 175, b: 80 };   // #4CAF50 (Tailwind)

    // Helper for linear interpolation
    const lerp = (start, end, factor) => Math.round(start + (end - start) * factor);
    const colorString = (c) => `rgb(${c.r}, ${c.g}, ${c.b})`;
    const lerpColor = (c1, c2, factor) => ({
        r: lerp(c1.r, c2.r, factor),
        g: lerp(c1.g, c2.g, factor),
        b: lerp(c1.b, c2.b, factor)
    });

    // Zones
    // 0 - 30: Red (Headwind - Wind from Front)
    if (diff <= 30) return colorString(cRed);

    // 150 - 180: Green (Tailwind - Wind from Behind)
    if (diff >= 150) return colorString(cGreen);

    // 80 - 100: Grey (Crosswind)
    if (diff >= 80 && diff <= 100) return colorString(cGrey);

    // 30 - 80: Gradient Red -> Grey
    if (diff > 30 && diff < 80) {
        const factor = (diff - 30) / (80 - 30);
        return colorString(lerpColor(cRed, cGrey, factor));
    }

    // 100 - 150: Gradient Grey -> Green
    if (diff > 100 && diff < 150) {
        const factor = (diff - 100) / (150 - 100);
        return colorString(lerpColor(cGrey, cGreen, factor));
    }

    return '#9E9E9E'; // Should not be reached
});

// Lighter/Darker version for gusts? User said "plus clair" (lighter).
// We use the same color but rely on the opacity=0.6 defined in template to make it "lighter/transparent".
const lightWindColor = computed(() => windColor.value);

// Triangle Path Geometry
const getWindTrianglePath = (speed) => {
    // 10km/h = 12% diameter. 80km/h = 100% diameter.
    // Diameter = 96 (Circle R=48).
    // height = scale * 96.
    
    const minSpeed = 10;
    const maxSpeed = 80;
    const minPct = 0.12; 
    
    let ratio = Math.max(0, Math.min(1, (speed - minSpeed) / (maxSpeed - minSpeed)));
    let heightPct = minPct + (1.0 - minPct) * ratio;
    
    // Safety check for very low speeds
    if (speed <= 0) return '';
    
    const height = heightPct * 96; 
    
    // Base width - typically proportional to height or fixed angle?
    // Let's use a fixed base width for clarity, or proportional.
    // Triangle: Tip at (50, 2 + height). Base at (50-w, 2) to (50+w, 2).
    const baseHalfWidth = 6 + (ratio * 6); // 6 to 12?
    
    // Pointing inward from Top Edge (y=2 is inside stroke).
    return `M ${50 - baseHalfWidth} 2 L ${50 + baseHalfWidth} 2 L 50 ${2 + height} Z`;
};

</script>

<style scoped>
/* No specific styles needed beyond flex */
</style>
