<template>
  <div v-if="majCommuneIsRunning && updatingCircuitId" class="d-flex align-center">
    <!-- Status Icon -->
    <v-icon color="green" class="pl-2" size="36">mdi-city-variant</v-icon>

    <div class="d-flex flex-column ml-4">
        <!-- Circuit Name -->
        <div class="text-caption">{{ updatingCircuitName }}</div>
        <!-- Progress Bar -->
        <div style="width: 150px;">
            <v-progress-linear
                :model-value="progress"
                :color="progressColor"
                height="8"
                rounded
            ></v-progress-linear>
        </div>
    </div>

    <!-- API Toggles -->
    <div class="d-flex align-center ml-4">
      <v-switch
        v-model="isIgnEnabled"
        label="IGN"
        color="success"
        base-color="error"
        density="compact"
        hide-details
      ></v-switch>
      <v-switch
        v-model="isMapboxEnabled"
        label="Mapbox"
        color="success"
        base-color="error"
        density="compact"
        hide-details
        class="ml-4"
      ></v-switch>
    </div>

    <!-- Stop Button -->
    <v-btn icon="mdi-stop-circle-outline" variant="text" @click="interruptUpdate" color="error" class="ml-2"></v-btn>
  </div>
</template>

<script setup>
import { computed, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import { useCommuneColor } from '@/composables/useCommuneColor';

const { majCommuneIsRunning, circuitsProgress, updatingCircuitId, updatingCircuitName, interruptUpdate } = useCommunesUpdate();

const isIgnEnabled = ref(true);
const isMapboxEnabled = ref(true);

const progress = computed(() => {
  if (!updatingCircuitId.value || !circuitsProgress.value[updatingCircuitId.value]) {
    return 0;
  }
  return circuitsProgress.value[updatingCircuitId.value];
});

const { color: progressColor } = useCommuneColor(progress);

// Fetch initial switch states when the component becomes active
watch(majCommuneIsRunning, async (isRunning) => {
  if (isRunning) {
    try {
      isIgnEnabled.value = await invoke('get_ign_status');
      isMapboxEnabled.value = await invoke('get_mapbox_status');
    } catch (e) {
      console.error("Failed to get initial API statuses:", e);
    }
  }
});

// Watch for changes and call Tauri commands
watch(isIgnEnabled, (newValue) => {
  invoke('toggle_ign_api', { enable: newValue });
});

watch(isMapboxEnabled, (newValue) => {
  invoke('toggle_mapbox_api', { enable: newValue });
});

</script>
