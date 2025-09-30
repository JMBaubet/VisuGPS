<template>
  <v-container fluid class="pa-0 fill-height">
    <div id="map-container" ref="mapContainer" class="fill-height"></div>

    <!-- Top UI Container -->
    <div style="position: absolute; top: 10px; left: 10px; right: 16px; z-index: 1000; display: flex; align-items: center; gap: 16px;">
      <v-btn icon="mdi-arrow-left" @click="goBack"></v-btn>
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
        :showZoom="showZoom"
        :showPitch="showPitch"
        :showBearingDelta="showBearingDelta"
        :showBearingTotalDelta="showBearingTotalDelta"
        :showEditedZoom="showEditedZoom"
        :showEditedPitch="showEditedPitch"
        :showEditedBearingDelta="showEditedBearingDelta"
        :showEditedBearingTotalDelta="showEditedBearingTotalDelta"
        :currentCameraBearing="currentBearing"
        :initialCameraBearing="trackingPoints[0]?.cap"
        :currentCameraZoom="currentZoom"
        :defaultCameraZoom="defaultZoom"
        :currentCameraPitch="currentPitch"
        :defaultCameraPitch="defaultPitch"
        :pause-events="pauseEventsForDisplay"
        :flyto-events="flytoEventsForDisplay"
        @seek-distance="handleSeekDistance"
      />
      <ControlTabsWidget
        v-model:showOriginalCurves="showOriginalCurves"
        v-model:showEditedCurves="showEditedCurves"
        v-model:showBearingDeltaPair="showBearingDeltaPair"
        v-model:showBearingTotalDeltaPair="showBearingTotalDeltaPair"
        v-model:showZoomPair="showZoomPair"
        v-model:showPitchPair="showPitchPair"
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
        :known-message-texts="knownMessageTexts"
        v-model:message-background-color-setting="messageBackgroundColorSetting"
        v-model:message-border-color-setting="messageBorderColorSetting"
        v-model:message-border-width-setting="messageBorderWidthSetting"
        v-model:message-pre-affichage-setting="messagePreAffichageSetting"
        v-model:message-post-affichage-setting="messagePostAffichageSetting"
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

const route = useRoute();
const router = useRouter();
const { showSnackbar } = useSnackbar();
const { getSettingValue } = useSettings();

// Confirmation Dialog
const showConfirmationDialog = ref(false);
const confirmationProps = ref({});
const resolveConfirmation = ref(null);

const updateCameraInfo = () => {
  if (!map) return;
  currentZoom.value = parseFloat(map.getZoom().toFixed(1));
  currentPitch.value = Math.round(map.getPitch());
  currentBearing.value = Math.round(map.getBearing());
};



