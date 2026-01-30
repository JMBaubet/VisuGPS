<template>
  <div class="d-flex flex-row fill-height overflow-hidden" style="height: 100vh; width: 100vw;">
    <!-- Main Content Area (Toolbar + Map) -->
    <div class="d-flex flex-column flex-grow-1" style="min-width: 0;">
       <VariantToolbar
         v-model:mode="currentMode"
         v-model:profile="variantConfig.routingProfile"
         :circuit-name="circuitName"
         :error-profile="routingErrorProfile"
         @save="saveVariant"
         @close="goHome"
         @open-doc="isDocDialogVisible = true"
       />

       <div class="flex-grow-1 w-100 position-relative">
         <div id="map-container" class="fill-height w-100"></div>
         
         <v-overlay
           v-model="isLoading"
           contained
           class="align-center justify-center"
         >
           <v-progress-circular indeterminate color="primary"></v-progress-circular>
         </v-overlay>
       </div>
    </div>

    <!-- Sidebar Area (Full Height) -->
    <div 
      v-if="showSidebar" 
      class="fill-height border-s bg-white" 
      style="width: 400px; flex: 0 0 400px;"
    >
      <VariantSidebar
        :circuit-name="circuitName"
        :active-mode="currentMode"
        :config="variantConfig"
        :modifications="modifications"
        :saved-variants="savedVariants"
        :can-generate-preview="canGeneratePreview"
        :is-valid="isValid"
        :is-editing="!!loadedVariantId"
        :is-modified="isModified"
        :variant-name="loadedVariantName"
        :trackingPoints="trackingPoints"
        :segment-length="trackingSegmentLength"
        :projected-stats="variantProjectedStats"
        @generate="generatePreview"
        @save="saveVariant"
        @delete-point="handleDeletePoint"
        @delete-mod="handleDeleteMod"
        @finalize-mod="finalizeMod"
        @rename-mod="handleRenameMod"
        @update-routing="handleUpdateRouting"
        @fly-to-mod="handleFlyToMod"
        @load-variant="handleLoadVariant"
        @delete-saved-variant="handleDeleteSavedVariant"
        @rename-saved-variant="handleRenameSavedVariant"
        @reset="resetPoints"
      />
    </div>

    <!-- Renaming Dialog -->
    <v-dialog v-model="showRenameDialog" max-width="400px">
      <v-card>
        <v-card-title class="bg-primary text-white px-4 py-2 d-flex align-center">
          <v-icon start icon="mdi-pencil"></v-icon>
          Renommer le segment
        </v-card-title>
        <v-card-text class="pa-4">
          <v-text-field
            v-model="renameValue"
            label="Nom du segment"
            hide-details
            autofocus
            @keyup.enter="confirmRename"
          ></v-text-field>
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn variant="text" @click="showRenameDialog = false">Annuler</v-btn>
          <v-btn color="primary" variant="flat" @click="confirmRename">Valider</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Variant Save Dialog -->
    <v-dialog v-model="showSaveDialog" max-width="400px">
      <v-card>
        <v-card-title class="bg-success text-white px-4 py-2 d-flex align-center">
          <v-icon start icon="mdi-content-save"></v-icon>
          Enregistrer la variante
        </v-card-title>
        <v-card-text class="pa-4">
          <v-text-field
            v-model="variantName"
            label="Nom de la variante"
            hide-details
            autofocus
            @keyup.enter="confirmSaveVariant"
          ></v-text-field>
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn variant="text" @click="showSaveDialog = false">Annuler</v-btn>
          <v-btn color="success" variant="flat" @click="confirmSaveVariant">Enregistrer</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Delete Confirmation Dialog -->
    <v-dialog v-model="showDeleteDialog" max-width="400px">
      <v-card>
        <v-card-title class="bg-error text-white px-4 py-2 d-flex align-center">
          <v-icon start icon="mdi-alert-circle-outline"></v-icon>
          Supprimer la variante ?
        </v-card-title>
        <v-card-text class="pa-4">
          Êtes-vous sûr de vouloir supprimer définitivement la variante <strong>{{ variantToDelete?.name }}</strong> ?<br><br>
          Cette action est irréversible.
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn variant="text" @click="showDeleteDialog = false">Annuler</v-btn>
          <v-btn color="error" variant="flat" @click="confirmDeleteSavedVariant">Supprimer</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Rename Saved Variant Dialog -->
    <v-dialog v-model="showVariantRenameDialog" max-width="500">
      <v-card>
        <v-card-title class="bg-primary text-white px-4 py-2 d-flex align-center">
          <v-icon start icon="mdi-pencil"></v-icon>
          Renommer la variante
        </v-card-title>
        <v-card-text class="pa-4">
          <v-text-field
            v-model="variantRenameValue"
            label="Nouveau nom"
            variant="outlined"
            hide-details
            autofocus
            @keyup.enter="confirmVariantRename"
          ></v-text-field>
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn variant="text" @click="showVariantRenameDialog = false">Annuler</v-btn>
          <v-btn color="primary" variant="flat" @click="confirmVariantRename">Renommer</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <!-- Documentation Dialog -->
    <v-dialog v-model="isDocDialogVisible" max-width="900px">
      <DocDisplay doc-path="docs/DocUtilisateur/route_builder.md" @close="isDocDialogVisible = false" />
    </v-dialog>

    <!-- Critical Error Dialog -->
    <v-dialog v-model="showConfigErrorDialog" max-width="500px">
      <v-card>
        <v-card-title class="bg-error text-white px-4 py-2 d-flex align-center">
          <v-icon start icon="mdi-alert-octagon"></v-icon>
          Erreur de Configuration
        </v-card-title>
        <v-card-text class="pa-4">
          {{ configErrorMessage }}
          <br><br>
          Veuillez vérifier vos clés API dans les <strong>Paramètres</strong> de l'application.
        </v-card-text>
        <v-card-actions class="pa-4 pt-0">
          <v-spacer></v-spacer>
          <v-btn color="error" variant="flat" @click="showConfigErrorDialog = false">Fermer</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch, reactive } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import VariantToolbar from '../components/Variant/VariantToolbar.vue';
import VariantSidebar from '../components/Variant/VariantSidebar.vue';
import DocDisplay from '../components/DocDisplay.vue';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';
import * as turf from '@turf/turf';

import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import { useSnackbar } from '@/composables/useSnackbar';

const props = defineProps({
  circuitId: {
    type: String,
    required: true
  }
});

const router = useRouter();
const { getSettingValue, initSettings } = useSettings();
const { toHex } = useVuetifyColors();
const { showSnackbar } = useSnackbar();

const currentMode = ref('SEGMENT'); 
const showSidebar = ref(true);
const isLoading = ref(true);
const isDocDialogVisible = ref(false);
const routingErrorProfile = ref(null);
const isSaving = ref(false);

// Renaming Dialog State
const showRenameDialog = ref(false);
const renameValue = ref('');
const renameIndex = ref(-1);

// Save Variant Dialog State
const showSaveDialog = ref(false);
const variantName = ref('');

// Rename Saved Variant State
const showVariantRenameDialog = ref(false);
const variantRenameValue = ref('');
const variantToRenameId = ref(null);

// Delete Confirmation Dialog State
const showDeleteDialog = ref(false);
const variantToDelete = ref(null);

// Config Error Dialog State
const showConfigErrorDialog = ref(false);
const configErrorMessage = ref('');

const variantConfig = reactive({
  routingService: 'GraphHopper',
  routingProfile: 'bike'
});

const isModified = ref(false);
const loadedVariantId = ref(null);
const loadedVariantName = ref('');

const circuitName = ref('');

const map = ref(null);
const masterTraceGeojson = ref(null);
const trackingPoints = ref([]);
const modifications = ref([]);
const savedVariants = ref([]);
const previewGeojson = ref(null);
const canGeneratePreview = computed(() => {
  return modifications.value.some(m => m.points.length >= 2);
});

const trackingSegmentLength = computed(() => {
    return getSettingValue('Importation/Tracking/LongueurSegment') || 100;
});

import { useVariantCalculator } from '@/composables/useVariantCalculator';

const { calculateVariantStats } = useVariantCalculator();

const variantProjectedStats = computed(() => {
    return calculateVariantStats(
        masterTraceGeojson.value ? turf.length(masterTraceGeojson.value, { units: 'kilometers' }) : 0,
        modifications.value,
        trackingSegmentLength.value
    );
});

