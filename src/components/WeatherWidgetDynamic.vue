<template>
<div class="weather-widget-container d-flex flex-column align-end">
    

    <!-- Card 1: Weather Info (Time + Data) -->
    <transition name="fade">
        <v-card v-if="showInfo" class="weather-info-card d-flex flex-column align-center justify-center px-4 py-1 mb-2" elevation="4">
            
            <div class="text-caption font-weight-bold mb-1 align-self-start">{{ forecastDateFormatted }}</div>

            <!-- Multi-Scenario Mode -->
            <template v-if="processedScenarios.length > 0">
                <div v-for="(scen, idx) in processedScenarios" :key="idx" class="w-100 py-1" 
                     :class="{'border-b': idx < processedScenarios.length - 1}">
                    <div class="d-flex align-center justify-space-between w-100">
                        <!-- Group Name & Time -->
                        <div class="d-flex flex-column mr-3" style="min-width: 80px;">
                            <div class="d-flex align-center">
                                <span class="text-caption text-truncate font-weight-bold" style="max-width: 100px;">
                                    {{ scen.nom || `Groupe ${idx+1}` }}
                                </span>
                                <v-icon v-if="scen.isReference || (!hasExplicitRef && idx === 0)" size="10" color="primary" class="ml-1">mdi-star</v-icon>
                            </div>
                            <span class="text-grey-darken-1" style="font-size: 0.7rem; margin-top: -4px;">{{ formatRowTime(scen.arrivalTime) }}</span>
                        </div>
                        
                        <!-- Weather Data -->
                        <div v-if="scen.weather" class="d-flex align-center flex-grow-1 justify-end">
                            <!-- Icon -->
                            <v-icon v-if="scen.weatherInfo" size="small" :color="resolveIconColor(scen.weatherInfo.color)" class="mr-2">{{ scen.weatherInfo.icon }}</v-icon>
                            
                            <!-- Temp -->
                            <div class="d-flex align-center mr-2">
                                <span class="font-weight-bold text-body-2" :class="getTempColor(scen.weather.temperature) ? 'text-' + getTempColor(scen.weather.temperature) : ''">{{ Math.round(scen.weather.temperature) }}°</span>
                            </div>

                            <!-- Rain -->
                            <div v-if="scen.weather.precip > 0" class="d-flex align-center mr-2">
                                <v-icon size="x-small" :color="precipIconColor" class="mr-1">mdi-water-percent</v-icon>
                                <span class="text-caption font-weight-bold" :class="precipColorClass">
                                    {{ scen.weather.precipProb }}% ({{ scen.weather.precip }}mm)
                                </span>
                            </div>

                            <!-- Wind -->
                            <div class="d-flex align-center" style="min-width: 40px;">
                                <v-icon size="x-small" color="grey" class="mr-1" :style="{ transform: `rotate(${scen.weather.windDir + 180}deg)` }">mdi-navigation</v-icon>
                                <span class="text-caption font-weight-bold">
                                    {{ Math.round(scen.weather.windSpeed) }}
                                    <span v-if="scen.weather.windGusts && scen.weather.windGusts > scen.weather.windSpeed" class="text-grey-darken-1" style="font-size: 0.85em;">
                                        ({{ Math.round(scen.weather.windGusts) }})
                                    </span>
                                </span>
                            </div>
                        </div>
                        <div v-else class="text-caption text-disabled text-end flex-grow-1">N/A</div>
                    </div>
                </div>
            </template>

            <!-- Single Scenario Mode (Legacy) -->
            <template v-else-if="weatherToDisplay">
                <!-- Time Range Label -->
                <span class="text-caption font-weight-bold mb-0 text-grey-darken-1">{{ timeLabel }}</span>
        
                <!-- Weather Data Row -->
                <div class="d-flex align-center justify-space-between w-100">
                    <!-- Icon -->
                    <v-icon v-if="weatherInfo" size="default" :color="resolveIconColor(weatherInfo.color)" :title="weatherInfo.desc" class="mr-3">{{ weatherInfo.icon }}</v-icon>
                    
                    <!-- Temp -->
                    <div class="d-flex align-center mr-3">
                        <v-icon size="small" class="mr-1" :color="getTempColor(weatherToDisplay.temperature)">mdi-thermometer</v-icon>
                        <div class="d-flex align-baseline">
                            <span class="font-weight-bold" :class="getTempColor(weatherToDisplay.temperature) ? 'text-' + getTempColor(weatherToDisplay.temperature) : ''">{{ Math.round(weatherToDisplay.temperature) }}°C</span>
                            <span v-if="weatherToDisplay.apparentTemperature" class="text-caption text-grey ml-1" style="font-size: 0.7rem !important;">(Ress. {{ Math.round(weatherToDisplay.apparentTemperature) }}°)</span>
                        </div>
                    </div>
        
                    <!-- Precip -->
                    <div v-if="weatherToDisplay.precip > 0" class="d-flex align-center mr-3">
                        <v-icon size="small" class="mr-1" :color="precipIconColor" :icon="weatherToDisplay.precipProb > 50 ? 'mdi-water-percent' : 'mdi-water-outline'"></v-icon>
                        <span class="font-weight-bold" :class="precipColorClass">
                            {{ weatherToDisplay.precipProb }}% ({{ weatherToDisplay.precip }}mm)
                        </span>
                    </div>

                    <!-- Wind -->
                    <div class="d-flex align-center">
                        <v-icon size="small" class="mr-1" color="grey" :style="{ transform: `rotate(${weatherToDisplay.windDir + 180}deg)` }">mdi-navigation</v-icon>
                        <span class="font-weight-bold">
                            {{ Math.round(weatherToDisplay.windSpeed) }}
                            <span v-if="weatherToDisplay.windGusts && weatherToDisplay.windGusts > weatherToDisplay.windSpeed" class="text-grey-darken-1" style="font-size: 0.85em;">
                                ({{ Math.round(weatherToDisplay.windGusts) }})
                            </span>
                            <span class="text-caption">km/h</span>
                        </span>
                    </div>
                </div>
            </template>
        </v-card>
    </transition>

    <!-- Card 2: Compass -->
    <transition name="fade-opacity">
        <v-card v-if="showCompass && weatherToDisplay" class="compass-card d-flex align-center justify-center pa-1" elevation="4">
                <CompassWidget 
                    :size="80"
                    :camera-bearing="bearing"
                    :track-bearing="traceBearing"
                    :wind-direction="weatherToDisplay.windDir"
                    :wind-speed="weatherToDisplay.windSpeed"
                    :wind-gusts="weatherToDisplay.windGusts || 0"
                    :orientation-mode="orientationMode"
                />
        </v-card>
    </transition>

