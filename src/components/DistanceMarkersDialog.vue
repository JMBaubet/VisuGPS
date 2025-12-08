<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="600px" persistent>
    <v-card>
      <v-card-title class="headline">
        <v-icon class="mr-2">mdi-map-marker-distance</v-icon>
        Configuration des bornes kilométriques
      </v-card-title>

      <v-card-text>
        <v-container>
          <!-- Configuration -->
          <v-row>
            <v-col cols="12">
              <v-slider
                v-model="config.intervalle"
                
                :label="`Intervalle : ${config.intervalle} km`"
                :min="1"
                :max="20"
                :step="1"
                thumb-label
                hide-details
              ></v-slider>
            </v-col>
          </v-row>

          <v-row>
            <v-col cols="12">
              <v-slider
                v-model="config.preAffichage"
                
                :label="`Pré-affichage : ${config.preAffichage} incréments`"
                :min="0"
                :max="20"
                :step="1"
                thumb-label
                hide-details
              ></v-slider>
            </v-col>
          </v-row>

          <v-row>
            <v-col cols="12">
              <v-slider
                v-model="config.postAffichage"
                
                :label="`Post-affichage : ${config.postAffichage} incréments`"
                :min="0"
                :max="20"
                :step="1"
                thumb-label
                hide-details
              ></v-slider>
            </v-col>
          </v-row>

          <v-row>
            <v-col cols="12" class="d-flex align-center">
              <span class="mr-4">Orientation :</span>
              <v-btn-toggle
                v-model="orientationToggle"
                
                color="primary"
                mandatory
              >
                <v-btn value="Gauche">
                  <v-icon>mdi-arrow-left</v-icon>
                  Gauche
                </v-btn>
                <v-btn value="Droite">
                  <v-icon>mdi-arrow-right</v-icon>
                  Droite
                </v-btn>
              </v-btn-toggle>
            </v-col>
          </v-row>

          <v-row>
            <v-col cols="12">
               <v-label class="text-caption font-weight-light mb-1">Couleur des messages :</v-label>
               <v-color-picker
                v-model="selectedColorHex"
                mode="hex"
                :swatches="materialSwatches"
                hide-alpha
                hide-eye-dropper
                show-swatches
                hide-inputs
                class="mx-auto"
              ></v-color-picker>
              <div v-if="selectedColorName" class="mt-2 text-center text-caption">
                Couleur sélectionnée : {{ selectedColorName }}
              </div>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="grey" variant="text" @click="cancel">
          Annuler
        </v-btn>
        <v-btn color="primary" variant="text" @click="apply" :loading="loading">
          Appliquer
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import { useVuetifyColors } from '@/composables/useVuetifyColors';

const props = defineProps({
  modelValue: Boolean,
  circuitId: {
    type: String,
    required: true,
  },
  totalDistanceKm: {
    type: Number,
    required: true,
  },
  initialConfig: {
    type: Object,
    default: null,
  },
});

const emit = defineEmits(['update:modelValue', 'updated']);

const { showSnackbar } = useSnackbar();
const { toHex, toName, swatches: materialSwatches } = useVuetifyColors();

// Configuration state
const config = ref({
  intervalle: 10,
  preAffichage: 10,
  postAffichage: 10,
  orientation: 'Droite',
});

// Color state
const selectedColorHex = ref('#F44336'); // Default red

const selectedColorName = computed(() => {
  return toName(selectedColorHex.value);
});

const loading = ref(false);

// Orientation toggle (converts between boolean and string)
const orientationToggle = computed({
  get: () => config.value.orientation,
  set: (value) => {
    config.value.orientation = value;
  },
});

// Load settings when dialog opens
watch(() => props.modelValue, async (isOpen) => {
  if (isOpen) {
    await loadSettings();
  }
});

// Load settings from backend or initial config
const loadSettings = async () => {
  try {
    if (props.initialConfig) {
      // Use existing config if provided
      config.value.intervalle = props.initialConfig.intervalle;
      config.value.preAffichage = props.initialConfig.preAffichage;
      config.value.postAffichage = props.initialConfig.postAffichage;
      config.value.orientation = props.initialConfig.orientation;
      // Try to load color if available, default to red (or fetch default if we want to be precise)
      // Since initialConfig comes from backend which stores "couleur" string name
       if (props.initialConfig.couleur) {
          selectedColorHex.value = toHex(props.initialConfig.couleur);
       }
    } else {
      // Load default settings from backend using the dedicated command
      const defaults = await invoke('get_distance_markers_defaults');
      config.value.intervalle = defaults.intervalle;
      config.value.preAffichage = defaults.preAffichage;
      config.value.postAffichage = defaults.postAffichage;
      config.value.orientation = defaults.orientation;
      if (defaults.couleur) {
        selectedColorHex.value = toHex(defaults.couleur);
      }
    }
  } catch (error) {
    console.error('Failed to load settings:', error);
    showSnackbar('Erreur lors du chargement des paramètres', 'error');
  }
};

// Apply configuration
const apply = async () => {
  loading.value = true;
  try {
    // Generate distance markers (no more 'afficher' switch in dialog)
    await invoke('generate_distance_markers', {
      circuitId: props.circuitId,
      config: {
        intervalle: config.value.intervalle,
        preAffichage: config.value.preAffichage,
        postAffichage: config.value.postAffichage,
        orientation: config.value.orientation,
        couleur: selectedColorName.value, // Pass the color name
      },
      totalDistanceKm: props.totalDistanceKm,
    });
    showSnackbar('Bornes kilométriques configurées avec succès', 'success');
    
    emit('updated');
    emit('update:modelValue', false);
  } catch (error) {
    console.error('Failed to apply distance markers configuration:', error);
    showSnackbar(`Erreur lors de l'application de la configuration : ${error}`, 'error');
  } finally {
    loading.value = false;
  }
};

// Cancel and close dialog
const cancel = () => {
  emit('update:modelValue', false);
};
</script>

<style scoped>
.v-card-title {
  background-color: rgb(var(--v-theme-primary));
  color: white;
}
</style>
