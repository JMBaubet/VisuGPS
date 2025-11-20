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
        v-if="trackingPoints.length > 0 && showGraph && activeControlTab === 'camera'"
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
        @seek-distance="handleSeekDistance"
      />
      <MessageGraph
        v-else-if="trackingPoints.length > 0 && showGraph && activeControlTab === 'message'"
        :trackingPoints="trackingPoints"
        :totalLength="totalLineLength"
        :currentDistance="currentProgressDistance"
        :range-events="eventsFile.rangeEvents"
        @seek-distance="handleSeekDistance"
        :message-graph-height="messageGraphHeight"
        :message-library="messageLibrary"
      />
      <PauseFlytoGraph
        v-else-if="trackingPoints.length > 0 && showGraph && activeControlTab === 'stop'"
        :trackingPoints="trackingPoints"
        :totalLength="totalLineLength"
        :currentDistance="currentProgressDistance"
        :pointEventsData="eventsFile.pointEvents"
        @seek-distance="handleSeekDistance"
        @verify-flyto="handleVerifyFlyto"
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
        :current-message-event="currentMessageEvent"
        :selected-message="selectedMessageForNewEvent"
        v-model:message-pre-affichage-setting="messagePreAffichageSetting"
        v-model:message-post-affichage-setting="messagePostAffichageSetting"
        v-model:message-orientation="messageOrientation"
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
        @open-message-library="handleOpenMessageLibrary"
        @save-control-point="saveControlPoint"
        @delete-control-point="deleteControlPoint"
        @update:marker-visible="showCenterMarker = $event"
        @tab-changed="handleTabChange"
        @verify-flyto="handleVerifyFlyto"
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
    <MessageLibraryModal 
      v-model="showLibraryModal" 
      @select-message="handleSelectMessage" 
    />
    <ConfirmationDialog
      v-model="showMissingMessageErrorModal"
      :title="'Erreur de Message Manquant'"
      :message="missingMessageErrorDetailsMessage"
      :confirm-text="'Supprimer l\'événement'"
      :cancel-text="'Ignorer'"
      @confirm="handleMissingMessageErrorResolution(true)"
      @cancel="handleMissingMessageErrorResolution(false)"
    />
  </v-container>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
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
import MessageLibraryModal from '@/components/MessageLibraryModal.vue';
import MessageGraph from '@/components/MessageGraph.vue';
import PauseFlytoGraph from '@/components/PauseFlytoGraph.vue';

import { useSharedUiState } from '@/composables/useSharedUiState';
import { useMessageDisplay } from '@/composables/useMessageDisplay.js';

const showMissingMessageErrorModal = ref(false);
const missingMessageErrorDetails = ref({ messageId: '', messageName: '', circuitId: '', increment: 0, eventId: '' });
const resolveMissingMessageError = ref(null);

const missingMessageErrorDetailsMessage = computed(() => {
  const details = missingMessageErrorDetails.value;
  const segmentLengthKm = 0.1; // Assurez-vous que cette valeur est cohérente avec celle utilisée pour le calcul des trackingPoints
  const distanceKm = (details.increment * segmentLengthKm).toFixed(1);
  return `Le message du km ${distanceKm} est introuvable dans la bibliothèque.<br><br>Que voulez-vous faire ?`;
});

const route = useRoute();
const router = useRouter();
const { showSnackbar } = useSnackbar();
const { getSettingValue } = useSettings();
const { isBackButtonVisible } = useSharedUiState();
const { createMessageSVG } = useMessageDisplay(); // Import the SVG creation function

const pitchTransitionDuration = ref(500); // Default value
let previousPitch = 0;
let originalDragPanState = false; // To store whether dragPan was enabled before mousedown

const isCustomPanning = ref(false);
const lastMousePosition = { x: 0, y: 0 };

