<template>
  <v-container fluid class="pa-0 fill-height">
    <div id="map-container" ref="mapContainer" class="fill-height"></div>

    <!-- Top UI Container -->
    <div style="position: absolute; top: 10px; left: 10px; right: 16px; z-index: 1000; display: flex; align-items: center; gap: 16px;">
      <v-btn v-if="isBackButtonVisible" icon="mdi-arrow-left" @click="goBack"></v-btn>
      <TrackProgressWidget 
        v-model="trackProgress" 
        :max="trackingPoints.length - 1" 
        :tracking-data="trackingPoints"
        :total-length="totalLineLength"
        :current-distance="currentProgressDistance"
        :key="circuitId" 
        style="flex-grow: 1;"
      />
    </div>

    <CameraInfoWidget
      :bearing="currentBearing"
      :zoom="currentZoom"
      :pitch="currentPitch"
      :defaultZoom="defaultZoom"
      :defaultPitch="defaultPitch"
    />

    <div class="bottom-ui-container">
      <CameraGraph 
        v-if="trackingPoints.length > 0 && showGraph"
        :trackingPoints="trackingPoints"
        :totalLength="totalLineLength"
        :currentDistance="currentProgressDistance"

        :show-bearing-delta="showCalculeeBearingDelta"
        :show-bearing-total-delta="showCalculeeBearingTotalDelta"
        :show-edited-zoom="showEditeeZoom"
        :show-edited-pitch="showEditeePitch"
        :show-edited-bearing-delta="showEditeeBearingDelta"
        :show-edited-bearing-total-delta="showEditeeBearingTotalDelta"
        :currentCameraBearing="currentBearing"
        :initialCameraBearing="trackingPoints[0]?.cap"
        :currentCameraZoom="currentZoom"
        :defaultCameraZoom="defaultZoom"
        :currentCameraPitch="currentPitch"
        :defaultCameraPitch="defaultPitch"
        :pause-events="pauseEventsForDisplay"
        :flyto-events="flytoEventsForDisplay"
        :range-events="eventsFile.rangeEvents"
        @seek-distance="handleSeekDistance"
      />
      <ControlTabsWidget
        ref="controlTabsWidgetRef"
        @mouseenter="isMouseOverControlTabsWidget = true"
        @mouseleave="isMouseOverControlTabsWidget = false"
        v-model:show-calculee-bearing-delta="showCalculeeBearingDelta"
        v-model:show-editee-bearing-delta="showEditeeBearingDelta"
        v-model:show-calculee-bearing-total-delta="showCalculeeBearingTotalDelta"
        v-model:show-editee-bearing-total-delta="showEditeeBearingTotalDelta"
        v-model:show-editee-zoom="showEditeeZoom"
        v-model:show-editee-pitch="showEditeePitch"
        :color-origine-bearing-delta="colorOrigineBearingDelta"
        :color-edited-bearing-delta="colorEditedBearingDelta"
        :color-origine-bearing-total-delta="colorOrigineBearingTotalDelta"
        :color-edited-bearing-total-delta="colorEditedBearingTotalDelta"
        :color-origine-zoom="colorOrigineZoom"
        :color-edited-zoom="colorEditedZoom"
        :color-origine-pitch="colorOriginePitch"
        :color-edited-pitch="colorEditedPitch"
        v-model:cameraSyncMode="cameraSyncMode"
        :graph-bearing-delta-color="graphBearingDeltaColor"
        :graph-bearing-total-delta-color="graphBearingTotalDeltaColor"
        :graph-zoom-color="graphZoomColor"
        :graph-pitch-color="graphPitchColor"
        :current-increment="trackProgress"
        :pause-events="pauseEventsForDisplay"
        :flyto-events="flytoEventsForDisplay"
        v-model:flyto-duration-setting="flytoDurationSetting" 
        :is-current-point-control-point="isCurrentPointControlPoint"
        :message-events="messageEventsForDisplay"
        :full-message-events="eventsFile.rangeEvents"
        :known-message-texts="knownMessageTexts"
        v-model:message-background-color-setting="messageBackgroundColorSetting"
        v-model:message-border-color-setting="messageBorderColorSetting"
        v-model:message-border-width-setting="messageBorderWidthSetting"
        v-model:message-pre-affichage-setting="messagePreAffichageSetting"
        v-model:message-post-affichage-setting="messagePostAffichageSetting"
        v-model:message-border-radius-setting="messageBorderRadiusSetting"
        v-model:zoom-depart="zoomDepart"
        v-model:zoom-depart-valeur="zoomDepartValeur"
        v-model:zoom-depart-distance="zoomDepartDistance"
        v-model:zoom-arrivee="zoomArrivee"
        v-model:zoom-arrivee-valeur="zoomArriveeValeur"
        v-model:distance-zoom-arrivee="distanceZoomArrivee"
        :zoom-depart-is-active="zoomDepartIsActive"
        @add-pause="handleAddPauseEvent"
        @delete-pause="handleDeletePauseEvent"
        @add-flyto="handleAddFlytoEvent"
        @delete-flyto="handleDeleteFlytoEvent"
        @add-message="handleAddMessageEvent"
        @delete-message="handleDeleteMessageEvent"
        @save-control-point="saveControlPoint"
        @delete-control-point="deleteControlPoint"
        @update:marker-visible="showCenterMarker = $event"
        @tab-changed="handleTabChange"
      />
    </div>

    <CenterMarker v-if="showCenterMarker" />
    <ConfirmationDialog
      v-model="showConfirmationDialog"
      :title="confirmationProps.title"
      :message="confirmationProps.message"
      @confirm="resolveConfirmation(true)"
      @cancel="resolveConfirmation(false)"
    />
  </v-container>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'; // Import computed
