<template>
  <v-sheet rounded="lg" class="control-tabs-widget">
    <v-tabs v-model="tab" bg-color="surface" density="compact" color="orange-darken-1">
      <v-tab value="camera">Caméra</v-tab>
      <v-tab value="events">Événements</v-tab>
    </v-tabs>

    <v-window v-model="tab" style="flex-grow: 1;" class="fill-height">
      <!-- Onglet Caméra -->
      <v-window-item value="camera" class="fill-height">
        <div style="height: 100%; overflow-y: auto;">
          <!-- Toolbar Buttons -->
          <div class="d-flex justify-space-around align-center pa-2">
            <v-btn icon="mdi-content-save" @click="emit('save-control-point')" color="primary"></v-btn>
            <v-btn v-if="isCurrentPointControlPoint" icon="mdi-delete" @click="emit('delete-control-point')" color="warning"></v-btn>
            <v-btn-toggle
              v-model="cameraSyncModeModel"
              rounded="pill"
              mandatory
            >
              <v-btn value="off" icon="mdi-camera-off" color="primary"></v-btn>
              <v-btn value="original" icon="mdi-camera-outline" color="warning"></v-btn>
              <v-btn value="edited" icon="mdi-camera-plus-outline" color="success"></v-btn>
            </v-btn-toggle>
          </div>
          <v-divider></v-divider>

          <!-- Graph Controls -->
          <div class="pa-2" style="display: flex; flex-direction: column; gap: 4px;">
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
        <div style="height: 100%; display: flex; flex-direction: column;">
          <v-tabs v-model="eventTab" density="compact" bg-color="surface" color="orange-darken-1">
            <v-tab value="pause">Pause</v-tab>
            <v-tab value="flyto">Flyto</v-tab>
            <v-tab value="message">Message</v-tab>
          </v-tabs>

          <v-window v-model="eventTab" style="flex-grow: 1;">
            <v-window-item value="pause">
              <div class="pa-2 d-flex align-center">
                <v-btn v-if="isPauseEvent" color="error" variant="text" @click="onDeletePause">
                  <span class="mr-2">Supprimer la pause</span>
                  <v-icon icon="mdi-delete"></v-icon>
                </v-btn>
                <v-btn v-else color="primary" variant="text" :disabled="isFlytoEvent" @click="emit('add-pause')">
                  <span class="mr-2">Ajouter Pause</span>
                  <v-icon icon="mdi-plus"></v-icon>
                </v-btn>
                <span v-if="isFlytoEvent" class="text-caption ml-4 text-error">
                  (Conflit avec Flyto)
                </span>
              </div>
            </v-window-item>

            <v-window-item value="flyto">
              <div class="pa-2 d-flex flex-column">
                <div class="d-flex align-center mt-2">
                  <v-btn v-if="isFlytoEvent" color="error" variant="text" @click="onDeleteFlyto">
                    <span class="mr-2">Supprimer Flyto</span>
                    <v-icon icon="mdi-delete"></v-icon>
                  </v-btn>
                  <v-btn v-else color="primary" variant="text" :disabled="isPauseEvent" @click="onAddFlyto">
                    <span class="mr-2">Ajouter Flyto</span>
                    <v-icon icon="mdi-plus"></v-icon>
                  </v-btn>
                  <span v-if="isPauseEvent" class="text-caption ml-4 text-error">
                    (Conflit avec Pause)
                  </span>
                </div>
                <v-divider class="my-2"></v-divider>
                <div class="d-flex align-center justify-center mb-2">
                  <span class="text-subtitle-2">Durée du survol : {{ (props.flytoDurationSetting / 1000).toFixed(1) }} s</span>
                </div>
                <v-slider
                  :model-value="props.flytoDurationSetting"
                  @update:model-value="(val) => emit('update:flytoDurationSetting', val)"
                  min="200"
                  max="10000"
                  step="100"
                  hide-details
                  class="align-center"
                >
                </v-slider>
              </div>
            </v-window-item>

            <v-window-item value="message">
              <div class="pa-2 d-flex flex-column">
                <v-combobox
                  label="Texte du message"
                  :items="knownMessageTexts"
                  v-model="messageText"
                  density="compact"
                  variant="outlined"
                  hide-details
                  class="mb-2"
                ></v-combobox>

                <div class="d-flex align-center mb-2">
                  <v-btn icon="mdi-palette" size="small" :color="messageBackgroundColor" @click="showColorPicker('background')"></v-btn>
                  <span class="ml-2 text-caption">Fond</span>
                  <v-spacer></v-spacer>
                  <v-btn icon="mdi-palette" size="small" :color="messageBorderColor" @click="showColorPicker('border')"></v-btn>
                  <span class="ml-2 text-caption">Bordure</span>
                </div>

                <v-slider
                  label="Taille bordure (px)"
                  v-model="messageBorderWidth"
                  :min="messageBorderWidthMin"
                  :max="messageBorderWidthMax"
                  :step="messageBorderWidthStep"
                  thumb-label
                  hide-details
                  class="mb-2"
                ></v-slider>

                <v-slider
                  label="Pré-affichage (incréments)"
                  v-model="messagePreAffichage"
                  :min="messagePreAffichageMin"
                  :max="messagePreAffichageMax"
                  :step="messagePreAffichageStep"
                  thumb-label
                  hide-details
                  class="mb-2"
                ></v-slider>

                <v-slider
                  label="Post-affichage (incréments)"
                  v-model="messagePostAffichage"
                  :min="messagePostAffichageMin"
                  :max="messagePostAffichageMax"
                  :step="messagePostAffichageStep"
                  thumb-label
                  hide-details
                  class="mb-2"
                ></v-slider>


                <div class="d-flex justify-space-around align-center mt-2">
                  <v-btn v-if="isMessageEvent" color="error" variant="text" @click="onDeleteMessage">
                    <span class="mr-2">Supprimer Message</span>
                    <v-icon icon="mdi-delete"></v-icon>
                  </v-btn>
                  <v-btn v-else color="primary" variant="text" @click="onAddMessage">
                    <span class="mr-2">Ajouter Message</span>
                    <v-icon icon="mdi-plus"></v-icon>
                  </v-btn>
                </div>
              </div>
            </v-window-item>

            <v-dialog v-model="colorPickerDialog" max-width="400px">
              <v-card>
                <v-card-title>Sélectionner une couleur</v-card-title>
                <v-card-text>
                  <v-color-picker v-model="selectedColor" :swatches="baseSwatches" show-swatches hide-inputs class="mx-auto"></v-color-picker>
                </v-card-text>
                <v-card-actions>
                  <v-spacer></v-spacer>
                  <v-btn text @click="colorPickerDialog = false">Annuler</v-btn>
                  <v-btn color="primary" @click="applyColor">Appliquer</v-btn>
                </v-card-actions>
              </v-card>
            </v-dialog>

          </v-window>
        </div>
      </v-window-item>
    </v-window>
  </v-sheet>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const tab = ref('camera');
