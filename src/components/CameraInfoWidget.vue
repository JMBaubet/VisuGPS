<template>
  <v-card class="camera-widget" elevation="4" variant="tonal">
    <v-card-text class="pa-2">
      <div class="d-flex justify-space-between align-center">
        <span class="label">Cap:</span>
        <span class="value">{{ bearing.toFixed(0) }}°</span>
      </div>
      <v-divider class="my-1"></v-divider>
      <div class="d-flex justify-space-between align-center">
        <span class="label">Zoom:</span>
        <span class="value" :class="{ 'text-green': isZoomDefault }">{{ zoom.toFixed(1) }}</span>
      </div>
      <v-divider class="my-1"></v-divider>
      <div class="d-flex justify-space-between align-center">
        <span class="label">Pitch:</span>
        <span class="value" :class="{ 'text-green': isPitchDefault }">{{ pitch.toFixed(0) }}°</span>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  bearing: {
    type: Number,
    required: true,
  },
  zoom: {
    type: Number,
    required: true,
  },
  pitch: {
    type: Number,
    required: true,
  },
  defaultZoom: {
    type: Number,
    required: true,
  },
  defaultPitch: {
    type: Number,
    required: true,
  },
});

// Compare with a small tolerance to handle floating point inaccuracies
const isZoomDefault = computed(() => Math.abs(props.zoom - props.defaultZoom) < 0.05);
const isPitchDefault = computed(() => Math.abs(props.pitch - props.defaultPitch) < 0.5);
</script>

<style scoped>
.camera-widget {
  position: absolute;
  top: 10px;
  right: 10px;
  z-index: 1000;
  width: 180px;
  background-color: rgba(var(--v-theme-surface), 0.8);
  backdrop-filter: blur(4px);
}

.label {
  font-weight: bold;
  font-size: 0.9rem;
}

.value {
  font-family: monospace;
  font-size: 1rem;
}

.text-green {
  color: rgb(var(--v-theme-success));
  font-weight: bold;
}
</style>
