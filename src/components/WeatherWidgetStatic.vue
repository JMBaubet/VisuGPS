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
            <td class="px-2 font-weight-medium">
                <div>{{ item.timeLabel }}</div>
                <div class="text-caption text-grey" style="font-size: 0.7rem !important;">{{ item.distanceLabel }}</div>
            </td>
            <td class="px-1 text-center">
                 <v-icon :color="item.weatherIconColor" :title="item.weatherDesc">{{ item.weatherIcon }}</v-icon>
            </td>
            <td class="px-2 text-center text-caption">
                <span class="text-blue">{{ Math.round(item.stats.temp.min) }}°</span> / 
                <span :class="getTempColorClass(item.stats.temp.avg)" class="font-weight-bold">{{ Math.round(item.stats.temp.avg) }}°</span> / 
                <span class="text-red">{{ Math.round(item.stats.temp.max) }}°</span>
                <div class="text-caption text-grey" style="font-size: 0.7rem !important;">
                    (Ress. {{ Math.round(item.stats.apparentT.avg) }}°)
                </div>
            </td>
            <td class="px-2 text-center text-caption">
              <div class="d-flex align-center justify-center">
                <v-icon :style="{ transform: `rotate(${item.stats.wind.dirAvg + 180}deg)` }" size="small" class="mr-1">mdi-arrow-up</v-icon>
                {{ Math.round(item.stats.wind.speedAvg) }}
                <span v-if="item.stats.wind.gustMax > item.stats.wind.speedAvg + 5" class="text-grey ml-1">
                    ({{ Math.round(item.stats.wind.gustMax) }})
                </span>
              </div>
            </td>
            <td class="px-2 text-center" :class="getRainColorClass(item.stats.precip.probMax)">
              {{ item.stats.precip.probMax }}%
              <span class="text-caption">
                ({{ item.stats.precip.sum.toFixed(1) }}mm)
              </span>
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
    const startHour = date.getHours();
    const endHour = (startHour + 1) % 24;
    
    const startStr = String(startHour).padStart(2, '0') + 'h';
    const endStr = String(endHour).padStart(2, '0') + 'h';
    
    return `${startStr} - ${endStr}`;
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

    const sortedKeys = Object.keys(groups).sort();
    
    return sortedKeys.map((time, index) => {
        const items = groups[time];
        const temps = items.map(i => i.weather.temperature); // FIX: Use real temperature
        const apparentTemps = items.map(i => i.weather.apparentTemperature); // Added
        const windSpeeds = items.map(i => i.weather.windSpeed);
        const windGusts = items.map(i => i.weather.windGusts);
        const precipProbs = items.map(i => i.weather.precipProb);
        
        // Calculate average wind direction
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

        const maxCode = Math.max(...items.map(i => i.weather.code));
        const weatherInfo = getWeatherInfo(maxCode);

        // Time Label Formatting
        const bucketDate = new Date(time);
        let startObj = bucketDate;
        
        // For the very first row, use the exact start time of the first point
        if (index === 0 && items.length > 0) {
            startObj = new Date(items[0].point.timestamp);
        }

        const bucketNextHour = new Date(bucketDate);
        bucketNextHour.setHours(bucketNextHour.getHours() + 1);

        const formatH = (d) => String(d.getHours()).padStart(2,'0') + 'h';
        const formatHM = (d) => {
            const m = d.getMinutes();
            if (m === 0) return formatH(d);
            return String(d.getHours()).padStart(2,'0') + 'h' + String(m).padStart(2,'0');
        };

        const startStr = (index === 0) ? formatHM(startObj) : formatH(startObj);
        
        let endStr;
        if (index === sortedKeys.length - 1 && items.length > 0) {
             const lastPointTime = new Date(items[items.length - 1].point.timestamp);
             endStr = formatHM(lastPointTime);
        } else {
             endStr = formatH(bucketNextHour);
        }

        // Distance Range calculation
        const startDist = items[0].point.distance / 1000;
        const endDist = items[items.length - 1].point.distance / 1000;
        const distanceLabel = `${Math.round(startDist)} - ${Math.round(endDist)} km`;

        const timeLabel = `${startStr} - ${endStr}`;

        return {
            time,
            timeLabel,
            distanceLabel,
            distanceKm: startDist, // Keep for sorting if needed, though index is used
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
                apparentT: {
                    avg: apparentTemps.reduce((a, b) => a + b, 0) / apparentTemps.length
                },
                wind: {
                    speedAvg: windSpeeds.reduce((a, b) => a + b, 0) / windSpeeds.length,
                    gustMax: Math.max(...windGusts),
                    dirAvg: avgDeg 
                },
                precip: {
                    probMax: Math.max(...precipProbs),
                    sum: items.map(i => i.weather.precip).reduce((a, b) => a + b, 0)
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