const eventTab = ref('pause');

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
  pauseEvents: Array, // This prop now receives pauseEventsForDisplay from EditView.vue
  flytoEvents: Array,
  flytoDurationSetting: Number,
  // Message props
  messageEvents: Array,
  knownMessageTexts: Array,
  messageBackgroundColorSetting: String,
  messageBorderColorSetting: String,
  messageBorderWidthSetting: Number,
  messagePreAffichageSetting: Number,
  messagePostAffichageSetting: Number,
  // Toolbar props
  isCurrentPointControlPoint: Boolean,
  cameraSyncMode: String,
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
  'add-pause', // Added this
  'add-flyto',
  'delete-flyto',
  'update:flytoDurationSetting',
  // Message emits
  'add-message',
  'delete-message',
  'update:messageBackgroundColorSetting',
  'update:messageBorderColorSetting',
  'update:messageBorderWidthSetting',
  'update:messagePreAffichageSetting',
  'update:messagePostAffichageSetting',
  // Toolbar emits
  'save-control-point',
  'delete-control-point',
  'update:cameraSyncMode',
  // Mode emits
  'flyto-active',
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

const cameraSyncModeModel = createModel('cameraSyncMode');

// --- Message Event Logic ---
const messageText = ref('');
const messageBackgroundColor = createModel('messageBackgroundColorSetting');
const messageBorderColor = createModel('messageBorderColorSetting');
const messageBorderWidth = createModel('messageBorderWidthSetting');
const messagePreAffichage = createModel('messagePreAffichageSetting');
const messagePostAffichage = createModel('messagePostAffichageSetting');

