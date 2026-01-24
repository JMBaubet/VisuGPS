<template>
  <!-- Map Container (Absolute Full) -->
  <div ref="mapVisualizationContainer" class="map-container">
      
      <!-- Back Button (Floating Top Left) -->
      <transition name="fade">
        <div v-if="showBackButton" class="back-button">
          <v-btn
            icon="mdi-arrow-left"
            variant="elevated"
            density="comfortable"
            @click="$emit('go-back')"
            title="Revenir"
          ></v-btn>
        </div>
      </transition>

      <!-- Center Cross Marker (Optional) -->
      <div v-if="isCenterMarkerVisible" class="center-marker">
          <svg width="20" height="20" viewBox="0 0 20 20">
              <line x1="0" y1="10" x2="20" y2="10" :stroke="toHex(props.centerMarkerColor) || 'red'" stroke-width="2" />
              <line x1="10" y1="0" x2="10" y2="20" :stroke="toHex(props.centerMarkerColor) || 'red'" stroke-width="2" />
          </svg>
      </div>

  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const props = defineProps({
  isCursorHidden: { type: Boolean, default: false },
  isCenterMarkerVisible: { type: Boolean, default: false },
  centerMarkerColor: { type: String, default: 'red' },
  showBackButton: { type: Boolean, default: true }
});

const emit = defineEmits(['go-back', 'register-map-container']);

const mapVisualizationContainer = ref(null);
const { toHex } = useVuetifyColors();

onMounted(() => {
    emit('register-map-container', mapVisualizationContainer.value);
});
</script>

<style scoped>
.map-container {
  width: 100vw;
  height: 100vh;
  position: absolute;
  top: 0;
  left: 0;
  overflow: hidden;
}

.back-button {
  position: absolute;
  top: 20px;
  left: 20px;
  z-index: 1000;
  /* background-color automatically handled by v-btn variant or default */
}

.center-marker {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
  z-index: 500;
}

/* Transitions */
.fade-enter-active, .fade-leave-active { transition: opacity 0.5s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
