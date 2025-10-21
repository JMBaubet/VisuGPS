<template>
  <v-card>
    <v-card-title class="headline d-flex justify-space-between align-center">
      {{ circuit.nom }}
      <v-btn icon @click="closeDialog">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-card-title>
    <v-card-text>
      <v-row>
        <v-col cols="12">
          <v-list density="compact">
            <v-row no-gutters>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Distance :</strong> {{ circuit.distanceKm.toFixed(2) }} km
                  </v-list-item-title>
                </v-list-item>
              </v-col>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Ville de départ :</strong> {{ communeNom }}
                  </v-list-item-title>
                </v-list-item>
              </v-col>
            </v-row>
            <v-row no-gutters>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Dénivelé positif :</strong> {{ circuit.deniveleM }} m
                  </v-list-item-title>
                </v-list-item>
              </v-col>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Point le plus haut :</strong> {{ circuit.sommet.altitudeM }} m (à {{ circuit.sommet.km.toFixed(2) }} km)
                  </v-list-item-title>
                </v-list-item>
              </v-col>
            </v-row>
            <v-row no-gutters>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Édition caméra :</strong> <span :style="{ color: trackingProgressColor }">{{ trackingProgress.toFixed(2) }} % ({{ circuit.trackingKm.toFixed(2) }} km)</span>
                  </v-list-item-title>
                </v-list-item>
              </v-col>
              <v-col cols="6">
                <v-list-item>
                  <v-list-item-title>
                    <strong>Recherche Communes :</strong> <span :style="{ color: communeProgressColor }">{{ communeProgress.toFixed(2) }} %</span>
                  </v-list-item-title>
                </v-list-item>
              </v-col>
            </v-row>
            </v-list>
            <div class="d-flex flex-row align-start " style="width: 100%; max-width: 500px;">
              <span class="text-subtitle-1 mr-2 align-self-center" style="white-space: nowrap; margin-bottom: 20px; padding-left: 16px;">Traceur : </span>
              <v-combobox
                v-model="editedTraceur"
                :items="traceurNames"
                variant="outlined"
                density="compact"
                class="w-100 mr-2"
              ></v-combobox>
              <div class="d-flex align-center">
                <v-btn
                  color="primary"
                  :disabled="editedTraceur === props.circuit.traceur"
                  @click="saveTraceur"
                  style="margin-top: 2px;"
                >Sauvegarder</v-btn>
              </div>
            </div>
            <v-row class="mt-4">
              <v-col cols="12" md="6" class="d-flex justify-center">
                <v-img
                  v-if="qrCodePath"
                  :src="qrCodePath"
                  alt="QR Code du circuit"
                  contain
                  max-height="200px"
                  max-width="200px"
                ></v-img>
                <span v-else>QR Code non disponible</span>
              </v-col>
              <v-col cols="12" md="6">
                <v-list density="compact">
                  <v-list-item>
                    <v-list-item-title>
                      <strong>Éditeur :</strong> {{ circuit.editeur }}
                    </v-list-item-title>
                  </v-list-item>
                  <v-list-item v-if="circuit.url">
                    <v-list-item-title>
                      <strong>URL :</strong> <a :href="circuit.url" target="_blank">{{ circuit.url }}</a>
                    </v-list-item-title>
                  </v-list-item>
                </v-list>
              </v-col>
            </v-row>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue';
import { useEnvironment } from '@/composables/useEnvironment';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';

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

const emit = defineEmits(['close', 'update-circuit']);

const { appEnvPath } = useEnvironment();
const { showSnackbar } = useSnackbar();

const communeNom = ref('');
const qrCodePath = ref('');
const editedTraceur = ref(props.circuit.traceur);

const trackingProgress = computed(() => {
  if (props.circuit.distanceKm === 0) return 0;
  return (props.circuit.trackingKm / props.circuit.distanceKm) * 100;
});

const trackingProgressColor = computed(() => {
  const progress = trackingProgress.value;
  if (progress === 0) return 'red';
  if (progress === 100) return 'green';
  return 'orange';
});

const communeProgress = computed(() => {
  return props.circuit.avancementCommunes;
});

const communeProgressColor = computed(() => {
  const progress = communeProgress.value;
  if (progress === 0) return 'purple';
  if (progress < 7) return 'red';
  if (progress < 13) return 'orange';
  if (progress < 25) return 'yellow';
  if (progress < 50) return 'blue';
  if (progress < 100) return 'teal';
  return 'green';
});

const traceurNames = computed(() => {
  return props.allTraceurs.map(t => t.nom);
});

const getCommuneNom = () => {
  console.log('props.circuit.villeDepartId:', props.circuit.villeDepartId);
  console.log('props.allCommunes:', props.allCommunes);
  const commune = props.allCommunes.find(c => c.id === props.circuit.villeDepartId);
  console.log('Found commune:', commune);
  communeNom.value = commune ? commune.nom : 'Inconnu';
};

console.log('InformationCircuit - props.circuit:', props.circuit);
console.log('InformationCircuit - denivelePositif:', props.circuit.denivelePositif);
console.log('InformationCircuit - sommet:', props.circuit.sommet);

const getQrCodePath = async () => {
  if (props.circuit.circuitId) {
    try {
      qrCodePath.value = await invoke('get_qrcode_as_base64', { circuitId: props.circuit.circuitId });
    } catch (error) {
      console.error('Failed to load QR Code:', error);
      qrCodePath.value = '';
    }
  }
};

const saveTraceur = async () => {
  const newTraceurName = editedTraceur.value;
  if (newTraceurName === props.circuit.traceur) return;

  let traceurId = props.allTraceurs.find(t => t.nom === newTraceurName)?.id;

  if (!traceurId) {
    // New traceur, create a new ID
    const newId = `tr-${(props.allTraceurs.length + 1).toString().padStart(4, '0')}`;
    traceurId = newId;
    showSnackbar('Nouveau traceur ajouté : ' + newTraceurName, 'info');
  }

  try {
    await invoke('update_circuit_traceur', {
      circuitId: props.circuit.circuitId,
      newTraceur: newTraceurName,
      newTraceurId: traceurId,
    });
    emit('update-circuit', { ...props.circuit, traceur: newTraceurName, traceurId: traceurId });
    showSnackbar('Traceur mis à jour avec succès !', 'success');
  } catch (error) {
    showSnackbar('Erreur lors de la mise à jour du traceur : ' + error, 'error');
    editedTraceur.value = props.circuit.traceur; // Revert on error
  }
};

const closeDialog = () => {
  emit('close');
};

onMounted(() => {
  getCommuneNom();
  getQrCodePath();
});

watch(() => props.circuit, () => {
  getCommuneNom();
  getQrCodePath();
  editedTraceur.value = props.circuit.traceur;
}, { deep: true });

watch(appEnvPath, () => {
  getQrCodePath();
});
</script>

<style scoped>
.v-card-title {
  background-color: rgb(var(--v-theme-primary));
  color: white;
}
</style>
