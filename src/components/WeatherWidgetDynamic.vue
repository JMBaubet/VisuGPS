<template>
  <v-card class="weather-widget-dynamic pl-2 pr-2 pt-1 pb-1 d-flex align-center" elevation="4">
    <!-- Weather Icon -->
    <v-icon size="large" :color="weatherInfo.color" class="mr-2" :title="weatherInfo.desc">{{ weatherInfo.icon }}</v-icon>

    <!-- Weather Info Section -->
    <div class="d-flex flex-column mr-2">
        <div class="d-flex align-center">
            <v-icon size="small" class="mr-1" :color="getTempColor(weather.apparentTemperature)">mdi-thermometer</v-icon>
            <span class="font-weight-bold">{{ Math.round(weather.apparentTemperature) }}°</span>
        </div>
        <div class="d-flex align-center">
             <v-icon size="small" class="mr-1" color="blue-grey lighten-2">mdi-weather-windy</v-icon>
             <span class="text-caption">{{ Math.round(weather.windSpeed) }} <span class="text-grey-darken-1" v-if="weather.windGusts > weather.windSpeed">({{Math.round(weather.windGusts)}})</span></span>
        </div>
        <div class="d-flex align-center">
             <v-icon size="small" class="mr-1" color="blue" :icon="weather.precipProb > 50 ? 'mdi-water-percent' : 'mdi-water-outline'"></v-icon>
             <span class="text-caption" :class="{'font-weight-bold text-blue': weather.precipProb > 50}">{{ weather.precipProb }}%</span>
        </div>
    </div>

    <!-- Compass Section -->
    <div class="compass-container ml-2" :style="{ transform: `rotate(${-myHeading}deg)` }">
        <div class="compass-rose">
            <div class="north-marker">N</div>
            <!-- Track Arrow: Points in direction of travel -->
             <v-icon 
                class="track-arrow" 
                size="48"
                :style="{ transform: `rotate(${traceBearing}deg) scaleY(1.2)` }"
                color="black"
            >
                mdi-arrow-up
            </v-icon>
            <!-- Wind Arrow: Points to where wind is coming FROM (arrow points down wind direction?) -->
            <!-- Actually arrow-down-thick at 0 deg points down. If windDir is 0 (North), it points South. -->
            <!-- This visualizes the wind flow properly. -->
             <v-icon 
                class="wind-arrow" 
                size="48"
                :style="{ transform: `rotate(${weather.windDir}deg) scaleY(1.2)` }"
                color="blue"
            >
                mdi-arrow-down-thick
            </v-icon>
        </div>
    </div>
  </v-card>
</template>

<script setup>
import { computed } from 'vue';
import { getWeatherInfo } from '@/services/WeatherIcons';

const props = defineProps({
  weather: {
    type: Object,
    required: true, // { apparentTemperature, windSpeed, windGusts, windDir, code, ... }
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
    default: 'Trace', // 'Trace' or 'Camera'
  }
});

const myHeading = computed(() => {
    // The compass rose should always rotate opposite to the camera to keep "N" pointing North relative to the view.
    return props.bearing;
});

const weatherInfo = computed(() => getWeatherInfo(props.weather.code));

const getTempColor = (temp) => {
    if (temp < 5) return 'blue';
    if (temp > 25) return 'red';
    return 'orange'; // default
};
</script>

<style scoped>
.weather-widget-dynamic {
  /* Position handling moved to parent container */
  position: relative;
  z-index: 1000;
  background-color: rgba(var(--v-theme-surface), 0.85);
  color: rgb(var(--v-theme-on-surface));
  backdrop-filter: blur(3px);
  border-radius: 20px;
  pointer-events: auto;
}

.compass-container {
    width: 60px;
    height: 60px;
    position: relative;
    transition: transform 0.1s linear;
}

.compass-rose {
    width: 100%;
    height: 100%;
    border: 2px solid rgb(var(--v-theme-on-surface));
    border-radius: 50%;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
}

.north-marker {
    position: absolute;
    top: -10px;
    left: 50%;
    transform: translateX(-50%);
    font-size: 12px;
    font-weight: bold;
    color: red;
    background: rgb(var(--v-theme-surface));
    padding: 0 2px;
}

.wind-arrow {
    position: absolute;
    /* Rotation handled in template */
}

.track-arrow {
    position: absolute;
    /* Rotation handled in template */
    opacity: 0.7; /* Make it slightly distinct */
}
</style>
