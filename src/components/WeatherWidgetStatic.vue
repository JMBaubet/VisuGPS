<template>
  <v-card class="weather-widget-static" elevation="4">
    <v-card-title class="text-subtitle-2 font-weight-bold d-flex align-center justify-space-between py-1 px-2">
      <span>Météo Parcours (Heure par Heure)</span>
      <v-btn icon x-small variant="text" @click="$emit('close')">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-card-title>
    <v-card-text class="pa-0">
      <v-table density="compact" class="weather-table">
        <thead>
          <tr>
            <th class="text-left px-2">Heure</th>
            <th class="text-center px-1">Ciel</th>
            <th class="text-center px-2">Temp (Min/Moy/Max)</th>
            <th class="text-center px-2">Vent (Moy/Raf)</th>
            <th class="text-center px-2">Pluie</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, index) in hourlyForecasts" :key="index">
            <td class="px-2 font-weight-medium">{{ formatTime(item.time) }} ({{ Math.round(item.distanceKm) }}km)</td>
            <td class="px-1 text-center">
                 <v-icon :color="item.weatherIconColor" :title="item.weatherDesc">{{ item.weatherIcon }}</v-icon>
            </td>
            <td class="px-2 text-center text-caption">
                <span class="text-blue">{{ Math.round(item.stats.temp.min) }}°</span> / 
                <span :class="getTempColorClass(item.stats.temp.avg)" class="font-weight-bold">{{ Math.round(item.stats.temp.avg) }}°</span> / 
                <span class="text-red">{{ Math.round(item.stats.temp.max) }}°</span>
            </td>
            <td class="px-2 text-center text-caption">
              <div class="d-flex align-center justify-center">
                <v-icon :style="{ transform: `rotate(${item.stats.wind.dirAvg}deg)` }" size="small" class="mr-1">mdi-arrow-up</v-icon>
                {{ Math.round(item.stats.wind.speedAvg) }}
                <span v-if="item.stats.wind.gustMax > item.stats.wind.speedAvg + 5" class="text-grey ml-1">
                    ({{ Math.round(item.stats.wind.gustMax) }})
                </span>
              </div>
            </td>
            <td class="px-2 text-center" :class="getRainColorClass(item.stats.precip.probMax)">
              {{ item.stats.precip.probMax }}%
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from 'vuetify';

const theme = useTheme();

const props = defineProps({
  forecasts: {
    type: Array,
    required: true,
  }
});

defineEmits(['close']);

const formatTime = (isoString) => {
    if (!isoString) return '';
    const date = new Date(isoString);
    // Use getHours() directly for "HHh" format, preventing invalid option errors
    return String(date.getHours()).padStart(2, '0') + 'h';
};

import { getWeatherInfo } from '@/services/WeatherIcons';

const hourlyForecasts = computed(() => {
    if (!props.forecasts || props.forecasts.length === 0) return [];

    const groups = {};

    props.forecasts.forEach(item => {
        const time = item.weather.time; // Hourly ISO string
        if (!groups[time]) {
            groups[time] = [];
        }
        groups[time].push(item);
    });

    return Object.keys(groups).sort().map(time => {
        const items = groups[time];
        const temps = items.map(i => i.weather.apparentTemperature);
        const windSpeeds = items.map(i => i.weather.windSpeed);
        const windGusts = items.map(i => i.weather.windGusts);
        const precipProbs = items.map(i => i.weather.precipProb);
        
        // Calculate average wind direction: vector averaging
        let sumSin = 0;
        let sumCos = 0;
        items.forEach(i => {
            const rad = i.weather.windDir * Math.PI / 180;
            sumSin += Math.sin(rad);
            sumCos += Math.cos(rad);
        });
        let avgRad = Math.atan2(sumSin, sumCos);
        let avgDeg = avgRad * 180 / Math.PI;
        if (avgDeg < 0) avgDeg += 360;

        // Weather Code: Use the one that occurs most, or explicitly prioritize worse weather?
        // Let's take the max code usually implies "significant weather" vs clear.
        // But 0=clear, 3=overcast, 99=thunderstorm. Generally higher is worse/more active.
        const maxCode = Math.max(...items.map(i => i.weather.code));
        const weatherInfo = getWeatherInfo(maxCode);

        // Distance at the start of this hour (approx)
        const distanceKm = items[0].point.distance / 1000;

        return {
            time,
            distanceKm,
            weatherCode: maxCode,
            weatherIcon: weatherInfo.icon,
            weatherIconColor: weatherInfo.color,
            weatherDesc: weatherInfo.desc,
            stats: {
                temp: {
                    min: Math.min(...temps),
                    max: Math.max(...temps),
                    avg: temps.reduce((a, b) => a + b, 0) / temps.length
                },
                wind: {
                    speedAvg: windSpeeds.reduce((a, b) => a + b, 0) / windSpeeds.length,
                    gustMax: Math.max(...windGusts),
                    dirAvg: avgDeg // Use computed average direction
                },
                precip: {
                    probMax: Math.max(...precipProbs)
                }
            }
        };
    });
});

const getTempColorClass = (temp) => {
    // Logic for colorizing the value itself if not using icons
    return '';
};

const getRainColorClass = (prob) => {
    if (prob > 50) return 'text-blue font-weight-bold';
    if (prob > 20) return 'text-blue';
    return '';
};
</script>

<style scoped>
.weather-widget-static {
  position: absolute;
  top: 20px;
  right: 10px;
  max-width: 450px;
  max-height: 60vh;
  overflow-y: auto;
  z-index: 1000;
  backdrop-filter: blur(5px);
  background-color: rgba(var(--v-theme-surface), 0.9);
  color: rgb(var(--v-theme-on-surface));
}

.weather-table {
    background: transparent;
    color: inherit;
}

.weather-table th, .weather-table td {
    font-size: 0.75rem !important;
    height: 36px !important;
    white-space: nowrap;
}
</style>
