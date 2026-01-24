<template>
  <!-- Commune Widget (Positioned Top Left relative to BackButton) -->
  <transition name="fade-opacity">
    <div 
      v-if="isVisible && showCommune && communeName !== 'N/A'" 
      class="commune-display" 
      @wheel.stop
    >
       <v-card variant="elevated" class="pa-2 d-flex align-center commune-card" :style="{ border: `4px solid ${communeBorderColor}` }">
          <span class="font-weight-bold ml-1 mr-1">{{ communeName }}</span>
       </v-card>
    </div>
  </transition>

  <!-- Distance Widget (Top Center) -->
  <div class="top-center-container">
    <transition name="fade-opacity">
      <v-card 
        v-if="isVisible && showDistance" 
        variant="elevated" 
        class="distance-display widget-card" 
        @wheel.stop
      >
        <div class="d-flex align-center justify-center fill-height px-4">
          <span class="font-weight-bold">Distance :&nbsp;</span>
          <span class="font-weight-bold">{{ distanceDisplay }}</span> 
          <span class="font-weight-bold">&nbsp;/ {{ totalDistanceFormatted }} km</span>
        </div>  
      </v-card>
    </transition>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  isVisible: { type: Boolean, default: true },
  
  // Distance
  showDistance: { type: Boolean, default: true },
  distanceDisplay: { type: String, default: '0.00' },
  totalDistance: { type: Number, default: 0 },
  
  // Commune
  showCommune: { type: Boolean, default: false },
  communeName: { type: String, default: 'N/A' },
  communeBorderColor: { type: String, default: '#F44336' }
});

const totalDistanceFormatted = computed(() => {
    return props.totalDistance.toFixed(2);
});
</script>

<style scoped>
.top-center-container {
  position: absolute;
  top: 10px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 1000;
  pointer-events: none;
}

.commune-display {
  position: absolute;
  top: 20px;
  left: 80px; /* Offset for Back Button */
  z-index: 999;
  pointer-events: none; /* Container none, card auto */
}

/* Common Widget Styling (Theme Aware) - Used for Distance */
.widget-card {
  pointer-events: auto;
  backdrop-filter: blur(4px);
  background-color: rgb(var(--v-theme-surface), 0.9) !important;
}

/* Specific Commune Styling (French Sign Style: EB10) */
.commune-card {
  pointer-events: auto;
  background-color: white !important;
  color: black !important;
  border-radius: 4px !important;
  /* Border is handled via inline style for dynamic color if needed, or we can fix it here if always red */
  font-weight: 900 !important; /* Bold like a sign */
  font-family: "Tauri", sans-serif; /* Or generic sans-serif if Tauri font not loaded */
  text-transform: uppercase;
  letter-spacing: 1px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.2) !important;
}

.distance-display {
  min-width: 200px;
  height: 40px;
  border-radius: 4px !important;
}

/* Transitions */
.fade-opacity-enter-active, .fade-opacity-leave-active { transition: opacity 0.5s; }
.fade-opacity-enter-from, .fade-opacity-leave-to { opacity: 0; }
</style>
