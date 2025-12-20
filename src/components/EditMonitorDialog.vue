<template>
  <v-dialog :model-value="show" @update:model-value="$emit('update:show', $event)" persistent max-width="600px">
    <v-card v-if="parameter">
      <v-card-title class="d-flex justify-space-between align-center">
        <span class="text-h5" :class="{ 'text-warning': parameter.critique }">{{ parameter.libelle }}</span>
        <v-icon
          v-if="parameter.doc"
          color="info"
          @click="showDocDialog = true"
          title="Afficher la documentation"
        >mdi-book-open-page-variant-outline</v-icon>
      </v-card-title>
      <v-card-subtitle>{{ parameter.description }}</v-card-subtitle>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12">
              <v-row class="ma-0" align="start">
                <v-col cols="12" class="pa-0">
                  <v-label class="text-caption font-weight-light mb-1 text-white">Valeur par défaut :&nbsp;</v-label>
                  <span class="text-caption font-weight-light text-white">Ecran {{ parameter.defaut }}</span>
                </v-col>
                <v-col cols="11" class="pa-0">
                   <v-select
                    label="Ecran"
                    v-model="editableValue"
                    :items="monitors"
                    item-title="title"
                    item-value="index"
                    required
                    autofocus
                    :loading="loading"
                  ></v-select>
                </v-col>
                <v-col cols="1" class="d-flex justify-center mt-3">
                  <v-icon v-if="isModified" @click="revertChanges" title="Annuler les modifications" color="info">mdi-undo</v-icon>
                  <v-icon v-if="hasSurcharge && !isModified" @click="removeSurcharge" title="Supprimer la surcharge" color="info">mdi mdi-format-color-marker-cancel</v-icon>
                </v-col>
              </v-row>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="closeDialog">
          Annuler
        </v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="save" :disabled="!isModified">
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
import { ref, watch, computed, defineProps, defineEmits, onMounted } from 'vue';
import { useSettings } from '@/composables/useSettings';
import DocDisplay from './DocDisplay.vue';
import { invoke } from '@tauri-apps/api/core';

const showDocDialog = ref(false);

const surchargeRemoved = ref(false);

const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show']);

const { updateSetting } = useSettings();

const editableValue = ref(null);
const monitors = ref([]);
const loading = ref(false);

const hasSurcharge = computed(() => props.parameter && props.parameter.surcharge != null && props.parameter.surcharge !== '');
const isModified = computed(() => {
    if (surchargeRemoved.value) return true;
    const currentEffective = props.parameter?.surcharge ?? props.parameter?.defaut;
    return editableValue.value !== currentEffective;
});

const loadMonitors = async () => {
    loading.value = true;
    try {
        const availableMonitors = await invoke('get_available_monitors');
        monitors.value = availableMonitors.map(m => ({
            title: `Ecran ${m.index} : ${m.name} (${m.width}x${m.height})`,
            index: m.index
        }));
    } catch (e) {
        console.error("Failed to load monitors", e);
        // Fallback or error handling
        monitors.value = [];
    } finally {
        loading.value = false;
    }
}

watch(() => props.show, async (isVisible) => {
  if (isVisible && props.parameter) {
    await loadMonitors();
    
    // Determine effective value
    const effectiveValue = props.parameter.surcharge != null ? props.parameter.surcharge : props.parameter.defaut;
    editableValue.value = effectiveValue;
    surchargeRemoved.value = false;
  }
}, { immediate: true });

const revertChanges = () => {
  editableValue.value = props.parameter.surcharge != null ? props.parameter.surcharge : props.parameter.defaut;
  surchargeRemoved.value = false;
};

const removeSurcharge = () => {
  editableValue.value = props.parameter.defaut;
  surchargeRemoved.value = true;
};

const closeDialog = () => {
  emit('update:show', false);
};

const save = async () => {
  try {
    const valueToSave = surchargeRemoved.value ? null : editableValue.value;
    await updateSetting(props.groupPath, props.parameter.identifiant, valueToSave);
    closeDialog();
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du paramètre:", error);
  }
};
</script>
