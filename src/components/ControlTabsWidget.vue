<template>
  <v-sheet rounded="lg" class="control-tabs-widget">
    <v-tabs v-model="tab" bg-color="primary" grow height="48">
      <v-tab value="camera">Caméra</v-tab>
      <v-tab value="events">Événements</v-tab>
    </v-tabs>

    <v-window v-model="tab" style="flex-grow: 1;" class="fill-height">
      <!-- Onglet Caméra -->
      <v-window-item value="camera" class="fill-height">
        <div class="pa-2" style="height: 100%; overflow-y: auto;">
          <div style="display: flex; flex-direction: column; gap: 4px;">
            <div style="display: flex; flex-direction: column; margin-left: 8px;">
              <v-checkbox v-model="showOriginalCurvesModel" label="Origine" hide-details density="compact" color="warning"></v-checkbox>
              <v-checkbox v-model="showEditedCurvesModel" label="Edité" hide-details density="compact" color="success"></v-checkbox>
            </div>
            <v-divider class="my-1"></v-divider>
            <v-switch v-model="showBearingDeltaPairModel" label="Δ Cap" :color="graphBearingDeltaColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
            <v-switch v-model="showBearingTotalDeltaPairModel" label="Cap" :color="graphBearingTotalDeltaColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
            <v-switch v-model="showZoomPairModel" label="Zoom" :color="graphZoomColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
            <v-switch v-model="showPitchPairModel" label="Pitch" :color="graphPitchColor" hide-details density="compact" style="margin-left: 8px; margin-right: 8px;"></v-switch>
          </div>
        </div>
      </v-window-item>

      <!-- Onglet Événements -->
      <v-window-item value="events" class="fill-height">
        <div class="pa-2">
            <v-btn v-if="isPauseEvent" icon="mdi-delete" color="error" variant="text"
                @click="onDeletePause" title="Supprimer la pause"></v-btn>
            <span v-else class="text-caption pa-4">Appuyez sur 'P' pour ajouter une pause.</span>
        </div>
      </v-window-item>
    </v-window>
  </v-sheet>
</template>

<script setup>
import { ref, computed } from 'vue';

const tab = ref('camera');

// --- Props --- 
const props = defineProps({
  // Graph props
  showOriginalCurves: Boolean,
  showEditedCurves: Boolean,
  showBearingDeltaPair: Boolean,
  showBearingTotalDeltaPair: Boolean,
  showZoomPair: Boolean,
  showPitchPair: Boolean,
  graphBearingDeltaColor: String,
  graphBearingTotalDeltaColor: String,
  graphZoomColor: String,
  graphPitchColor: String,
  // Event props
  currentIncrement: Number,
  pauseEvents: Array,
});

// --- Emits --- 
const emit = defineEmits([
  // Graph emits
  'update:showOriginalCurves',
  'update:showEditedCurves',
  'update:showBearingDeltaPair',
  'update:showBearingTotalDeltaPair',
  'update:showZoomPair',
  'update:showPitchPair',
  // Event emits
  'delete-pause',
]);

// --- Models for Graph Tab --- 
const createModel = (propName) => computed({
  get: () => props[propName],
  set: (value) => emit(`update:${propName}`, value),
});

const showOriginalCurvesModel = createModel('showOriginalCurves');
const showEditedCurvesModel = createModel('showEditedCurves');
const showBearingDeltaPairModel = createModel('showBearingDeltaPair');
const showBearingTotalDeltaPairModel = createModel('showBearingTotalDeltaPair');
const showZoomPairModel = createModel('showZoomPair');
const showPitchPairModel = createModel('showPitchPair');

// --- Logic for Events Tab --- 
const isPauseEvent = computed(() => {
    return props.pauseEvents.includes(props.currentIncrement);
});

const onDeletePause = () => {
    emit('delete-pause');
};
</script>

<style scoped>
.control-tabs-widget {
  background-color: rgba(var(--v-theme-surface), 0.8);
  backdrop-filter: blur(4px);
  width: 392px; /* Fixed width */
  flex-shrink: 0; /* Prevent shrinking */
  height: 400px; /* Match graph height */
  display: flex;
  flex-direction: column;
}
</style>
