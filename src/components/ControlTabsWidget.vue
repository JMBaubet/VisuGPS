<template>
  <v-sheet rounded="lg" class="control-tabs-widget">
    <v-tabs v-model="mainTab" bg-color="surface" density="compact" color="orange-darken-1">
      <v-tab value="camera">Caméra</v-tab>
      <v-tab value="stop">Pause/Zooms</v-tab>
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

          <v-divider class="my-2"></v-divider>

          <v-tabs v-model="zoomTab" bg-color="surface" density="compact" color="primary" class="mb-2">
            <v-tab value="depart">Zoom Départ</v-tab>
            <v-tab value="arrivee">Zoom Arrivée</v-tab>
          </v-tabs>

          <v-window v-model="zoomTab">
            <v-window-item value="depart">
              <div class="d-flex flex-column pa-0">
                <v-row dense no-gutters class="align-center">
                  <v-col cols="2">
                    <v-checkbox
                      v-model="zoomDepartModel"
                      color="primary"
                      density="compact"
                      hide-details
                      class="flex-grow-0 mt-0 pt-0"
                    ></v-checkbox>
                  </v-col>
                  <v-col cols="10">
                    <v-slider
                      v-model="zoomDepartValeurModel"
                      :disabled="!zoomDepartModel"
                      :label="`Zoom : ${zoomDepartValeurModel.toFixed(1)}`"
                      :min="16.5"
                      :max="20"
                      :step="0.1"
                      hide-details
                      density="compact"
                      class="mt-2"
                    >
                    </v-slider>
                  </v-col>
                </v-row>
                <v-row dense no-gutters>
                  <v-col cols="12">
                    <v-slider
                      v-model="zoomDepartDistanceModel"
                      :disabled="!zoomDepartModel"
                      :label="`Distance : ${zoomDepartDistanceModel * 100}m`"
                      :min="5"
                      :max="50"
                      :step="1"
                      hide-details
                      density="compact"
                      class="mt-2"
                    >
                    </v-slider>
                  </v-col>
                </v-row>
              </div>
            </v-window-item>

            <v-window-item value="arrivee">
              <div class="d-flex flex-column pa-0">
                <v-row dense no-gutters class="align-center">
                  <v-col cols="2">
                    <v-checkbox
                      v-model="zoomArriveeModel"
                      color="primary"
                      density="compact"
                      hide-details
                      class="flex-grow-0 mt-0 pt-0"
                    ></v-checkbox>
                  </v-col>
                  <v-col cols="10">
                    <v-slider
                      v-model="zoomArriveeValeurModel"
                      :disabled="!zoomArriveeModel"
                      :label="`Zoom : ${zoomArriveeValeurModel.toFixed(1)}`"
                      :min="16.5"
                      :max="20"
                      :step="0.1"
                      hide-details
                      density="compact"
                      class="mt-2"
                    >
                    </v-slider>
                  </v-col>
                </v-row>
                <v-row dense no-gutters>
                  <v-col cols="12">
                    <v-slider
                      v-model="zoomArriveeDistanceModel"
                      :disabled="!zoomArriveeModel"
                      :label="`Distance : ${zoomArriveeDistanceModel * 100}m`"
                      :min="5"
                      :max="50"
                      :step="1"
                      hide-details
                      density="compact"
                      class="mt-2"
                    >
                    </v-slider>
                  </v-col>
                </v-row>
              </div>
            </v-window-item>
          </v-window>
        </div>
      </v-window-item>

      <!-- Onglet Message -->
      <v-window-item value="message">
        <div class="pa-2 d-flex flex-column">
          <v-row dense>
            <v-col cols="12">
              <v-combobox
                label="Texte du message"
                :items="knownMessageTexts"
                v-model="messageText"
                density="compact"
                variant="outlined"
                hide-details
              ></v-combobox>
            </v-col>
          </v-row>

          <v-divider class="my-4"></v-divider>

          <v-row dense class="align-center">
            <v-col cols="2" class="d-flex justify-center">
              <v-btn icon="mdi-palette" size="small" :color="messageBackgroundColor" @click="showColorPicker('background')"></v-btn>
            </v-col>
            <v-col cols="2" class="d-flex justify-center">
              <v-btn icon="mdi-palette" size="small" :color="messageBorderColor" @click="showColorPicker('border')"></v-btn>
            </v-col>
            <v-col cols="4">
              <v-slider
                v-model="messageBorderWidth"
                :min="messageBorderWidthMin"
                :max="messageBorderWidthMax"
                :step="messageBorderWidthStep"
                thumb-label
                hide-details
                :thumb-color="messageBorderColor"
              ></v-slider>
            </v-col>
            <v-col cols="4">
              <v-slider
                v-model="messageBorderRadius"
                :min="messageBorderRadiusMin"
                :max="messageBorderRadiusMax"
                :step="messageBorderRadiusStep"
                thumb-label
                hide-details
                :thumb-color="messageBorderColor"
              ></v-slider>
            </v-col>
          </v-row>

          <v-row dense>
            <v-col cols="2" class="text-center text-caption">Fond</v-col>
            <v-col cols="2" class="text-center text-caption">Bordure</v-col>
            <v-col cols="4" class="text-center text-caption">Taille (px)</v-col>
            <v-col cols="4" class="text-center text-caption">Rayon (px)</v-col>
          </v-row>

          <v-divider class="my-4"></v-divider>

          <v-row dense>
            <v-col cols="12">
              <v-range-slider
                v-model="messageDisplayRange"
                :min="messagePreAffichageMin"
                :max="messagePostAffichageMax"
                :step="messagePreAffichageStep"
                thumb-label="always"
                hide-details
              >
                <template v-slot:thumb-label="{ modelValue }">
                  <span v-if="modelValue < 0">{{ Math.abs(modelValue) }}</span>
                  <span v-else>{{ modelValue }}</span>
                </template>
              </v-range-slider>
              <div class="text-caption text-center">Durée d'affichage (Pré et Post incréments)</div>
            </v-col>
          </v-row>


          <v-row dense>
            <v-col cols="12" class="d-flex justify-space-around align-center">
              <v-btn v-if="isMessageEvent" color="error" variant="text" @click="onDeleteMessage">
                <span class="mr-2">Supprimer Message</span>
                <v-icon icon="mdi-delete"></v-icon>
              </v-btn>
              <v-btn v-else color="primary" variant="text" @click="onAddMessage">
                <span class="mr-2">Ajouter Message</span>
                <v-icon icon="mdi-plus"></v-icon>
              </v-btn>
            </v-col>
          </v-row>

          <v-row dense>
            <v-col cols="12">
              <v-card
                class="mt-1 mb-1 mx-auto"
                :style="previewMessageStyle"
                elevation="2"
              >
                <span v-if="messageText">{{ messageText }}</span>
                <span v-else class="text-disabled">Prévisualisation du message</span>
              </v-card>
            </v-col>
          </v-row>
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
const zoomTab = ref('depart');