</div>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from 'vuetify';
import { getWeatherInfo } from '@/services/WeatherIcons';
import CompassWidget from '@/components/CompassWidget.vue';

const props = defineProps({
  weather: {
    type: Object,
    default: null
  },
  bearing: {
    type: Number,
    required: true
  },
  traceBearing: {
    type: Number,
    default: 0,
  },
  orientationMode: {
    type: String,
    default: 'Trace',
  },
  showInfo: {
    type: Boolean,
    default: true
  },
  showCompass: {
    type: Boolean,
    default: true
  },
  
  // Multi-Scenario Props
  scenarios: {
    type: Array,
    default: () => []
  },
  weatherMatrix: {
    type: Array,
    default: () => []
  },
  currentDistance: {
    type: Number,
    default: 0
  },
  simulationStartDate: {
    type: [Date, String, Object],
    default: null
  }
});

const theme = useTheme();
const isDark = computed(() => theme.global.current.value.dark);

const myHeading = computed(() => {
    // The compass rose should always rotate opposite to the camera to keep "N" pointing North relative to the view.
    return props.bearing;
});

const weatherToDisplay = computed(() => {
    if (processedScenarios.value.length === 1) {
        return processedScenarios.value[0].weather || props.weather;
    }
    return props.weather;
});

const weatherInfo = computed(() => weatherToDisplay.value ? getWeatherInfo(weatherToDisplay.value.code) : null);

const timeLabel = computed(() => {
    if (!props.weather || !props.weather.time) return "Météo Actuelle";
    const date = new Date(props.weather.time);
    const end = new Date(date.getTime() + 3600000); // Assuming weather data is hourly
    return `De ${date.getHours()}h à ${end.getHours()}h`;
});

