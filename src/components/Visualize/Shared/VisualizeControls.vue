<template>
  <div class="bottom-center-container">
    
    <!-- Altitude Chart Slot (Bottom Item in column-reverse) -->
    <transition name="fade">
      <div v-if="isVisible && isAltitudeVisible" class="altitude-svg-container" @wheel.stop>
          <slot name="altitude-chart"></slot>
      </div>
    </transition>

    <!-- Resume / Actions Overlay (Top Item) -->
    <transition name="fade">
      <div v-if="isVisible && isPaused && !controlsVisible" class="d-flex flex-column align-center bottom-controls" @wheel.stop>
          <!-- Extra Actions Slot (Variants etc) -->
          <slot name="extra-overlay-actions"></slot>
      </div>
    </transition>

    <!-- Floating Controls Card (Top Item) -->
    <transition name="fade-opacity">
      <div v-if="isVisible && controlsVisible" class="bottom-controls" title="Afficher/Masquer (Espace)" @wheel.stop>
        <v-card variant="elevated" class="controls-card">
            <div class="d-flex align-center pa-1">
                <v-btn icon="mdi-rewind" variant="text" size="x-small"
                        :disabled="isAnimationFinished"
                        @mousedown="$emit('update:isRewinding', true)"
                        @mouseup="$emit('update:isRewinding', false)" 
                        @mouseleave="$emit('update:isRewinding', false)"
                ></v-btn>
                
                <v-btn :icon="playPauseIcon" variant="text" @click="togglePlayPauseOrReset"></v-btn>
                
                <v-divider vertical class="mx-2"></v-divider>
                
                <v-slider
                    v-model="sliderPosition"
                    :min="0"
                    :max="100"
                    :step="1"
                    hide-details
                    class="align-center speed-slider"
                    :disabled="isAnimationFinished"
                >
                    <template v-slot:append>
                        <span class="speed-value-display">{{ currentSpeed.toFixed(1) }}x</span>
                        <v-btn icon="mdi-numeric-1-box-outline" variant="text" @click="$emit('update:currentSpeed', defaultSpeed)" :disabled="isAnimationFinished"></v-btn>
                    </template>
                </v-slider>

                <v-divider vertical class="mx-2"></v-divider>
                 
                 <slot name="final-action">
                    <v-btn icon="mdi-clock-end" variant="text" size="small"
                            title="Vue Finale (Fin de trace)"
                            :disabled="!isPaused || isAnimationFinished"
                            @click="$emit('trigger-final-view')"
                    ></v-btn>
                 </slot>

                 <!-- Extra Controls Slot (Variant Link etc) -->
                 <slot name="extra-controls"></slot>
          </div>
        </v-card>
      </div>
    </transition>

  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  isVisible: { type: Boolean, default: true },
  isAltitudeVisible: { type: Boolean, default: true },
  
  // Animation State
  isPaused: { type: Boolean, required: true }, // v-model
  isAnimationFinished: { type: Boolean, default: false },
  isRewinding: { type: Boolean, default: false }, // v-model
  
  // Speed
  currentSpeed: { type: Number, required: true }, // v-model
  minSpeed: { type: Number, default: 0.1 },
  maxSpeed: { type: Number, default: 100.0 },
  defaultSpeed: { type: Number, default: 1.0 },
  
  // UI
  controlsVisible: { type: Boolean, default: true }
});

const emits = defineEmits(['update:isPaused', 'update:isRewinding', 'update:currentSpeed', 'reset', 'trigger-final-view']);

const playPauseIcon = computed(() => {
    if (props.isAnimationFinished) return 'mdi-replay';
    return props.isPaused ? 'mdi-play' : 'mdi-pause';
});

const handleResume = () => {
    if (props.isAnimationFinished) {
        emits('reset');
    } else {
        emits('update:isPaused', false);
    }
};

const togglePlayPauseOrReset = () => {
    if (props.isAnimationFinished) {
        emits('reset');
        emits('update:isPaused', false);
    } else {
        emits('update:isPaused', !props.isPaused);
    }
};

// --- Speed Slider Logic (Copied from visualize) ---

const sliderPosition = computed({
    get: () => mapSpeedToSlider(props.currentSpeed),
    set: (val) => emits('update:currentSpeed', mapSliderToSpeed(val))
});

function mapSliderToSpeed(sliderValue) {
    const min = props.minSpeed;
    const max = props.maxSpeed;
    
    if (sliderValue <= 0) return min;
    if (sliderValue >= 100) return max;
    
    const minLog = Math.log(min);
    const maxLog = Math.log(max);
    
    const logVal = minLog + (maxLog - minLog) * (sliderValue / 100);
    return Math.exp(logVal);
}

function mapSpeedToSlider(speed) {
    const min = props.minSpeed;
    const max = props.maxSpeed;
    
    if (speed <= min) return 0;
    if (speed >= max) return 100;
    
    const minLog = Math.log(min);
    const maxLog = Math.log(max);
    
    return ((Math.log(speed) - minLog) / (maxLog - minLog)) * 100;
}
</script>

<style scoped>
.bottom-center-container {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 100%;
  display: flex;
  flex-direction: column-reverse; /* Bottom-up stacking */
  gap: 20px; /* Space between Controls and Altitude */
  align-items: center;
  z-index: 1000;
  pointer-events: none;
}

.altitude-svg-container {
  width: fit-content;
  min-width: 300px; /* Ensure it's not too tiny for controls */
  max-width: 95%; /* Responsive max width */
  height: auto; /* Allow growth */
  min-height: 150px; /* Base height */
  background: rgb(var(--v-theme-surface), 0.9); /* Theme aware */
  border-radius: 8px 8px 0 0; /* Reduced from 10px */
  margin-bottom: 20px; /* Lift from screen bottom */
  pointer-events: auto;
  overflow: hidden; /* Ensure SVG doesn't overflow */
  box-shadow: 0 -2px 10px rgba(0,0,0,0.1);
  display: flex;
  justify-content: center;
  align-items: center;
  backdrop-filter: blur(4px);
}

.bottom-controls {
  /* Position absolute removed to stack cleanly in flex column */
  pointer-events: auto;
  z-index: 1001; /* Above altitude chart */
  /* Margin removed in favor of gap */
}

.controls-card {
  background-color: rgb(var(--v-theme-surface), 0.95) !important;
  border-radius: 4px; /* Reduced from 24px */
  padding: 0 8px;
  min-width: 320px;
  backdrop-filter: blur(4px);
}

.speed-slider {
  width: 200px;
  margin-left: 10px;
  margin-right: 10px;
}

.speed-value-display {
  font-size: 0.75rem;
  font-weight: bold;
  min-width: 35px;
  text-align: right;
  margin-right: 4px;
}

/* Transitions */
.fade-enter-active, .fade-leave-active { transition: opacity 0.5s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
.fade-opacity-enter-active, .fade-opacity-leave-active { transition: opacity 0.5s; }
.fade-opacity-enter-from, .fade-opacity-leave-to { opacity: 0; }
</style>
