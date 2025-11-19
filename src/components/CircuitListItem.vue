<template>
  <v-list-item :value="circuit.circuitId" class="py-2">
    <v-row align="center" class="w-100">
      <!-- Colonne 1: Nom, Départ -->
      <v-col cols="12" md="4">
        <div class="font-weight-bold">{{ circuit.nom }}</div>
        <div>
          <span class="text-caption">Départ : {{ circuit.villeDepart }}</span>
        </div>
      </v-col>

      <!-- Colonne 2: Distance, Dénivelé, Sommet -->
      <v-col cols="12" md="3">
        <div class="font-weight-bold">
          Distance : {{ circuit.distanceKm }} km
          <span class="mx-1">|</span>
          Dénivelé : {{ circuit.deniveleM }} m
        </div>
        <div v-if="circuit.sommet">
          <span class="text-caption">Sommet : {{ circuit.sommet.altitudeM }} m à {{ circuit.sommet.km }} km</span>
        </div>
      </v-col>

      <!-- Colonne 3: Jauge et Traceur -->
      <v-col cols="12" md="3" class="text-md-right">
        <div class="d-flex flex-row align-center justify-end">
          <span class="text-caption mr-4">Par : {{ circuit.traceur }}</span>
          <div class="d-flex flex-row" style="width: 150px;">
            <div class="w-100" v-if="showTrackingProgress">
              <div class="text-caption text-center">% d'édition</div>
              <v-progress-linear
                :model-value="trackingProgress"
                :bg-color="trackingBgColor"
                :color="trackingProgressColor"
                height="8"
                rounded
              ></v-progress-linear>
            </div>
          </div>
        </div>
      </v-col>

      <!-- Colonne 4: Actions -->
      <v-col cols="12" md="2" class="d-flex justify-end align-center">
        <v-btn icon="mdi-bug" variant="text" v-if="isDev" @click.stop="debugCircuit" color="warning"></v-btn>
        
        <v-btn v-if="communeProgress < 100" icon="mdi-city" variant="text" @click.stop="updateCommunes" :disabled="majCommuneIsRunning" :color="communeIconColor"></v-btn>

        <v-menu open-on-hover location="start">
          <template v-slot:activator="{ props: menuProps }">
            <v-btn
              icon="mdi-information"
              variant="text"
              v-bind="menuProps"
              @click.stop="showInfoDialog = true"
              :color="informationIconColor"
            ></v-btn>
          </template>
          <v-card>
            <template v-if="vignetteUrl">
              <v-img
                :src="vignetteUrl"
                :key="vignetteUrl"
                width="400"
                aspect-ratio="16/9"
                cover
              >
                <template v-slot:placeholder>
                  <div class="d-flex align-center justify-center fill-height">
                    <v-progress-circular
                      color="grey-lighten-4"
                      indeterminate
                    ></v-progress-circular>
                  </div>
                </template>
              </v-img>
            </template>
            <template v-else>
              <div class="d-flex align-center justify-center fill-height" style="width: 400px; height: 225px; background-color: #f0f0f0;">
                <v-progress-circular
                  color="grey-lighten-4"
                  indeterminate
                ></v-progress-circular>
              </div>
            </template>
          </v-card>
        </v-menu>

        <v-dialog v-model="showInfoDialog" max-width="800">
          <InformationCircuit
            :circuit="circuit"
            :all-communes="allCommunes"
            :all-traceurs="allTraceurs"
            @close="showInfoDialog = false"
            @update-circuit="handleCircuitUpdate"
          />
        </v-dialog>

        <v-btn :color="editButtonColor" icon="mdi-pencil" variant="text" @click.stop="editTracking"></v-btn>
        <v-btn 
          :color="view3DButtonColor" 
          icon="mdi-eye" 
          variant="text" 
          @click.stop="view3D"
          :disabled="isView3dDisabled"
          :title="isView3dDisabled ? 'Service Mapbox non disponible (vérifiez le token ou la connexion)' : 'Visualiser le circuit en 3D'">
        </v-btn>
        <v-btn icon="mdi-delete" variant="text" @click.stop="deleteCircuit" color="error"></v-btn>
      </v-col>
    </v-row>
  </v-list-item>

  <ConfirmationDialog
    v-model="showConfirmDialog"
    title="Confirmation de suppression"
    :message="`Êtes-vous sûr de vouloir supprimer le circuit '${props.circuit.nom}' ? Cette action est irréversible.`"
    confirm-text="Supprimer"
    cancel-text="Annuler"
    @confirm="proceedDeletion"
    @cancel="showConfirmDialog = false"
  />
</template>

