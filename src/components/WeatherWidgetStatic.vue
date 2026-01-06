<template>
  <div class="weather-matrix-overlay" @click.self="$emit('close')">
    <v-card class="matrix-card" elevation="10">
      <v-card-title class="d-flex align-center justify-space-between py-2 px-4 bg-primary text-white">
        <div class="d-flex align-center">
            <span>Météo pour le</span>
            <span v-if="date" class="ml-2">
                {{ formatDate(date) }}
            </span>
        </div>

        <div class="d-flex align-center">
             <v-select
                v-model="selectedStep"
                :items="[1, 2, 5, 10]"
                suffix="km"
                density="compact"
                variant="outlined"
                hide-details
                class="mr-3 text-caption"
                style="max-width: 110px;"
                bg-color="transparent"
                base-color="white"
                color="white"
            ></v-select>
            <v-btn icon variant="text" color="white" @click="$emit('close')">
              <v-icon>mdi-close</v-icon>
            </v-btn>
        </div>
      </v-card-title>
      
      <v-card-text class="pa-0 table-container">
        <table class="matrix-table">
          <thead>
            <tr>
              <th class="sticky-col text-center">Distance</th>
              <th v-for="(scen, idx) in localScenarios" :key="idx" class="text-center" style="min-width: 100px;">
                <div class="d-flex flex-column align-center">
                   <div class="text-caption font-weight-bold mb-1">{{ scen.nom || `Groupe ${idx + 1}` }}</div>
                   <div class="d-flex align-center mb-1">
                       <span class="text-caption font-weight-bold mr-1">Départ</span>
                       <input 
                           type="time" 
                           v-model="scen.heureDepart"
                           style="border: 1px solid rgba(0,0,0,0.3); border-radius: 3px; font-size: 0.8rem; padding: 1px 4px; width: 70px;" 
                       />
                   </div>
                   <div class="d-flex align-center">
                       <input 
                           type="number" 
                           v-model.number="scen.vitesseMoyenne"
                           style="border: 1px solid rgba(0,0,0,0.3); border-radius: 3px; font-size: 0.8rem; padding: 1px 4px; width: 40px; text-align: center;" 
                       />
                       <span class="text-caption ml-1">km/h</span>
                   </div>
                </div>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in matrixRows" :key="row.increment">
              <td class="sticky-col font-weight-bold text-center">{{ row.km.toFixed(1) }} km</td>
              <template v-for="(cell, cIdx) in row.cells">
                <td 
                    v-if="!cell.hidden" 
                    :key="cIdx" 
                    :colspan="cell.colspan"
                    class="text-center cell-content"
                    :class="cell.groupClass"
                >
                   <div class="cell-time-tag text-grey-darken-1">{{ cell.hourLabel }}</div>
                   <div class="d-flex align-center justify-center py-1" style="white-space: nowrap;">
                      
                      <template v-if="cell.weather">
                          <v-icon :color="cell.weather.iconColor" size="small" class="mr-2">{{ cell.weather.icon }}</v-icon>
                          
                          <div class="d-flex align-center mr-3">
                              <v-icon size="x-small" class="mr-1 text-grey-darken-1">mdi-thermometer</v-icon>
                              <span :class="getTempColor(cell.weather.temp)" class="font-weight-bold text-caption" style="min-width: 20px; text-align: right;">{{ Math.round(cell.weather.temp) }}°</span>
                          </div>
                          
                          <div class="d-flex align-center mr-3">
                             <v-icon size="x-small" class="mr-1 text-grey-darken-1">mdi-weather-windy</v-icon>
                             <v-icon size="x-small" :style="{ transform: `rotate(${cell.weather.windDir + 180}deg)` }" class="mr-1">mdi-arrow-up</v-icon>
                             <span class="text-caption">{{ Math.round(cell.weather.windSpeed) }}</span>
                          </div>

                          <div class="d-flex align-center" style="min-width: 45px;">
                             <div v-if="cell.weather.precipProb !== undefined && cell.weather.precipProb !== null" class="d-flex align-center">
                                <v-icon size="x-small" color="blue" class="mr-1">mdi-water</v-icon>
                                <span :class="getRainColor(cell.weather.precipProb)" class="text-caption font-weight-bold">{{ cell.weather.precipProb }}%</span>
                                <span class="text-grey-darken-1 ml-1 text-caption" v-if="cell.weather.precip > 0" style="font-size: 0.7rem;">
                                    {{ cell.weather.precip.toFixed(1) }}mm
                                </span>
                             </div>
                             <span v-else class="text-caption text-disabled">?</span>
                          </div>
                      </template>
                      <div v-else class="text-caption text-error">N/A</div>
                   </div>
                </td>
              </template>
            </tr>
          </tbody>
        </table>
      </v-card-text>
      
    </v-card>
  </div>
</template>

<script setup>
import { computed, ref, watch } from 'vue';
import { getWeatherInfo } from '@/services/WeatherIcons';

const props = defineProps({
  weatherMatrix: { type: Array, required: true },
  scenarios: { type: Array, required: true },
  date: { type: Date, default: null }
});

const selectedStep = ref(5);
const localScenarios = ref([]);

watch(() => props.scenarios, (newVal) => {
   if (newVal) {
       localScenarios.value = JSON.parse(JSON.stringify(newVal));
   }
}, { immediate: true, deep: true });

const formatDate = (d) => {
    if (!d) return '';
    // Capitalize first letter as French dates are lowercase
    const s = new Intl.DateTimeFormat('fr-FR', { weekday: 'long', day: 'numeric', month: 'long' }).format(d);
    return s.charAt(0).toUpperCase() + s.slice(1);
};

