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
          <v-form v-model="isValid">
            <div class="text-caption font-weight-light mb-2">Valeur par défaut : {{ parameter.defaut ?? '(non définie)' }} {{ parameter.unite || '' }}</div>
            <v-row align="center" no-gutters>
              <v-col cols="11">
                <v-text-field
                  label="Valeur"
                  v-model="editableValue"
                  type="text"
                  :rules="validationRules"
                  :suffix="parameter.unite"
                  autofocus
                  autocomplete="off"
                  hide-details="auto"
                ></v-text-field>
                <div class="text-caption font-weight-light mt-2">Valeur actuelle : {{ parameter.surcharge ?? parameter.defaut }} {{ parameter.unite || '' }}</div>
              </v-col>
              <v-col cols="1" class="text-center">
                <v-icon v-if="isModified" @click="revertChanges" title="Annuler les modifications" color="info">mdi-undo</v-icon>
                <v-icon v-if="hasSurcharge && !isModified" @click="removeSurcharge" title="Supprimer la surcharge" color="info">mdi mdi-format-color-marker-cancel</v-icon>
              </v-col>
            </v-row>
            <v-slider
              v-if="parameter.min != null && parameter.max != null"
              v-model="sliderValue"
              :min="parameter.min"
              :max="parameter.max"
              :step="parameter.step || 0.01"
              thumb-label
              class="mt-4"
              color="primary"
            ></v-slider>
          </v-form>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="closeDialog">
          Annuler
        </v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="save" :disabled="!isModified || !isValid">
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
import { ref, watch, computed, defineProps, defineEmits, nextTick } from 'vue';
import { useSettings } from '@/composables/useSettings';
import DocDisplay from './DocDisplay.vue';

const showDocDialog = ref(false);
const isValid = ref(true);
const surchargeRemoved = ref(false);

const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show']);

const { updateSetting } = useSettings();

const editableValue = ref(null);

const hasSurcharge = computed(() => props.parameter?.surcharge != null);
const isModified = computed(() => {
  const initial = props.parameter.surcharge ?? null;
  let current = null;
  if (editableValue.value !== null && editableValue.value !== '') {
    const parsed = parseFloat(editableValue.value);
    if (!isNaN(parsed)) {
      current = parsed;
    }
  }
  return initial !== current;
});

const sliderValue = computed({
  get: () => {
    const val = parseFloat(editableValue.value);
    return isNaN(val) ? (props.parameter.min || 0) : val;
  },
  set: (val) => {
    // On conserve le format chaîne pour le champ texte
    editableValue.value = String(val);
  }
});

watch(editableValue, (newValue, oldValue) => {
  if (newValue === null || newValue === undefined || newValue === '') return;

  let correctedValue = String(newValue);

  // Remplacer la virgule par un point
  if (correctedValue.includes(',')) {
    correctedValue = correctedValue.replace(',', '.');
  }

  // S'assurer qu'il n'y a qu'un seul point
  const parts = correctedValue.split('.');
  if (parts.length > 2) {
    correctedValue = `${parts[0]}.${parts.slice(1).join('')}`;
  }

  // Limiter le nombre de décimales
  const decimals = props.parameter.decimales;
  if (decimals != null && parts.length > 1 && parts[1].length > decimals) {
    const num = parseFloat(correctedValue);
    if (!isNaN(num)) {
        correctedValue = num.toFixed(decimals);
    }
  }

  // Mettre à jour le modèle si la valeur a été corrigée
  if (correctedValue !== String(newValue)) {
    nextTick(() => {
      editableValue.value = correctedValue;
    });
  }
});

const validationRules = computed(() => [
  v => {
    if (v === null || v === '') return true; // Champ vide est valide
    // Regex pour valider un nombre flottant (avec point comme séparateur)
    if (!/^-?\d*\.?\d*$/.test(v)) return 'Format invalide.';
    const val = parseFloat(v);
    if (isNaN(val)) return 'Doit être un nombre.';

    const min = props.parameter.min;
    if (min != null && val < min) return `La valeur doit être supérieure ou égale à ${min}.`;

    const max = props.parameter.max;
    if (max != null && val > max) return `La valeur doit être inférieure ou égale à ${max}.`;

    return true;
  }
]);

watch(() => props.parameter, (param) => {
  if (param) {
    const initialValue = param.surcharge ?? null;
    editableValue.value = initialValue !== null ? String(initialValue) : null;
    surchargeRemoved.value = false;
    isValid.value = true;
  }
}, { immediate: true, deep: true });

const revertChanges = () => {
  const initialValue = props.parameter.surcharge ?? null;
  editableValue.value = initialValue !== null ? String(initialValue) : null;
  surchargeRemoved.value = false;
};

const removeSurcharge = () => {
  editableValue.value = null;
  surchargeRemoved.value = true;
};

const closeDialog = () => {
  emit('update:show', false);
};

const save = async () => {
  if (!isValid.value) return;

  try {
    let valueToSave = null;
    if (editableValue.value !== null && editableValue.value !== '') {
      const numericValue = parseFloat(editableValue.value);
      if (!isNaN(numericValue)) {
        // Ne pas sauvegarder si la valeur est la même que la valeur par défaut
        if (numericValue !== props.parameter.defaut) {
          valueToSave = numericValue;
        }
      }
    }

    await updateSetting(props.groupPath, props.parameter.identifiant, valueToSave);
    closeDialog();
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du paramètre:", error);
    // TODO: Utiliser un snackbar pour informer l'utilisateur de l'erreur
  }
};
</script>

<style scoped>
:deep(.v-text-field__suffix) {
  margin-left: 8px;
}
</style>
