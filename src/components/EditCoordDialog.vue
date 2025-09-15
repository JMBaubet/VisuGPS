<template>
  <v-dialog :model-value="show" @update:model-value="$emit('update:show', $event)" persistent max-width="800px">
    <v-card v-if="parameter">
      <v-card-title class="d-flex justify-space-between align-center">
        <span class="text-h5" :class="{ 'text-warning': parameter.critique }">{{ parameter.libelle }}</span>
        <v-icon
          v-if="parameter.doc"
          color="info"
          @click="showDocDialog = true"
          title="Afficher la documentation"
        >mdi-book-open-outline</v-icon>
      </v-card-title>
      <v-card-subtitle>{{ parameter.description }}</v-card-subtitle>
      
      <v-card-text class="pa-0">
        <v-alert v-if="initializationError" type="error" variant="tonal" prominent class="ma-4">
          <h5 class="alert-title">Erreur d'initialisation</h5>
          {{ initializationError }}
        </v-alert>
        <div v-else class="map-container">
          <div ref="mapContainer" class="map"></div>
          <div class="crosshair">+</div>
          <div class="coords-display">
            <div v-if="editableCoord">
              <div>Lon: {{ editableCoord.lon.toFixed(6) }}</div>
              <div>Lat: {{ editableCoord.lat.toFixed(6) }}</div>
            </div>
          </div>
        </div>
      </v-card-text>

      <v-card-actions>
        <v-btn
          v-if="hasSurcharge"
          icon="mdi-format-color-marker-cancel"
          variant="text"
          @click="resetToDefault"
          title="Annuler la surcharge"
        ></v-btn>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="closeDialog">
          Annuler
        </v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="save">
          Sauvegarder
        </v-btn>
      </v-card-actions>
    </v-card>

    <v-dialog v-model="showDocDialog" max-width="800px">
      <DocDisplay :doc-path="parameter.doc" @close="showDocDialog = false" />
    </v-dialog>
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed, onMounted, onUnmounted, nextTick, defineProps, defineEmits } from 'vue';
import { useSettings } from '@/composables/useSettings';
import DocDisplay from './DocDisplay.vue';
import mapboxgl from 'mapbox-gl';
import 'mapbox-gl/dist/mapbox-gl.css';

// --- Data Parsing Helpers ---

/**
 * Parses a coordinate string e.g., "[-77.0364,38.8951]" into an object.
 * @param {string} str The coordinate string.
 * @returns {{lon: number, lat: number} | null}
 */
function parseCoordString(str) {
  try {
    const arr = JSON.parse(str);
    if (Array.isArray(arr) && arr.length === 2 && typeof arr[0] === 'number' && typeof arr[1] === 'number') {
      return { lon: arr[0], lat: arr[1] };
    }
  } catch (e) {
    console.error("Error parsing coordinate string:", str, e);
  }
  return null;
}

/**
 * Formats a coordinate object into a string with a fixed number of decimals.
 * @param {{lon: number, lat: number}} coord The coordinate object.
 * @param {number} decimals The number of decimal places.
 * @returns {string}
 */
function formatCoordString(coord, decimals) {
  const lon = coord.lon.toFixed(decimals);
  const lat = coord.lat.toFixed(decimals);
  return `[${lon},${lat}]`;
}


const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show']);

const { getSettingValue } = useSettings();
const { updateSetting } = useSettings();

const showDocDialog = ref(false);
const initializationError = ref(null);

// --- Map State ---
const mapContainer = ref(null);
let map = null;
let surchargeMarker = null;

// --- Component State ---
const defaultCoord = computed(() => parseCoordString(props.parameter.defaut));
const surchargeCoord = computed(() => props.parameter.surcharge ? parseCoordString(props.parameter.surcharge) : null);
const editableCoord = ref(null);

const hasSurcharge = computed(() => props.parameter?.surcharge != null);