const messageBorderWidthMin = ref(0); // TODO: Get from settings
const messageBorderWidthMax = ref(10); // TODO: Get from settings
const messageBorderWidthStep = ref(1); // TODO: Get from settings

const messagePreAffichageMin = ref(0); // TODO: Get from settings
const messagePreAffichageMax = ref(50); // TODO: Get from settings
const messagePreAffichageStep = ref(1); // TODO: Get from settings

const messagePostAffichageMin = ref(0); // TODO: Get from settings
const messagePostAffichageMax = ref(100); // TODO: Get from settings
const messagePostAffichageStep = ref(1); // TODO: Get from settings

const colorPickerDialog = ref(false);
const selectedColor = ref('');
const colorPickerTarget = ref(''); // 'background' or 'border'

const showColorPicker = async (target) => {
  colorPickerTarget.value = target;
  const colorName = target === 'background' ? messageBackgroundColor.value : messageBorderColor.value;
  
  try {
    // Convert Vuetify name to hex for the color picker
    selectedColor.value = await invoke('convert_vuetify_color', { colorName: colorName });
  } catch (error) {
    console.error(`Failed to convert color ${colorName}:`, error);
    selectedColor.value = '#000000'; // Fallback to black on error
  }
  
  colorPickerDialog.value = true;
};

const { toName, baseSwatches } = useVuetifyColors();

const applyColor = () => {
  // Convert the selected hex color back to its closest Vuetify name
  const colorName = toName(selectedColor.value);

  if (colorPickerTarget.value === 'background') {
    messageBackgroundColor.value = colorName;
  } else {
    messageBorderColor.value = colorName;
  }
  colorPickerDialog.value = false;
};

const isMessageEvent = computed(() => {
  return props.messageEvents.includes(props.currentIncrement);
});

const onAddMessage = () => {
  if (!messageText.value) {
    // Optionally show a snackbar error
    return;
  }
  const messageData = {
    text: messageText.value,
    preAffichage: messagePreAffichage.value,
    postAffichage: messagePostAffichage.value,
    coord: null, // Placeholder, will be filled in parent
    backgroundColor: messageBackgroundColor.value,
    borderColor: messageBorderColor.value,
    borderWidth: messageBorderWidth.value,
    borderRadius: 5, // TODO: Get from settings
  };
  console.log('Emitting add-message with data:', JSON.stringify(messageData, null, 2));
  emit('add-message', messageData);
};

const onDeleteMessage = () => {
  emit('delete-message');
};


// --- Logic for Events Tab --- 
const isPauseEvent = computed(() => {
    return props.pauseEvents.includes(props.currentIncrement);
});

const isFlytoEvent = computed(() => {
    return props.flytoEvents.includes(props.currentIncrement);
});

const onDeletePause = () => {
    emit('delete-pause');
};

const onAddFlyto = () => {
    emit('add-flyto', props.flytoDurationSetting);
};

const onDeleteFlyto = () => {
    emit('delete-flyto');
};

// --- Logic for Flyto mode --- 
const isFlytoActive = computed(() => {
  return tab.value === 'events' && eventTab.value === 'flyto';
});

watch(isFlytoActive, (newValue) => {
  emit('flyto-active', newValue);
});
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
