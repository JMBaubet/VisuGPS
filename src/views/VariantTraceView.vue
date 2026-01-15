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
        v-model:config="variantConfig"
        :modifications="modifications"
        :saved-variants="savedVariants"
        :can-generate-preview="canGeneratePreview"
        :is-valid="isValid"
        :is-editing="!!loadedVariantId"
        :is-modified="isModified"
        :variant-name="loadedVariantName"
        @update:config="variantConfig = $event"
        @generate="generatePreview"
        @save="saveVariant"
        @delete-point="handleDeletePoint"
        @delete-mod="handleDeleteMod"
        @finalize-mod="finalizeMod"
        @rename-mod="handleRenameMod"
        @flyto-mod="handleFlyToMod"
        @load-variant="handleLoadVariant"
        @delete-saved-variant="handleDeleteSavedVariant"
        @rename-saved-variant="handleRenameSavedVariant"
        @reset="resetPoints"
      />
    </div>

    <!-- Renaming Dialog -->
    <v-dialog v-model="showRenameDialog" max-width="400px">
      <v-card>
        <v-card-title class="bg-primary text-white px-4 py-2">
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
        <v-card-title class="bg-success text-white px-4 py-2">
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
        <v-card-title class="bg-primary text-white px-4 py-2">
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
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import VariantToolbar from '../components/Variant/VariantToolbar.vue';
import VariantSidebar from '../components/Variant/VariantSidebar.vue';
import DocDisplay from '../components/DocDisplay.vue';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';
import * as turf from '@turf/turf';

import { useSettings } from '../composables/useSettings';
import { useVuetifyColors } from '../composables/useVuetifyColors';
import { useSnackbar } from '../composables/useSnackbar';

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

