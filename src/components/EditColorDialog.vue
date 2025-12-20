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
              <v-row class="ma-0" align="center">
                <v-col cols="12" class="pa-0">
                  <v-label class="text-caption font-weight-light mb-1 text-white">Valeur par défaut :&nbsp;</v-label>
                  <span class="text-caption font-weight-light text-white">{{ parameter.defaut }}</span>
                </v-col>
                <v-col cols="11" class="pa-0">
                  <v-color-picker
                    v-model="editableValue"
                    :mode="pickerMode"
                    :swatches="parameter.materialDesign ? (parameter.materialDesignStrict ? baseSwatches : materialSwatches) : undefined"
                    :hide-alpha="parameter.materialDesign"
                    :hide-eye-dropper="parameter.materialDesign"
                    :show-swatches="parameter.materialDesign"
                    :hide-inputs="parameter.materialDesign"
                    :hide-canvas="parameter.materialDesignStrict"
                    :hide-sliders="parameter.materialDesignStrict"
                    class="mx-auto"
                  ></v-color-picker>
                  <div v-if="displayedValue" class="mt-2">
                    <v-row align="center" class="ma-0">
                      <v-col cols="auto" class="pa-0">
                        <v-label class="text-caption font-weight-light text-white">Couleur sélectionnée :&nbsp;</v-label>
                        <span class="text-caption font-weight-light">{{ displayedValue }}</span>
                      </v-col>
                      <v-spacer></v-spacer>
                      <v-col cols="auto" class="pa-0">
                        <v-icon v-if="isModified" @click="revertChanges" title="Annuler les modifications" color="info" class="mr-2">mdi-undo</v-icon>
                        <v-icon v-if="hasSurcharge && !isModified" @click="removeSurcharge" title="Supprimer la surcharge" color="info">mdi-format-color-marker-cancel</v-icon>
                      </v-col>
                    </v-row>
                  </div>
                </v-col>
                <v-col cols="1" class="d-flex justify-center">
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
import { ref, watch, computed, defineProps, defineEmits } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import DocDisplay from './DocDisplay.vue';

const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show']);

const { updateSetting } = useSettings();
const { toHex, toName, toBaseName, swatches: materialSwatches, baseSwatches } = useVuetifyColors();

const showDocDialog = ref(false);
const editableValue = ref(null);
const initialValue = ref(null);

const hasSurcharge = computed(() => props.parameter.surcharge != null);
const isModified = computed(() => editableValue.value !== initialValue.value);

const pickerMode = computed(() => {
  return props.parameter?.materialDesign ? 'hex' : 'hexa';
});

const selectedColorName = computed(() => {
  if (props.parameter?.materialDesign && editableValue.value) {
    return props.parameter.materialDesignStrict ? toBaseName(editableValue.value) : toName(editableValue.value);
  }
  return null;
});

const displayedValue = computed(() => {
  if (!editableValue.value) return null;
  return props.parameter?.materialDesign ? selectedColorName.value : editableValue.value;
});

watch(() => props.show, (isVisible) => {
  if (isVisible && props.parameter) {
    const value = props.parameter.surcharge != null ? props.parameter.surcharge : props.parameter.defaut;
    const hexValue = props.parameter.materialDesign ? toHex(value) : value;
    editableValue.value = hexValue;
    initialValue.value = hexValue;
  }
}, { immediate: true });

const revertChanges = () => {
  editableValue.value = initialValue.value;
};

const removeSurcharge = () => {
  const defaultValue = props.parameter.defaut;
  editableValue.value = props.parameter.materialDesign ? toHex(defaultValue) : defaultValue;
};

const closeDialog = () => {
  emit('update:show', false);
};

const save = async () => {
  try {
    const finalValue = props.parameter.materialDesign
      ? (props.parameter.materialDesignStrict ? toBaseName(editableValue.value) : toName(editableValue.value))
      : editableValue.value;

    // Si la valeur finale est la même que la valeur par défaut, on enregistre null pour supprimer la surcharge
    const valueToSave = (finalValue === props.parameter.defaut) ? null : finalValue;

    await updateSetting(props.groupPath, props.parameter.identifiant, valueToSave);
    closeDialog();
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du paramètre:", error);
    // TODO: Utiliser un snackbar pour informer l'utilisateur de l'erreur
  }
};
</script>
