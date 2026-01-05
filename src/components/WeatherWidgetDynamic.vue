<template>
<div class="weather-widget-container d-flex flex-row align-start">
    
    <!-- Card 1: Weather Info (Time + Data) -->
    <v-card class="weather-info-card d-flex flex-column align-center justify-center px-4 py-1 mr-2" elevation="4">
        
        <!-- Time Range Label -->
        <span class="text-caption font-weight-bold mb-0 text-grey-darken-1">{{ timeLabel }}</span>

        <!-- Weather Data Row -->
        <div class="d-flex align-center justify-space-between w-100">
            <!-- Icon -->
            <v-icon size="default" :color="weatherInfo.color" :title="weatherInfo.desc" class="mr-3">{{ weatherInfo.icon }}</v-icon>
            
            <!-- Temp -->
            <div class="d-flex align-center mr-3">
                <v-icon size="small" class="mr-1" :color="getTempColor(weather.temperature)">mdi-thermometer</v-icon>
                <div class="d-flex align-baseline">
                    <span class="font-weight-bold">{{ Math.round(weather.temperature) }}°C</span>
                    <span class="text-caption text-grey ml-1" style="font-size: 0.7rem !important;">(Ress. {{ Math.round(weather.apparentTemperature) }}°)</span>
                </div>
            </div>

            <!-- Precip -->
            <div class="d-flex align-center">
                <v-icon size="small" class="mr-1" color="blue" :icon="weather.precipProb > 50 ? 'mdi-water-percent' : 'mdi-water-outline'"></v-icon>
                <span class="font-weight-bold" :class="{'text-blue': weather.precipProb > 50}">
                    {{ weather.precipProb }}% ({{ weather.precip }}mm)
                </span>
            </div>
        </div>
    </v-card>

    <!-- Card 2: Compass -->
    <v-card class="compass-card d-flex align-center justify-center pa-1" elevation="4">
            <CompassWidget 
                :size="80"
                :heading="myHeading"
                :track-bearing="traceBearing"
                :wind-direction="weather.windDir"
                :wind-speed="weather.windSpeed"
                :wind-gusts="weather.windGusts"
            />
        </v-card>

</div>
</template>

<script setup>
import { computed } from 'vue';
import { getWeatherInfo } from '@/services/WeatherIcons';
import CompassWidget from '@/components/CompassWidget.vue';

const props = defineProps({
  weather: {
    type: Object,
    required: true, // { apparentTemperature, windSpeed, windGusts, windDir, code, time ... }
  },
  bearing: {
    type: Number,
    default: 0, // Camera ID
  },
  traceBearing: {
    type: Number,
    default: 0,
  },
  orientationMode: {
    type: String,
    default: 'Trace', // 'Trace' or 'Camera' - Unused now for heading, but kept for props compatibility
  }
});

const myHeading = computed(() => {
    // The compass rose should always rotate opposite to the camera to keep "N" pointing North relative to the view.
    return props.bearing;
});

const weatherInfo = computed(() => getWeatherInfo(props.weather.code));

const timeLabel = computed(() => {
    if (!props.weather.time) return '';
    const date = new Date(props.weather.time);
    
    const days = ['Dim.', 'Lun.', 'Mar.', 'Mer.', 'Jeu.', 'Ven.', 'Sam.'];
    const months = ['Jan.', 'Fév.', 'Mars', 'Avr.', 'Mai', 'Juin', 'Juil.', 'Août', 'Sept.', 'Oct.', 'Nov.', 'Déc.'];
    
    const dayName = days[date.getDay()];
    const dayNum = String(date.getDate()).padStart(2, '0');
    const monthName = months[date.getMonth()];
    
    const startHour = date.getHours();
    const endHour = (startHour + 1) % 24;
    
    return `${dayName} ${dayNum} ${monthName} : ${String(startHour).padStart(2, '0')}h - ${String(endHour).padStart(2, '0')}h`;
});

const getTempColor = (temp) => {
    if (temp < 5) return 'blue';
    if (temp > 25) return 'red';
    return 'orange'; // default
};
</script>

<style scoped>
.weather-widget-container {
  position: relative;
  z-index: 1000;
  pointer-events: none; /* Let clicks pass through container gaps */
}

/* Match Distance Widget Style (Standard v-card) */
.weather-info-card {
  pointer-events: auto;
  /* Use default v-card background/radius, no custom override */
}

.compass-card {
  background-color: rgba(var(--v-theme-surface), 0.85);
  color: rgb(var(--v-theme-on-surface));
  backdrop-filter: blur(3px);
  border-radius: 20px;
  pointer-events: auto; /* Re-enable clicks on cards */
}

</style>
