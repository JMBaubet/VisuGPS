<template>
  <div class="w-100">
    <!-- 1. Playback Controls Row -->
    <v-row class="w-100 flex-grow-0 mb-4" justify="space-between" align="center">
      <v-col cols="4" class="text-center">
        <!-- Rewind needs touch events for hold-to-rewind -->
        <v-btn 
            size="80" 
            rounded="circle"
            :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
            variant="text" 
            @mousedown="startRewind" 
            @mouseup="stopRewind"
            @mouseleave="stopRewind"
            @touchstart.prevent="startRewind" 
            @touchend.prevent="stopRewind"
            :disabled="isFinished || isFlytoActive"
        >
            <v-icon icon="mdi-rewind" size="64"></v-icon>
        </v-btn>
      </v-col>
      <v-col cols="4" class="text-center">
        <v-btn 
            size="80" 
            rounded="circle"
            :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
            variant="text"
            @click="isFinished ? restart() : togglePlay()"
            :disabled="isFlytoActive"
        >
            <v-icon :icon="isFinished ? 'mdi-refresh' : (isPlaying ? 'mdi-pause' : 'mdi-play')" size="72"></v-icon>
        </v-btn>
      </v-col>
      <v-col cols="4" class="text-center">
        <!-- Slot for the 3rd button (x1 or Final View) -->
        <slot name="third-button"></slot>
      </v-col>
    </v-row>

    <!-- 2. Speed Control Row -->
    <!-- Speed -->
    <v-row class="w-100 flex-grow-0 mb-2 mt-0 px-2" align="center">
        <v-col cols="12">
            <v-slider
                v-model="speedModel"
                :min="0"
                :max="100"
                :step="1"
                hide-details
                density="compact"
                :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'"
                :track-color="isDark ? 'grey-lighten-1' : 'grey-darken-1'"
                @update:model-value="onSpeedChange"
                :disabled="isFlytoActive"
            >
                <template v-slot:append>
                    <span :class="isDark ? 'text-grey-lighten-1' : 'text-grey-darken-1'" class="font-weight-bold" style="min-width: 40px; text-align: right;">
                        {{ displaySpeed }}
                    </span>
                </template>
            </v-slider>
        </v-col>
    </v-row>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useRemoteStore } from '@/stores/remoteStore'
import { useTheme } from 'vuetify'

const store = useRemoteStore()
const theme = useTheme()

const isDark = computed(() => theme.global.current.value.dark)

// --- Playback Logic ---
const isPlaying = computed(() => store.visualizeViewState?.animationState === 'En_Animation');
const isFinished = computed(() => store.visualizeViewState?.animationState === 'Termine' || store.visualizeViewState?.animationState === 'Vol_Final');
const isFlytoActive = computed(() => store.visualizeViewState?.isFlytoActive ?? false);

function togglePlay() { store.sendCommand('toggle_play'); }
function restart() { store.sendCommand('restart_animation'); }

const isRewinding = ref(false)
function startRewind() { isRewinding.value = true; store.sendCommand('start_rewind'); }
function stopRewind() { 
    if (isRewinding.value) { 
        isRewinding.value = false; 
        store.sendCommand('stop_rewind'); 
    } 
}

// --- Speed Logic (Logarithmic Scale) ---
const speedModel = ref(50) 

// Fetch settings from store (camelCase IDs)
const minSpeed = computed(() => store.remoteSettings?.speedMinValue ?? 0.1);
const maxSpeed = computed(() => store.remoteSettings?.speedMaxValue ?? 20.0);
const defaultSpeed = computed(() => store.remoteSettings?.speedDefaultValue ?? 1.0);

function mapSliderToSpeed(sliderValue) {
    const min = minSpeed.value;
    const max = maxSpeed.value;
    
    if (sliderValue <= 0) return min;
    if (sliderValue >= 100) return max;
    
    const minLog = Math.log(min);
    const maxLog = Math.log(max);
    
    const logVal = minLog + (maxLog - minLog) * (sliderValue / 100);
    return Math.exp(logVal);
}

function mapSpeedToSlider(speed) {
    const min = minSpeed.value;
    const max = maxSpeed.value;

    if (speed <= min) return 0;
    if (speed >= max) return 100;
    
    const minLog = Math.log(min);
    const maxLog = Math.log(max);
    
    return ((Math.log(speed) - minLog) / (maxLog - minLog)) * 100;
}

watch(() => store.visualizeViewState?.currentSpeed, (newSpeed) => {
    if (newSpeed !== undefined) {
        const val = mapSpeedToSlider(newSpeed);
        // Avoid jitter
        if (Math.abs(speedModel.value - val) > 1) {
             speedModel.value = val;
        }
    }
}, { immediate: true })

const displaySpeed = computed(() => {
    return mapSliderToSpeed(speedModel.value).toFixed(1) + 'x';
});

function onSpeedChange(val) {
    const speed = mapSliderToSpeed(val);
    store.sendCommand('update_speed', { speed });
}

// Expose resetSpeed for parent usage via Template Ref if needed, 
// OR parent handles its own button logic calling the store directly if it's simple,
// BUT here we have encapsulated speed logic.
// Ideally, the "resetSpeed" button in AnimationView should call a method here.
// Let's expose it.
defineExpose({
    resetSpeed: () => {
        const val = mapSpeedToSlider(defaultSpeed.value);
        speedModel.value = val; 
        onSpeedChange(val); 
    }
})

</script>
