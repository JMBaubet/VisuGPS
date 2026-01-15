<template>
  <v-sheet v-if="majCommuneIsRunning && updatingCircuitId" border rounded class="pa-2 d-flex align-center">
    <!-- Status Icon -->
    <v-icon color="green" size="36">mdi-city-variant</v-icon>

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
    <div class="d-flex align-center ml-8">
      <div class="d-flex flex-column align-center">
        <div class="text-caption">IGN</div>
        <v-switch
          v-model="isIgnEnabled"
          color="success"
          base-color="error"
          density="compact"
          hide-details
        ></v-switch>
      </div>
      <div class="d-flex flex-column align-center ml-4">
        <div class="text-caption">Mapbox</div>
        <v-switch
          v-model="isMapboxEnabled"
          color="success"
          base-color="error"
          density="compact"
          hide-details
        ></v-switch>
      </div>
    </div>

    <!-- Stop Button -->
    <v-btn icon="mdi-stop-circle-outline" variant="text" @click="interruptUpdate" color="error" class="ml-2"></v-btn>
  </v-sheet>
</template>

<script setup>
import { computed, ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import { useCommuneColor } from '@/composables/useCommuneColor';

const { majCommuneIsRunning, circuitsProgress, updatingCircuitId, updatingCircuitName, interruptUpdate, isIgnEnabled, isMapboxEnabled } = useCommunesUpdate();

const progress = computed(() => {
  if (!updatingCircuitId.value || !circuitsProgress.value[updatingCircuitId.value]) {
    return 0;
  }
  return circuitsProgress.value[updatingCircuitId.value];
});

const { color: progressColor } = useCommuneColor(progress);

// Watch for user changes and call Tauri commands
watch(isIgnEnabled, (newValue) => {
  invoke('toggle_ign_api', { enable: newValue });
});

watch(isMapboxEnabled, (newValue) => {
  invoke('toggle_mapbox_api', { enable: newValue });
});

</script>
