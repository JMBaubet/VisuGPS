<template>
  <v-container class="fill-height d-flex flex-column" style="max-width: 600px;">
    
    <!-- 1. Playback Controls Row -->
    <v-row class="w-100 flex-grow-0 mb-4" justify="space-between" align="center">
      <v-col cols="4" class="text-center">
        <!-- Rewind needs touch events for hold-to-rewind -->
        <v-btn 
            icon="mdi-rewind" 
            size="x-large" 
            color="secondary" 
            variant="tonal" 
            @mousedown="startRewind" 
            @mouseup="stopRewind"
            @mouseleave="stopRewind"
            @touchstart.prevent="startRewind" 
            @touchend.prevent="stopRewind"
        ></v-btn>
      </v-col>
      <v-col cols="4" class="text-center">
        <v-btn 
            :icon="isPlaying ? 'mdi-pause' : 'mdi-play'" 
            size="80" 
            color="primary" 
            elevation="4"
            @click="togglePlay()"
        ></v-btn>
      </v-col>
      <v-col cols="4" class="text-center">
        <v-btn icon="mdi-refresh" size="x-large" color="secondary" variant="tonal" @click="restart()"></v-btn>
      </v-col>
    </v-row>

    <!-- 2. Speed Control Row -->
    <v-card class="w-100 mb-6 pa-4" variant="outlined" color="surface-variant">
        <div class="d-flex align-center">
            <v-btn size="small" variant="text" color="primary" class="mr-2" @click="resetSpeed()">x1</v-btn>
            <v-slider
                v-model="speedModel"
                :min="0"
                :max="100"
                :step="1"
                hide-details
                color="primary"
                track-color="grey-darken-2"
                thumb-label="always"
                @update:model-value="onSpeedChange"
            >
                <template v-slot:thumb-label="{ modelValue }">
                    {{ displaySpeed(modelValue) }}
                </template>
            </v-slider>
        </div>
    </v-card>

    <!-- 3. Variant Segments (Phase 5) -->
    <VariantSegmentList 
        v-if="isVariantMode" 
        :segments="variantSegments" 
        @select="onSegmentSelect" 
    />

    <v-divider class="w-100 mb-6"></v-divider>

    <!-- 4. Toggles Grid -->
    <!-- mdi-city, mdi-counter, mdi-movie-play-outline, mdi-chart-areaspline-variant, mdi-sun-clock-outline, mdi-compass-outline -->
    <v-row dense>
        <v-col cols="4" sm="2" v-for="toggle in toggles" :key="toggle.id">
            <v-btn 
                block 
                height="60" 
                :color="toggle.isActive ? 'primary' : 'surface-light'" 
                :variant="toggle.isActive ? 'flat' : 'text'"
                class="d-flex flex-column align-center justify-center pa-1"
                @click="sendToggle(toggle.command)"
            >
                <v-icon :icon="toggle.icon" size="24" class="mb-1"></v-icon>
                <span class="text-caption text-truncate w-100">{{ toggle.label }}</span>
            </v-btn>
        </v-col>
    </v-row>

  </v-container>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useRemoteStore } from '@/stores/remoteStore'
import VariantSegmentList from '@/components/VariantSegmentList.vue'

const store = useRemoteStore()

const isVariantMode = computed(() => store.appState?.viewName === 'VisualizeVariantView');
// Mock or real data from store
const variantSegments = computed(() => store.visualizeViewState?.segments || []); 

function onSegmentSelect({ segment, index }) {
    // Command unknown, let's assume 'jump_to_segment'
    store.sendCommand('jump_to_segment', { index });
}

const isPlaying = computed(() => store.visualizeViewState?.animationState === 'En_Animation');

const speedModel = ref(20) 

watch(() => store.visualizeViewState?.currentSpeed, (newSpeed) => {
    if (newSpeed !== undefined) {
        // speed = 0.5 + (val * 0.05) => val = (speed - 0.5) / 0.05
        const val = (newSpeed - 0.5) / 0.05;
        if (Math.abs(speedModel.value - val) > 1) {
             speedModel.value = val;
        }
    }
}, { immediate: true })

function displaySpeed(val) {
    const speed = 0.5 + (val * 0.05);
    return speed.toFixed(1) + 'x';
}

function onSpeedChange(val) {
    const speed = 0.5 + (val * 0.05);
    store.sendCommand('update_speed', { speed });
}

function resetSpeed() {
    speedModel.value = 10; 
    onSpeedChange(10);
}

function togglePlay() {
    store.sendCommand('toggle_play');
}

const isRewinding = ref(false)
function startRewind() {
    isRewinding.value = true
    store.sendCommand('start_rewind')
}
function stopRewind() {
    if (isRewinding.value) {
        isRewinding.value = false
        store.sendCommand('stop_rewind')
    }
}

function restart() {
    store.sendCommand('restart_animation'); 
}

const toggles = computed(() => [
    { 
        id: 'communes', 
        label: 'Villes', 
        icon: 'mdi-city', 
        command: 'toggle_communes', 
        isActive: store.visualizeViewState?.isCommuneWidgetVisible 
    },
    { 
        id: 'distance', 
        label: 'Distance', 
        icon: 'mdi-counter', 
        command: 'toggle_distance_display', 
        isActive: store.visualizeViewState?.isDistanceDisplayVisible 
    },
    { 
        id: 'commands', 
        label: 'Commandes', 
        icon: 'mdi-movie-play-outline', 
        command: 'toggle_commands_widget', 
        isActive: store.visualizeViewState?.isControlsCardVisible 
    },
    { 
        id: 'altitude', 
        label: 'Altitude', 
        icon: 'mdi-chart-areaspline-variant', 
        command: 'toggle_altitude_profile', 
        isActive: store.visualizeViewState?.isAltitudeVisible 
    },
    { 
        id: 'weather', 
        label: 'Météo', 
        icon: 'mdi-sun-clock-outline', 
        command: 'toggle_weather_static', 
        isActive: store.visualizeViewState?.isStaticWeatherVisible 
    },
    { 
        id: 'compass', 
        label: 'Boussole', 
        icon: 'mdi-compass-outline', 
        command: 'toggle_weather_dynamic', 
        isActive: store.visualizeViewState?.isDynamicWeatherVisible 
    }
])

function sendToggle(cmd) {
    store.sendCommand(cmd);
}

</script>