const isValid = computed(() => {
  return modifications.value.length > 0 && modifications.value.every(m => m.finalized);
});

// Watch specific to the active mod (dynamic during first creation only)
watch(() => variantConfig.routingProfile, (newVal) => {
    // Only affects segments that are NOT yet finalized AND have never been locked
    const activeMod = modifications.value.find(m => !m.finalized && !m.routingLocked && m.type === currentMode.value);
    if (activeMod) {
        activeMod.routingProfile = newVal;
        const modIndex = modifications.value.indexOf(activeMod);
        generatePreviewForMod(modIndex);
    }
});

watch(() => variantConfig.routingService, (newVal) => {
    const activeMod = modifications.value.find(m => !m.finalized && !m.routingLocked && m.type === currentMode.value);
    if (activeMod) {
        activeMod.routingService = newVal;
        const modIndex = modifications.value.indexOf(activeMod);
        generatePreviewForMod(modIndex);
    }
});

const resetPoints = () => {
    modifications.value = [];
    previewGeojson.value = null;
    isModified.value = false;
    loadedVariantId.value = null;
    loadedVariantName.value = '';
    if (map.value && map.value.getSource('preview-source')) {
        map.value.getSource('preview-source').setData({type: 'FeatureCollection', features: []});
    }
    if (map.value && map.value.getSource('markers-source')) {
        map.value.getSource('markers-source').setData({type: 'FeatureCollection', features: []});
    }
};

// Helper to ensure Mapbox gets a valid hex color
const resolveColor = (colorValue, fallback) => {
    if (!colorValue) return fallback;
    // Attempt to convert Vuetify name to hex
    const hex = toHex(colorValue);
    // If toHex returned a valid hex string, use it. Otherwise uses fallback.
    // Note: toHex returns input if not found in palette. So we check for # again.
    return hex.startsWith('#') ? hex : fallback;
};

const initMap = async () => {
  try {
    await initSettings(); // Ensure settings are loaded
    
    // Retrieve token via composable
    const token = getSettingValue('Système/Tokens/mapbox');
    if (!token) {
        console.error("Mapbox token not found in settings");
        isLoading.value = false;
        return;
    }
    
    // Load behavior settings
    variantConfig.routingService = getSettingValue('Variante/Parametres/routingService') || 'GraphHopper';
    
    // Mapping French labels from settings to technical keys for Toolbar/API
    const profileLabel = getSettingValue('Variante/Parametres/routingType') || 'Route uniquement';
    const profileMap = {
        'Route + Pistes cyclables': 'car',
        'Route uniquement': 'racingbike',
        'VTT / Chemin': 'bike'
    };
    variantConfig.routingProfile = profileMap[profileLabel] || 'racingbike';
    
    console.log(`[Init] Loaded profile: "${profileLabel}" mapped to "${variantConfig.routingProfile}"`);
    console.log(`[Init] Using routing service: ${variantConfig.routingService}`);

    // Watch for profile changes to reset error state
    // Watch moved for better scoping

    mapboxgl.accessToken = token;

    map.value = new mapboxgl.Map({
      container: 'map-container',
      style: getSettingValue('Variante/Edition/Carte/style') || 'mapbox://styles/mapbox/outdoors-v12',
      center: [2.2137, 46.2276],
      zoom: 5
    });

    // Add navigation control (the compass)
    map.value.addControl(new mapboxgl.NavigationControl({ visualizePitch: true }), 'top-right');

    map.value.on('load', async () => {
        // Add 3D Terrain
        map.value.addSource('mapbox-dem', {
            'type': 'raster-dem',
            'url': 'mapbox://mapbox.mapbox-terrain-dem-v1',
            'tileSize': 512,
            'maxzoom': 14
        });
        map.value.setTerrain({ 'source': 'mapbox-dem', 'exaggeration': 1.5 });

        try {
            const circuitData = await invoke('get_circuit_data', { circuitId: props.circuitId });
            circuitName.value = circuitData.nom;
        } catch (err) {
            console.error("Error fetching circuit name:", err);
        }

       await loadCircuitTrace();
       await loadSavedVariants();
       isLoading.value = false;
       
       map.value.on('click', handleMapClick);
       
       // Resolve colors with fallbacks
       const previewColor = resolveColor(getSettingValue('Variante/Edition/Trace/couleur'), '#651FFF');
       const previewWidth = getSettingValue('Variante/Edition/Trace/largeur') || 4;
              map.value.addSource('preview-source', { type: 'geojson', data: { type: 'FeatureCollection', features: [] } });
        map.value.addLayer({
            id: 'preview-layer',
            type: 'line',
            source: 'preview-source',
            paint: {
                'line-color': [
                    'match',
                    ['get', 'type'],
                    'DEPART', '#4CAF50', // Green
                    'ARRIVEE', '#F44336',  // Red
                    previewColor             // Configured color for segments
                ],
                'line-width': previewWidth,
                'line-dasharray': [
                    'case',
                    ['boolean', ['get', 'finalized'], false],
                    ['literal', [1]],         // Solid
                    ['literal', [2, 1]]       // Dashed
                ]
            }
        });



       const nodeColor = resolveColor(getSettingValue('Variante/Edition/Noeuds/couleur'), '#FF9800');
       
       // Tracking Points Layer
       try {
            const trackingData = await invoke('read_tracking_file', { circuitId: props.circuitId });
            console.log(`[Init] Loaded ${trackingData.length} tracking points for circuit ${props.circuitId}`);
            trackingPoints.value = trackingData;
            const trackingFeatures = trackingData.map(p => ({
                type: 'Feature',
                geometry: {
                    type: 'Point',
                    coordinates: p.coordonnee
                },
                properties: {
                    increment: p.increment,
                    altitude: p.altitude,
                    pointDeControl: p.pointDeControl
                }
            }));
            
            map.value.addSource('tracking-source', {
                type: 'geojson',
                data: { type: 'FeatureCollection', features: trackingFeatures }
            });

            const zoomVisuNode = getSettingValue('Variante/Edition/Noeuds/zoomVisu') || 13;

            map.value.addLayer({
                id: 'tracking-layer',
                type: 'circle',
                source: 'tracking-source',
                minzoom: zoomVisuNode,
                paint: {
                    'circle-radius': [
                        'case',
                        ['boolean', ['get', 'pointDeControl'], false],
                        9,
                        4
                    ],
                    'circle-color': [
                        'case',
                        ['boolean', ['get', 'pointDeControl'], false],
                        '#FF9800', // Orange for control points
                        nodeColor  // Default node color
                    ],
                    'circle-stroke-width': [
                        'case',
                        ['boolean', ['get', 'pointDeControl'], false],
                        2,
                        0
                    ],
                    'circle-stroke-color': '#FFFFFF',
                    'circle-opacity': 0.8
                }
            });
       } catch (err) {
            console.warn("Could not load tracking points:", err);
       }
              map.value.addSource('markers-source', { type: 'geojson', data: { type: 'FeatureCollection', features: [] } });
        
        // Finalized Pins Layer (Start/End)
        map.value.addLayer({
            id: 'pins-layer',
            type: 'circle',
            source: 'markers-source',
            filter: ['match', ['get', 'type'], ['START_PIN', 'END_PIN'], true, false],
            paint: {
                'circle-radius': 7,
                'circle-color': [
                    'match',
                    ['get', 'type'],
                    'START_PIN', '#4CAF50', // Green
                    'END_PIN', '#F44336',   // Red
                    '#000000'
                ],
                'circle-stroke-width': 1.5,
                'circle-stroke-color': '#ffffff'
            }
        });

        // Standard Markers Layer (excluding pins)
        map.value.addLayer({
            id: 'markers-layer',
            type: 'circle',
            source: 'markers-source',
            filter: ['match', ['get', 'type'], ['START_PIN', 'END_PIN'], false, true],
            paint: {
                'circle-radius': [
                    'match',
                    ['get', 'type'],
                    'SEGMENT_START', 5,
                    'SEGMENT_END', 5,
                    4 // Default for others
                ],
                'circle-color': [
                    'match',
                    ['get', 'type'],
                    'ANCHOR', '#ffffff',
                    'WAYPOINT', '#FFEB3B', 
                    'SEGMENT_START', '#43A047', // Green 600
                    'SEGMENT_END', '#E53935',   // Red 600
                    '#2196F3'
                ],
                'circle-stroke-width': [
                    'case',
                    ['boolean', ['get', 'finalized'], false],
                    1,
                    1.5
                ],
                'circle-stroke-color': '#000000'
            }
        });
    });

  } catch (e) {
    console.error("Map init error", e);
    isLoading.value = false;
  }
};

