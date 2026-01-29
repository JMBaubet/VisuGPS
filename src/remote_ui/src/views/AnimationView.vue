<template>
  <v-container class="fill-height d-flex flex-column" style="max-width: 600px;">
    
    <PlaybackControls ref="playbackControls">
        <template #third-button>
             <v-btn 
                size="80" 
                rounded="circle"
                :color="isDark ? 'white' : 'grey-darken-3'" 
                variant="text" 
                class="text-h4 font-weight-bold"
                @click="resetSpeed()"
            >x1</v-btn>
        </template>
    </PlaybackControls>

    <!-- 3. Variant Segments (Phase 5) -->
    <VariantSegmentList 
        v-if="isVariantMode" 
        :segments="variantSegments" 
        @select="onSegmentSelect" 
    />

    <v-divider class="w-100 mb-6"></v-divider>
    
    <!-- 4. Toggles Grid - Forced 2 Rows -->
    <!-- Row 1: Top 3 (Altitude, Commandes, Boussole) -->
    <v-row class="w-100 flex-grow-0 mb-0" justify="space-between" align="center">
        <v-col cols="4" class="text-center pa-1" v-for="toggle in toggles.slice(0, 3)" :key="toggle.id">
            <v-btn 
                :icon="toggle.icon"
                size="x-large"
                :color="toggle.isActive ? (isDark ? 'green-accent-3' : 'green-darken-1') : (isDark ? 'white' : 'grey-darken-3')"
                variant="text"
                class="ma-1 pa-0"
                style="width: 80px; height: 80px;"
                @click="sendToggle(toggle.command)"
            >
                <v-icon :icon="toggle.icon" size="64"></v-icon>
            </v-btn>
        </v-col>
    </v-row>

    <!-- Row 2: Bottom 3 (Villes, Distance, Météo) -->
    <v-row class="w-100 flex-grow-0 mt-0" justify="space-between" align="center">
        <v-col cols="4" class="text-center pa-1" v-for="toggle in toggles.slice(3, 6)" :key="toggle.id">
            <v-btn 
                :icon="toggle.icon"
                size="x-large"
                :color="toggle.isActive ? (isDark ? 'green-accent-3' : 'green-darken-1') : (isDark ? 'white' : 'grey-darken-3')"
                variant="text"
                class="ma-1 pa-0"
                style="width: 80px; height: 80px;"
                @click="sendToggle(toggle.command)"
            >
                <v-icon :icon="toggle.icon" size="64"></v-icon>
            </v-btn>
        </v-col>
    </v-row>

  </v-container>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRemoteStore } from '@/stores/remoteStore'
import VariantSegmentList from '@/components/VariantSegmentList.vue'
import PlaybackControls from '@/components/PlaybackControls.vue'
import { useTheme } from 'vuetify'

const store = useRemoteStore()
const theme = useTheme()

const isDark = computed(() => theme.global.current.value.dark)

const isVariantMode = computed(() => store.appState?.viewName === 'VisualizeVariantView');
// Mock or real data from store
const variantSegments = computed(() => store.visualizeViewState?.segments || []); 

function onSegmentSelect({ segment, index }) {
    // Command unknown, let's assume 'jump_to_segment'
    store.sendCommand('jump_to_segment', { index });
}

const playbackControls = ref(null)

function resetSpeed() {
    playbackControls.value?.resetSpeed()
}

// Playback and Speed logic moved to PlaybackControls component
// const isPlaying = computed(() => store.visualizeViewState?.animationState === 'En_Animation');
// const isFinished = computed(() => store.visualizeViewState?.animationState === 'Termine' || store.visualizeViewState?.animationState === 'Vol_Final');

// const speedModel = ref(50) 

// --- Speed Logic (Logarithmic Scale) ---
// Fetch settings from store (camelCase IDs from Rust struct)
// const minSpeed = computed(() => store.remoteSettings?.speedMinValue ?? 0.1);
// const maxSpeed = computed(() => store.remoteSettings?.speedMaxValue ?? 20.0);
// const defaultSpeed = computed(() => store.remoteSettings?.speedDefaultValue ?? 1.0);

// function mapSliderToSpeed(sliderValue) {
//     const min = minSpeed.value;
//     const max = maxSpeed.value;
    
//     // ... logic consistent
//     if (sliderValue <= 0) return min;
//     if (sliderValue >= 100) return max;
    
//     const minLog = Math.log(min);
//     const maxLog = Math.log(max);
    
//     const logVal = minLog + (maxLog - minLog) * (sliderValue / 100);
//     return Math.exp(logVal);
// }

// function mapSpeedToSlider(speed) {
//     const min = minSpeed.value;
//     const max = maxSpeed.value;

//     if (speed <= min) return 0;
//     if (speed >= max) return 100;
    
//     const minLog = Math.log(min);
//     const maxLog = Math.log(max);
    
//     return ((Math.log(speed) - minLog) / (maxLog - minLog)) * 100;
    


const toggles = computed(() => [
    { 
        id: 'communes', 
        label: 'Villes', 
        icon: 'mdi-city', 
        command: 'toggle_communes_display', 
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
        id: 'weather', 
        label: 'Météo', 
        icon: 'mdi-sun-clock-outline', 
        command: 'toggle_weather_static', 
        isActive: store.visualizeViewState?.isStaticWeatherVisible 
    },
    { 
        id: 'altitude', 
        label: 'Altitude', 
        icon: 'mdi-chart-areaspline-variant', 
        command: 'toggle_altitude_profile', 
        isActive: store.visualizeViewState?.isAltitudeVisible 
    },
    { 
        id: 'commands', 
        label: 'Commandes', 
        icon: 'mdi-movie-play-outline', 
        command: 'toggle_commands_widget', 
        isActive: store.visualizeViewState?.isControlsCardVisible 
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