const variantConfig = ref({
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

const isValid = computed(() => {
  return modifications.value.length > 0 && modifications.value.every(m => m.finalized);
});

watch(() => variantConfig.value.routingProfile, () => {
    if (modifications.value.length > 0) {
        generatePreview();
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
    variantConfig.value.routingService = getSettingValue('Variante/routingService') || 'GraphHopper';
    
    // Mapping French labels from settings to technical keys for Toolbar/API
    const profileLabel = getSettingValue('Variante/routingType') || 'Route uniquement';
    const profileMap = {
        'Route + Pistes cyclables': 'car',
        'Route uniquement': 'racingbike',
        'VTT / Chemin': 'bike'
    };
    variantConfig.value.routingProfile = profileMap[profileLabel] || 'racingbike';
    
    console.log(`[Init] Loaded profile: "${profileLabel}" mapped to "${variantConfig.value.routingProfile}"`);
    console.log(`[Init] Using routing service: ${variantConfig.value.routingService}`);

    // Watch for profile changes to reset error state
    watch(() => variantConfig.value.routingProfile, () => {
        routingErrorProfile.value = null;
    });

    mapboxgl.accessToken = token;

    map.value = new mapboxgl.Map({
      container: 'map-container',
      style: getSettingValue('Variante/mapStyle') || 'mapbox://styles/mapbox/outdoors-v12',
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
       const previewColor = resolveColor(getSettingValue('Variante/previewColor'), '#651FFF');
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
                    '#2196F3'             // Blue for segments
                ],
                'line-width': 4,
                'line-dasharray': [
                    'case',
                    ['boolean', ['get', 'finalized'], false],
                    ['literal', [1]],         // Solid
                    ['literal', [2, 1]]       // Dashed
                ]
            }
        });



       const nodeColor = resolveColor(getSettingValue('Variante/nodeColor'), '#FF9800');
       
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

            const zoomVisuNode = getSettingValue('Variante/zoomVisuNode') || 13;

            map.value.addLayer({
                id: 'tracking-layer',
                type: 'circle',
                source: 'tracking-source',
                minzoom: zoomVisuNode,
                paint: {
                    'circle-radius': [
                        'case',
                        ['boolean', ['get', 'pointDeControl'], false],
                        6,
                        4
                    ],
                    'circle-color': [
                        'case',
                        ['boolean', ['get', 'pointDeControl'], false],
                        '#FF9800', // Orange for control points
                        nodeColor  // Default node color
                    ],
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
                'circle-radius': 4,
                'circle-color': [
                    'match',
                    ['get', 'type'],
                    'ANCHOR', '#ffffff',
                    'WAYPOINT', '#FFEB3B', 
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
     
     const originalColor = resolveColor(getSettingValue('Variante/originalColor'), '#BDBDBD');
     
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
                 'line-width': 4
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
    }

    // Create new group if none found for this mode that is active
    if (!activeMod) {
        activeMod = { type: currentMode.value, points: [], preview: null, finalized: false };
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

    activeMod.points.push(newPoint);
    isModified.value = true;

    // Auto-finalize SEGMENT if it has 2 anchors
    if (activeMod.type === 'SEGMENT' && activeMod.points.filter(p => p.type === 'ANCHOR').length >= 2) {
        activeMod.finalized = true;
    }

    updateMarkers();
    if (activeMod.points.length >= 2) {
        const modIndex = modifications.value.indexOf(activeMod);
        generatePreviewForMod(modIndex);
    }
};

const finalizeMod = (modIndex) => {
    const mod = modifications.value[modIndex];
    if (mod) {
        mod.finalized = true;
        isModified.value = true;
        generatePreviewForMod(modIndex);
        updateMarkers();
    }
};

const handleRenameMod = (modIndex) => {
    const mod = modifications.value[modIndex];
    if (!mod) return;
    
    renameIndex.value = modIndex;
    renameValue.value = mod.name || (mod.type === 'SEGMENT' ? `Segment ${modIndex + 1}` : mod.type);
    showRenameDialog.value = true;
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
        for (const rm of archive.modifications) {
            let mod = {
                type: '',
                points: [],
                finalized: true,
                preview: null,
                name: rm.name || null
            };

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
                    coordinates: rm.fullGeometry.map(p => [p.lon, p.lat])
                };
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

const confirmRename = () => {
    if (renameIndex.value !== -1 && renameValue.value.trim() !== "") {
        modifications.value[renameIndex.value].name = renameValue.value.trim();
        isModified.value = true;
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
};

const updateMarkers = () => {
    const features = [];
    modifications.value.forEach(mod => {
        mod.points.forEach((p, pIndex) => {
            let type = p.type;
            
            // Only the LAST point of a finalized DEPART/ARRIVEE becomes a PIN
            if (mod.finalized && pIndex === mod.points.length - 1) {
                if (mod.type === 'DEPART') type = 'START_PIN';
                else if (mod.type === 'ARRIVEE') type = 'END_PIN';
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

        const geojsonStr = await invoke('calculate_route', {
            service: variantConfig.value.routingService,
            profile: variantConfig.value.routingProfile,
            points: coords
        });

        mod.preview = JSON.parse(geojsonStr);
        updatePreviewSource();
    } catch (e) {
        console.error("Routing error for mod", modIndex, e);
        
        let msg = "Erreur routage : Tracé direct utilisé.";
        if (variantConfig.value.routingProfile === 'racingbike') {
             msg += " Essayez le profil 'VTT' ou 'Route + Pistes'.";
             routingErrorProfile.value = 'racingbike';
        }
        showSnackbar(msg, "warning");
        
        // Fallback: Create straight line
        const fallbackCoords = mod.points.map(p => p.coords);
         if (mod.type === 'DEPART') {
            fallbackCoords.reverse();
        }
        mod.preview = {
            type: 'LineString',
            coordinates: fallbackCoords
        };
        updatePreviewSource();
    } finally {
        isLoading.value = false;
    }
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

const confirmSaveVariant = async () => {
    const finalName = loadedVariantId.value ? loadedVariantName.value : variantName.value.trim();
    
    if (!finalName) {
        showSnackbar("Veuillez saisir un nom pour la variante.", "warning");
        return;
    }

    showSaveDialog.value = false;
    isLoading.value = true;
    
    try {
        const mods = modifications.value.map(mod => {
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
            
            // Preparation de la géométrie complète (pour les fichiers permanents)
            const fullGeometry = (mod.preview && mod.preview.coordinates) 
                ? mod.preview.coordinates.map(c => ({ lat: c[1], lon: c[0], alt: c[2] }))
                : mod.points.map(p => ({ lat: p.coords[1], lon: p.coords[0] }));

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
                    name: mod.name
                };
            } else if (mod.type === 'DEPART') {
                return {
                    type: 'DEPART_DEPORTE',
                    anchorIndexOnMaster: anchors[0].index,
                    points: rawPoints(),
                    fullGeometry: fullGeometry,
                    longueur: longueur,
                    name: mod.name
                };
            } else if (mod.type === 'ARRIVEE') {
                return {
                    type: 'ARRIVEE_REPORTEE',
                    anchorIndexOnMaster: anchors[0].index,
                    points: rawPoints(),
                    fullGeometry: fullGeometry,
                    longueur: longueur,
                    name: mod.name
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
        
        if (warning) {
            showSnackbar(warning, "warning");
        } else {
            showSnackbar("Variante sauvegardée !", "success");
        }
        
        resetPoints();
        await loadSavedVariants();
        
        // Reset dirty state after successful save
        isModified.value = false;
        
    } catch (e) {
        console.error("Save failed", e);
        showSnackbar("Erreur lors de la sauvegarde: " + e, "error");
    } finally {
        isLoading.value = false;
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

onMounted(() => {
  initMap();
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