const processedScenarios = computed(() => {
    if (!props.scenarios.length || !props.weatherMatrix.length || !props.simulationStartDate) return [];
    
    // Ensure simulationStartDate is a Date object
    const baseDate = new Date(props.simulationStartDate);
    if (isNaN(baseDate.getTime())) return [];

    return props.scenarios.map(scen => {
        const startTimeStr = scen.heureDepart || scen.start || "09:00";
        const speed = scen.vitesseMoyenne || scen.speed || 20;

        // 1. Calculate Estimated Arrival Date/Time
        const [h, m] = startTimeStr.split(':').map(Number);
        const arrivalDate = new Date(baseDate);
        arrivalDate.setHours(isNaN(h) ? 9 : h, isNaN(m) ? 0 : m, 0, 0);
        
        const travelTimeHours = props.currentDistance / (speed || 20);
        arrivalDate.setTime(arrivalDate.getTime() + travelTimeHours * 3600000);
        
        // 2. Find Best Location Point in Matrix
        let bestPoint = null;
        let minDiff = Infinity;
        
        for (const pt of props.weatherMatrix) {
            // Use km if available, fallback to increment estimation
            const ptKm = (pt.km != null) ? pt.km : (pt.increment * 0.1);
            const diff = Math.abs(ptKm - props.currentDistance);
            
            if (diff < minDiff) {
                minDiff = diff;
                bestPoint = pt;
            }
        }
        
        // Fallback to first point if matrix not empty
        if (!bestPoint && props.weatherMatrix.length > 0) {
            bestPoint = props.weatherMatrix[0];
        }

        // 3. Find Best Hour in Point Data
        let weatherAtHour = null;
        if (bestPoint && bestPoint.hours) {
            const targetHour = arrivalDate.getHours();
            
            // Direct lookup
            weatherAtHour = bestPoint.hours[targetHour];

            // Fallback: Matrix might only have data for certain hours (e.g. 6-20)
            if (!weatherAtHour) {
                const hours = Object.keys(bestPoint.hours).map(Number).sort((a,b) => a-b);
                if (hours.length > 0) {
                    // Find closest available hour
                    const closestHour = hours.reduce((prev, curr) => {
                        return (Math.abs(curr - targetHour) < Math.abs(prev - targetHour) ? curr : prev);
                    });
                    weatherAtHour = bestPoint.hours[closestHour];
                }
            }
        }

        return {
            ...scen,
            arrivalTime: arrivalDate,
            weather: weatherAtHour || props.weather, // Ultimate fallback
            weatherInfo: (weatherAtHour || props.weather) ? getWeatherInfo((weatherAtHour || props.weather).code) : null
        };
    });
});

const formatRowTime = (date) => {
    if (!date) return '--:--';
    return date.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });
};

const hasExplicitRef = computed(() => processedScenarios.value.some(s => s.isReference));

const forecastDateFormatted = computed(() => {
    let d = null;
    if (processedScenarios.value.length > 0) {
        d = new Date(props.simulationStartDate);
    } else if (weatherToDisplay.value && weatherToDisplay.value.time) {
        d = new Date(weatherToDisplay.value.time);
    }

    if (!d || isNaN(d.getTime())) return "";

    let str = new Intl.DateTimeFormat('fr-FR', { weekday: 'short', day: '2-digit', month: 'short', year: 'numeric' }).format(d);
    return str.charAt(0).toUpperCase() + str.slice(1);
});

const precipColorClass = computed(() => {
    return isDark.value ? 'text-white' : 'text-blue-grey-darken-3';
});

const precipIconColor = computed(() => {
    return isDark.value ? 'white' : 'blue-grey-darken-3';
});

const resolveIconColor = (color) => {
    if (!color) return undefined;

    if (isDark.value) {
        // Dark Mode: bright colors
        if (color.includes('darken')) return color.replace('darken', 'lighten');
        if (color === 'grey') return 'grey-lighten-1';
    } else {
        // Light Mode: dark colors
        if (color === 'white') return 'grey-darken-3';
        if (color.includes('lighten')) return color.replace('lighten', 'darken');
        if (color === 'grey') return 'grey-darken-1';
    }
    return color;
};

const getTempColor = (temp) => {
    const suffix = isDark.value ? 'lighten-2' : 'darken-3';
    const suffixYellow = isDark.value ? 'lighten-2' : 'darken-4'; // Yellow needs more contrast in light mode
    
    if (temp <= 0) return `blue-${suffix}`;
    if (temp <= 7) return undefined; // Will create no class -> inherit default color
    if (temp <= 14) return `grey-${isDark.value ? 'lighten-1' : 'darken-2'}`;
    if (temp <= 20) return `green-${suffix}`;
    if (temp <= 26) return `yellow-${suffixYellow}`;
    if (temp <= 30) return `orange-${suffix}`;
    return `red-${suffix}`;
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
  pointer-events: auto; /* Re-enable clicks on cards */
}

/* Animation Styles */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.75s ease, max-height 0.75s ease, margin 0.75s ease, padding 0.75s ease;
  overflow: hidden;
  max-height: 400px; /* Sufficient for info/compass */
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  max-height: 0;
  margin-top: 0 !important;
  margin-bottom: 0 !important;
  padding-top: 0 !important;
  padding-bottom: 0 !important;
}

.fade-opacity-enter-active,
.fade-opacity-leave-active {
  transition: opacity 0.75s ease;
}

.fade-opacity-enter-from,
.fade-opacity-leave-to {
  opacity: 0;
}

.border-b {
    border-bottom: 1px solid rgba(0,0,0,0.1);
}
</style>