defineEmits(['close']);

const getTempColor = (t) => {
    if (t < 10) return 'text-blue';
    if (t > 25) return 'text-red';
    return ''; // Default
};

const getRainColor = (p) => {
    if (p > 50) return 'text-blue-darken-2';
    if (p > 0) return 'text-blue';
    return 'text-grey-lighten-1';
};

const matrixRows = computed(() => {
    if (!props.weatherMatrix || !localScenarios.value.length) return [];

    // Sort by increment just in case
    const sortedMatrix = [...props.weatherMatrix].sort((a,b) => a.increment - b.increment);

    const filtered = sortedMatrix.filter(p => p.increment % selectedStep.value === 0);

    return filtered.map(point => {
        const increment = point.increment;
        const km = point.km !== undefined ? point.km : (point.increment / 10);

        // Calculate cells for each scenario
        const cells = localScenarios.value.map(scen => {
            // Parse start time "HH:MM"
            const startTimeStr = scen.heureDepart || scen.start || "09:00";
            const speed = scen.vitesseMoyenne || scen.speed || 20;

            const [h, m] = startTimeStr.split(':').map(Number);
            const startMinutes = h * 60 + m;
            
            // Travel time in minutes = (dist km / speed km/h) * 60
            const travelMinutes = (km / speed) * 60;
            const arrivalTotalMinutes = startMinutes + travelMinutes;
            
            // Hour for weather lookup
            const arrivalDate = new Date(); // Mock date base
            arrivalDate.setHours(0,0,0,0);
            arrivalDate.setMinutes(arrivalTotalMinutes);
            
            let hour = arrivalDate.getHours();
            const minute = arrivalDate.getMinutes();
            // Strict hourly slot: 08:00 to 08:59 uses 08h data.
            
            const hourLabel = `${String(arrivalDate.getHours()).padStart(2,'0')}h${String(minute).padStart(2,'0')}`;
            
            const weatherData = point.hours ? point.hours[hour] : null;
            
            let processedWeather = null;
            if (weatherData) {
                const info = getWeatherInfo(weatherData.code);
                processedWeather = {
                    hourIdx: hour, // Uses the rounded hour for merging key
                    temp: weatherData.temperature,
                    windSpeed: weatherData.windSpeed,
                    windDir: weatherData.windDir,
                    precip: weatherData.precip,
                    precipProb: weatherData.precipProb,
                    icon: info.icon,
                    iconColor: info.color
                };
            }

            return {
                hourLabel,
                hourIdx: hour, 
                weather: processedWeather,
                colspan: 1,
                hidden: false
            };
        });

        // Visual Grouping Logic (Mark groups for coloring instead of merging)
        // 1. Assign Group IDs based on adjacent same-hour cells
        let currentGroupIdx = 0;
        for (let i = 0; i < cells.length; i++) {
             if (i > 0 && cells[i].hourIdx !== cells[i-1].hourIdx) {
                 currentGroupIdx++;
             }
             cells[i].groupId = currentGroupIdx;
        }

        // 2. Calculate group sizes
        const groupSizes = {};
        cells.forEach(c => {
            groupSizes[c.groupId] = (groupSizes[c.groupId] || 0) + 1;
        });

        // 3. Assign properties
        cells.forEach(c => {
             const size = groupSizes[c.groupId];
             c.isGrouped = size > 1;
             // Alternating colors: groups 0, 2, 4... vs 1, 3, 5...
             // Only apply if it's actually a group of multiple cells (size > 1)
             c.groupClass = c.isGrouped ? `group-bg-${c.groupId % 2}` : '';
             
             // Ensure hidden/colspan are reset/default since we don't merge anymore
             c.hidden = false;
             c.colspan = 1;
        });
        
        return {
            increment,
            km,
            cells
        };
    });
});
</script>

<style scoped>
.weather-matrix-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 2000;
  background-color: rgba(0,0,0,0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: auto;
}

.matrix-card {
  width: 95%;
  max-width: 1200px;
  height: 85vh;
  display: flex;
  flex-direction: column;
}

.table-container {
    overflow: auto;
    flex: 1;
}

.matrix-table {
    width: 100%;
    border-collapse: separate; /* Required for sticky headers borders to work better usually, or collapse can work */
    border-spacing: 0;
    font-size: 0.85rem;
}

.matrix-table th, .matrix-table td {
    border: 1px solid rgba(0,0,0,0.12);
    padding: 6px;
    background-color: rgb(var(--v-theme-surface));
    color: rgb(var(--v-theme-on-surface));
}

.matrix-table th {
    background-color: rgb(var(--v-theme-surface-light));
    position: sticky;
    top: 0;
    z-index: 20;
    box-shadow: 0 1px 2px rgba(0,0,0,0.1);
}

.sticky-col {
    position: sticky;
    left: 0;
    z-index: 30;
    border-right: 2px solid rgba(0,0,0,0.2) !important;
}

.matrix-table thead th.sticky-col {
    z-index: 40; /* Top-Left corner highest */
}

.cell-merged {
    background-color: rgba(var(--v-theme-primary), 0.08);
}

.group-bg-0 {
    background-color: rgba(var(--v-theme-on-surface), 0.08) !important;
}
.group-bg-1 {
    background-color: rgba(var(--v-theme-on-surface), 0.16) !important;
}

.cell-content {
    position: relative;
}

.cell-time-tag {
    position: absolute;
    top: 0px;
    left: 4px;
    font-size: 0.65rem;
    font-weight: 700;
    opacity: 0.6;
}
</style>
