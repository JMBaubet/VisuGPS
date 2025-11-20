<template>
  <v-sheet rounded="lg" class="control-tabs-widget">
    <v-tabs v-model="mainTab" bg-color="surface" density="compact" color="orange-darken-1">
      <v-tab value="camera">Caméra</v-tab>
      <v-tab value="stop">Pause/FlyTo</v-tab>
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

          <!-- Checkbox Table in Expansion Panel -->
          <div class="pa-2">
            <v-expansion-panels>
              <v-expansion-panel>
                <v-expansion-panel-title collapse-icon="mdi-menu-down" expand-icon="mdi-menu-down" class="py-1" style="min-height: 40px;">
                  <span class="text-subtitle-2">Affichage des courbes</span>
                </v-expansion-panel-title>
                <v-expansion-panel-text>
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
                </v-expansion-panel-text>
              </v-expansion-panel>
            </v-expansion-panels>
          </div>

          <v-divider></v-divider>

          <!-- Zoom Tabs -->
          <div class="pa-2">
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
            <v-btn color="info" variant="text" @click="emit('verify-flyto')">
              <span class="mr-2">Vérifier</span>
              <v-icon icon="mdi-play"></v-icon>
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
        <div class="pa-2 d-flex flex-column fill-height">
          <v-btn block color="primary" variant="tonal" @click="emit('open-message-library')">
            Sélectionner un message
          </v-btn>

          <v-divider class="my-1"></v-divider>

          <div v-if="messageToDisplay" class="text-center">
            <p class="text-caption">Message sélectionné :</p>
            <div
              :style="{
                backgroundColor: toHex(messageToDisplay.style.backgroundColor),
                color: getContrastColor(messageToDisplay.style.backgroundColor),
                padding: '4px 8px',
                borderRadius: '4px',
                display: 'inline-block',
                margin: '8px'
              }"
            >
              {{ messageToDisplay.text }}
            </div>
          </div>
          <div v-else class="text-center text-disabled">
            Aucun message sélectionné
          </div>

          <div class="my-4"></div>

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

          <v-row dense class="justify-center align-center">
            <v-col cols="auto" class="text-right pr-2">
              <label class="text-caption">Gauche </label>
            </v-col>
            <v-col cols="auto">
              <v-switch
                v-model="messageOrientationModel"
                class="primary-big-switch"
                color="primary"
                hide-details
                inset
              ></v-switch>
            </v-col>
            <v-col cols="auto" class="text-left pl-2">
              <label class="text-caption"> Droite</label>
            </v-col>
          </v-row>

          <v-spacer></v-spacer>

          <v-row dense>
            <v-col cols="12" class="d-flex justify-space-around align-center">
              <v-btn v-if="isMessageEvent" color="error" variant="text" @click="onDeleteMessage">
                <span class="mr-2">Supprimer</span>
                <v-icon icon="mdi-delete"></v-icon>
              </v-btn>
              <v-btn :disabled="!messageToDisplay" color="primary" variant="text" @click="onAddMessage">
                <span class="mr-2">{{ isMessageEvent ? 'Mettre à jour' : 'Ajouter Message' }}</span>
                <v-icon>{{ isMessageEvent ? 'mdi-check' : 'mdi-plus' }}</v-icon>
              </v-btn>
            </v-col>
          </v-row>
        </div>
      </v-window-item>

    </v-window>
  </v-sheet>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import CameraSyncModeSelector from './CameraSyncModeSelector.vue';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const { toHex, getContrastColor } = useVuetifyColors();