const loadCircuitTrace = async () => {
  try {
     const geojson = await invoke('read_line_string_file', { circuitId: props.circuitId });
     masterTraceGeojson.value = geojson;
     console.log(`[Init] Master trace loaded: ${geojson.coordinates.length} points`);
     
     const originalColor = resolveColor(getSettingValue('Variante/Edition/Trace Maîtresse/couleur'), '#BDBDBD');
     const originalWidth = getSettingValue('Variante/Edition/Trace Maîtresse/largeur') || 4;
     const originalOpacity = getSettingValue('Variante/Edition/Trace Maîtresse/opacite') !== undefined 
                           ? getSettingValue('Variante/Edition/Trace Maîtresse/opacite') 
                           : 0.3;
     
     if(map.value.getSource('trace-source')) {
         map.value.getSource('trace-source').setData(geojson);
     } else {
         map.value.addSource('trace-source', {
             type: 'geojson',
             data: geojson
         });
         
         map.value.addLayer({
             id: 'trace-layer',
             type: 'line',
             source: 'trace-source',
             layout: {
                 'line-join': 'round',
                 'line-cap': 'round'
             },
             paint: {
                 'line-color': originalColor,
                 'line-width': originalWidth,
                 'line-opacity': originalOpacity
             }
         });
         
         // Fit bounds
         const coords = geojson.coordinates;
         const bounds = new mapboxgl.LngLatBounds(coords[0], coords[0]);
         for (const coord of coords) {
             bounds.extend(coord);
         }
         map.value.fitBounds(bounds, { padding: 50 });
     }
  } catch (e) {
      console.error("Failed to load trace", e);
  }
};