import { useRoute, useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';
import * as turf from '@turf/turf';
import CameraInfoWidget from '@/components/CameraInfoWidget.vue';
import CenterMarker from '@/components/CenterMarker.vue';
import TrackProgressWidget from '@/components/TrackProgressWidget.vue';
import ControlTabsWidget from '@/components/ControlTabsWidget.vue';
import CameraGraph from '@/components/CameraGraph.vue';
import ConfirmationDialog from '@/components/ConfirmationDialog.vue';

import { useSharedUiState } from '@/composables/useSharedUiState';

const route = useRoute();
const router = useRouter();
const { showSnackbar } = useSnackbar();
const { getSettingValue } = useSettings();
const { isBackButtonVisible } = useSharedUiState();

// Confirmation Dialog
const showConfirmationDialog = ref(false);
const confirmationProps = ref({});
const resolveConfirmation = ref(null);
const dataLoaded = ref(false);

// Zoom Depart Refs
const zoomDepart = ref(true);
const zoomDepartValeur = ref(18);
const zoomDepartDistance = ref(20);
const zoomDepartIsActive = ref(false);

// Zoom Arrivee Refs
const zoomArrivee = ref(true);
const zoomArriveeValeur = ref(18);
const distanceZoomArrivee = ref(20);
const zoomArriveeIsActive = ref(false);

// Commandes Clavier
const incrementAvancement = ref(1);
const incrementAvancementShift = ref(10);
const incrementPitch = ref(1);
const incrementPitchShift = ref(5);

// Touches configurables
const toucheAvancementAvant = ref('ArrowRight');
const toucheAvancementArriere = ref('ArrowLeft');
const touchePitchHaut = ref('ArrowUp');
const touchePitchBas = ref('ArrowDown');

// Commandes Souris
const incrementZoom = ref(0.1);
const incrementZoomShift = ref(1.0);
const incrementBearing = ref(1); // New: Bearing increment for mouse wheel
const incrementBearingShift = ref(5); // New: Bearing shift increment for mouse wheel

// New: Refs for ControlTabsWidget interaction
const controlTabsWidgetRef = ref(null);
const isMouseOverControlTabsWidget = ref(false);
const activeControlTab = ref('camera'); // Assumed default tab name

watch(zoomDepart, async (newValue) => {
  if (!dataLoaded.value) return;
  if (newValue) {
    await applyZoomDepart();
  } else {
    await removeZoomDepart();
  }
  await updateCircuitZoomSettings();
});

const updateCameraInfo = () => {
  if (!map) return;
  currentZoom.value = parseFloat(map.getZoom().toFixed(1));
  currentPitch.value = Math.round(map.getPitch());
  currentBearing.value = Math.round(map.getBearing());
};

const handleTabChange = (newTab) => {
  activeControlTab.value = newTab; // New: Update the active tab state
  if (!map) return;
  if (newTab === 'camera') {
    map.dragPan.disable();
    showSnackbar('Déplacement de la carte désactivé.', 'info');
  } else {
    map.dragPan.enable();
    showSnackbar('Déplacement de la carte activé.', 'info');
  }
};

const handleAddPauseEvent = async (override = false) => {
    try {
        const updatedEventsFile = await invoke('add_pause_event', {
            circuitId: circuitId,
            increment: trackProgress.value,
            overrideExisting: override,
        });
        eventsFile.value = updatedEventsFile;
        showSnackbar('Pause ajoutée avec succès', 'success');
    } catch (error) {
        console.error("Failed to add pause event:", error);
        if (error.includes("Cannot add Pause event: A Flyto event already exists")) {
            const confirmed = await askForConfirmation(
                'Conflit d\'événement',
                'Un événement Flyto existe déjà à cet incrément. Voulez-vous le remplacer par une Pause ?'
            );
            if (confirmed) {
                await handleAddPauseEvent(true); // Retry with override
            } else {
                showSnackbar('Ajout de la Pause annulé.', 'info');
            }
        } else {
            showSnackbar(`Erreur lors de l\'ajout de la la pause: ${error}`, 'error');
        }
    }
};

const handleDeletePauseEvent = async () => {
    try {
        const updatedEventsFile = await invoke('delete_pause_event', {
            circuitId: circuitId,
            increment: trackProgress.value,
        });
        eventsFile.value = updatedEventsFile;
        showSnackbar('Pause supprimée avec succès', 'success');
    } catch (error) {
        console.error("Failed to delete pause event:", error);
        showSnackbar(`Erreur lors de la suppression de la pause: ${error}`, 'error');
    }
};

const handleAddFlytoEvent = async (duration, override = false) => {
  if (!map) return;
  try {
    const cameraPos = map.getFreeCameraOptions().position;
    const lngLat = cameraPos.toLngLat();

    const flytoContent = {
      cap: Math.round((map.getBearing() % 360 + 360) % 360), // Normalize bearing to 0-360
      coord: [
        parseFloat(lngLat.lng.toFixed(5)),
        parseFloat(lngLat.lat.toFixed(5))
      ],
      duree: duration, // Use duration from ControlTabsWidget
      pitch: Math.round(map.getPitch()),
      zoom: Math.round(map.getZoom()),
    };

    const updatedEventsFile = await invoke('add_flyto_event', {
      circuitId: circuitId,
      increment: trackProgress.value,
      flytoContent: flytoContent,
      overrideExisting: override,
    });
    eventsFile.value = updatedEventsFile;
    showSnackbar('Événement Survol ajouté avec succès', 'success');
  } catch (error) {
    console.error("Failed to add flyto event:", error);
    if (error.includes("Cannot add Flyto event: A Pause event already exists")) {
        const confirmed = await askForConfirmation(
            'Conflit d\'événement',
            'Un événement Pause existe déjà à cet incrément. Voulez-vous le remplacer par un Flyto ?'
        );
        if (confirmed) {
            await handleAddFlytoEvent(duration, true); // Retry with override
        } else {
            showSnackbar('Ajout du Survol annulé.', 'info');
        }
    } else {
        showSnackbar(`Erreur lors de l\'ajout de l\'événement Survol: ${error}`, 'error');
    }
  }
};

const handleDeleteFlytoEvent = async () => {
  try {
    const updatedEventsFile = await invoke('delete_flyto_event', {
      circuitId: circuitId,
      increment: trackProgress.value,
    });
    eventsFile.value = updatedEventsFile;
    showSnackbar('Événement Survol supprimé avec succès', 'success');
  } catch (error) {
    console.error("Failed to delete flyto event:", error);
    showSnackbar(`Erreur lors de la suppression de l\'événement Survol: ${error}`, 'error');
  }
};

const askForConfirmation = (title, message) => {
  confirmationProps.value = { title, message };
  showConfirmationDialog.value = true;
  return new Promise((resolve) => {
    resolveConfirmation.value = (decision) => {
      showConfirmationDialog.value = false;
      resolve(decision);
    };
  });
};

const circuitId = route.params.circuitId;
const mapContainer = ref(null);
let map = null;
const currentPointIndex = ref(0);
const trackingPoints = ref([]);
const lineStringCoordinates = ref([]);
const totalLineLength = ref(0);
const progressPercentage = ref(0);
const currentProgressDistance = ref(0);
const cameraSyncMode = ref('original'); // 'off', 'original', 'edited'
const showCenterMarker = ref(false);
const trackProgress = ref(0);
const eventsFile = ref({ pointEvents: {}, rangeEvents: [] }); // Nouvelle structure pour stocker les événements

// Computed property to extract increments with Pause events for display
const pauseEventsForDisplay = computed(() => {
  if (!eventsFile.value.pointEvents) return [];
  const pauses = [];
  for (const increment in eventsFile.value.pointEvents) {
    if (eventsFile.value.pointEvents[increment].some(event => event.type === 'Pause')) {
      pauses.push(parseInt(increment));
    }
  }
  return pauses;
});

// Computed property to extract increments with Flyto events for display
const flytoEventsForDisplay = computed(() => {
  if (!eventsFile.value.pointEvents) return [];
  const flytos = [];
  for (const increment in eventsFile.value.pointEvents) {
    if (eventsFile.value.pointEvents[increment].some(event => event.type === 'Flyto')) {
      flytos.push(parseInt(increment));
    }
  }
  return flytos;
});

// Computed property to extract increments with Message events for display
const messageEventsForDisplay = computed(() => {
  if (!eventsFile.value.rangeEvents) return [];
  // Use a Set to get unique anchor increments
  const messageIncrements = new Set(eventsFile.value.rangeEvents.map(event => event.anchorIncrement));
  return Array.from(messageIncrements);
});

const knownMessageTexts = ref([]);
const messageBackgroundColorSetting = ref('');
const messageBorderColorSetting = ref('');
const messageBorderWidthSetting = ref(0);
const messagePreAffichageSetting = ref(0);
const messagePostAffichageSetting = ref(0);
const messageBorderRadiusSetting = ref(5); // Déclaration de la ref manquante
const handleAddMessageEvent = async (messageData) => {
  // Always add coordinates
  if (map) {
    const center = map.getCenter();
    messageData.coord = [center.lng, center.lat];
  } else {
    showSnackbar('Erreur: La carte n\'est pas initialisée.', 'error');
    return;
  }

  try {
    console.log('Invoking add_message_event with payload:', JSON.stringify(messageData, null, 2));
    const updatedEventsFile = await invoke('add_message_event', {
      circuitId: circuitId,
      increment: trackProgress.value,
      messagePayload: messageData,
    });
    eventsFile.value = updatedEventsFile;
    showSnackbar('Message ajouté avec succès', 'success');
  } catch (error) {
    console.error("Failed to add message event:", error);
    showSnackbar(`Erreur lors de l\'ajout du message: ${error}`, 'error');
  }
};

const handleDeleteMessageEvent = async () => {
  if (!eventsFile.value.rangeEvents) return;

  // Find the first message at the current increment to delete it.
  // This is a limitation of the current UI.
  const eventToDelete = eventsFile.value.rangeEvents.find(
    event => event.anchorIncrement === trackProgress.value
  );

  if (!eventToDelete) {
    showSnackbar('Aucun message à supprimer à ce point.', 'info');
    return;
  }

  try {
    const updatedEventsFile = await invoke('delete_message_event', {
      circuitId: circuitId,
      eventId: eventToDelete.id,
    });
    eventsFile.value = updatedEventsFile;
    showSnackbar('Message supprimé avec succès', 'success');
  } catch (error) {
    console.error("Failed to delete message event:", error);
    showSnackbar(`Erreur lors de la suppression du message: ${error}`, 'error');
  }
};

const flytoDurationSetting = ref(2000); // Default value, will be loaded from settings

// Synchronize the progress bar widget with the internal point index
watch(trackProgress, (newProgress) => {
  if (currentPointIndex.value !== newProgress) {
    currentPointIndex.value = newProgress;
    updateCameraPosition(newProgress);
  }
});
          watch(currentPointIndex, (newIndex) => {  if (trackProgress.value !== newIndex) {
    trackProgress.value = newIndex;
  }
});

const showGraph = ref(true);
const couleurAvancement = ref('');
const traceColor = ref('#FFA726');
const mapboxAvancementColorHex = ref('');

// New granular visibility toggles
const showCalculeeBearingDelta = ref(false);
const showEditeeBearingDelta = ref(false);
const showCalculeeBearingTotalDelta = ref(false);
const showEditeeBearingTotalDelta = ref(false);
const showEditeeZoom = ref(false);
const showEditeePitch = ref(false);

// Graph curve colors
const graphZoomColor = ref('green');
const graphPitchColor = ref('blue');
const graphBearingDeltaColor = ref('amber');
const graphBearingTotalDeltaColor = ref('pink');

const graphEditedZoomColor = ref('light-green-accent-3');
const graphEditedPitchColor = ref('cyan-accent-3');
const graphEditedBearingDeltaColor = ref('yellow-accent-3');
const graphEditedBearingTotalDeltaColor = ref('#000000');

// Refs for checkbox colors to be passed to ControlTabsWidget
const colorOrigineBearingDelta = ref('#000000');
const colorEditedBearingDelta = ref('#000000');
const colorOrigineBearingTotalDelta = ref('#000000');
const colorEditedBearingTotalDelta = ref('#000000');
const colorOrigineZoom = ref('#000000');
const colorEditedZoom = ref('#000000');
const colorOriginePitch = ref('#000000');
const colorEditedPitch = ref('#000000');

const { toHex } = useVuetifyColors(); // New line

const currentZoom = ref(0);
const currentPitch = ref(0);
const currentBearing = ref(0);
const defaultZoom = ref(0);
const defaultPitch = ref(0);

const isCurrentPointControlPoint = computed(() => {
  if (!trackingPoints.value[currentPointIndex.value]) return false;
  return trackingPoints.value[currentPointIndex.value].pointDeControl;
});

const applyZoomDepart = async () => {
  if (!trackingPoints.value || trackingPoints.value.length === 0) return;

  const endIndex = zoomDepartDistance.value;
  if (endIndex >= trackingPoints.value.length) {
    showSnackbar("La distance du zoom de départ est plus grande que la trace.", "error");
    return;
  }

  const point0 = trackingPoints.value[0];
  const pointEnd = trackingPoints.value[endIndex];

  point0.pointDeControl = true;
  point0.editedZoom = zoomDepartValeur.value;
  point0.editedPitch = point0.pitch;
  point0.editedCap = point0.cap;

  pointEnd.pointDeControl = true;
  pointEnd.editedZoom = pointEnd.zoom;
  // Ne pas modifier editedPitch et editedCap pour conserver les modifications utilisateur existantes

  zoomDepartIsActive.value = true;
  updateInterpolation();

  // Après l'interpolation générale, forcer la rampe de zoom linéaire pour la zone de départ.
  // Cela garantit que le zoom est correct même s'il y a des points de contrôle intermédiaires.
  const endIndexForRamp = zoomDepartDistance.value;
  const startZoom = zoomDepartValeur.value;
  const endZoom = pointEnd.zoom; // La rampe se termine sur le zoom original du point final.
  const zoomStep = (endZoom - startZoom) / endIndexForRamp;

  for (let i = 0; i <= endIndexForRamp; i++) {
    trackingPoints.value[i].editedZoom = parseFloat((startZoom + i * zoomStep).toFixed(1));
  }

  try {
    await invoke('save_tracking_file', {
      circuitId: circuitId,
      trackingData: trackingPoints.value,
    });
    showSnackbar('Zoom de départ appliqué.', 'success');
  } catch (error) {
    console.error('Erreur lors de la sauvegarde du zoom de départ:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }
};

const removeZoomDepart = async () => {
  if (!trackingPoints.value || trackingPoints.value.length === 0) return;

  const endIndex = zoomDepartDistance.value;
  if (endIndex >= trackingPoints.value.length) {
    return; // Nothing to do
  }

  // Explicitly reset the editedZoom values in the affected range to their original zoom value
  for (let i = 0; i <= endIndex; i++) {
    if (trackingPoints.value[i]) {
      trackingPoints.value[i].editedZoom = trackingPoints.value[i].zoom;
    }
  }

  const pointEnd = trackingPoints.value[endIndex];

  pointEnd.pointDeControl = false;

  zoomDepartIsActive.value = false;
  updateInterpolation();

  try {
    await invoke('save_tracking_file', {
      circuitId: circuitId,
      trackingData: trackingPoints.value,
    });
    showSnackbar('Zoom de départ supprimé.', 'info');
  } catch (error) {
    console.error('Erreur lors de la suppression du zoom de départ:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }
};

const applyZoomArrivee = async () => {
  if (!trackingPoints.value || trackingPoints.value.length === 0) return;

  const lastIndex = trackingPoints.value.length - 1;
  const startIndex = lastIndex - distanceZoomArrivee.value;

  if (startIndex < 0) {
    showSnackbar("La distance du zoom d'arrivée est plus grande que la trace.", "error");
    return;
  }

  const pointStart = trackingPoints.value[startIndex];
  const pointEnd = trackingPoints.value[lastIndex];

  pointStart.pointDeControl = true;
  pointStart.editedZoom = pointStart.zoom;

  pointEnd.pointDeControl = true;
  pointEnd.editedZoom = zoomArriveeValeur.value;
  pointEnd.editedPitch = pointEnd.pitch;
  pointEnd.editedCap = pointEnd.cap;

  zoomArriveeIsActive.value = true;
  updateInterpolation();

  const startZoom = pointStart.zoom;
  const endZoom = zoomArriveeValeur.value;
  const numSegments = lastIndex - startIndex;
  const zoomStep = (endZoom - startZoom) / numSegments;

  for (let i = 0; i <= numSegments; i++) {
    const currentIndex = startIndex + i;
    trackingPoints.value[currentIndex].editedZoom = parseFloat((startZoom + i * zoomStep).toFixed(1));
  }

  try {
    await invoke('save_tracking_file', {
      circuitId: circuitId,
      trackingData: trackingPoints.value,
    });
    showSnackbar('Zoom d\'arrivée appliqué.', 'success');
  } catch (error) {
    console.error('Erreur lors de la sauvegarde du zoom d\'arrivée:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }
};

const removeZoomArrivee = async () => {
  if (!trackingPoints.value || trackingPoints.value.length === 0) return;

  const lastIndex = trackingPoints.value.length - 1;
  const startIndex = lastIndex - distanceZoomArrivee.value;

  if (startIndex < 0) return;

  for (let i = startIndex; i <= lastIndex; i++) {
    if (trackingPoints.value[i]) {
      trackingPoints.value[i].editedZoom = trackingPoints.value[i].zoom;
    }
  }

  const pointStart = trackingPoints.value[startIndex];
  pointStart.pointDeControl = false;

  zoomArriveeIsActive.value = false;
  updateInterpolation();

  try {
    await invoke('save_tracking_file', {
      circuitId: circuitId,
      trackingData: trackingPoints.value,
    });
    showSnackbar('Zoom d\'arrivée supprimé.', 'info');
  } catch (error) {
    console.error('Erreur lors de la suppression du zoom d\'arrivée:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }
};

watch(zoomArrivee, async (newValue) => {
  if (!dataLoaded.value) return;
  if (newValue) {
    await applyZoomArrivee();
  } else {
    await removeZoomArrivee();
  }
  await updateCircuitZoomSettings();
});

    // Load Zoom Depart settings
    const circuitData = await invoke('get_circuit_data', { circuitId: circuitId });
    zoomDepart.value = circuitData.zoom.depart.enabled;
    zoomDepartValeur.value = circuitData.zoom.depart.valeur;
    zoomDepartDistance.value = circuitData.zoom.depart.distance;

    // Load Zoom Arrivee settings
    zoomArrivee.value = circuitData.zoom.arrivee.enabled;
    zoomArriveeValeur.value = circuitData.zoom.arrivee.valeur;
    distanceZoomArrivee.value = circuitData.zoom.arrivee.distance;

async function updateCircuitZoomSettings() {
  try {
    const zoomSettings = {
      depart: {
        enabled: zoomDepart.value,
        valeur: zoomDepartValeur.value,
        distance: zoomDepartDistance.value,
      },
      arrivee: {
        enabled: zoomArrivee.value,
        valeur: zoomArriveeValeur.value,
        distance: distanceZoomArrivee.value,
      },
    };
    await invoke('update_circuit_zoom_settings', { circuitId: circuitId, zoomSettings: zoomSettings });
    showSnackbar('Paramètres de zoom mis à jour dans circuits.json.', 'success');
  } catch (error) {
    console.error('Erreur lors de la mise à jour des paramètres de zoom dans circuits.json:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }
}

const goBack = () => {
  router.push({ name: 'Main' });
};

const updateInterpolation = () => {
  const points = trackingPoints.value;
  const controlPoints = points.map((p, i) => ({ ...p, originalIndex: i })).filter(p => p.pointDeControl);

  // Reset all nbrSegment to 0 initially
  points.forEach(p => p.nbrSegment = 0);

  if (controlPoints.length === 0) {
    // No control points, reset all edited values to original
    for (let i = 0; i < points.length; i++) {
      points[i].editedZoom = points[i].zoom;
      points[i].editedPitch = points[i].pitch;
      points[i].editedCap = points[i].cap;
    }
  } else {
    // Interpolate between control points
    for (let i = 0; i < controlPoints.length - 1; i++) {
      const startCp = controlPoints[i];
      const endCp = controlPoints[i + 1];
      const startIndex = startCp.originalIndex;
      const endIndex = endCp.originalIndex;
      const numSegments = endIndex - startIndex;
      points[startIndex].nbrSegment = numSegments;

      if (numSegments > 0) {
        const zoomStep = (endCp.editedZoom - startCp.editedZoom) / numSegments;
        const pitchStep = (endCp.editedPitch - startCp.editedPitch) / numSegments;

        let bearingDiff = endCp.editedCap - startCp.editedCap;
        if (bearingDiff > 180) bearingDiff -= 360;
        if (bearingDiff < -180) bearingDiff += 360;
        const bearingStep = bearingDiff / numSegments;

        for (let j = 1; j < numSegments; j++) {
          const currentIndex = startIndex + j;
          points[currentIndex].editedZoom = parseFloat((startCp.editedZoom + j * zoomStep).toFixed(1));
          points[currentIndex].editedPitch = Math.round(startCp.editedPitch + j * pitchStep);
          let newBearing = startCp.editedCap + j * bearingStep;
          points[currentIndex].editedCap = Math.round((newBearing + 360) % 360);
        }
      }
    }

    // Reset points after the last control point
    const lastCpIndex = controlPoints[controlPoints.length - 1].originalIndex;
    for (let i = lastCpIndex + 1; i < points.length; i++) {
      points[i].editedZoom = points[i].zoom;
      points[i].editedPitch = points[i].pitch;
      points[i].editedCap = points[i].cap;
    }
  }

  trackingPoints.value = [...points];
};

const saveControlPoint = async () => {
  if (!map) return;

  const point = trackingPoints.value[currentPointIndex.value];
  if (!point) return;

  if (point.pointDeControl) {
    const confirmed = await askForConfirmation(
      'Confirmer l\'enregistrement',
      'Ce point est déjà un point de contrôle. Voulez-vous vraiment écraser ses valeurs ?'
    );
    if (!confirmed) return;
  }

  point.pointDeControl = true;

  if (zoomDepartIsActive.value && currentPointIndex.value <= zoomDepartDistance.value) {
    // In the departure zone, lock the zoom to the linear ramp
    const endIndex = zoomDepartDistance.value;
    const startZoom = zoomDepartValeur.value;
    const endZoom = trackingPoints.value[endIndex].zoom; // Use the original zoom of the end point
    const zoomStep = (endZoom - startZoom) / endIndex;
    point.editedZoom = parseFloat((startZoom + currentPointIndex.value * zoomStep).toFixed(1));
  } else if (zoomArriveeIsActive.value && currentPointIndex.value >= (trackingPoints.value.length - 1 - distanceZoomArrivee.value)) {
    // In the arrival zone, lock the zoom to the linear ramp
    const lastIndex = trackingPoints.value.length - 1;
    const startIndex = lastIndex - distanceZoomArrivee.value;
    const startZoom = trackingPoints.value[startIndex].zoom;
    const endZoom = zoomArriveeValeur.value;
    const numSegments = lastIndex - startIndex;
    const zoomStep = (endZoom - startZoom) / numSegments;
    point.editedZoom = parseFloat((startZoom + (currentPointIndex.value - startIndex) * zoomStep).toFixed(1));
  } else {
    point.editedZoom = parseFloat(currentZoom.value.toFixed(1));
  }

  point.editedPitch = Math.round(currentPitch.value);
  point.editedCap = Math.round(currentBearing.value);

  const cameraPos = map.getFreeCameraOptions().position;
  const lngLat = cameraPos.toLngLat();
  point.coordonneeCamera = [
    parseFloat(lngLat.lng.toFixed(5)),
    parseFloat(lngLat.lat.toFixed(5))
  ];
  point.altitudeCamera = Math.round(cameraPos.toAltitude());

  updateInterpolation();

  try {
    await invoke('save_tracking_file', {
      circuitId: circuitId,
      trackingData: trackingPoints.value,
    });
    showSnackbar('Point de contrôle enregistré et tracking mis à jour.', 'success');

    // Update trackingKm in circuits.json if this is the furthest control point
    const controlPoints = trackingPoints.value.filter(p => p.pointDeControl);
    if (controlPoints.length > 0) {
      const furthestControlPoint = controlPoints.reduce((max, p) => p.distance > max.distance ? p : max, controlPoints[0]);
      if (point.increment === furthestControlPoint.increment) {
        try {
          await invoke('update_tracking_km', { 
            circuitId: circuitId,
            trackingKm: furthestControlPoint.distance
          });
          showSnackbar('Distance de tracking mise à jour.', 'success');
        } catch (e) {
          console.error('Erreur lors de la mise à jour de trackingKm:', e);
          showSnackbar(`Erreur trackingKm: ${e.message || e}`, 'error');
        }
      }
    }

  } catch (error) {
    console.error('Erreur lors de la sauvegarde du fichier de tracking:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
    // Revert on error?
  }
};

const deleteControlPoint = async () => {
  const point = trackingPoints.value[currentPointIndex.value];
  if (!point || !point.pointDeControl) return;

  if (zoomDepartIsActive.value && (currentPointIndex.value === 0 || currentPointIndex.value === zoomDepartDistance.value)) {
    const confirmed = await askForConfirmation(
      'Suppression du point de contrôle',
      'Ce point fait partie du zoom de départ. Le supprimer désactivera le zoom de départ. Continuer ?'
    );
    if (confirmed) {
      zoomDepart.value = false; // This will trigger the watcher to call removeZoomDepart
    }
    return; 
  }

  const lastIndex = trackingPoints.value.length - 1;
  const startIndex = lastIndex - distanceZoomArrivee.value;
  if (zoomArriveeIsActive.value && (currentPointIndex.value === startIndex || currentPointIndex.value === lastIndex)) {
    const confirmed = await askForConfirmation(
      'Suppression du point de contrôle',
      'Ce point fait partie du zoom d\'arrivée. Le supprimer désactivera le zoom d\'arrivée. Continuer ?'
    );
    if (confirmed) {
      zoomArrivee.value = false; // This will trigger the watcher
    }
    return;
  }

  const confirmed = await askForConfirmation(
    'Confirmer la suppression',
    'Voulez-vous vraiment supprimer ce point de contrôle ? Les valeurs de la caméra pour ce segment seront recalculées.'
  );
  if (!confirmed) return;

  point.pointDeControl = false;
  point.nbrSegment = 0;

  updateInterpolation();

  // Find the new furthest control point to update trackingKm
  const controlPoints = trackingPoints.value.filter(p => p.pointDeControl);
  let newTrackingKm = 0;
  if (controlPoints.length > 0) {
    const furthestControlPoint = controlPoints.reduce((max, p) => p.distance > max.distance ? p : max, controlPoints[0]);
    newTrackingKm = furthestControlPoint.distance;
  }

  try {
    // Save tracking file and update trackingKm in parallel
    await Promise.all([
      invoke('save_tracking_file', {
        circuitId: circuitId,
        trackingData: trackingPoints.value,
      }),
      invoke('update_tracking_km', {
        circuitId: circuitId,
        trackingKm: newTrackingKm
      })
    ]);
    showSnackbar('Point de contrôle supprimé et tracking mis à jour.', 'info');
  } catch (error) {
    console.error('Erreur lors de la suppression du point de contrôle:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }
};

const forceUpdateCamera = () => {
  if (cameraSyncMode.value !== 'off') return; // Only works in manual mode

  const point = trackingPoints.value[currentPointIndex.value];
  if (!point) return;

  // Snap to ORIGINAL values
  currentZoom.value = point.zoom;
  currentPitch.value = point.pitch;
  currentBearing.value = point.cap;

  map.flyTo({
    center: point.coordonnee,
    zoom: currentZoom.value,
    pitch: currentPitch.value,
    bearing: currentBearing.value,
    essential: true,
    duration: 1000
  });
};

const handleSeekDistance = (distanceInKm) => {
  if (!trackingPoints.value || trackingPoints.value.length === 0) return;

  // Find the closest point in trackingPoints to the clicked distance
  const closest = trackingPoints.value.reduce((prev, curr) => {
    return (Math.abs(curr.distance - distanceInKm) < Math.abs(prev.distance - distanceInKm) ? curr : prev);
  });

  // Find the index of that closest point
  const closestIndex = trackingPoints.value.findIndex(p => p.increment === closest.increment);

  if (closestIndex !== -1) {
    currentPointIndex.value = closestIndex;
    updateCameraPosition(closestIndex);
  }
};

const updateCameraPosition = (index) => {
  if (!map || !trackingPoints.value.length) return;

  // Detach listeners to prevent state corruption during animation
  map.off('zoom', updateCameraInfo);
  map.off('pitch', updateCameraInfo);
  map.off('rotate', updateCameraInfo);

  const point = trackingPoints.value[index];

  const flyToOptions = {
    center: point.coordonnee,
    essential: true,
    duration: 150 // A short duration for smoother repeated calls
  };

  // Determine target camera values based on mode and set them for the animation
  if (cameraSyncMode.value === 'original') {
    flyToOptions.zoom = point.zoom;
    flyToOptions.pitch = point.pitch;
    flyToOptions.bearing = point.cap;
  } else if (cameraSyncMode.value === 'edited') {
    flyToOptions.zoom = point.editedZoom;
    flyToOptions.pitch = point.editedPitch;
    flyToOptions.bearing = point.editedCap;
  } else if (cameraSyncMode.value === 'off') {
    // In 'off' mode, maintain the current user-set camera values
    flyToOptions.zoom = currentZoom.value;
    flyToOptions.pitch = currentPitch.value;
    flyToOptions.bearing = currentBearing.value;
  }

  map.once('moveend', () => {
    // After movement, forcefully sync the component state with the intended final values
    // This prevents the state from reflecting intermediate, incorrect animation values.
    if (flyToOptions.zoom !== undefined) {
        currentZoom.value = flyToOptions.zoom;
        currentPitch.value = flyToOptions.pitch;
        currentBearing.value = flyToOptions.bearing;
    } else {
        // Fallback: if options weren't set for some reason, sync from map's final state
        updateCameraInfo();
    }

    // Re-attach listeners for manual user control
    map.on('zoom', updateCameraInfo);
    map.on('pitch', updateCameraInfo);
    map.on('rotate', updateCameraInfo);
  });

  map.flyTo(flyToOptions);

  // This part of the function handles the progress line and can run immediately
  if (totalLineLength.value > 0) {
    currentProgressDistance.value = point.distance;
    progressPercentage.value = (point.distance / totalLineLength.value) * 100;

    const line = turf.lineString(lineStringCoordinates.value);
    let slicedLine;

    if (point.distance === 0) {
      slicedLine = {
        type: 'Feature',
        properties: {},
        geometry: {
          type: 'LineString',
          coordinates: [],
        },
      };
    } else {
      slicedLine = turf.lineSliceAlong(line, 0, point.distance, { units: 'kilometers' });
    }

    if (map.getSource('progress-line')) {
      map.getSource('progress-line').setData(slicedLine);
    }
  } else {
    progressPercentage.value = 0;
    currentProgressDistance.value = 0;
  }
};



const destroyMap = () => {
  if (map) {
    map.remove();
    map = null;
  }
};

const handleWheel = (event) => {
  if (!map) return;

  // Check if the event target is within the map container or the control tabs widget
  const isOverMap = mapContainer.value && mapContainer.value.contains(event.target);
  // For Vue components, we need to access the root DOM element via .$el
  const isOverWidget = controlTabsWidgetRef.value && controlTabsWidgetRef.value.$el && controlTabsWidgetRef.value.$el.contains(event.target);

  if (!isOverMap && !isOverWidget) {
    // The wheel event is not over the map or the widget, do nothing and allow default scroll.
    return;
  }

  event.preventDefault(); // Now, prevent default only for our target areas.

  // If the mouse is over ControlTabsWidget and the active tab is 'camera'
  if (isMouseOverControlTabsWidget.value && activeControlTab.value === 'camera') {
    let bearingStep = incrementBearing.value;
    if (event.shiftKey) {
      bearingStep = incrementBearingShift.value;
    }

    const currentBearing = map.getBearing();
    let newBearing;

    if (event.deltaY < 0) { // Scroll up: increase bearing (rotate right)
      newBearing = currentBearing + bearingStep;
    } else { // Scroll down: decrease bearing (rotate left)
      newBearing = currentBearing - bearingStep;
    }
    map.setBearing(newBearing);
  } else {
    // Existing zoom behavior (when not over the widget or not on the camera tab)
    let zoomStep = incrementZoom.value;
    if (event.shiftKey) {
      zoomStep = incrementZoomShift.value;
    }

    const currentZoom = map.getZoom();
    let newZoom;

    if (event.deltaY < 0) {
      // Zoom in
      newZoom = currentZoom + zoomStep;
    } else {
      // Zoom out
      newZoom = currentZoom - zoomStep;
    }
    map.setZoom(newZoom);
  }
};

const handleContextMenu = (event) => {
  event.preventDefault(); // Prevent default context menu on right-click
};

onUnmounted(() => {
  destroyMap();
  window.removeEventListener('keydown', handleKeydown);
  window.removeEventListener('wheel', handleWheel);
  if (mapContainer.value) {
    mapContainer.value.removeEventListener('contextmenu', handleContextMenu);
  }
});

const handleKeydown = (event) => {
  const targetNodeName = event.target.nodeName;
  if (targetNodeName === 'INPUT' || targetNodeName === 'TEXTAREA' || event.target.isContentEditable) {
    return;
  }



  const pressedKey = event.key.toLowerCase();

  let stepAvancement = incrementAvancement.value;
  if (event.shiftKey) {
    stepAvancement = incrementAvancementShift.value;
  }

  let stepPitch = incrementPitch.value;
  if (event.shiftKey) {
    stepPitch = incrementPitchShift.value;
  }

  let newIndex = currentPointIndex.value;
  let handled = false;

  switch (pressedKey) {
    case toucheAvancementAvant.value.toLowerCase():
      newIndex = Math.min(currentPointIndex.value + stepAvancement, trackingPoints.value.length - 1);
      handled = true;
      break;
    case toucheAvancementArriere.value.toLowerCase():
      newIndex = Math.max(currentPointIndex.value - stepAvancement, 0);
      handled = true;
      break;
    case touchePitchHaut.value.toLowerCase(): {
      if (!map) return;
      const newPitch = Math.min(map.getPitch() + stepPitch, 85);
      map.easeTo({ pitch: newPitch, duration: 50 });
      handled = true;
      break;
    }
    case touchePitchBas.value.toLowerCase(): {
      if (!map) return;
      const newPitch = Math.max(map.getPitch() - stepPitch, 0);
      map.easeTo({ pitch: newPitch, duration: 50 });
      handled = true;
      break;
    }
  }

  if (handled) {
    event.preventDefault();
    if (newIndex !== currentPointIndex.value) {
      currentPointIndex.value = newIndex;
      updateCameraPosition(newIndex);
    }
  }
};

onMounted(async () => {
  if (!circuitId) {
    showSnackbar('ID du circuit manquant pour l\'édition.', 'error');
    router.push({ name: 'Main' });
    return;
  }

  try {
    const mapboxToken = await getSettingValue('Système/Tokens/mapbox');
    if (!mapboxToken) {
      showSnackbar('Token Mapbox non configuré.', 'error');
      router.push({ name: 'Main' });
      return;
    }
    mapboxgl.accessToken = mapboxToken;

    const styleVisualisation = await getSettingValue('Edition/Mapbox/styleVisualisation');
    let rawTraceColor = await getSettingValue('Edition/Mapbox/Trace/couleur');
    if (rawTraceColor && !rawTraceColor.startsWith('#')) {
      traceColor.value = await invoke('convert_vuetify_color', { colorName: rawTraceColor });
    } else {
      traceColor.value = rawTraceColor;
    }
    const traceWidth = await getSettingValue('Edition/Mapbox/Trace/epaisseur');
    const exaggeration = await getSettingValue('Edition/Mapbox/Relief/exaggeration');
    let rawCouleurAvancement = await getSettingValue('Edition/Mapbox/Trace/couleurAvancement');
    // Extract the base color name (e.g., "yellow" from "yellow-darken-4")
    if (rawCouleurAvancement) {
      const parts = rawCouleurAvancement.split('-');
      couleurAvancement.value = parts[0]; // Take the first part as the base color name
      mapboxAvancementColorHex.value = toHex(couleurAvancement.value); // Convert to hex for Mapbox
    } else {
      couleurAvancement.value = 'primary'; // Fallback if not found
      mapboxAvancementColorHex.value = toHex('primary'); // Convert fallback to hex
    }
    const epaisseurAvancement = await getSettingValue('Edition/Mapbox/Trace/epaisseurAvancement');

    // Load graph curve colors
    const rawGraphZoomColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurZoom');
    const rawGraphPitchColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurPitch');
    const rawGraphBearingDeltaColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurBearingDelta');
    const rawGraphBearingTotalDeltaColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurBearingTotalDelta');
    const rawGraphEditedZoomColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedZoom');
    const rawGraphEditedPitchColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedPitch');
    const rawGraphEditedBearingDeltaColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedBearingDelta');
    const rawGraphEditedBearingTotalDeltaColor = await getSettingValue('Edition/Graphe/Couleur courbes/couleurEditedBearingTotalDelta');

    graphZoomColor.value = toHex(rawGraphZoomColor);
    graphPitchColor.value = toHex(rawGraphPitchColor);
    graphBearingDeltaColor.value = toHex(rawGraphBearingDeltaColor);
    graphBearingTotalDeltaColor.value = toHex(rawGraphBearingTotalDeltaColor);
    graphEditedZoomColor.value = toHex(rawGraphEditedZoomColor);
    graphEditedPitchColor.value = toHex(rawGraphEditedPitchColor);
    graphEditedBearingDeltaColor.value = toHex(rawGraphEditedBearingDeltaColor);
    graphEditedBearingTotalDeltaColor.value = toHex(rawGraphEditedBearingTotalDeltaColor);

    // Load graph visibility settings
    showCalculeeBearingDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingDeltaCalcule');
    showEditeeBearingDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingDeltaEdite');
    showCalculeeBearingTotalDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingTotalDeltaCalcule');
    showEditeeBearingTotalDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingTotalDeltaEdite');
    showEditeeZoom.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherZoomEdite');
    showEditeePitch.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherPitchEdite');

    // Load colors for checkboxes
    colorOrigineBearingDelta.value = toHex(rawGraphBearingDeltaColor);
    colorEditedBearingDelta.value = toHex(rawGraphEditedBearingDeltaColor);
    colorOrigineBearingTotalDelta.value = toHex(rawGraphBearingTotalDeltaColor);
    colorEditedBearingTotalDelta.value = toHex(rawGraphEditedBearingTotalDeltaColor);
    colorOrigineZoom.value = toHex(rawGraphZoomColor);
    colorEditedZoom.value = toHex(rawGraphEditedZoomColor);
    colorOriginePitch.value = toHex(rawGraphPitchColor);
    colorEditedPitch.value = toHex(rawGraphEditedPitchColor);



    const rawTrackingData = await invoke('read_tracking_file', { circuitId: circuitId });
    const rawLineStringData = await invoke('read_line_string_file', { circuitId: circuitId });
    lineStringCoordinates.value = rawLineStringData.coordinates;

    const line = turf.lineString(lineStringCoordinates.value);
    totalLineLength.value = turf.length(line, { units: 'kilometers' });

    if (!rawTrackingData || rawTrackingData.length === 0) {
      showSnackbar('Données de tracking introuvables ou vides.', 'error');
      router.push({ name: 'Main' });
      return;
    }

    // Process tracking data to add cumulative distance (performant method)
    const segmentLengthKm = 0.1; // Each tracking point represents a 100m segment
    const processedTrackingPoints = rawTrackingData.map((point, index) => ({
      ...point,
      distance: parseFloat((index * segmentLengthKm).toFixed(2)),
      // Initialize edited values only if they don't already exist, for persistence
      editedZoom: typeof point.editedZoom === 'number' ? point.editedZoom : point.zoom,
      editedPitch: typeof point.editedPitch === 'number' ? point.editedPitch : point.pitch,
      editedCap: typeof point.editedCap === 'number' ? point.editedCap : point.cap,
    }));
    trackingPoints.value = processedTrackingPoints;

    const events = await invoke('get_events', { circuitId: circuitId });
    eventsFile.value = events;

    // Load Flyto duration setting
    flytoDurationSetting.value = await getSettingValue('Edition/Evenements/Flyto/duree');

    // Load Message settings
    knownMessageTexts.value = await invoke('get_known_message_texts', { circuitId: circuitId });
    messageBackgroundColorSetting.value = await getSettingValue('Edition/Evenements/Message/couleurFond');
    messageBorderColorSetting.value = await getSettingValue('Edition/Evenements/Message/couleurBordure');
    messageBorderWidthSetting.value = await getSettingValue('Edition/Evenements/Message/tailleBordure');
    messagePreAffichageSetting.value = await getSettingValue('Edition/Evenements/Message/preAffichage');
    messagePostAffichageSetting.value = await getSettingValue('Edition/Evenements/Message/postAffichage');
    messageBorderRadiusSetting.value = await getSettingValue('Edition/Evenements/Message/rayonBordure');

    // Load Zoom Depart settings
    const circuitData = await invoke('get_circuit_data', { circuitId: circuitId });
    zoomDepart.value = circuitData.zoom.depart.enabled;
    zoomDepartValeur.value = circuitData.zoom.depart.valeur;
    zoomDepartDistance.value = circuitData.zoom.depart.distance;

    // Load Zoom Arrivee settings
    zoomArrivee.value = circuitData.zoom.arrivee.enabled;
    zoomArriveeValeur.value = circuitData.zoom.arrivee.valeur;
    distanceZoomArrivee.value = circuitData.zoom.arrivee.distance;

    // Load Commandes Clavier settings
    incrementAvancement.value = await getSettingValue('Edition/Commandes clavier/incrementAvancement');
    incrementAvancementShift.value = await getSettingValue('Edition/Commandes clavier/incrementAvancementShift');
    incrementPitch.value = await getSettingValue('Edition/Commandes clavier/incrementPitch');
    incrementPitchShift.value = await getSettingValue('Edition/Commandes clavier/incrementPitchShift');

    // Load Touches configurables
    toucheAvancementAvant.value = await getSettingValue('Edition/Commandes clavier/toucheAvancementAvant');
    toucheAvancementArriere.value = await getSettingValue('Edition/Commandes clavier/toucheAvancementArriere');
    touchePitchHaut.value = await getSettingValue('Edition/Commandes clavier/touchePitchHaut');
    touchePitchBas.value = await getSettingValue('Edition/Commandes clavier/touchePitchBas');

    // Load Commandes Souris settings
    incrementZoom.value = await getSettingValue('Edition/Commandes souris/incrementZoom');
    incrementZoomShift.value = await getSettingValue('Edition/Commandes souris/incrementZoomShift');
    incrementBearing.value = await getSettingValue('Edition/Commandes souris/incrementBearing') || 1; // Default to 1 if not found
    incrementBearingShift.value = await getSettingValue('Edition/Commandes souris/incrementBearingShift') || 5; // Default to 5 if not found

    // Initial interpolation
    updateInterpolation();

    currentPointIndex.value = 0;

    // Set data loaded flag to true
    dataLoaded.value = true;

    // Apply initial zoom depart state
    if (zoomDepart.value) {
      await applyZoomDepart();
    } else {
      zoomDepartIsActive.value = false;
    }

    // Apply initial zoom arrivee state
    if (zoomArrivee.value) {
      await applyZoomArrivee();
    } else {
      zoomArriveeIsActive.value = false;
    }

    const firstPoint = trackingPoints.value[currentPointIndex.value];
    const initialCenter = firstPoint.coordonnee;
    const initialZoom = firstPoint.zoom;
    const initialPitch = firstPoint.pitch;
    const initialBearing = firstPoint.cap;

    defaultZoom.value = initialZoom;
    defaultPitch.value = initialPitch;
    currentZoom.value = initialZoom;
    currentPitch.value = initialPitch;
    currentBearing.value = initialBearing;

    map = new mapboxgl.Map({
      container: mapContainer.value,
      style: styleVisualisation,
      center: initialCenter,
      zoom: initialZoom,
      pitch: initialPitch,
      bearing: initialBearing,
      antialias: true,
      scrollZoom: false, // Disable default scroll zoom
      dragRotate: true, // Enable default drag rotate
      dragPan: true, // Enable default drag pan
      pitchWithRotate: false,
      keyboard: false, // Disable default keyboard navigation
    });

    // Add custom event listeners
    window.addEventListener('wheel', handleWheel, { passive: false });
    mapContainer.value.addEventListener('contextmenu', handleContextMenu);

    map.on('load', () => {
      // Initial state for dragPan based on default tab (camera)
      map.dragPan.disable();

      // Listen for map movements to update the info widget
      map.on('zoom', updateCameraInfo);
      map.on('pitch', updateCameraInfo);
      map.on('rotate', updateCameraInfo);

      map.addSource('mapbox-dem', {
        'type': 'raster-dem',
        'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
        'tileSize': 512
      });
      map.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': exaggeration });

      map.addSource('circuit-line', {
        type: 'geojson',
        data: {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'LineString',
            coordinates: lineStringCoordinates.value,
          },
        },
      });

      map.addLayer({
        id: 'circuit-line',
        type: 'line',
        source: 'circuit-line',
        layout: {
          'line-join': 'round',
          'line-cap': 'round',
        },
                  paint: {
                    'line-color': traceColor.value, // Explicitly use .value
                    'line-width': traceWidth,
                  },      });

      map.addSource('progress-line', {
        type: 'geojson',
        data: {
          type: 'Feature',
          properties: {},
          geometry: {
            type: 'LineString',
            coordinates: [],
          },
        },
      });

      map.addLayer({
        id: 'progress-line',
        type: 'line',
        source: 'progress-line',
        layout: {
          'line-join': 'round',
          'line-cap': 'round',
        },
        paint: {
          'line-color': mapboxAvancementColorHex.value, // Use hex color for Mapbox
          'line-width': epaisseurAvancement,
        },
      });
    });

    map.on('error', (e) => {
      console.error('Mapbox error:', e.error);
      showSnackbar(`Erreur Mapbox: ${e.error.message}`, 'error');
    });

  } catch (error) {
    console.error('Erreur lors de l\'initialisation de la vue d\'édition:', error);
    showSnackbar(`Erreur: ${error.message || error}`, 'error');
  }

  window.addEventListener('keydown', handleKeydown);
});

</script>

<style scoped>
.bottom-ui-container {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  display: flex;
  align-items: flex-end; /* Aligns children (graph, widget) to the bottom */
  gap: 8px; /* Add space between children */
  z-index: 1000;
  pointer-events: none; /* Allows map interaction through the container */
}

.bottom-ui-container > * {
  pointer-events: auto; /* Re-enables pointer events for children */
}

#map-container {
  width: 100%;
  height: 100%;
}
</style>