const mainTab = ref('camera');
const zoomTab = ref('depart');

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
  pauseEvents: Array,
  flytoEvents: Array,
  flytoDurationSetting: Number,
  // Message props
  messageEvents: Array,
  currentMessageEvent: Object, // An existing event at the current increment
  selectedMessage: Object, // A new message selected from the library
  messagePreAffichageSetting: Number,
  messagePostAffichageSetting: Number,
  messageOrientation: String, // New prop: 'Gauche' or 'Droite'
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
  'add-pause',
  'add-flyto',
  'delete-flyto',
  'update:flytoDurationSetting',
  // Message emits
  'add-message',
  'delete-message',
  'open-message-library',
  'update:messagePreAffichageSetting',
  'update:messagePostAffichageSetting',
  'update:messageOrientation', // New emit
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
  'verify-flyto',
]);
  
watch(mainTab, (newTab) => {
  emit('tab-changed', newTab);
});
  
// --- Models for Graph Tab ---
const createModel = (propName) => computed({
  get: () => props[propName],
  set: (value) => emit(`update:${propName}`, value),
});

const createSwitchModel = (propName, trueValue, falseValue) => computed({
  get: () => props[propName] === trueValue,
  set: (value) => emit(`update:${propName}`, value ? trueValue : falseValue),
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
const messageOrientationModel = createSwitchModel('messageOrientation', 'Droite', 'Gauche');
const messagePreAffichageMin = ref(-50);
const messagePostAffichageMax = ref(100);
const messagePreAffichageStep = ref(1);

const messageToDisplay = computed(() => 
  props.currentMessageEvent?.message || props.selectedMessage
);

const messageDisplayRange = computed({
  get: () => {
    const pre = props.messagePreAffichageSetting ?? 0;
    const post = props.messagePostAffichageSetting ?? 0;
    return [-pre, post];
  },
  set: (value) => {
    let newPreAffichage = -value[0];
    let newPostAffichage = value[1];

    newPreAffichage = Math.max(0, Math.min(50, newPreAffichage));
    newPostAffichage = Math.max(1, Math.min(100, newPostAffichage));

    if (props.messagePreAffichageSetting !== newPreAffichage) {
      emit('update:messagePreAffichageSetting', newPreAffichage);
    }
    if (props.messagePostAffichageSetting !== newPostAffichage) {
      emit('update:messagePostAffichageSetting', newPostAffichage);
    }
  },
});

const isMessageEvent = computed(() => {
  return !!props.currentMessageEvent;
});

const onAddMessage = () => {
  if (!messageToDisplay.value) {
    return;
  }
  const messageData = {
    messageId: messageToDisplay.value.id,
    preAffichage: props.messagePreAffichageSetting,
    postAffichage: props.messagePostAffichageSetting,
    orientation: props.messageOrientation,
  };
  emit('add-message', messageData);
};

const onDeleteMessage = () => {
  emit('delete-message');
};

watch(() => props.currentMessageEvent, (event) => {
  if (event) {
    const pre = event.anchorIncrement - event.startIncrement;
    const post = event.endIncrement - event.anchorIncrement;
    emit('update:messagePreAffichageSetting', pre);
    emit('update:messagePostAffichageSetting', post);
    emit('update:messageOrientation', event.orientation || 'Droite');
  }
}, { immediate: true });


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

/* --- SWITCH PRIMARY AVEC THUMB GROS EN PERMANENCE --- */

/* Track toujours Primary */
:deep(.primary-big-switch .v-switch__track) {
  background-color: rgb(var(--v-theme-primary)) !important;
  opacity: 1 !important;
}

/* Thumb toujours gros */
:deep(.primary-big-switch .v-switch__thumb) {
  background-color: white !important;
  box-shadow: none !important;
  opacity: 1 !important;

  /* Taille du thumb -> tu peux modifier 1.3 pour plus ou moins gros */
  transform: scale(1) !important;
}

/* Neutralise l'effet actif de Vuetify (ON) */
:deep(.primary-big-switch.v-input--dirty .v-switch__track) {
  background-color: rgb(var(--v-theme-primary)) !important;
}

:deep(.primary-big-switch.v-input--dirty .v-switch__thumb) {
  background-color: white !important;
  transform: scale(1) !important; /* même taille en ON */
}
</style>
