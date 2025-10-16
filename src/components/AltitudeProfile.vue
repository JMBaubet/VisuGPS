<template>
    <v-chart class="chart" :option="option" autoresize />
</template>

<script setup>
import { ref, onMounted, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const props = defineProps({
    circuitId: {
        type: String,
        required: true
    },
    currentDistance: {
        type: Number,
        required: true
    }
});

const option = ref({});
const totalDistance = ref(0);
let lastUpdatedDistance = 0;

const { getSettingValue } = useSettings();
const { toHex } = useVuetifyColors();

const trackingData = ref([]);
const segmentLength = computed(() => getSettingValue('Importation/Tracking/LongueurSegment') || 100);

async function loadTrackingData() {
    if (!props.circuitId) return;
    try {
        const data = await invoke('read_tracking_file', { circuitId: props.circuitId });
        trackingData.value = data;
        initChart();
    } catch (error) {
        console.error("AltitudeProfile: Error loading tracking data:", error);
    }
}

function initChart() {
    if (trackingData.value.length < 2) return;

    const altitudes = trackingData.value.map(p => p.altitude);
    const minAltitude = Math.min(...altitudes);

    const dataWithSlope = trackingData.value.map((point, index) => {
        let slope = 0;
        if (index > 0) {
            const prevPoint = trackingData.value[index - 1];
            const altitudeChange = point.altitude - prevPoint.altitude;
            slope = (altitudeChange / segmentLength.value) * 100;
        }
        const distance = index * segmentLength.value;
        return [distance, point.altitude, slope];
    });

    totalDistance.value = dataWithSlope[dataWithSlope.length - 1][0];

    const negativeSlopeFactor = getSettingValue('Altitude/Couleurs/NegativeSlopeFactor');
    const tranche1ColorName = getSettingValue('Altitude/Couleurs/Tranche1');
    const negativeSlopeColorName = `${tranche1ColorName}-${negativeSlopeFactor}`;

    const getSlopeColor = (slope) => {
        if (slope <= 0) return toHex(negativeSlopeColorName);
        if (slope <= 2) return toHex(getSettingValue('Altitude/Couleurs/Tranche2'));
        if (slope <= 4) return toHex(getSettingValue('Altitude/Couleurs/Tranche3'));
        if (slope <= 7) return toHex(getSettingValue('Altitude/Couleurs/Tranche4'));
        if (slope <= 10) return toHex(getSettingValue('Altitude/Couleurs/Tranche5'));
        if (slope <= 12) return toHex(getSettingValue('Altitude/Couleurs/Tranche6'));
        if (slope <= 15) return toHex(getSettingValue('Altitude/Couleurs/Tranche7'));
        return toHex(getSettingValue('Altitude/Couleurs/Tranche8'));
    };

    const series = [];
    const colorPalette = [];

    for (let i = 1; i < dataWithSlope.length; i++) {
        const prevPoint = dataWithSlope[i - 1];
        const currentPoint = dataWithSlope[i];
        const segmentColor = getSlopeColor(currentPoint[2]);
        colorPalette.push(segmentColor);

        series.push({
            type: 'line',
            data: [[prevPoint[0], prevPoint[1]], [currentPoint[0], currentPoint[1]]],
            showSymbol: false,
            lineStyle: {
                color: segmentColor,
                width: 2
            },
            areaStyle: {
                color: segmentColor,
                opacity: 0.6
            }
        });
    }

    const zoomWindowKm = getSettingValue('Altitude/Visualisation/FenetreZoomKm') || 50;
    const zoomWindowMeters = zoomWindowKm * 1000;
    const endPercentage = Math.min(100, (zoomWindowMeters / totalDistance.value) * 100);

    option.value = {
        color: colorPalette,
        grid: {
            left: '50px',
            right: '20px',
            top: '20px',
            bottom: '50px' // Make space for dataZoom
        },
        dataZoom: [
            {
                type: 'slider', // Reverted to slider
                xAxisIndex: 0,
                start: 0,
                end: endPercentage,
                bottom: 10,
                height: 20,
                showDetail: false,
            }
        ],
        xAxis: {
            type: 'value',
            min: 0,
            max: totalDistance.value,
            axisLabel: {
                formatter: val => `${val / 1000}km`,
            },
            splitLine: {
                show: true,
                lineStyle: {
                    color: '#444'
                }
            },
            minInterval: (getSettingValue('Altitude/Visualisation/RepereDistance') || 10) * 1000, // Force interval
            interval: (getSettingValue('Altitude/Visualisation/RepereDistance') || 10) * 1000,
        },
        yAxis: {
            type: 'value',
            min: Math.floor(minAltitude / 100) * 100 - 50,
            axisLabel: {
                formatter: '{value} m'
            },
            splitLine: {
                show: true,
                lineStyle: {
                    color: '#444'
                }
            },
            interval: getSettingValue('Altitude/Visualisation/RepereAltitude') || 500,
        },
        tooltip: {
            trigger: 'axis',
            formatter: (params) => {
                if (!params || params.length === 0) return;
                const dataPoint = params[0].data;
                return `Distance: ${(dataPoint[0] / 1000).toFixed(2)} km<br/>Altitude: ${dataPoint[1].toFixed(1)} m`;
            }
        },
        series: series
    };
}

onMounted(() => {
    loadTrackingData();
});

/*
watch(() => props.currentDistance, (newDistance) => {
    if (Math.abs(newDistance - lastUpdatedDistance) < 100) {
        return; // Throttle updates
    }
    lastUpdatedDistance = newDistance;

    if (!option.value.dataZoom || totalDistance.value === 0) return;

    const zoomWindowKm = getSettingValue('Altitude/Visualisation/FenetreZoomKm') || 50;
    const zoomWindowMeters = zoomWindowKm * 1000;
    const windowPercentage = (zoomWindowMeters / totalDistance.value) * 100;
    const currentPercentage = (newDistance / totalDistance.value) * 100;

    let start = currentPercentage - (windowPercentage / 2);
    let end = currentPercentage + (windowPercentage / 2);

    if (start < 0) {
        start = 0;
        end = windowPercentage;
    }
    if (end > 100) {
        end = 100;
        start = 100 - windowPercentage;
    }

    // Update reactive option
    option.value.dataZoom[0].start = start;
    option.value.dataZoom[0].end = end;
});
*/

</script>

<style scoped>
.chart {
    height: 180px; /* Increased height to accommodate dataZoom */
    width: 100%;
}
</style>