const handleTabChange = (newTab) => {
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
            showSnackbar(`Erreur lors de l'ajout de la la pause: ${error}`, 'error');
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
      coord: [lngLat.lng, lngLat.lat],
      duree: duration, // Use duration from ControlTabsWidget
      pitch: Math.round(map.getPitch()),
      zoom: parseFloat(map.getZoom().toFixed(1)),
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
        showSnackbar(`Erreur lors de l'ajout de l'événement Survol: ${error}`, 'error');
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
    showSnackbar(`Erreur lors de l'ajout du message: ${error}`, 'error');
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

// Global visibility toggles
const showOriginalCurves = ref(true);
const showEditedCurves = ref(false);

// Paired visibility toggles
const showZoomPair = ref(false);
const showPitchPair = ref(false);
const showBearingDeltaPair = ref(true);
const showBearingTotalDeltaPair = ref(true);

// Computed visibility for individual curves
const showZoom = computed(() => showZoomPair.value && showOriginalCurves.value);
const showPitch = computed(() => showPitchPair.value && showOriginalCurves.value);
const showBearingDelta = computed(() => showBearingDeltaPair.value && showOriginalCurves.value);
const showBearingTotalDelta = computed(() => showBearingTotalDeltaPair.value && showOriginalCurves.value);

const showEditedZoom = computed(() => showZoomPair.value && showEditedCurves.value);
const showEditedPitch = computed(() => showPitchPair.value && showEditedCurves.value);
const showEditedBearingDelta = computed(() => showBearingDeltaPair.value && showEditedCurves.value);
const showEditedBearingTotalDelta = computed(() => showBearingTotalDeltaPair.value && showEditedCurves.value);

// Graph curve colors
const graphZoomColor = ref('green');
const graphPitchColor = ref('blue');
const graphBearingDeltaColor = ref('amber');
const graphBearingTotalDeltaColor = ref('pink');

const graphEditedZoomColor = ref('light-green-accent-3');
const graphEditedPitchColor = ref('cyan-accent-3');
const graphEditedBearingDeltaColor = ref('yellow-accent-3');
const graphEditedBearingTotalDeltaColor = ref('purple-accent-3');

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
          points[currentIndex].editedZoom = startCp.editedZoom + j * zoomStep;
          points[currentIndex].editedPitch = startCp.editedPitch + j * pitchStep;
          let newBearing = startCp.editedCap + j * bearingStep;
          points[currentIndex].editedCap = (newBearing + 360) % 360;
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
  point.editedZoom = currentZoom.value;
  point.editedPitch = currentPitch.value;
  point.editedCap = currentBearing.value;

  const cameraPos = map.getFreeCameraOptions().position;
  point.coordonneeCamera = [cameraPos.toLngLat().lng, cameraPos.toLngLat().lat];
  point.altitudeCamera = cameraPos.toAltitude();

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

  const point = trackingPoints.value[index];

  const flyToOptions = {
    center: point.coordonnee,
    essential: true
  };

  if (cameraSyncMode.value === 'original') {
    currentZoom.value = point.zoom;
    currentPitch.value = point.pitch;
    currentBearing.value = point.cap;
    flyToOptions.zoom = currentZoom.value;
    flyToOptions.pitch = currentPitch.value;
    flyToOptions.bearing = currentBearing.value;
  }

  map.flyTo(flyToOptions);

  if (totalLineLength.value > 0) {
    currentProgressDistance.value = point.distance;
    progressPercentage.value = (point.distance / totalLineLength.value) * 100;

    const line = turf.lineString(lineStringCoordinates.value); // Use original ref value directly
    let slicedLine;

    if (point.distance === 0) {
      // If at the start, set to an empty LineString or a LineString with just the first point
      slicedLine = {
        type: 'Feature',
        properties: {},
        geometry: {
          type: 'LineString',
          coordinates: [], // Empty LineString
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



onUnmounted(() => {
  destroyMap();
  window.removeEventListener('keydown', handleKeydown);
});

const handleKeydown = (event) => {
  const targetNodeName = event.target.nodeName;
  if (targetNodeName === 'INPUT' || targetNodeName === 'TEXTAREA' || event.target.isContentEditable) {
    return;
  }

  let step = 1;
  if (event.ctrlKey) {
    step = 100;
  } else if (event.shiftKey) {
    step = 10;
  }

  let newIndex = currentPointIndex.value;
  let handled = false;

  switch (event.key) {
    case 'ArrowRight':
      newIndex = Math.min(currentPointIndex.value + step, trackingPoints.value.length - 1);
      handled = true;
      break;
    case 'ArrowLeft':
      newIndex = Math.max(currentPointIndex.value - step, 0);
      handled = true;
      break;
    case 'ArrowUp': {
      if (!map) return;
      const newPitch = Math.min(map.getPitch() + 1, 85);
      map.easeTo({ pitch: newPitch, duration: 50 });
      handled = true;
      break;
    }
    case 'ArrowDown': {
      if (!map) return;
      const newPitch = Math.max(map.getPitch() - 1, 0);
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
    const rawGraphZoomColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurZoom');
    graphZoomColor.value = toHex(rawGraphZoomColor);

    const rawGraphPitchColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurPitch');
    graphPitchColor.value = toHex(rawGraphPitchColor);

    const rawGraphBearingDeltaColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurBearingDelta');
    graphBearingDeltaColor.value = toHex(rawGraphBearingDeltaColor);

    const rawGraphBearingTotalDeltaColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurBearingTotalDelta');
    graphBearingTotalDeltaColor.value = toHex(rawGraphBearingTotalDeltaColor);

    // Load edited graph curve colors
    const rawGraphEditedZoomColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurEditedZoom');
    graphEditedZoomColor.value = toHex(rawGraphEditedZoomColor);

    const rawGraphEditedPitchColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurEditedPitch');
    graphEditedPitchColor.value = toHex(rawGraphEditedPitchColor);

    const rawGraphEditedBearingDeltaColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurEditedBearingDelta');
    graphEditedBearingDeltaColor.value = toHex(rawGraphEditedBearingDeltaColor);

    const rawGraphEditedBearingTotalDeltaColor = await getSettingValue('Edition/Graphe/CouleurCourbes/couleurEditedBearingTotalDelta');
    graphEditedBearingTotalDeltaColor.value = toHex(rawGraphEditedBearingTotalDeltaColor);



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
      distance: index * segmentLengthKm,
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

    // Initial interpolation
    updateInterpolation();

    currentPointIndex.value = 0;

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
      scrollZoom: { around: 'center' },
      pitchWithRotate: false,
    });

    map.on('load', () => {
      map.dragPan.disable(); // Disable pan by default as the initial tab is 'camera'

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