const handleMapClick = (e) => {
    if (!masterTraceGeojson.value) return;

    const clickPoint = turf.point([e.lngLat.lng, e.lngLat.lat]);
    
    let isSnap = false;
    let newPoint = null;

    // Snapping logic
    if (trackingPoints.value.length > 0) {
        let minInfo = { dist: Infinity, point: null };
        for (const p of trackingPoints.value) {
            const pt = turf.point(p.coordonnee);
            const d = turf.distance(clickPoint, pt, { units: 'meters' });
            if (d < minInfo.dist) { minInfo = { dist: d, point: p }; }
        }
        if (minInfo.dist < 30) {
            if (!minInfo.point.pointDeControl) {
                showSnackbar("Veuillez utiliser les points de contrôle pour l'ancrage.", "warning");
                return;
            }
            isSnap = true;
            newPoint = { coords: minInfo.point.coordonnee, type: 'ANCHOR', index: minInfo.point.increment };
        }
    }

    // Fallback snapping to lineString is REMOVED as per user request (Phase 23)
    // We only want to snap to points present in tracking.json

    // Check if a modification of this type is already finalized
    if (currentMode.value === 'DEPART' || currentMode.value === 'ARRIVEE') {
        const finalizedExists = modifications.value.some(m => m.type === currentMode.value && m.finalized);
        if (finalizedExists) {
            const label = currentMode.value === 'DEPART' ? "un départ" : "une arrivée";
            showSnackbar(`Vous avez déjà validé ${label} pour cette variante.`, "warning");
            return;
        }
    }

    // Find an active (non-finalized) modification of the current type (redundant now for DEPART/ARRIVEE but kept for logic)
    let activeMod = modifications.value.find(m => m.type === currentMode.value && !m.finalized);

    if (!isSnap) {
        // Validation: Every mod MUST start with an anchor on the trace
        if (!activeMod || activeMod.points.length === 0) {
            const typeLabels = {
                'DEPART': 'Un départ',
                'ARRIVEE': 'Une arrivée',
                'SEGMENT': 'Un segment'
            };
            const label = typeLabels[currentMode.value] || 'Une modification';
            showSnackbar(`${label} doit impérativement commencer par un point d'ancrage sur la trace.`, "error");
            return;
        }
        
        newPoint = { coords: [e.lngLat.lng, e.lngLat.lat], type: 'WAYPOINT' };
    } else {
        // Validation: DEPART and ARRIVEE only allowed one anchor
        if (activeMod && (currentMode.value === 'DEPART' || currentMode.value === 'ARRIVEE')) {
            const hasAnchor = activeMod.points.some(p => p.type === 'ANCHOR');
            if (hasAnchor) {
                showSnackbar("Un seul point d'ancrage est autorisé pour un départ ou une arrivée.", "error");
                return;
            }
        }

        // --- NEW CONSISTENCY CHECKS ---
        const anchorIndex = newPoint.index;
        const finalizedMods = modifications.value.filter(m => m.finalized);

        // 1. General Overlap Check: Cannot anchor inside an existing finalized Segment
        for (const mod of finalizedMods) {
            if (mod.type === 'SEGMENT') {
                const anchors = mod.points.filter(p => p.type === 'ANCHOR');
                if (anchors.length >= 2) {
                    const idx1 = anchors[0].index;
                    const idx2 = anchors[anchors.length - 1].index;
                    const minIdx = Math.min(idx1, idx2);
                    const maxIdx = Math.max(idx1, idx2);
                    
                    if (anchorIndex > minIdx && anchorIndex < maxIdx) {
                         showSnackbar("Impossible d'ancrer dans une zone déjà segmentée par une autre variante.", "error");
                         return;
                    }
                }
            }
        }

        // 2. DEPART Constraints
        if (currentMode.value === 'DEPART') {
            // Must be BEFORE any finalized Segment
            for (const mod of finalizedMods) {
                if (mod.type === 'SEGMENT') {
                     const validAnchors = mod.points.filter(p => p.type === 'ANCHOR');
                     if (validAnchors.length > 0) {
                         // A segment typically starts at its first anchor index (min)
                         const segMin = Math.min(...validAnchors.map(a => a.index));
                         if (anchorIndex >= segMin) {
                             showSnackbar("Le départ doit être placé AVANT le début du premier segment.", "error");
                             return;
                         }
                     }
                }
                if (mod.type === 'ARRIVEE') {
                     const arrAnchor = mod.points.find(p => p.type === 'ANCHOR');
                     if (arrAnchor && anchorIndex >= arrAnchor.index) {
                          showSnackbar("Le départ doit être placé AVANT l'arrivée.", "error");
                          return;
                     }
                }
            }
        }

        // 3. ARRIVEE Constraints
        if (currentMode.value === 'ARRIVEE') {
             // Must be AFTER any finalized Segment
            for (const mod of finalizedMods) {
                if (mod.type === 'SEGMENT') {
                     const validAnchors = mod.points.filter(p => p.type === 'ANCHOR');
                     if (validAnchors.length > 0) {
                         const segMax = Math.max(...validAnchors.map(a => a.index));
                         if (anchorIndex <= segMax) {
                             showSnackbar("L'arrivée doit être placée APRÈS la fin du dernier segment.", "error");
                             return;
                         }
                     }
                }
                if (mod.type === 'DEPART') {
                     const depAnchor = mod.points.find(p => p.type === 'ANCHOR');
                     if (depAnchor && anchorIndex <= depAnchor.index) {
                          showSnackbar("L'arrivée doit être placée APRÈS le départ.", "error");
                          return;
                     }
                }
            }
        }

        // 4. SEGMENT Constraints
        if (currentMode.value === 'SEGMENT') {
            // Must be AFTER any finalized DEPART
            const departMod = finalizedMods.find(m => m.type === 'DEPART');
            if (departMod) {
                const depAnchor = departMod.points.find(p => p.type === 'ANCHOR');
                if (depAnchor && anchorIndex <= depAnchor.index) {
                    showSnackbar("Tout segment doit commencer APRÈS le point de départ défini.", "error");
                    return;
                }
            }

            // Must be BEFORE any finalized ARRIVEE
            const arriveeMod = finalizedMods.find(m => m.type === 'ARRIVEE');
            if (arriveeMod) {
                const arrAnchor = arriveeMod.points.find(p => p.type === 'ANCHOR');
                if (arrAnchor && anchorIndex >= arrAnchor.index) {
                     showSnackbar("Tout segment doit finir AVANT le point d'arrivée défini.", "error");
                     return;
                }
            }
        }
    }

    // Create new group if none found for this mode that is active
    if (!activeMod) {
        // --- INITIATE NEW VARIANT NAMING ---
        if (!loadedVariantId.value) {
            loadedVariantId.value = `var_${crypto.randomUUID()}`;
            // Set a default and show dialog
            variantName.value = `Variante ${new Date().toLocaleDateString()} ${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
            showSaveDialog.value = true;
            console.log(`[AutoSave] New Variant ID generated, waiting for name...`);
        }

        activeMod = { 
            type: currentMode.value, 
            points: [], 
            preview: null, 
            finalized: false,
            routingLocked: false, // New flag to distinguish first creation from later edits
            routingService: variantConfig.routingService,
            routingProfile: variantConfig.routingProfile,
            routingStatus: 'SUCCESS'
        };
        modifications.value.push(activeMod);
    }

    // Add point to active group
    if (isSnap && activeMod && activeMod.type === 'SEGMENT' && activeMod.points.length === 1) {
        const startAnchor = activeMod.points[0];
        if (newPoint.index <= startAnchor.index) {
            showSnackbar("L'ancre de fin doit se situer après l'ancre de départ sur le tracé.", "error");
            return;
        }
    }

    // --- NEW VALIDATION: VARIANT LENGTH CHECK ---
    
    // Calculates length of the "cut" section on main trace
    const calculateMainTraceSectionLength = (anchorIndex1, anchorIndex2) => {
        if (!trackingPoints.value || trackingPoints.value.length === 0) return Infinity;
        
        let startIdx = 0;
        let endIdx = 0;
        
        // Handle single anchor cases (Start/End)
        if (anchorIndex2 === undefined) { 
             // Logic depends on context, handled by caller or assume full trace?
             // Actually for DEPART: 0 to anchorIndex1
             // For ARRIVEE: anchorIndex1 to LAST
             return 0; 
        }

        startIdx = Math.min(anchorIndex1, anchorIndex2);
        endIdx = Math.max(anchorIndex1, anchorIndex2);
        
        // Ensure bounds
        startIdx = Math.max(0, startIdx);
        endIdx = Math.min(trackingPoints.value.length - 1, endIdx);
        
        // Get distances from tracking points (assuming 'cumulDist' or calculate)
        // trackingPoints usually has 'distance' (km) or we calculate from coordinates
        const p1 = trackingPoints.value[startIdx];
        const p2 = trackingPoints.value[endIdx];
        
        // Use pre-calculated distances if available in tracking.json
        // tracking.json has 'distance' field? Let's check initMap
        // initMap uses invoke('read_tracking_file'), which returns fields.
        // Usually tracking points have cumulative distance.
        // Let's assume linear distance sum if not available.
        
        // Using turf distance on the slice of lineString is safest if 'distance' field missing
        if (masterTraceGeojson.value) {
             const slice = turf.lineSlice(
                 turf.point(p1.coordonnee), 
                 turf.point(p2.coordonnee), 
                 masterTraceGeojson.value
             );
             return turf.length(slice, { units: 'kilometers' });
        }
        return 0;
    };

    // Calculate current modification length + new point
    const calculateProjectedVariantLength = (mod, pointToAdd) => {
        // We simulate adding the point to the list
        const currentPoints = mod.points.map(p => p.coords);
        const newPoints = [...currentPoints, pointToAdd.coords];
        
        if (newPoints.length < 2) return 0;
        
        // If we have a preview (routed), use it for the EXISTING part?
        // No, we want to know the length WITH the new point.
        // Since we don't have the routed path for the new point yet, 
        // we must Estimate it.
        // BUT user says "we have precise length via router".
        // This implies we should Route the new segment Proposal?
        // That is async and slow for a synchronous click handler.
        // Compromise: Use Turf distance (crow flies) between last point and new point,
        // added to existing routed length.
        
        let existingLength = 0;
        if (mod.preview && mod.preview.coordinates) {
             existingLength = turf.length(mod.preview, { units: 'kilometers' });
        } else if (mod.points.length >= 2) {
             // Fallback if no preview
             existingLength = turf.length(turf.lineString(currentPoints), { units: 'kilometers' });
        }
        
        const lastPoint = currentPoints[currentPoints.length - 1];
        const distToAdd = turf.distance(
            turf.point(lastPoint), 
            turf.point(pointToAdd.coords), 
            { units: 'kilometers' }
        );
        
        // We multiply distToAdd by a factor (e.g. 1.1) to account for road winding?
        // Better strict check later, loose check now?
        // User wants strict check.
        // Let's trust the user knows straight lines != road.
        // But if we block strictly on "straight line < curved road", we might block valid paths 
        // that are actually shorter but look longer in straight lines? No, straight is always shorter.
        // The problem is: estimated NEW length (straight) < Real New Length (Road).
        // So we UNDER-estimate the new length.
        // If even the Under-estimation is > Original, then DEFINITELY block.
        // If Under-estimation < Original, we might still accept it, and then Route check fails later.
        
        return existingLength + distToAdd;
    };

    if (activeMod) {
         let projectedVarLen = calculateProjectedVariantLength(activeMod, newPoint);
         let originalSectionLen = Infinity;
         
         if (currentMode.value === 'DEPART') {
             // Section: Start(0) to Anchor (current newPoint must be Anchor?)
             // No, DEPART builds FROM end TO start (reversed internally) or user clicks 
             // typically: Anchor (Start of mod, but End of variant) -> Waypoints -> Start.
             // Wait, DEPART_DEPORTE implementation: 
             // "Points: ... Waypoints ... Anchor(IndexOnMaster)"? 
             // Or "Anchor(IndexOnMaster) ... Waypoints ... Start"?
             // Let's check `generatePreviewForMod`:
             // if (mod.type === 'DEPART') coords = reversed...
             // Usually user clicks Anchor first (on trace) then moves away?
             // If activeMod has 0 points, newPoint IS the Anchor.
             
             // Case A: First point (Anchor)
             if (activeMod.points.length === 0) {
                 if (!isSnap) { /* already handled above */ }
                 // Anchor defined. Length is 0. Valid.
             } else {
                 // Case B: Adding Waypoints (moving away from anchor)
                 // The "Variant" is the path created.
                 // The "Original" is the path from Trace Start (0) to The Anchor.
                 const anchor = activeMod.points[0]; // First point is Anchor
                 originalSectionLen = calculateMainTraceSectionLength(0, anchor.index);
                 
                 if (projectedVarLen > originalSectionLen) {
                     showSnackbar(`La variante projetée (${projectedVarLen.toFixed(2)}km) dépasse la section originale (${originalSectionLen.toFixed(2)}km).`, "error");
                     return;
                 }
             }
         }
         else if (currentMode.value === 'ARRIVEE') {
             // ARRIVEE: Anchor ... Waypoints ... End
             // Original: Anchor to Trace End
             if (activeMod.points.length === 0) {
                  // Anchor defined.
             } else {
                 const anchor = activeMod.points[0];
                 const lastTraceIdx = trackingPoints.value.length - 1;
                 originalSectionLen = calculateMainTraceSectionLength(anchor.index, lastTraceIdx);
                 
                  if (projectedVarLen > originalSectionLen) {
                     showSnackbar(`La variante projetée (${projectedVarLen.toFixed(2)}km) dépasse la section originale (${originalSectionLen.toFixed(2)}km).`, "error");
                     return;
                 }
             }
         }
         else if (currentMode.value === 'SEGMENT') {
             // Anchor1 ... Waypoints ... Anchor2
             if (activeMod.points.length === 0) {
                 // First Anchor
             } else {
                 // Check if Closing (isSnap = true, second anchor)
                 if (isSnap) {
                      const anchor1 = activeMod.points[0];
                      const anchor2 = newPoint; // This is the closing anchor
                      
                      originalSectionLen = calculateMainTraceSectionLength(anchor1.index, anchor2.index);
                      
                      // For the closure, calculateProjectedVariantLength adds distance from last waypoint to anchor2
                      // This gives total variant length estimate
                      if (projectedVarLen > originalSectionLen) {
                         showSnackbar(`Le segment variante (${projectedVarLen.toFixed(2)}km) est plus long que la trace originale (${originalSectionLen.toFixed(2)}km).`, "error");
                         return;
                     }
                 } else {
                     // Adding Waypoint
                     // We don't know the end anchor yet, so we don't know the Original Length limit.
                     // But strictly speaking, the variant is ALREADY creating a detour between Anchor1 and "Current closest point on trace"?
                     // No, that's too restrictive. We only validate on CLOSURE.
                 }
             }
         }
    }

    activeMod.points.push(newPoint);
    isModified.value = true;

    // Auto-finalize if SEGMENT is complete (2 anchors)
    if (activeMod.type === 'SEGMENT' && activeMod.points.filter(p => p.type === 'ANCHOR').length >= 2) {
        activeMod.finalized = true;
        activeMod.routingLocked = true;
    }

    updateMarkers();
    if (activeMod.points.length >= 2) {
        const modIndex = modifications.value.indexOf(activeMod);
        generatePreviewForMod(modIndex).then(() => {
            if (activeMod && activeMod.finalized) {
                console.log("[AutoSave] Segment auto-finalized, triggering save...");
                triggerAutoSave();
            }
        });
    }
};

const finalizeMod = (modIndex) => {
    const mod = modifications.value[modIndex];
    if (mod) {
        mod.finalized = true;
        mod.routingLocked = true; // Lock settings
        isModified.value = true;
        generatePreviewForMod(modIndex).then(() => {
            triggerAutoSave();
        });
        updateMarkers();
    }
};

const triggerAutoSave = async () => {
    if (isSaving.value || modifications.value.length === 0) return;
    
    // Ensure ID exists before triggering auto-save
    if (!loadedVariantId.value) {
        loadedVariantId.value = `var_${crypto.randomUUID()}`;
        loadedVariantName.value = `Variante ${new Date().toLocaleDateString()} ${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
    }

    console.log("[AutoSave] Triggering for ID:", loadedVariantId.value);
    await confirmSaveVariant(true); // true = silent
};

const handleRenameMod = (modIndex) => {
    const mod = modifications.value[modIndex];
    if (!mod) return;
    
    renameIndex.value = modIndex;
    renameValue.value = mod.name || (mod.type === 'SEGMENT' ? `Segment ${modIndex + 1}` : mod.type);
    showRenameDialog.value = true;
};

const handleUpdateRouting = async (modIndex) => {
    const mod = modifications.value[modIndex];
    if (!mod) return;
    
    // Assign current global config to this specific mod
    mod.routingService = variantConfig.routingService;
    mod.routingProfile = variantConfig.routingProfile;
    
    const label = mod.name || (mod.type === 'SEGMENT' ? `le segment ${modIndex + 1}` : mod.type);
    // showSnackbar(`Mise à jour du profil vers "${mod.routingProfile}" pour ${label}...`, "info");
    
    // Regenerate preview (and altitude if finalized)
    await generatePreviewForMod(modIndex);
    isModified.value = true;
    triggerAutoSave();
};

const handleFlyToMod = (modIndex) => {
    const mod = modifications.value[modIndex];
    if (!mod || !map.value) return;

    let coords = [];
    
    // If we have a calculated geometry/preview, use it for better accuracy
    if (mod.preview && mod.preview.coordinates) {
        coords = mod.preview.coordinates;
    } else {
        // Fallback to raw points if preview not yet generated
        coords = mod.points.map(p => p.coords);
    }

    if (coords.length === 0) return;

    // Use turf to get the bounding box
    // If it's just one point (started but not previewed), we create a fake segment for bbox
    const line = turf.lineString(coords.length === 1 ? [coords[0], coords[0]] : coords);
    const bbox = turf.bbox(line);

    map.value.fitBounds(bbox, {
        padding: 80,
        duration: 2000,
        maxZoom: 16
    });
};

const handleLoadVariant = async (variantId) => {
    isLoading.value = true;
    try {
        const archive = await invoke('get_variant_details', { circuitId: props.circuitId, variantId });
        
        // Clear current work
        modifications.value = [];
        
        // Map back
        for (const [index, rm] of archive.modifications.entries()) {
            let mod = {
                type: '',
                points: [],
                finalized: true,
                routingLocked: true, // Loaded mods are always locked by default
                preview: null,
                name: rm.name || null,
                routingService: rm.routingService || null,
                routingProfile: rm.routingProfile || null,
                routingStatus: rm.routingStatus || null
            };

            console.log(`[LoadVariant] Mod ${index} status:`, rm.routingStatus);

            if (rm.type === 'SEGMENT_DEVIATION') {
                mod.type = 'SEGMENT';
                // Add start anchor
                mod.points.push({
                    coords: rm.anchorStart.coords,
                    index: rm.anchorStart.index,
                    type: 'ANCHOR'
                });
                // Add waypoints
                for (const p of rm.waypoints) {
                    mod.points.push({
                        coords: [p.lon, p.lat],
                        type: 'WAYPOINT'
                    });
                }
                // Add end anchor
                mod.points.push({
                    coords: rm.anchorEnd.coords,
                    index: rm.anchorEnd.index,
                    type: 'ANCHOR'
                });
            } else {
                mod.type = (rm.type === 'DEPART_DEPORTE') ? 'DEPART' : 'ARRIVEE';
                const anchorIndex = rm.anchorIndexOnMaster;
                for (let i = 0; i < rm.points.length; i++) {
                    const p = rm.points[i];
                    mod.points.push({
                        coords: [p.lon, p.lat],
                        index: (i === 0 && p.type === 'anchor') ? anchorIndex : (p.index || null),
                        type: p.type === 'anchor' ? 'ANCHOR' : 'WAYPOINT'
                    });
                }
            }
            
            if (rm.fullGeometry && Array.isArray(rm.fullGeometry)) {
                mod.preview = {
                    type: 'LineString',
                    coordinates: rm.fullGeometry.map(p => [p.lon, p.lat, p.alt || 0])
                };
            }
            
            if (rm.longueur) {
                mod.length = rm.longueur;
            } else if (mod.preview) {
                try {
                    const line = turf.lineString(mod.preview.coordinates);
                    mod.length = turf.length(line, { units: 'kilometers' });
                } catch (e) {
                    console.warn("Could not calc length from preview", e);
                }
            }
            
            modifications.value.push(mod);
        }
        
        // Update map immediately with loaded geometry
        updateMarkers();
        updatePreviewSource();
        
        // Mark as NOT modified since it's just loaded
        loadedVariantId.value = variantId;
        loadedVariantName.value = archive.metadata.name;
        isModified.value = false;
        
        showSnackbar("Variant chargé pour édition.", "success");
        
    } catch (e) {
        console.error("Load failed", e);
        showSnackbar("Erreur lors du chargement: " + e, "error");
    } finally {
        isLoading.value = false;
    }
};

const confirmRename = async () => {
    if (renameIndex.value !== -1 && renameValue.value.trim() !== "") {
        modifications.value[renameIndex.value].name = renameValue.value.trim();
        isModified.value = true;
        
        // Trigger auto-save to persist the name change immediately
        await triggerAutoSave();
    }
    showRenameDialog.value = false;
};

const handleDeletePoint = (modIndex, pIndex) => {
    // If we are deleting an anchor that finalized the segment, un-finalize it
    // Note: pIndex === 1 check assumes the structure [AnchorStart, ...Waypoints, AnchorEnd]
    // But since we splice, we just check if it was finalized.
    // Actually, logic is simpler: if we modify points of a finalized segment, it becomes un-finalized.
    
    modifications.value[modIndex].finalized = false;
    modifications.value[modIndex].points.splice(pIndex, 1);
    
    isModified.value = true;
    if (modifications.value[modIndex].points.length < 2) {
        modifications.value[modIndex].preview = null;
    } else {
        generatePreviewForMod(modIndex);
    }
    updateMarkers();
    updatePreviewSource();
};

const handleDeleteMod = (modIndex) => {
    modifications.value.splice(modIndex, 1);
    isModified.value = true;
    updateMarkers();
    updatePreviewSource();
    triggerAutoSave();
};

const updateMarkers = () => {
    const features = [];
    modifications.value.forEach(mod => {
        mod.points.forEach((p, pIndex) => {
            let type = p.type;
            
            // Specialized pins for Depart/Arrivee when finalized (last point)
            if (mod.finalized && (mod.type === 'DEPART' || mod.type === 'ARRIVEE') && pIndex === mod.points.length - 1) {
                if (mod.type === 'DEPART') type = 'START_PIN';
                else if (mod.type === 'ARRIVEE') type = 'END_PIN';
            } else if (mod.type === 'SEGMENT' && p.type === 'ANCHOR') {
                 // Color coding for Segment Direction
                 if (pIndex === 0) type = 'SEGMENT_START';
                 else if (pIndex > 0) type = 'SEGMENT_END';
            }
            
            features.push({
                type: 'Feature',
                geometry: { type: 'Point', coordinates: p.coords },
                properties: { 
                    type: type, 
                    modType: mod.type,
                    finalized: mod.finalized 
                }
            });
        });
    });
    if (map.value && map.value.getSource('markers-source')) {
        map.value.getSource('markers-source').setData({ type: 'FeatureCollection', features });
    }
};

const generatePreviewForMod = async (modIndex) => {
    const mod = modifications.value[modIndex];
    if (mod.points.length < 2) return;

    isLoading.value = true;
    try {
        let coords = mod.points.map(p => p.coords);
        if (mod.type === 'DEPART') {
            coords = [...mod.points].reverse().map(p => p.coords);
        }

        const routeResultStr = await invoke('calculate_route', {
            service: mod.routingService || variantConfig.routingService,
            profile: mod.routingProfile || variantConfig.routingProfile,
            points: coords
        });
        
        const routeResult = routeResultStr; 
        
        let status = 'SUCCESS';
        let altErrorShown = false;

        if (routeResult.warning) {
             console.log("[Preview] Routing warning:", routeResult.warning);
             // Detect specific types of warnings
             if (routeResult.warning.includes("ALTITUDE_FETCH_ERROR")) {
                 status = 'ALT_FAIL';
                 // On n'affiche le snackbar que si le segment est finalisé (demande utilisateur)
                 if (mod.finalized) {
                    const label = mod.name || (mod.type === 'SEGMENT' ? `le segment ${modIndex + 1}` : mod.type);
                    showSnackbar(`Récupération des altitudes pour le segment <b>${label}</b>, en échec !`, "warning");
                    altErrorShown = true;
                 }
             } else if (routeResult.warning.toLowerCase().includes("direct") || routeResult.warning.toLowerCase().includes("échec du routage")) {
                 status = 'ROUTE_FAIL';
                 showSnackbar(routeResult.warning, "error");
             } else {
                 showSnackbar(routeResult.warning, "warning");
             }
        }
        mod.routingStatus = status;

        mod.preview = JSON.parse(routeResult.geojson);
        
        // --- ZOOM ET MISE À JOUR VISUELLE AVANT ALTITUDES ---
        updatePreviewSource();
        handleFlyToMod(modIndex);

        // --- IMMEDIATELY FETCH ALTITUDES IF FINALIZED ---
        if (mod.finalized && mod.preview && mod.preview.coordinates) {
             try {
                const pointsToFetch = mod.preview.coordinates.map(c => [c[0], c[1]]);
                const altitudes = await invoke('get_altitudes', { points: pointsToFetch });
                
                // Inject altitudes into preview coordinates
                mod.preview.coordinates = mod.preview.coordinates.map((c, i) => [c[0], c[1], altitudes[i] || 0]);
                console.log(`[Preview] Altitudes fetched for finalized mod ${modIndex}`);
                
             } catch (altError) {
                console.warn("Could not fetch altitudes during preview:", altError);
                if (mod.routingStatus !== 'ROUTE_FAIL') {
                    mod.routingStatus = 'ALT_FAIL';
                    if (!altErrorShown) {
                        const label = mod.name || (mod.type === 'SEGMENT' ? `le segment ${modIndex + 1}` : mod.type);
                        showSnackbar(`Récupération des altitudes pour le segment <b>${label}</b>, en échec !`, "warning");
                    }
                }
             }
        }

        // Calculate length
        if (mod.preview && mod.preview.coordinates) {
             try {
                const line = turf.lineString(mod.preview.coordinates);
                mod.length = turf.length(line, { units: 'kilometers' });
             } catch (e) {
                console.warn("Error calculating length:", e);
             }
        }

        updatePreviewSource();

    } catch (e) {
        console.error("Routing error for mod", modIndex, e);
        
        let msg = "Erreur routage : Tracé direct utilisé. (" + e + ")";
        let color = "warning";
        let timeout = 5000;

        // Détection des erreurs de routage restantes (profil incompatible, etc.)
        const errMessage = e.toString();
        if (variantConfig.routingProfile === 'racingbike' && !errMessage.includes("Clé API")) {
             msg += " Essayez le profil 'VTT' ou 'Route + Pistes'.";
             routingErrorProfile.value = 'racingbike';
        } else if (errMessage.includes("Clé API") || errMessage.includes("API key")) {
            msg = "Erreur de routage : Problème persistant avec les clés API.";
            color = "error";
            timeout = 10000;
        }

        showSnackbar(msg, color, timeout);
        
        // Fallback: Create straight line
        const fallbackCoords = mod.points.map(p => p.coords);
         if (mod.type === 'DEPART') {
            fallbackCoords.reverse();
        }
        mod.preview = {
            type: 'LineString',
            coordinates: fallbackCoords
        };
        
        // Calculate length for fallback
        try {
            const line = turf.lineString(fallbackCoords);
            mod.length = turf.length(line, { units: 'kilometers' });
        } catch (e) {}
        updatePreviewSource();
    } finally {
        isLoading.value = false;
      triggerAutoSave(); }
};

const updatePreviewSource = () => {
    const allFeatures = [];
    modifications.value.forEach(mod => {
        if (mod.preview) {
             allFeatures.push({
                 type: 'Feature',
                 geometry: mod.preview,
                 properties: { 
                     type: mod.type,
                     finalized: mod.finalized
                 }
             });
        }
    });
    if (map.value && map.value.getSource('preview-source')) {
        map.value.getSource('preview-source').setData({ type: 'FeatureCollection', features: allFeatures });
    }
};

const generatePreview = async () => {
    // Regenerate all previews? Or just update source?
    for (let i = 0; i < modifications.value.length; i++) {
        await generatePreviewForMod(i);
    }
};

const saveVariant = () => {
    if (modifications.value.length === 0) return;
    if (!isValid.value) {
        showSnackbar("Toutes les modifications doivent être finalisées avant l'enregistrement.", "warning");
        return;
    }
    
    // If it's a loaded variant, we just overwrite (no name prompt)
    if (loadedVariantId.value) {
        confirmSaveVariant();
        return;
    }

    variantName.value = `Variante ${new Date().toLocaleDateString()} ${new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}`;
    showSaveDialog.value = true;
};



const confirmSaveVariant = async (silent = false) => {
    if (isSaving.value) return;

    // Only process fully finalized modifications for the permanent files
    const finalizedModsOnly = modifications.value.filter(m => m.finalized);
    
    // During the very first save (naming), we might not have finalized mods yet, 
    // but we need to save the archive to persist the name.
    if (!silent && finalizedModsOnly.length === 0 && modifications.value.length > 0) {
        // Allow saving just to set the name even if nothing is green yet
    } else if (finalizedModsOnly.length === 0 && !isModified.value) {
        if (!silent) showSnackbar("Aucune modification à enregistrer.", "warning");
        return;
    }

    // Ensure we have an ID and name
    if (!loadedVariantId.value) {
        loadedVariantId.value = `var_${crypto.randomUUID()}`;
    }
    
    // Capture the name from the dialog if it's the first save
    if (!loadedVariantName.value && variantName.value.trim()) {
        loadedVariantName.value = variantName.value.trim();
    }
    
    const finalName = loadedVariantName.value || variantName.value.trim() || `Variante ${new Date().toLocaleDateString()}`;

    showSaveDialog.value = false;
    isSaving.value = true;
    if (!silent) isLoading.value = true;
    
    try {
        const mods = finalizedModsOnly.map(mod => {
            const anchors = mod.points.filter(p => p.type === 'ANCHOR');
            
            // For the archive, we save RAW editing points (clicks), not the full geometry
            const rawPoints = (modType) => {
                return mod.points.map(p => ({
                    lat: p.coords[1],
                    lon: p.coords[0],
                    type: p.type === 'ANCHOR' ? 'anchor' : 'waypoint',
                    index: p.index
                }));
            };

            // Calc length in km (we still use preview if available for accurate distance)
            let longueur = 0;
            let coords = (mod.preview && mod.preview.coordinates) ? mod.preview.coordinates : mod.points.map(p => p.coords);

            console.log(`[SaveVariant] Mod ${mod.type}, points: ${mod.points.length}, coords: ${coords.length}`);
            if (coords && Array.isArray(coords) && coords.length >= 2) {
                try {
                    const line = turf.lineString(coords);
                    longueur = turf.length(line, { units: 'kilometers' });
                } catch (err) {
                    console.error("[SaveVariant] Turf error:", err, "with coords:", JSON.stringify(coords));
                    throw new Error(`Erreur lors du calcul de la distance: ${err.message}`);
                }
            }
            
            // --- PADDING LOGIC (Modulo 100m) ---
            if (mod.type === 'SEGMENT' && longueur > 0) {
                const lenMeters = longueur * 1000;
                const remainder = lenMeters % 100;
                
                // If we have a significant remainder (avoid micro-adjustments for float precision issues close to 0 or 100)
                if (remainder > 0.5 && remainder < 99.5) {
                    let needed = 100 - remainder;
                    
                    const startAnchor = anchors[0];
                    const endAnchor = anchors[anchors.length - 1];
                    const pStart = coords[0];
                    const pEnd = coords[coords.length - 1];
                    
                    // 1. Calculate Gaps (Anchor -> Snapped Point on Trace)
                    const line = masterTraceGeojson.value;
                    
                    const getGapInfo = (anchorCoords) => {
                         const ptAnchor = turf.point(anchorCoords);
                         const snapped = turf.nearestPointOnLine(line, ptAnchor);
                         const gapKm = turf.distance(ptAnchor, snapped, { units: 'kilometers' });
                         return { gapMeters: gapKm * 1000, snappedPoint: snapped };
                    };
                    
                    const startGap = getGapInfo(startAnchor.coords);
                    const endGap = getGapInfo(endAnchor.coords);
                    
                    const minRequired = 2 * (startGap.gapMeters + endGap.gapMeters);
                    
                    // 2. Adjust needed if minRequired > needed
                    // We increase needed by 100m steps until it's enough to cover the mandatory gaps
                    while (needed < minRequired) {
                        needed += 100;
                    }
                    
                    // 3. Distribute remaining budget to trace extensions
                    const budgetForTrace = needed - minRequired;
                    
                    // Split equally between start and end
                    const budgetStart = budgetForTrace / 2.0;
                    const budgetEnd = budgetForTrace / 2.0;
                    
                    // One-way distance on trace
                    const distOnTraceStart = budgetStart / 2.0;
                    const distOnTraceEnd = budgetEnd / 2.0;
                    
                    // Helper to generate extension path
                    const generateExtension = (snappedPt, distMeters, direction, label) => {
                         if (distMeters <= 0.01) return [];
                         
                         const snappedCoords = snappedPt.geometry.coordinates;
                         const projLocationKm = snappedPt.properties.location;
                         const distKm = distMeters / 1000.0;
                         
                         let targetLocationKm = projLocationKm + (distKm * direction);
                         
                         const totalLen = turf.length(line);
                         if (targetLocationKm < 0) targetLocationKm = 0;
                         if (targetLocationKm > totalLen) targetLocationKm = totalLen;
                         
                         const ptTarget = turf.along(line, targetLocationKm);
                         const slice = turf.lineSlice(snappedPt, ptTarget, line);
                         let sliceCoords = slice.geometry.coordinates;
                         
                         
                         // Measure actual slice length
                         const sliceLenKm = turf.length(turf.lineString(sliceCoords), { units: 'kilometers' });
                         const sliceLenMeters = sliceLenKm * 1000;
                         
                         // Ensure slice starts at snappedPt (Anchor) and goes to ptTarget
                         // lineSlice can return [Target...Snapped] if Target is before Snapped index-wise
                         // We check distance from first point to snappedPt vs last point to snappedPt
                         if (sliceCoords.length > 0) {
                             const firstDist = turf.distance(turf.point(sliceCoords[0]), snappedPt);
                             const lastDist = turf.distance(turf.point(sliceCoords[sliceCoords.length-1]), snappedPt);
                             
                             // If last point is closer to anchor than first point, we need to reverse
                             // to have [Snapped -> Target]
                             if (lastDist < firstDist) {
                                 sliceCoords = sliceCoords.reverse();
                             }
                         }
                         
                         // Out + Back
                         const outPath = sliceCoords;
                         const backPath = [...sliceCoords].reverse();
                         const combined = [...outPath, ...backPath.slice(1)];
                         
                         // Measure combined path
                         const combinedLine = turf.lineString(combined);
                         const combinedLenKm = turf.length(combinedLine, { units: 'kilometers' });
                         const combinedLenMeters = combinedLenKm * 1000;
                         
                         // [Proj, ..., Target, ..., Proj]
                         return combined;
                    };
                    
                    const startExtTrace = generateExtension(startGap.snappedPoint, distOnTraceStart, -1, "START");
                    const endExtTrace = generateExtension(endGap.snappedPoint, distOnTraceEnd, 1, "END");
                    
                    // Assemble: P_Start -> [GapStartPath] -> [TraceExtStart] -> [GapStartPathBack] -> P_Start
                    
                    const buildLeg = (pStart, gapInfo, tracePath, label) => {
                         const pProj = gapInfo.snappedPoint.geometry.coordinates;
                         
                         // Determine the leg path for return (stitching)
                         const legPath = (tracePath && tracePath.length > 0) ? tracePath : [pProj];
                         
                         // Measure Actual Added Length for this Leg
                         // Path: pStart -> pProj -> tracePath (out+back, already starts/ends at pProj) -> pProj -> pStart
                         // Since legPath starts and ends at pProj (or is just [pProj]), the full path is:
                         // [pStart, ...legPath, pStart]
                         
                         const fullLegPath = [pStart, ...legPath, pStart];
                         
                         try {
                              const legLine = turf.lineString(fullLegPath);
                              const legLenKm = turf.length(legLine, { units: 'kilometers' });
                              const legLen = legLenKm * 1000;
                              return { path: legPath, len: legLen };
                         } catch(e) { console.error(e); return { path: legPath, len: 0 }; }
                    };
                    
                    const startResult = buildLeg(startAnchor.coords, startGap, startExtTrace, "START");
                    const endResult = buildLeg(endAnchor.coords, endGap, endExtTrace, "END");

                    const startLeg = startResult.path;
                    const endLeg = endResult.path;

                     const middle = coords.slice(1, coords.length - 1);
                     
                     const newCoords = [
                         startAnchor.coords,
                         ...startLeg,
                         startAnchor.coords,
                         ...middle,
                         endAnchor.coords,
                         ...endLeg,
                         endAnchor.coords
                     ];
                     
                     coords = newCoords;
                     
                      // Recalculate length
                     try {
                        const line = turf.lineString(coords);
                        longueur = turf.length(line, { units: 'kilometers' });
                     } catch (e) {
                         console.error("Error recalc length", e);
                     }
                }
            }
            // -----------------------------------

            // Preparation de la géométrie complète (pour les fichiers permanents)
            // Note: Use 'coords' which might have been modified by padding logic
            const fullGeometry = coords.map(c => ({ lat: c[1], lon: c[0], alt: c[2] || 0 }));

            if (mod.type === 'SEGMENT') {
                // For segment, waypoints = intermediate points between anchors
                const waypoints = mod.points
                    .filter(p => p.type !== 'ANCHOR')
                    .map(p => ({ lat: p.coords[1], lon: p.coords[0], type: 'waypoint' }));

                return {
                    type: 'SEGMENT_DEVIATION',
                    anchorStart: { index: anchors[0].index, coords: anchors[0].coords },
                    anchorEnd: { index: anchors[anchors.length-1].index, coords: anchors[anchors.length-1].coords },
                    waypoints: waypoints,
                    fullGeometry: fullGeometry,
                    longueur: longueur,
                    name: mod.name,
                    routingService: mod.routingService,
                    routingProfile: mod.routingProfile, routingStatus: mod.routingStatus
                };
            } else if (mod.type === 'DEPART') {
                 // --- TRIMMING LOGIC FOR DEPART (Modulo 100m) ---
                 if (longueur > 0) {
                     const lenMeters = longueur * 1000;
                     const remainder = lenMeters % 100;
                     
                     // If remainder is significant (e.g. > 0.5m), we trim it from the START
                     if (remainder > 0.5) {
                         console.log(`[SaveVariant] Trimming DEPART segment by ${remainder.toFixed(2)}m to reach modulo 100.`);
                         
                         try {
                             const line = turf.lineString(coords);
                             // Cut point is at 'remainder' distance from start
                             // turf.along takes distance in unit of line (km usually)
                             const trimDistKm = remainder / 1000.0;
                             const newStartPt = turf.along(line, trimDistKm, { units: 'kilometers' });
                             const endPt = turf.point(coords[coords.length - 1]);
                             
                             // Slice from new start to end
                             const sliced = turf.lineSlice(newStartPt, endPt, line);
                             coords = sliced.geometry.coordinates;
                             
                             // Recalculate length
                             longueur = turf.length(turf.lineString(coords), { units: 'kilometers' });
                             console.log(`[SaveVariant] New DEPART length: ${(longueur * 1000).toFixed(2)}m`);
                             
                             // Update fullGeometry with trimmed coords
                             // (Note: we need to re-map fullGeometry because coords changed)
                         } catch(e) {
                             console.error("[SaveVariant] Error trimming DEPART:", e);
                         }
                     }
                 }
                 
                 // Re-generate fullGeometry in case coords changed
                 const trimmedFullGeometry = coords.map(c => ({ lat: c[1], lon: c[0], alt: c[2] || 0 }));

                 // Update the Start Point in the points list to match the new geometry start
                 let finalPoints = rawPoints();
                 if (coords.length > 0 && finalPoints.length > 0) {
                     // For a Departure, we want to update the point that is NOT the anchor
                     // (the one at the trimmed end of the LineString)
                     const startPointIdx = finalPoints.findIndex(p => p.type !== 'anchor');
                     
                     if (startPointIdx !== -1) {
                         finalPoints[startPointIdx].lon = coords[0][0];
                         finalPoints[startPointIdx].lat = coords[0][1];
                     } else {
                         // Fallback to index 0 if no waypoint found
                         finalPoints[0].lon = coords[0][0];
                         finalPoints[0].lat = coords[0][1];
                     }
                 }

                 return {
                    type: 'DEPART_DEPORTE',
                    anchorIndexOnMaster: anchors[0].index,
                    points: finalPoints,
                    fullGeometry: trimmedFullGeometry,
                    longueur: longueur,
                    name: mod.name,
                    routingService: mod.routingService,
                    routingProfile: mod.routingProfile, routingStatus: mod.routingStatus
                };
            } else if (mod.type === 'ARRIVEE') {
                return {
                    type: 'ARRIVEE_REPORTEE',
                    anchorIndexOnMaster: anchors[0].index,
                    points: rawPoints(),
                    fullGeometry: fullGeometry,
                    longueur: longueur,
                    name: mod.name,
                    routingService: mod.routingService,
                    routingProfile: mod.routingProfile,
                    routingStatus: mod.routingStatus
                };
            }
        });

        const metadata = {
            id: loadedVariantId.value || `var_${crypto.randomUUID()}`,
            name: finalName,
            description: `Créée le ${new Date().toLocaleDateString()}`,
            creationDate: new Date().toISOString(),
            color: '#651fff',
            stats: { 
                totalDistance: mods.reduce((sum, m) => sum + (m.longueur || 0), 0),
                totalAscent: 0.0 
            }
        };

        const warning = await invoke('create_variant_files', {
            request: {
                circuitId: props.circuitId,
                metadata: metadata,
                modifications: mods
            }
        });
        
        /* 
        if (warning && !silent) {
            // Uniquement si on a des segments en échec (orange ou rouge)
            const hasIssues = finalizedModsOnly.some(m => m.routingStatus !== 'SUCCESS');
            if (hasIssues) {
                showSnackbar(warning, "warning");
            }
        }
        */
        
        await loadSavedVariants();
        
        if (!silent) {
            showSnackbar("Variante enregistrée.", "success");
        }
        isModified.value = false;
        
    } catch (e) {
        console.error("Save failed", e);
        showSnackbar("Erreur lors de l'enregistrement: " + e, "error");
    } finally {
        isSaving.value = false;
        if (!silent) isLoading.value = false;
    }
};

const loadSavedVariants = async () => {
    try {
        savedVariants.value = await invoke('get_variants', { circuitId: props.circuitId });
    } catch (e) {
        console.error("Failed to load variants", e);
    }
};

const handleDeleteSavedVariant = (variantId, variantName) => {
    variantToDelete.value = { id: variantId, name: variantName };
    showDeleteDialog.value = true;
};

const confirmDeleteSavedVariant = async () => {
    if (!variantToDelete.value) return;
    
    showDeleteDialog.value = false;
    isLoading.value = true;
    try {
        if (loadedVariantId.value === variantToDelete.value.id) {
            resetPoints();
        }

        await invoke('delete_variant', { circuitId: props.circuitId, variantId: variantToDelete.value.id });
        showSnackbar("Variante supprimée.", "success");
        await loadSavedVariants();
    } catch (e) {
        showSnackbar("Erreur lors de la suppression: " + e, "error");
    } finally {
        isLoading.value = false;
        variantToDelete.value = null;
    }
};

const handleRenameSavedVariant = (id, currentName) => {
    variantToRenameId.value = id;
    variantRenameValue.value = currentName;
    showVariantRenameDialog.value = true;
};

const confirmVariantRename = async () => {
    if (!variantToRenameId.value || variantRenameValue.value.trim() === "") return;
    
    try {
        await invoke('rename_variant', {
            circuitId: props.circuitId,
            variantId: variantToRenameId.value,
            newName: variantRenameValue.value.trim()
        });
        
        // If we were editing this specific variant, update the title in sidebar
        if (loadedVariantId.value === variantToRenameId.value) {
            loadedVariantName.value = variantRenameValue.value.trim();
        }
        
        showVariantRenameDialog.value = false;
        await loadSavedVariants();
        showSnackbar("Variante renommée.", "success");
    } catch (e) {
        showSnackbar("Erreur lors du renommage: " + e, "error");
    }
};

const goHome = () => {
  router.push('/');
};

const checkRoutingServices = async () => {
    try {
        const status = await invoke('check_routing_services');
        console.log("[Routing check]", status);
        
        let msg = "";
        let color = "warning";
        let timeout = 8000;

        const gh = status.graphhopper;
        const ors = status.ors;

        if (gh === 'EMPTY' && ors === 'EMPTY') {
            msg = "Erreur : Les clés API GraphHopper et OpenRouteService sont manquantes.";
            color = "error";
            timeout = 10000;
        } else if (gh === 'INVALID' && ors === 'EMPTY') {
            msg = "Erreur : Clé GraphHopper invalide et clé OpenRouteService manquante.";
            color = "error";
            timeout = 10000;
        } else if (gh === 'EMPTY' && ors === 'INVALID') {
            msg = "Erreur : Clé GraphHopper manquante et clé OpenRouteService invalide.";
            color = "error";
            timeout = 10000;
        } else if (gh === 'INVALID' && ors === 'INVALID') {
            msg = "Erreur : Les deux clés API (GraphHopper et OpenRouteService) sont invalides.";
            color = "error";
            timeout = 10000;
        } else if (gh === 'VALID' && ors === 'EMPTY') {
            msg = "Avertissement : OpenRouteService n'a pas de clé API (Pas de backup).";
        } else if (gh === 'VALID' && ors === 'INVALID') {
            msg = "Avertissement : La clé OpenRouteService est invalide (Pas de backup).";
        } else if (gh === 'EMPTY' && ors === 'VALID') {
            msg = "Avertissement : GraphHopper n'a pas de clé API (Sera ignoré).";
        } else if (gh === 'INVALID' && ors === 'VALID') {
            msg = "Avertissement : La clé GraphHopper est invalide (Sera ignoré).";
        }

        if (msg) {
            showSnackbar(msg, color, timeout);
        }
    } catch (e) {
        console.error("Routing check failed", e);
    }
};

onMounted(() => {
  initMap();
  checkRoutingServices();
});

onUnmounted(() => {
    if (map.value) map.value.remove();
});
</script>

<style scoped>
#map-container {
  width: 100%;
  height: 100%;
}
</style>
