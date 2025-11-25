<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="600px" persistent>
    <v-card>
      <v-card-title class="headline">
        <v-icon class="mr-2">mdi-map-marker-distance</v-icon>
        Configuration des bornes kilométriques
      </v-card-title>

      <v-card-text>
        <v-container>
          <!-- Afficher les distances -->
          <v-row>
            <v-col cols="12">
              <v-switch
                v-model="config.afficher"
                label="Afficher les bornes kilométriques"
                color="primary"
                hide-details
              ></v-switch>
            </v-col>
          </v-row>

          <v-divider class="my-4"></v-divider>

          <!-- Configuration (disabled if afficher is false) -->
          <v-row>
            <v-col cols="12">
              <v-slider
                v-model="config.intervalle"
                :disabled="!config.afficher"
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
                :disabled="!config.afficher"
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
                :disabled="!config.afficher"
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
            <v-col cols="12">
              <v-select
                v-model="config.couleur"
                :disabled="!config.afficher"
                :items="colorOptions"
                label="Couleur des messages"
                hide-details
              >
                <template v-slot:selection="{ item }">
                  <v-chip :color="item.value" size="small" class="mr-2"></v-chip>
                  {{ item.title }}
                </template>
                <template v-slot:item="{ props, item }">
                  <v-list-item v-bind="props">
                    <template v-slot:prepend>
                      <v-chip :color="item.value" size="small"></v-chip>
                    </template>
                  </v-list-item>
                </template>
              </v-select>
            </v-col>
          </v-row>

          <v-row>
            <v-col cols="12" class="d-flex align-center">
              <span class="mr-4">Orientation :</span>
              <v-btn-toggle
                v-model="orientationToggle"
                :disabled="!config.afficher"
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
});

const emit = defineEmits(['update:modelValue', 'updated']);

const { showSnackbar } = useSnackbar();

// Configuration state
const config = ref({
  afficher: true,
  intervalle: 10,
  preAffichage: 10,
  postAffichage: 10,
  couleur: 'red',
  orientation: 'Droite',
});

const loading = ref(false);

// Color options for Material Design colors
const colorOptions = [
  { title: 'Rouge', value: 'red' },
  { title: 'Bleu', value: 'blue' },
  { title: 'Vert', value: 'green' },
  { title: 'Orange', value: 'orange' },
  { title: 'Violet', value: 'purple' },
  { title: 'Jaune', value: 'yellow' },
  { title: 'Rose', value: 'pink' },
  { title: 'Cyan', value: 'cyan' },
  { title: 'Indigo', value: 'indigo' },
  { title: 'Teal', value: 'teal' },
];

// Orientation toggle (converts between boolean and string)
const orientationToggle = computed({
  get: () => config.value.orientation,
  set: (value) => {
    config.value.orientation = value;
  },
});

// Load default settings when dialog opens
watch(() => props.modelValue, async (isOpen) => {
  if (isOpen) {
    await loadDefaultSettings();
  }
});

// Load default settings from backend
const loadDefaultSettings = async () => {
  try {
    const settings = await invoke('read_settings');
    
    config.value.afficher = settings?.data?.groupes
      ?.find(g => g.libelle === 'Edition')?.groupes
      ?.find(g => g.libelle === 'Distance')?.parametres
      ?.find(p => p.identifiant === 'afficher')?.defaut ?? true;

    config.value.intervalle = settings?.data?.groupes
      ?.find(g => g.libelle === 'Edition')?.groupes
      ?.find(g => g.libelle === 'Distance')?.parametres
      ?.find(p => p.identifiant === 'intervalle')?.defaut ?? 10;

    config.value.preAffichage = settings?.data?.groupes
      ?.find(g => g.libelle === 'Edition')?.groupes
      ?.find(g => g.libelle === 'Distance')?.parametres
      ?.find(p => p.identifiant === 'preAffichage')?.defaut ?? 10;

    config.value.postAffichage = settings?.data?.groupes
      ?.find(g => g.libelle === 'Edition')?.groupes
      ?.find(g => g.libelle === 'Distance')?.parametres
      ?.find(p => p.identifiant === 'postAffichage')?.defaut ?? 10;

    config.value.couleur = settings?.data?.groupes
      ?.find(g => g.libelle === 'Edition')?.groupes
      ?.find(g => g.libelle === 'Distance')?.parametres
      ?.find(p => p.identifiant === 'couleur')?.defaut ?? 'red';

    const orientationDefault = settings?.data?.groupes
      ?.find(g => g.libelle === 'Edition')?.groupes
      ?.find(g => g.libelle === 'Distance')?.parametres
      ?.find(p => p.identifiant === 'orientation')?.defaut ?? true;
    
    config.value.orientation = orientationDefault ? 'Droite' : 'Gauche';
  } catch (error) {
    console.error('Failed to load default settings:', error);
    showSnackbar('Erreur lors du chargement des paramètres par défaut', 'error');
  }
};

// Apply configuration
const apply = async () => {
  loading.value = true;
  try {
    if (config.value.afficher) {
      // Generate distance markers
      await invoke('generate_distance_markers', {
        circuitId: props.circuitId,
        config: {
          afficher: config.value.afficher,
          intervalle: config.value.intervalle,
          preAffichage: config.value.preAffichage,
          postAffichage: config.value.postAffichage,
          couleur: config.value.couleur,
          orientation: config.value.orientation,
        },
        totalDistanceKm: props.totalDistanceKm,
      });
      showSnackbar('Bornes kilométriques générées avec succès', 'success');
    } else {
      // Remove distance markers
      await invoke('remove_distance_markers', {
        circuitId: props.circuitId,
      });
      showSnackbar('Bornes kilométriques supprimées', 'info');
    }

    emit('updated');
    emit('update:modelValue', false);
  } catch (error) {
    console.error('Failed to apply distance markers configuration:', error);
    showSnackbar(`Erreur : ${error}`, 'error');
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