const { toHex, toName, baseSwatches } = useVuetifyColors();

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
  fullMessageEvents: { type: Array, default: () => [] },
  knownMessageTexts: Array,
  messageBackgroundColorSetting: String,
  messageBorderColorSetting: String,
  messageBorderWidthSetting: Number,
  messagePreAffichageSetting: Number,
    messagePostAffichageSetting: Number,
    messageBorderRadiusSetting: Number,
    // Toolbar props
    isCurrentPointControlPoint: Boolean,
    cameraSyncMode: String,
    // Zoom Depart props
    zoomDepart: Boolean,
    zoomDepartValeur: Number,
    zoomDepartDistance: Number,
    zoomArrivee: Boolean,
    zoomArriveeValeur: Number,
    distanceZoomArrivee: Number,
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
    'update:messageBorderRadiusSetting',
    // Toolbar emits
    'save-control-point',
    'delete-control-point',
    'update:cameraSyncMode',
    // Mode emits
    'update:marker-visible',
    'tab-changed',
    // Zoom Depart emits
    'update:zoomDepart',
    'update:zoomDepartValeur',
    'update:zoomDepartDistance',
    'update:zoomArrivee',
    'update:zoomArriveeValeur',
    'update:distanceZoomArrivee',
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
  const zoomDepartModel = createModel('zoomDepart');
  const zoomDepartValeurModel = createModel('zoomDepartValeur');
  const zoomDepartDistanceModel = createModel('zoomDepartDistance');
  const zoomArriveeModel = createModel('zoomArrivee');
  const zoomArriveeValeurModel = createModel('zoomArriveeValeur');
  const zoomArriveeDistanceModel = createModel('distanceZoomArrivee');
  
  // --- Message Event Logic ---
  const messageText = ref('');
  const messageBackgroundColor = createModel('messageBackgroundColorSetting');
  const messageBorderColor = createModel('messageBorderColorSetting');
  const messageBorderWidth = createModel('messageBorderWidthSetting');
  const messageBorderRadius = createModel('messageBorderRadiusSetting');
  
  const messageBorderWidthMin = ref(0); // TODO: Get from settings
  const messageBorderWidthMax = ref(10); // TODO: Get from settings
  const messageBorderWidthStep = ref(1); // TODO: Get from settings
  
  const messageBorderRadiusMin = ref(0); // TODO: Get from settings
  const messageBorderRadiusMax = ref(20); // TODO: Get from settings
  const messageBorderRadiusStep = ref(1); // TODO: Get from settings
  
  const messagePreAffichageMin = ref(-50); // From settings, now negative for pre-display
  const messagePostAffichageMax = ref(100); // From settings
  const messagePreAffichageStep = ref(1); // From settings
  
  const messageDisplayRange = computed({
    get: () => [-props.messagePreAffichageSetting, props.messagePostAffichageSetting],
    set: (value) => {
      let newPreAffichage = -value[0]; // Convert negative slider value to positive preAffichage
      let newPostAffichage = value[1];
  
      // Ensure preAffichage is within its valid range (0 to 50)
      newPreAffichage = Math.max(0, Math.min(50, newPreAffichage));
      // Ensure postAffichage is within its valid range (1 to 100)
      newPostAffichage = Math.max(1, Math.min(100, newPostAffichage));

      if (props.messagePreAffichageSetting !== newPreAffichage) {
        emit('update:messagePreAffichageSetting', newPreAffichage);
      }
      if (props.messagePostAffichageSetting !== newPostAffichage) {
        emit('update:messagePostAffichageSetting', newPostAffichage);
      }
    },
  });

