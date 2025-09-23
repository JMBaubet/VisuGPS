<template>
  <div class="progress-widget">
    <div class="d-flex flex-column align-center">
      <span class="value">{{ currentDistance.toFixed(2) }} / {{ totalLength.toFixed(2) }} km</span>
      <span class="label">Progression</span>
    </div>
  </div>
</template>

<script setup>
defineProps({
  totalLength: {
    type: Number,
    required: true,
  },
  currentDistance: {
    type: Number,
    required: true,
  },
});
</script>

<style scoped>
.progress-widget {
  position: absolute;
  top: 10px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1000;
  background-color: rgba(var(--v-theme-surface), 0.8);
  backdrop-filter: blur(4px);
  border-radius: 8px;
  padding: 4px 16px;
  text-align: center;
}

.value {
  font-family: monospace;
  font-size: 1.1rem;
  font-weight: bold;
}

.label {
  font-size: 0.75rem;
  text-transform: uppercase;
  opacity: 0.8;
}
</style>
