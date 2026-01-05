<template>
  <div class="compass-widget" :style="{ width: size + 'px', height: size + 'px' }">
    <svg 
      viewBox="0 0 100 100" 
      width="100%" 
      height="100%" 
      xmlns="http://www.w3.org/2000/svg"
    >
      <!-- Main Rotating Group (The Dial + N) -->
      <!-- Rotates opposite to heading to keep N pointing North relative to screen -->
      <g :transform="`rotate(${-heading}, 50, 50)`">
        
        <!-- Dial Circle -->
        <circle cx="50" cy="50" r="48" :fill="dialFill" stroke="currentColor" stroke-width="2" />
        
        <!-- Small Ticks (Cardinals) -->
        <line x1="50" y1="2" x2="50" y2="8" stroke="currentColor" stroke-width="2" /> <!-- N -->
        <line x1="50" y1="92" x2="50" y2="98" stroke="currentColor" stroke-width="2" /> <!-- S -->
        <line x1="92" y1="50" x2="98" y2="50" stroke="currentColor" stroke-width="2" /> <!-- E -->
        <line x1="2" y1="50" x2="8" y2="50" stroke="currentColor" stroke-width="2" /> <!-- W -->

        <!-- Track Arrow (Black, Thin) -->
        <!-- Rotated by traceBearing relative to the Dial (which is at -heading). -->
        <!-- Logic: If Dial is -90 (N is Left), and Bearing is 90 (East), Net is 0 (Up). Correct. -->
        <g :transform="`rotate(${trackBearing}, 50, 50)`">
           <!-- Needle Shape pointing UP -->
           <!-- long thin arrow with dynamic fill and adaptive border -->
           <path d="M50 15 L45 85 L50 80 L55 85 Z" :fill="trackArrowColor" :stroke="mainArrowColor" stroke-width="1" opacity="0.9" />
        </g>

        <!-- Wind Arrow (Adaptive Color, Needle) -->
        <g :transform="`rotate(${windDirection}, 50, 50)`">
            <!-- Needle shape pointing DOWN (Wind Flow: North wind flows South) -->
            <!-- Inverted coordinates of the track arrow -->
            <path d="M50 85 L45 15 L50 20 L55 15 Z" :fill="mainArrowColor" opacity="0.9" />
        </g>

        <!-- North Marker Text -->
        <text 
          class="north-label"
          x="50" 
          y="12" 
          font-family="Arial, sans-serif" 
          font-size="18" 
          font-weight="bold" 
          fill="#FF0000" 
          text-anchor="middle"
          dominant-baseline="central"
        >N</text>

      </g> <!-- End of Rotating Group -->

      <!-- Center Text (Static Orientation) -->
      <g>
        <text 
            x="50" 
            :y="showGusts ? 42 : 50" 
            font-family="Roboto, sans-serif" 
            font-size="18" 
            font-weight="bold" 
            :fill="centerTextColor" 
            :stroke="centerTextStroke" 
            stroke-width="3" 
            paint-order="stroke"
            stroke-linejoin="round"
            text-anchor="middle"
            dominant-baseline="central"
        >{{ Math.round(windSpeed) }}</text>
        <text 
            v-if="showGusts"
            x="50" 
            y="58" 
            font-family="Roboto, sans-serif" 
            font-size="12" 
            font-weight="bold" 
            :fill="centerTextColor" 
            :stroke="centerTextStroke" 
            stroke-width="3" 
            paint-order="stroke"
            stroke-linejoin="round"
            text-anchor="middle"
            dominant-baseline="central"
        >({{ Math.round(windGusts) }})</text>
      </g>
    </svg>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from 'vuetify';

const props = defineProps({
  size: {
    type: Number,
    default: 60
  },
  heading: {
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
  }
});

const theme = useTheme();
const isDark = computed(() => theme.global.current.value.dark);

// Adaptive Colors
const dialFill = computed(() => isDark.value ? 'rgba(50, 50, 50, 0.8)' : 'rgba(255, 255, 255, 0.7)');
const mainArrowColor = computed(() => isDark.value ? 'white' : 'black'); // Top elements
const centerTextColor = computed(() => isDark.value ? 'white' : 'black');
const centerTextStroke = computed(() => isDark.value ? 'black' : 'white');

const showGusts = computed(() => props.windGusts > props.windSpeed + 5);

const windLabel = computed(() => { // Unused but keeping for safety diff min logic
    let text = Math.round(props.windSpeed).toString();
    if (showGusts.value) {
        text += `\n(${Math.round(props.windGusts)})`;
    }
    return text;
});

const trackArrowColor = computed(() => {
    let diff = Math.abs(props.trackBearing - props.windDirection) % 360;
    if (diff > 180) diff = 360 - diff;

    // Colors
    const cRed = { r: 244, g: 67, b: 54 };   // #F44336
    const cYellow = { r: 255, g: 235, b: 59 }; // #FFEB3B
    const cGreen = { r: 76, g: 175, b: 80 };   // #4CAF50

    // Helper for linear interpolation
    const lerp = (start, end, factor) => Math.round(start + (end - start) * factor);
    const colorString = (c) => `rgb(${c.r}, ${c.g}, ${c.b})`;
    const lerpColor = (c1, c2, factor) => ({
        r: lerp(c1.r, c2.r, factor),
        g: lerp(c1.g, c2.g, factor),
        b: lerp(c1.b, c2.b, factor)
    });

    // Zones
    // 0 - 30: Red (Plateau)
    if (diff <= 30) return colorString(cRed);

    // 150 - 180: Green (Plateau)
    if (diff >= 150) return colorString(cGreen);

    // 80 - 100: Yellow (Plateau)
    if (diff >= 80 && diff <= 100) return colorString(cYellow);

    // 30 - 80: Gradient Red -> Yellow
    if (diff > 30 && diff < 80) {
        const factor = (diff - 30) / (80 - 30);
        return colorString(lerpColor(cRed, cYellow, factor));
    }

    // 100 - 150: Gradient Yellow -> Green
    if (diff > 100 && diff < 150) {
        const factor = (diff - 100) / (150 - 100);
        return colorString(lerpColor(cYellow, cGreen, factor));
    }

    return '#9E9E9E'; // Should not be reached
});
</script>

<style scoped>
.compass-widget {
  display: inline-block;
  /* Use current text color for strokes unless overridden */
  color: rgb(var(--v-theme-on-surface)); 
}
</style>