// --- Message Popups ---
const activePopups = ref(new Map());

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
    const center = map.getCenter(); // Obtient le centre de la carte

    const flytoContent = {
      cap: Math.round((map.getBearing() % 360 + 360) % 360), // Normalize bearing to 0-360
      coord: [
        parseFloat(center.lng.toFixed(5)),
        parseFloat(center.lat.toFixed(5))
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
    showSnackbar(`Erreur lors de la suppression de l'événement Survol: ${error}`, 'error');
  }
};

const handleVerifyFlyto = (eventData) => {
  if (!map) return;

  let flytoData = null;

  if (eventData) {
    // Event data passed directly (e.g. from graph click)
    flytoData = eventData.data;
  } else {
    // No event data passed (e.g. from "Verify" button), find event at current increment
    const currentEvents = eventsFile.value.pointEvents[trackProgress.value];
    if (currentEvents) {
      const flytoEvent = currentEvents.find(e => e.type === 'Flyto');
      if (flytoEvent) {
        flytoData = flytoEvent.data;
      }
    }
  }

  if (flytoData) {
    map.flyTo({
      center: flytoData.coord,
      zoom: flytoData.zoom,
      pitch: flytoData.pitch,
      bearing: flytoData.cap,
      duration: flytoData.duree || 2000,
      essential: true
    });
    showSnackbar('Vérification du Survol...', 'info');
  } else {
    showSnackbar('Aucun événement Survol trouvé à vérifier.', 'warning');
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

const handleMissingMessageErrorResolution = async (confirmDelete) => {
  if (confirmDelete) {
    try {
        const updatedEventsFile = await invoke('delete_message_event', {
            circuitId: circuitId,
            eventId: missingMessageErrorDetails.value.eventId,
        });
        eventsFile.value = updatedEventsFile;
        showSnackbar('Événement de message manquant supprimé avec succès.', 'success');

        // Supprimer également l'entrée correspondante dans errors.json
        await invoke('delete_error_entry', {
            circuitId: circuitId,
            eventId: missingMessageErrorDetails.value.eventId,
        });
    } catch (error) {
        console.error("Failed to delete missing message event:", error);
        showSnackbar(`Erreur lors de la suppression de l'événement de message manquant: ${error}`, 'error');
    }
  } else {
    showSnackbar('Erreur de message manquant ignorée. L\'événement restera dans la trace mais ne sera pas affiché.', 'warning');
  }
  if (resolveMissingMessageError.value) {
    resolveMissingMessageError.value(); // Resolve the promise that was blocking onMounted
  }
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
const eventsFile = ref({ pointEvents: {}, rangeEvents: [] });
const messageLibrary = ref([]); // New ref for message library

// --- New Message Library Logic ---
const showLibraryModal = ref(false);
const selectedMessageForNewEvent = ref(null);
const defaultMessagePreAffichage = ref(0);
const defaultMessagePostAffichage = ref(0);
const messageGraphHeight = ref(10);
const messageOrientation = ref('Droite'); // New state for orientation

const currentMessageEvent = computed(() => 
  eventsFile.value.rangeEvents?.find(e => e.anchorIncrement === trackProgress.value) || null
);

watch(currentMessageEvent, (event) => {
  if (event) {
    messageOrientation.value = event.orientation || 'Droite';
  }
}, { immediate: true });

watch(trackProgress, () => {
  selectedMessageForNewEvent.value = null;
  // Reset orientation to default when moving to a new increment without a message
  if (!currentMessageEvent.value) {
    messageOrientation.value = 'Droite';
  }
});

const handleOpenMessageLibrary = () => {
  showLibraryModal.value = true;
};

const handleSelectMessage = async (messageId) => {
  try {
    const library = await invoke('get_message_library');
    const selected = library.find(m => m.id === messageId);
    if (selected) {
      selectedMessageForNewEvent.value = selected;
      // Reset sliders to default values for the new selection
      messagePreAffichageSetting.value = defaultMessagePreAffichage.value;
      messagePostAffichageSetting.value = defaultMessagePostAffichage.value;
    }
  } catch (error) {
    console.error("Failed to fetch message from library:", error);
    showSnackbar(`Erreur: ${error}`, 'error');
  }
  showLibraryModal.value = false;
};
// --- End New Message Library Logic ---

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

const messageEventsForDisplay = computed(() => {
  if (!eventsFile.value.rangeEvents) return [];
  const messageIncrements = new Set(eventsFile.value.rangeEvents.map(event => event.anchorIncrement));
  return Array.from(messageIncrements);
});

const messagePreAffichageSetting = ref(0);
const messagePostAffichageSetting = ref(0);

const handleAddMessageEvent = async (messageData) => {
  if (!messageData.messageId) {
    showSnackbar('Aucun message sélectionné.', 'error');
    return;
  }
  
  if (map) {
    const isUpdate = !!currentMessageEvent.value;
    const eventIdToDelete = isUpdate ? currentMessageEvent.value.eventId : null;

    // If it's an update, delete the old event first.
    if (isUpdate && eventIdToDelete) {
      try {
        await invoke('delete_message_event', {
          circuitId: circuitId,
          eventId: eventIdToDelete,
        });
      } catch (error) {
        console.error("Failed to delete old message during update:", error);
        showSnackbar(`Erreur lors de la suppression de l'ancien message: ${error}`, 'error');
        return; // Stop if delete fails to prevent data loss.
      }
    }

    const center = map.getCenter();
    const payload = {
      messageId: messageData.messageId,
      preAffichage: messageData.preAffichage,
      postAffichage: messageData.postAffichage,
      coord: [center.lng, center.lat],
      anchorIncrement: trackProgress.value,
      orientation: messageData.orientation, // Pass orientation
    };

    try {
      const updatedEventsFile = await invoke('add_message_event', {
        circuitId: circuitId,
        payload: payload,
      });
      eventsFile.value = updatedEventsFile;
      selectedMessageForNewEvent.value = null; // Clear selection after adding
      showSnackbar(`Message ${isUpdate ? 'mis à jour' : 'ajouté'} avec succès`, 'success');
    } catch (error) {
      console.error("Failed to add message event:", error);
      showSnackbar(`Erreur lors de l'ajout du message: ${error}`, 'error');
      // Note: If this fails after a successful delete, the event is lost.
      // A proper backend transaction would be the ideal solution.
    }
  }
};

const handleDeleteMessageEvent = async () => {
  const eventToDelete = currentMessageEvent.value;

  if (!eventToDelete) {
    showSnackbar('Aucun message à supprimer à ce point.', 'info');
    return;
  }

  try {
    const updatedEventsFile = await invoke('delete_message_event', {
      circuitId: circuitId,
      eventId: eventToDelete.eventId,
    });
    eventsFile.value = updatedEventsFile;
    showSnackbar('Message supprimé avec succès', 'success');
  } catch (error) {
    console.error("Failed to delete message event:", error);
    showSnackbar(`Erreur lors de la suppression du message: ${error}`, 'error');
  }
};

const flytoDurationSetting = ref(2000);

watch(trackProgress, (newProgress) => {
  if (currentPointIndex.value !== newProgress) {
    currentPointIndex.value = newProgress;
    updateCameraPosition(newProgress);
  }
});

watch(currentPointIndex, (newIndex) => {
  if (trackProgress.value !== newIndex) {
    trackProgress.value = newIndex;
  }
});

const showGraph = ref(true);
const couleurAvancement = ref('');
const traceColor = ref('#FFA726');
const mapboxAvancementColorHex = ref('');

const showCalculeeBearingDelta = ref(false);
const showEditeeBearingDelta = ref(false);
const showCalculeeBearingTotalDelta = ref(false);
const showEditeeBearingTotalDelta = ref(false);
const showEditeeZoom = ref(false);
const showEditeePitch = ref(false);

const graphZoomColor = ref('green');
const graphPitchColor = ref('blue');
const graphBearingDeltaColor = ref('amber');
const graphBearingTotalDeltaColor = ref('pink');

const graphEditedZoomColor = ref('light-green-accent-3');
const graphEditedPitchColor = ref('cyan-accent-3');
const graphEditedBearingDeltaColor = ref('yellow-accent-3');
const graphEditedBearingTotalDeltaColor = ref('#000000');

const colorOrigineBearingDelta = ref('#000000');
const colorEditedBearingDelta = ref('#000000');
const colorOrigineBearingTotalDelta = ref('#000000');
const colorEditedBearingTotalDelta = ref('#000000');
const colorOrigineZoom = ref('#000000');
const colorEditedZoom = ref('#000000');
const colorOriginePitch = ref('#000000');
const colorEditedPitch = ref('#000000');

const { toHex } = useVuetifyColors();

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

  zoomDepartIsActive.value = true;
  updateInterpolation();

  const endIndexForRamp = zoomDepartDistance.value;
  const startZoom = zoomDepartValeur.value;
  const endZoom = pointEnd.zoom;
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
    return;
  }

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

  points.forEach(p => p.nbrSegment = 0);

  if (controlPoints.length === 0) {
    for (let i = 0; i < points.length; i++) {
      points[i].editedZoom = points[i].zoom;
      points[i].editedPitch = points[i].pitch;
      points[i].editedCap = points[i].cap;
    }
  } else {
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
    const endIndex = zoomDepartDistance.value;
    const startZoom = zoomDepartValeur.value;
    const endZoom = trackingPoints.value[endIndex].zoom;
    const zoomStep = (endZoom - startZoom) / endIndex;
    point.editedZoom = parseFloat((startZoom + currentPointIndex.value * zoomStep).toFixed(1));
  } else if (zoomArriveeIsActive.value && currentPointIndex.value >= (trackingPoints.value.length - 1 - distanceZoomArrivee.value)) {
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
      zoomDepart.value = false;
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
      zoomArrivee.value = false;
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

  const controlPoints = trackingPoints.value.filter(p => p.pointDeControl);
  let newTrackingKm = 0;
  if (controlPoints.length > 0) {
    const furthestControlPoint = controlPoints.reduce((max, p) => p.distance > max.distance ? p : max, controlPoints[0]);
    newTrackingKm = furthestControlPoint.distance;
  }

  try {
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
  if (cameraSyncMode.value !== 'off') return;

  const point = trackingPoints.value[currentPointIndex.value];
  if (!point) return;

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

  const closest = trackingPoints.value.reduce((prev, curr) => {
    return (Math.abs(curr.distance - distanceInKm) < Math.abs(prev.distance - distanceInKm) ? curr : prev);
  });

  const closestIndex = trackingPoints.value.findIndex(p => p.increment === closest.increment);

  if (closestIndex !== -1) {
    currentPointIndex.value = closestIndex;
    updateCameraPosition(closestIndex);
  }
};

const updateCameraPosition = (index) => {
  if (!map || !trackingPoints.value.length) return;

  map.off('zoom', updateCameraInfo);
  map.off('pitch', updateCameraInfo);
  map.off('rotate', updateCameraInfo);

  const point = trackingPoints.value[index];

  const flyToOptions = {
    center: point.coordonnee,
    essential: true,
    duration: 150
  };

  if (cameraSyncMode.value === 'original') {
    flyToOptions.zoom = point.zoom;
    flyToOptions.pitch = point.pitch;
    flyToOptions.bearing = point.cap;
  } else if (cameraSyncMode.value === 'edited') {
    flyToOptions.zoom = point.editedZoom;
    flyToOptions.pitch = point.editedPitch;
    flyToOptions.bearing = point.editedCap;
  } else if (cameraSyncMode.value === 'off') {
    flyToOptions.zoom = currentZoom.value;
    flyToOptions.pitch = currentPitch.value;
    flyToOptions.bearing = currentBearing.value;
  }

  map.once('moveend', () => {
    if (flyToOptions.zoom !== undefined) {
        currentZoom.value = flyToOptions.zoom;
        currentPitch.value = flyToOptions.pitch;
        currentBearing.value = flyToOptions.bearing;
    } else {
        updateCameraInfo();
    }

    map.on('zoom', updateCameraInfo);
    map.on('pitch', updateCameraInfo);
    map.on('rotate', updateCameraInfo);
  });

  map.flyTo(flyToOptions);

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
    map.off('mousedown');
    map.off('mouseup');
    map.remove();
    map = null;
  }
};

const handleWheel = (event) => {
  if (!map) return;

  const isOverMap = mapContainer.value && mapContainer.value.contains(event.target);
  const isOverWidget = controlTabsWidgetRef.value && controlTabsWidgetRef.value.$el && controlTabsWidgetRef.value.$el.contains(event.target);

  if (!isOverMap && !isOverWidget) {
    return;
  }

  event.preventDefault();

  const delta = Math.max(-1, Math.min(1, (event.wheelDelta || -event.deltaY)));

  if (delta === 0) {
    return;
  }
  const direction = delta > 0 ? 1 : -1;

  const isCameraTab = isMouseOverControlTabsWidget.value && activeControlTab.value === 'camera';

  if (isCameraTab) {
    const step = event.shiftKey ? parseFloat(incrementBearingShift.value) : parseFloat(incrementBearing.value);
    const change = step * direction;
    
    const currentBearing = map.getBearing();
    map.setBearing(currentBearing + change);

  } else {
    const step = event.shiftKey ? parseFloat(incrementZoomShift.value) : parseFloat(incrementZoom.value);
    const change = step * direction;

    const currentZoom = map.getZoom();
    map.setZoom(currentZoom + change);
  }
};

const handleContextMenu = (event) => {
  event.preventDefault();
};

const handleCustomMouseMove = (e) => {
  if (!map || !isCustomPanning.value) return;

  const deltaX = e.clientX - lastMousePosition.x;
  const deltaY = e.clientY - lastMousePosition.y;

  // panBy expects [x, y] where x is horizontal and y is vertical.
  // Positive x moves right, positive y moves down.
  // To pan the map in the direction of mouse movement, we use -deltaX and -deltaY.
  map.panBy([-deltaX, -deltaY], { animate: false });

  lastMousePosition.x = e.clientX;
  lastMousePosition.y = e.clientY;
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

    const colorTraceBySlope = await getSettingValue('Edition/Mapbox/Trace/colorerSelonPente');
    const traceWidth = await getSettingValue('Edition/Mapbox/Trace/epaisseur');
    let paintProps = { 'line-width': traceWidth };
    let useGradient = false;

    if (colorTraceBySlope) {
        try {
            const slopeColors = {
                TrancheNegative: toHex(await getSettingValue('Altitude/Couleurs/TrancheNegative')),
                Tranche1: toHex(await getSettingValue('Altitude/Couleurs/Tranche1')),
                Tranche2: toHex(await getSettingValue('Altitude/Couleurs/Tranche2')),
                Tranche3: toHex(await getSettingValue('Altitude/Couleurs/Tranche3')),
                Tranche4: toHex(await getSettingValue('Altitude/Couleurs/Tranche4')),
                Tranche5: toHex(await getSettingValue('Altitude/Couleurs/Tranche5')),
            };
            const segmentLength = await getSettingValue('Importation/Tracking/LongueurSegment');

            const colorExpression = await invoke('get_slope_color_expression', {
                circuitId: circuitId,
                slopeColors: slopeColors,
                segmentLength: segmentLength,
            });

            if (colorExpression && Array.isArray(colorExpression)) {
                paintProps['line-gradient'] = colorExpression;
                useGradient = true;
            } else {
                showSnackbar("Impossible de générer les couleurs de pente, utilisation de la couleur par défaut.", "warning");
            }
        } catch (e) {
            console.error("Erreur lors de la génération des couleurs de pente:", e);
            showSnackbar("Erreur lors de la génération des couleurs de pente, utilisation de la couleur par défaut.", "error");
        }
    }

    if (!useGradient) {
        let rawTraceColor = await getSettingValue('Edition/Mapbox/Trace/couleur');
        if (rawTraceColor && !rawTraceColor.startsWith('#')) {
            traceColor.value = await invoke('convert_vuetify_color', { colorName: rawTraceColor });
        } else {
            traceColor.value = rawTraceColor || '#FFA726'; // Default to orange if setting is missing
        }
        paintProps['line-color'] = traceColor.value;
    }
    const exaggeration = await getSettingValue('Edition/Mapbox/Relief/exaggeration');
    let rawCouleurAvancement = await getSettingValue('Edition/Mapbox/Trace/couleurAvancement');
    if (rawCouleurAvancement) {
      const parts = rawCouleurAvancement.split('-');
      couleurAvancement.value = parts[0];
      mapboxAvancementColorHex.value = toHex(couleurAvancement.value);
    } else {
      couleurAvancement.value = 'primary';
      mapboxAvancementColorHex.value = toHex('primary');
    }
    const epaisseurAvancement = await getSettingValue('Edition/Mapbox/Trace/epaisseurAvancement');

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

    showCalculeeBearingDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingDeltaCalcule');
    showEditeeBearingDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingDeltaEdite');
    showCalculeeBearingTotalDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingTotalDeltaCalcule');
    showEditeeBearingTotalDelta.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherBearingTotalDeltaEdite');
    showEditeeZoom.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherZoomEdite');
    showEditeePitch.value = await getSettingValue('Edition/Graphe/Affichage courbes/afficherPitchEdite');

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

    const segmentLengthKm = 0.1;
    const processedTrackingPoints = rawTrackingData.map((point, index) => ({
      ...point,
      distance: parseFloat((index * segmentLengthKm).toFixed(2)),
      editedZoom: typeof point.editedZoom === 'number' ? point.editedZoom : point.zoom,
      editedPitch: typeof point.editedPitch === 'number' ? point.editedPitch : point.pitch,
      editedCap: typeof point.editedCap === 'number' ? point.editedCap : point.cap,
    }));
    trackingPoints.value = processedTrackingPoints;

    const events = await invoke('get_events', { circuitId: circuitId });
    eventsFile.value.pointEvents = events.pointEvents; // Correction du camelCase
    eventsFile.value.rangeEvents = events.rangeEvents; // Correction du camelCase

    // Fetch the message library
    messageLibrary.value = await invoke('get_message_library');

    // --- Message Error Handling ---
    if (events.missingMessageErrors.length > 0) { // Accès direct à missingMessageErrors
      for (const errorDetail of events.missingMessageErrors) {
          missingMessageErrorDetails.value = {
              messageId: errorDetail.messageId, // Correction du camelCase
              messageName: 'Inconnu', // Le nom n'est pas disponible pour les messages manquants
              circuitId: errorDetail.circuitId, // Correction du camelCase
              increment: errorDetail.anchorIncrement, // Correction du camelCase
              eventId: errorDetail.eventId, // Correction du camelCase
          };
          showMissingMessageErrorModal.value = true;
          // Block further execution until the user resolves this specific error
          await new Promise(resolve => {
              resolveMissingMessageError.value = resolve;
          });
      }
    }

    flytoDurationSetting.value = await getSettingValue('Edition/Evenements/Flyto/duree');

    // Load Message settings
    defaultMessagePreAffichage.value = await getSettingValue('Edition/Evenements/Message/preAffichage');
    defaultMessagePostAffichage.value = await getSettingValue('Edition/Evenements/Message/postAffichage');
    messageGraphHeight.value = await getSettingValue('Edition/Evenements/Message/hauteurGraphique');
    messagePreAffichageSetting.value = defaultMessagePreAffichage.value;
    messagePostAffichageSetting.value = defaultMessagePostAffichage.value;

    pitchTransitionDuration.value = await getSettingValue('Edition/Evenements/Message/transitionDuree');

    const circuitData = await invoke('get_circuit_data', { circuitId: circuitId });
    zoomDepart.value = circuitData.zoom.depart.enabled;
    zoomDepartValeur.value = circuitData.zoom.depart.valeur;
    zoomDepartDistance.value = circuitData.zoom.depart.distance;

    zoomArrivee.value = circuitData.zoom.arrivee.enabled;
    zoomArriveeValeur.value = circuitData.zoom.arrivee.valeur;
    distanceZoomArrivee.value = circuitData.zoom.arrivee.distance;

    incrementAvancement.value = await getSettingValue('Edition/Commandes clavier/incrementAvancement');
    incrementAvancementShift.value = await getSettingValue('Edition/Commandes clavier/incrementAvancementShift');
    incrementPitch.value = await getSettingValue('Edition/Commandes clavier/incrementPitch');
    incrementPitchShift.value = await getSettingValue('Edition/Commandes clavier/incrementPitchShift');

    toucheAvancementAvant.value = await getSettingValue('Edition/Commandes clavier/toucheAvancementAvant');
    toucheAvancementArriere.value = await getSettingValue('Edition/Commandes clavier/toucheAvancementArriere');
    touchePitchHaut.value = await getSettingValue('Edition/Commandes clavier/touchePitchHaut');
    touchePitchBas.value = await getSettingValue('Edition/Commandes clavier/touchePitchBas');

    incrementZoom.value = await getSettingValue('Edition/Commandes souris/incrementZoom');
    incrementZoomShift.value = await getSettingValue('Edition/Commandes souris/incrementZoomShift');
    incrementBearing.value = await getSettingValue('Edition/Commandes souris/incrementBearing') || 1;
    incrementBearingShift.value = await getSettingValue('Edition/Commandes souris/incrementBearingShift') || 5;

    updateInterpolation();

    currentPointIndex.value = 0;

    dataLoaded.value = true;

    // Ensure default message orientation is 'Droite' on load
    messageOrientation.value = 'Droite';

    if (zoomDepart.value) {
      await applyZoomDepart();
    } else {
      zoomDepartIsActive.value = false;
    }

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
      scrollZoom: false,
      dragRotate: true,
      dragPan: true,
      pitchWithRotate: false,
      keyboard: false,
    });

    window.addEventListener('wheel', handleWheel, { passive: false });
    mapContainer.value.addEventListener('contextmenu', handleContextMenu);

    map.on('load', () => {
      map.dragPan.disable();

      map.on('mousedown', (e) => {
        if (activeControlTab.value === 'camera') {
          return;
        }
        if (e.originalEvent.button === 0) { // Left mouse button
          console.log('Mousedown detected (left button)');
          previousPitch = map.getPitch();
          console.log('Previous pitch saved:', previousPitch);

          originalDragPanState = map.dragPan.isEnabled();
          console.log('Original dragPan state:', originalDragPanState);

          // Always disable dragPan to prevent it from interfering with pitch animation and custom panning
          map.dragPan.disable();
          console.log('dragPan disabled for pitch animation and custom panning');

          map.easeTo({ pitch: 0, duration: pitchTransitionDuration.value });
          console.log('easeTo called for pitch 0');

          // Once the pitch animation completes, start custom panning
          map.once('moveend', () => {
            console.log('Pitch to 0 animation ended. Starting custom panning.');
            isCustomPanning.value = true;
            lastMousePosition.x = e.originalEvent.clientX;
            lastMousePosition.y = e.originalEvent.clientY;
            mapContainer.value.addEventListener('mousemove', handleCustomMouseMove);
            console.log('Custom panning activated. Mousemove listener added.');
          });
        }
      });

      map.on('mouseup', (e) => {
        if (activeControlTab.value === 'camera') {
          return;
        }
        if (e.originalEvent.button === 0) { // Left mouse button
          console.log('Mouseup detected (left button)');

          // Stop custom panning
          if (isCustomPanning.value) {
            mapContainer.value.removeEventListener('mousemove', handleCustomMouseMove);
            isCustomPanning.value = false;
            console.log('Custom panning deactivated. Mousemove listener removed.');
          }

          // Disable dragPan before restoring pitch (it should already be disabled by custom panning logic)
          map.dragPan.disable();
          console.log('dragPan disabled before pitch restoration');

          map.easeTo({ pitch: previousPitch, duration: pitchTransitionDuration.value });
          console.log('easeTo called for previous pitch:', previousPitch);

          // After the pitch animation completes, restore dragPan to its original state
          map.once('moveend', () => {
            console.log('Pitch restoration animation ended.');
            if (originalDragPanState) {
              map.dragPan.enable();
              console.log('dragPan re-enabled to original state (enabled)');
            } else {
              map.dragPan.disable();
              console.log('dragPan re-disabled to original state (disabled)');
            }
          });
        }
      });

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
        lineMetrics: true, // Required for line-gradient
      });

      map.addLayer({
        id: 'circuit-line',
        type: 'line',
        source: 'circuit-line',
        layout: {
          'line-join': 'round',
          'line-cap': 'round',
        },
        paint: paintProps,
      });

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
          'line-color': mapboxAvancementColorHex.value,
          'line-width': epaisseurAvancement,
        },
      });

      // Function to update message popups based on current increment
      const updateMessagePopups = (currentIncrement) => {
        if (!map || !eventsFile.value.rangeEvents) return;

        // Clear all existing popups first
        activePopups.value.forEach(popup => popup.remove());
        activePopups.value.clear();

        const visibleMessageIds = new Set();

        // 1. Determine visible messages
        for (const msg of eventsFile.value.rangeEvents) {
            if (currentIncrement >= msg.startIncrement && currentIncrement <= msg.endIncrement) {
                visibleMessageIds.add(msg.eventId);
            }
        }

        // 2. Add new visible popups
        for (const eventId of visibleMessageIds) {
            const messageEvent = eventsFile.value.rangeEvents.find(e => e.eventId === eventId);
            if (messageEvent && messageEvent.message) {
                const svgContent = createMessageSVG(messageEvent);
                const orientation = messageEvent.orientation || 'Droite';
                const anchor = orientation === 'Gauche' ? 'bottom-right' : 'bottom-left';

                const popup = new mapboxgl.Popup({ closeButton: false, closeOnClick: false, anchor: anchor, className: 'map-message-popup' })
                    .setLngLat(messageEvent.coord)
                    .setHTML(svgContent)
                    .addTo(map);
                
                activePopups.value.set(eventId, popup);
            }
        }
      };

      // Watch for progress changes to display messages
      watch(trackProgress, (newIncrement) => {
        updateMessagePopups(newIncrement);
      }, { immediate: true });

      // Watch for changes in message events to update display instantaneously
      watch(eventsFile, () => {
        updateMessagePopups(trackProgress.value);
      }, { deep: true });
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
  padding: 0 0 4px 8px; /* Add padding-left and padding-bottom */
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

/* Remove the default white box and pointer/tip from our custom popups */
.map-message-popup .mapboxgl-popup-content {
  background: none;
  padding: 0;
  box-shadow: none;
}

.map-message-popup .mapboxgl-popup-tip {
  display: none;
}
</style>