// Function to determine contrast color (black or white)
const getContrastColor = (hexColor) => {
  if (!hexColor) return 'black'; // Default to black if no color

  // Convert hex to RGB
  let r = 0, g = 0, b = 0;
  if (hexColor.length === 7) { // #RRGGBB
    r = parseInt(hexColor.substring(1, 3), 16);
    g = parseInt(hexColor.substring(3, 5), 16);
    b = parseInt(hexColor.substring(5, 7), 16);
  } else if (hexColor.length === 4) { // #RGB
    r = parseInt(hexColor[1] + hexColor[1], 16);
    g = parseInt(hexColor[2] + hexColor[2], 16);
    b = parseInt(hexColor[3] + hexColor[3], 16);
  }

  // Calculate luminance (perceived brightness)
  // Formula: (0.299*R + 0.587*G + 0.114*B)
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

  // Use a threshold to decide between black and white
  return luminance > 0.5 ? 'black' : 'white';
};

const previewTextColor = computed(() => {
  // Convert Vuetify color name to hex for luminance calculation
  const hexBgColor = toHex(messageBackgroundColor.value);
  return getContrastColor(hexBgColor);
});

const previewMessageStyle = computed(() => {
  const hexBgColor = toHex(messageBackgroundColor.value);
  const hexBorderColor = toHex(messageBorderColor.value);

  return {
    backgroundColor: hexBgColor,
    color: previewTextColor.value,
    border: `${messageBorderWidth.value}px solid ${hexBorderColor}`,
    borderRadius: `${messageBorderRadius.value}px`,
    padding: '4px 8px',
    textAlign: 'center',
    minHeight: '40px',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    wordBreak: 'break-word',
    whiteSpace: 'pre-wrap',
    width: 'fit-content',
    fontWeight: 'bold',
  };
});

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
    preAffichage: props.messagePreAffichageSetting,
    postAffichage: props.messagePostAffichageSetting,
    coord: null, // Always null, parent will fill with map center
    backgroundColor: messageBackgroundColor.value,
    borderColor: messageBorderColor.value,
    borderWidth: messageBorderWidth.value,
    borderRadius: messageBorderRadius.value,
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

watch(() => props.currentIncrement, (newIncrement) => {
  const currentEvent = props.fullMessageEvents.find(
    event => event.anchorIncrement === newIncrement
  );

  if (currentEvent) {
    // Recalculate pre/post from the event's increments
    const preAffichage = currentEvent.anchorIncrement - currentEvent.startIncrement;
    const postAffichage = currentEvent.endIncrement - currentEvent.anchorIncrement;

    messageText.value = currentEvent.text;
    messageBackgroundColor.value = currentEvent.backgroundColor;
    messageBorderColor.value = currentEvent.borderColor;
    messageBorderWidth.value = currentEvent.borderWidth;
    messageBorderRadius.value = currentEvent.borderRadius;
    
    emit('update:messagePreAffichageSetting', preAffichage);
    emit('update:messagePostAffichageSetting', postAffichage);
  }
  // No 'else' block: if no event is found, the controls retain their current state.
}, { immediate: true });
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
