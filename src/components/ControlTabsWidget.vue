<template>
  <v-sheet rounded="lg" class="control-tabs-widget">
    <v-tabs v-model="mainTab" bg-color="surface" density="compact" color="orange-darken-1">
      <v-tab value="camera">Caméra</v-tab>
      <v-tab value="stop">Pause / Survol</v-tab>
      <v-tab value="message">Message</v-tab>
    </v-tabs>

    <v-window v-model="mainTab" style="flex-grow: 1;" class="fill-height">
      <!-- Onglet Caméra -->
      <v-window-item value="camera" class="fill-height">
        <div style="height: 100%; overflow-y: auto;" class="d-flex flex-column">
          <!-- Top Toolbar -->
          <div class="pa-2">
            <CameraSyncModeSelector v-model="cameraSyncModeModel" />
          </div>
          <v-divider></v-divider>

          <!-- Checkbox Table -->
          <div class="pa-2">
            <v-table density="compact">
              <thead>
                <tr>
                  <th class="text-left">Courbe</th>
                  <th class="text-left">Calculée</th>
                  <th class="text-left">Editée</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <td>Δ Cap</td>
                  <td class="text-center">
                    <div class="d-flex justify-center">
                      <v-checkbox-btn v-model="showCalculeeBearingDeltaModel" :color="props.colorOrigineBearingDelta"></v-checkbox-btn>
                    </div>
                  </td>
                  <td class="text-center">
                    <div class="d-flex justify-center">
                      <v-checkbox-btn v-model="showEditeeBearingDeltaModel" :color="props.colorEditedBearingDelta"></v-checkbox-btn>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td>Cap</td>
                  <td class="text-center">
                    <div class="d-flex justify-center">
                      <v-checkbox-btn v-model="showCalculeeBearingTotalDeltaModel" :color="props.colorOrigineBearingTotalDelta"></v-checkbox-btn>
                    </div>
                  </td>
                  <td class="text-center">
                    <div class="d-flex justify-center">
                      <v-checkbox-btn v-model="showEditeeBearingTotalDeltaModel" :color="props.colorEditedBearingTotalDelta"></v-checkbox-btn>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td>Zoom</td>
                  <td class="text-center">
                    <!-- Empty cell -->
                  </td>
                  <td class="text-center">
                    <div class="d-flex justify-center">
                      <v-checkbox-btn v-model="showEditeeZoomModel" :color="props.colorEditedZoom"></v-checkbox-btn>
                    </div>
                  </td>
                </tr>
                <tr>
                  <td>Pitch</td>
                  <td class="text-center">
                    <!-- Empty cell -->
                  </td>
                  <td class="text-center">
                    <div class="d-flex justify-center">
                      <v-checkbox-btn v-model="showEditeePitchModel" :color="props.colorEditedPitch"></v-checkbox-btn>
                    </div>
                  </td>
                </tr>
              </tbody>
            </v-table>
          </div>

          <v-spacer></v-spacer>

          <!-- Bottom Buttons -->
          <v-divider></v-divider>
          <div class="pa-2 d-flex flex-column align-center">
              <v-btn color="primary" variant="text" @click="emit('save-control-point')">
                  <span class="mr-2">Ajouter Point</span>
                  <v-icon icon="mdi-plus"></v-icon>
              </v-btn>
              <v-btn v-if="isCurrentPointControlPoint" color="error" variant="text" @click="emit('delete-control-point')">
                  <span class="mr-2">Supprimer Point</span>
                  <v-icon icon="mdi-delete"></v-icon>
              </v-btn>
          </div>
        </div>
      </v-window-item>

      <!-- Onglet Pause / Survol -->
      <v-window-item value="stop">
        <div class="pa-2 d-flex flex-column">
          <CameraSyncModeSelector v-model="cameraSyncModeModel" class="mb-4"/>
          <v-divider class="mb-4"></v-divider>
          <!-- Flyto duration slider (always visible) -->
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
          <v-divider class="my-2"></v-divider>

          <!-- Conditional Buttons -->
          <!-- Case 1: A Pause event exists -->
          <div v-if="isPauseEvent" class="d-flex align-center mt-2">
            <v-btn color="error" variant="text" @click="onDeletePause">
              <span class="mr-2">Supprimer la Pause</span>
              <v-icon icon="mdi-delete"></v-icon>
            </v-btn>
          </div>

          <!-- Case 2: A Flyto event exists -->
          <div v-else-if="isFlytoEvent" class="d-flex align-center mt-2">
            <v-btn color="error" variant="text" @click="onDeleteFlyto">
              <span class="mr-2">Supprimer le Survol</span>
              <v-icon icon="mdi-delete"></v-icon>
            </v-btn>
          </div>

          <!-- Case 3: No event exists -->
          <div v-else class="d-flex justify-space-around align-center mt-2">
            <v-btn color="primary" variant="text" @click="emit('add-pause')">
              <span class="mr-2">Ajouter Pause</span>
              <v-icon icon="mdi-plus"></v-icon>
            </v-btn>
            <v-btn color="primary" variant="text" @click="onAddFlyto">
              <span class="mr-2">Ajouter Survol</span>
              <v-icon icon="mdi-plus"></v-icon>
            </v-btn>
          </div>
        </div>
      </v-window-item>

      <!-- Onglet Message -->
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
  </v-sheet>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import CameraSyncModeSelector from './CameraSyncModeSelector.vue';

const mainTab = ref('camera');

// --- Props --- 
const props = defineProps({
  // Graph props
  showCalculeeBearingDelta: Boolean,
  showEditeeBearingDelta: Boolean,
  showCalculeeBearingTotalDelta: Boolean,
  showEditeeBearingTotalDelta: Boolean,
  showEditeeZoom: Boolean,
  showEditeePitch: Boolean,
  colorOrigineBearingDelta: String,
  colorEditedBearingDelta: String,
  colorOrigineBearingTotalDelta: String,
  colorEditedBearingTotalDelta: String,
  colorOrigineZoom: String,
  colorEditedZoom: String,
  colorOriginePitch: String,
  colorEditedPitch: String,
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
  'update:showCalculeeBearingDelta',
  'update:showEditeeBearingDelta',
  'update:showCalculeeBearingTotalDelta',
  'update:showEditeeBearingTotalDelta',
  'update:showEditeeZoom',
  'update:showEditeePitch',
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
  'update:marker-visible',
  'tab-changed',
]);

watch(mainTab, (newTab) => {
  emit('tab-changed', newTab);
});

// --- Models for Graph Tab --- 
const createModel = (propName) => computed({
  get: () => props[propName],
  set: (value) => emit(`update:${propName}`, value),
});

const showCalculeeBearingDeltaModel = createModel('showCalculeeBearingDelta');
const showEditeeBearingDeltaModel = createModel('showEditeeBearingDelta');
const showCalculeeBearingTotalDeltaModel = createModel('showCalculeeBearingTotalDelta');
const showEditeeBearingTotalDeltaModel = createModel('showEditeeBearingTotalDelta');
const showEditeeZoomModel = createModel('showEditeeZoom');
const showEditeePitchModel = createModel('showEditeePitch');

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

const isMarkerVisible = computed(() => {
  return mainTab.value === 'stop' || mainTab.value === 'message';
});

watch(isMarkerVisible, (newValue) => {
  emit('update:marker-visible', newValue);
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