<script setup>
import { ref, computed, onMounted, watch, defineEmits } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { join } from '@tauri-apps/api/path';
import { useSnackbar } from '@/composables/useSnackbar';
import { useEnvironment } from '@/composables/useEnvironment';
import { useSettings } from '@/composables/useSettings';
import { useCommunesUpdate } from '@/composables/useCommunesUpdate';
import { useCommuneColor } from '@/composables/useCommuneColor';
import { useServiceStatus } from '@/composables/useServiceStatus';
import ConfirmationDialog from '@/components/ConfirmationDialog.vue';
import InformationCircuit from '@/components/InformationCircuit.vue';

const props = defineProps({
  circuit: {
    type: Object,
    required: true,
  },
  allCommunes: {
    type: Array,
    default: () => [],
  },
  allTraceurs: {
    type: Array,
    default: () => [],
  },
});

const emit = defineEmits(['circuit-deleted', 'circuit-updated']);

const isDev = ref(import.meta.env.DEV);
const router = useRouter();
const { showSnackbar } = useSnackbar();
const { appEnvPath } = useEnvironment();
const { getSettingValue } = useSettings();
const { majCommuneIsRunning, circuitsProgress, startUpdate, updatingCircuitId } = useCommunesUpdate();
const { serviceStatus } = useServiceStatus();

const showConfirmDialog = ref(false);
const showInfoDialog = ref(false);
const vignetteUrl = ref('');

const isView3dDisabled = computed(() => {
  return serviceStatus.value !== 'connected';
});

const getVignetteUrl = async () => {
  if (props.circuit.circuitId) {
    try {
      vignetteUrl.value = await invoke('get_thumbnail_as_base64', { circuitId: props.circuit.circuitId });
    } catch (error) {
      console.error('Failed to load thumbnail:', error);
      vignetteUrl.value = ''; // Clear on error
    }
  }
};

const communeProgress = computed(() => {
  return circuitsProgress.value[props.circuit.circuitId] !== undefined
    ? circuitsProgress.value[props.circuit.circuitId]
    : props.circuit.avancementCommunes;
});

const { color: communeIconColor } = useCommuneColor(communeProgress);

const showTrackingProgress = computed(() => {
  if (!props.circuit.distanceKm || props.circuit.distanceKm === 0) {
    return false;
  }
  // Show only if progress is > 0 and < 100
  return props.circuit.trackingKm > 0 && props.circuit.trackingKm < props.circuit.distanceKm;
});

const trackingProgress = computed(() => {
  if (!props.circuit.distanceKm || props.circuit.distanceKm === 0) {
    return 0;
  }
  return (props.circuit.trackingKm / props.circuit.distanceKm) * 100;
});

const trackingBgColor = computed(() => {
  return getSettingValue('Edition/Mapbox/Trace/couleur') || 'grey-lighten-2';
});

const trackingProgressColor = computed(() => {
  return getSettingValue('Edition/Mapbox/Trace/couleurAvancement') || 'primary';
});

const editButtonColor = computed(() => {
  if (props.circuit.trackingKm === 0) return 'red-darken-2';
  if (props.circuit.trackingKm === props.circuit.distanceKm) return 'primary';
  return trackingProgressColor.value;
});

const view3DButtonColor = computed(() => {
  if (props.circuit.trackingKm === 0) return 'error';
  if (props.circuit.trackingKm === props.circuit.distanceKm) return 'success';
  return 'warning';
});

const informationIconColor = computed(() => {
  return props.circuit.hasErrors ? 'error' : undefined;
});

const debugCircuit = () => {
  router.push({ name: 'DebugTracking', params: { circuitId: props.circuit.circuitId } });
};

const updateCommunes = () => {
  startUpdate(props.circuit.circuitId);
};

const editTracking = () => {
  router.push({ name: 'EditView', params: { circuitId: props.circuit.circuitId } });
};

const view3D = () => {
  router.push({ name: 'Visualize', params: { circuitId: props.circuit.circuitId } });
};

const deleteCircuit = async () => {
  showConfirmDialog.value = true;
};

const proceedDeletion = async () => {
  try {
    await invoke('delete_circuit', { circuitId: props.circuit.circuitId });
    showSnackbar('Circuit supprimé avec succès.', 'success');
    emit('circuit-deleted');
  } catch (error) {
    showSnackbar(`Erreur lors de la suppression du circuit : ${error}`, 'error');
    console.error('Error deleting circuit:', error);
  }
};

const handleCircuitUpdate = (updatedCircuit) => {
  emit('circuit-updated', updatedCircuit);
};

onMounted(() => {
  getVignetteUrl();
});

watch(appEnvPath, () => {
  getVignetteUrl();
});
</script>

<style scoped>
.v-list-item {
  border-bottom: 1px solid rgba(var(--v-theme-on-surface), 0.12);
}
</style>
