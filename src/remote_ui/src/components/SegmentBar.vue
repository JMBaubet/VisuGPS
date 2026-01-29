<template>
  <div v-if="segments.length > 0" class="d-flex justify-space-between align-center w-100 px-2 mt-2">
    <v-btn
      v-for="(segment, index) in segments"
      :key="segment.id"
      variant="text"
      size="x-large"
      class="ma-0 pa-0"
      style="height: 60px;"
      :style="{ minWidth: segment.segmentType === 'COMMON' ? '24px' : '60px' }"
      :color="getSegmentColor(segment, index)"
      :class="{'active-glow': index === currentIndex}"
      :ripple="segment.segmentType !== 'COMMON'"
      @click="segment.segmentType !== 'COMMON' && $emit('jump', index)"
    >
      <v-icon :size="getSegmentIconSize(segment.segmentType)">
        {{ getSegmentIcon(segment.segmentType) }}
      </v-icon>
    </v-btn>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  segments: {
    type: Array,
    required: true,
    default: () => []
  },
  currentIndex: {
    type: Number,
    default: null
  }
});

defineEmits(['jump']);

// --- Color Logic (Material Design) ---
const getBaseColor = (type) => {
    switch (type) {
        case 'DEPART': return 'green';
        case 'ARRIVEE': return 'red';
        case 'COMMON': return 'grey'; 
        default: return 'blue'; 
    }
};

const getSegmentColor = (segment, index) => {
    const base = getBaseColor(segment.segmentType);
    
    if (index === props.currentIndex) {
        // ACTIVE: 
        // Common segments stay White for visibility (as requested previously)
        if (segment.segmentType === 'COMMON') return 'white';
        // Others (Start, End, Variant) use their bright accent color (e.g., Blue)
        return `${base}-accent-3`;
    } else {
        // INACTIVE (Past or Future): Dark
        // Using darken-4 for very dark/subtle look, or darken-3
        return `${base}-darken-4`; 
    }
};

const getSegmentIcon = (type) => {
    switch (type) {
        case 'DEPART': return 'mdi-ray-start-arrow';
        case 'ARRIVEE': return 'mdi-ray-end-arrow';
        case 'COMMON': return 'mdi-link-variant'; 
        default: return 'mdi-map-marker-path'; 
    }
};

const getSegmentIconSize = (type) => {
    return type === 'COMMON' ? 12 : 48; // Requested exact size: 12px
};

</script>

<style scoped>
.active-glow {
  transform: scale(1.2);
  filter: drop-shadow(0 0 5px currentColor);
  z-index: 10;
}
</style>