const initializeMap = () => {
  if (map || !mapContainer.value || !editableCoord.value) return;

  const mapboxToken = getSettingValue('Système/Tokens/mapbox');
  if (!mapboxToken) {
    initializationError.value = "Le token Mapbox n'est pas configuré. Veuillez le définir dans les paramètres 'Système/Tokens'.";
    return;
  }

  mapboxgl.accessToken = mapboxToken;
  map = new mapboxgl.Map({
    container: mapContainer.value,
    style: 'mapbox://styles/mapbox/streets-v12',
    center: [editableCoord.value.lon, editableCoord.value.lat],
    zoom: 14,
  });

  map.on('load', () => {
    // Ajouter le marqueur pour la valeur par défaut
    if (defaultCoord.value) {
      const popup = new mapboxgl.Popup({ 
        offset: 25, 
        closeButton: false, 
        closeOnClick: false,
        className: 'visugps-popup' // Classe CSS personnalisée
      }).setText('Position par défaut');

      new mapboxgl.Marker({ color: '#2196F3' }) // Couleur bleue (info)
        .setLngLat([defaultCoord.value.lon, defaultCoord.value.lat])
        .setPopup(popup)
        .addTo(map)
        .togglePopup(); // Ouvre le popup par défaut
    }

    // Ajouter le marqueur pour la valeur de surcharge (si elle existe)
    if (surchargeCoord.value) {
      const popup = new mapboxgl.Popup({ 
        offset: 25, 
        closeButton: false, 
        closeOnClick: false,
        className: 'visugps-popup'
      }).setText('Position paramétrée');

      surchargeMarker = new mapboxgl.Marker({ color: 'green' })
        .setLngLat([surchargeCoord.value.lon, surchargeCoord.value.lat])
        .setPopup(popup)
        .addTo(map)
        .togglePopup();
    }
  });

  // Mettre à jour les coordonnées lors du déplacement de la carte
  map.on('move', () => {
    const center = map.getCenter();
    editableCoord.value = { lon: center.lng, lat: center.lat };
  });
};

const destroyMap = () => {
  if (map) {
    map.remove();
    map = null;
  }
};

watch(() => props.show, (isVisible) => {
  initializationError.value = null; // Réinitialiser l'erreur à chaque ouverture
  if (isVisible && props.parameter) {
    const initialValue = props.parameter.surcharge ?? props.parameter.defaut;
    editableCoord.value = parseCoordString(initialValue);

    if (!editableCoord.value) {
      initializationError.value = `La valeur de la coordonnée ("${initialValue}") est invalide ou mal formatée. Le format attendu est "[longitude,latitude]".`;
      return;
    }
    
    // Initialiser la carte une fois que le dialogue est visible et le conteneur est prêt
    nextTick(initializeMap);
  } else {
    destroyMap();
  }
}, { immediate: true });

onUnmounted(() => {
  destroyMap();
});


const closeDialog = () => {
  emit('update:show', false);
};

const save = async () => {
  if (!editableCoord.value) return;

  const decimals = props.parameter.decimales ?? 6; // 6 décimales par défaut
  const valueToSave = formatCoordString(editableCoord.value, decimals);

  // Ne pas sauvegarder si la valeur est la même que la valeur par défaut
  if (valueToSave === props.parameter.defaut) {
    await updateSetting(props.groupPath, props.parameter.identifiant, null);
  } else {
    await updateSetting(props.groupPath, props.parameter.identifiant, valueToSave);
  }
  
  closeDialog();
};

const resetToDefault = () => {
  if (defaultCoord.value) {
    editableCoord.value = { ...defaultCoord.value };
    if (map) {
      map.flyTo({ 
        center: [defaultCoord.value.lon, defaultCoord.value.lat],
        essential: true
      });
    }
    // Supprimer le marqueur de surcharge de la carte et de la mémoire
    if (surchargeMarker) {
      surchargeMarker.remove();
      surchargeMarker = null;
    }
  }
};

</script>

<style scoped>
.map-container {
  position: relative;
  width: 100%;
  height: 500px; /* Hauteur de la carte */
}

.map {
  width: 100%;
  height: 100%;
}

.crosshair {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  font-size: 32px;
  color: red;
  pointer-events: none; /* Pour que les clics passent à travers */
}

.coords-display {
  position: absolute;
  bottom: 10px;
  left: 10px;
  background-color: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 5px 10px;
  border-radius: 4px;
  font-family: monospace;
  pointer-events: none;
}

/* Correction du style de la popup pour les thèmes clair/sombre */
:deep(.visugps-popup .mapboxgl-popup-content) {
  background-color: rgb(var(--v-theme-surface));
  color: rgb(var(--v-theme-on-surface));
  padding: 8px 12px;
  box-shadow: none;
  border-radius: 4px;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
}

:deep(.visugps-popup .mapboxgl-popup-anchor-bottom .mapboxgl-popup-tip) {
  border-top-color: rgb(var(--v-theme-surface));
}

:deep(.visugps-popup .mapboxgl-popup-anchor-top .mapboxgl-popup-tip) {
  border-bottom-color: rgb(var(--v-theme-surface));
}

:deep(.visugps-popup .mapboxgl-popup-anchor-left .mapboxgl-popup-tip) {
  border-right-color: rgb(var(--v-theme-surface));
}

:deep(.visugps-popup .mapboxgl-popup-anchor-right .mapboxgl-popup-tip) {
  border-left-color: rgb(var(--v-theme-surface));
}

</style>
