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

    <!-- Stop Button -->
    <v-btn icon="mdi-stop-circle-outline" variant="text" @click="interruptUpdate" color="error" class="ml-2"></v-btn>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import { useCommuneColor } from '@/composables/useCommuneColor';

const { majCommuneIsRunning, circuitsProgress, updatingCircuitId, updatingCircuitName, interruptUpdate } = useCommunesUpdate();

const progress = computed(() => {
  if (!updatingCircuitId.value || !circuitsProgress.value[updatingCircuitId.value]) {
    return 0;
  }
  return circuitsProgress.value[updatingCircuitId.value];
});

const { color: progressColor } = useCommuneColor(progress);

</script>
