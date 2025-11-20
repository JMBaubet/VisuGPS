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
        >mdi-book-open-outline</v-icon>
      </v-card-title>
      <v-card-subtitle>{{ parameter.description }}</v-card-subtitle>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12">
              <v-row class="ma-0" align="start">
                <v-col cols="12" class="pa-0">
                  <v-label class="text-caption font-weight-light mb-1 text-white">Valeur par défault :&nbsp</v-label>
                  <span class="text-caption font-weight-light text-white">{{ parameter.defaut === '' ? '""' : '******' }}</span>
                </v-col>
                <v-col cols="11" class="pa-0">
                  <v-text-field
                    label="Valeur"
                    v-model="editableValue"
                    :type="showPassword ? 'text' : 'password'"
                    :append-inner-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
                    @click:append-inner="showPassword = !showPassword"
                    required
                    autofocus
                  ></v-text-field>
                  <v-row v-if="parameter.min || parameter.max" class="align-center">
                    <v-col cols="4" class="py-0 text-left">
                      <span v-if="parameter.min" class="text-white text-caption font-weight-light">Long. min : {{ parameter.min || 0 }}</span>
                    </v-col>
                    <v-col cols="4" class="py-0 text-center">
                      <span v-if="currentLength > 0" :class="currentLengthColor" class="text-caption font-weight-light">Long. actuelle : {{ currentLength }}</span>
                    </v-col>
                    <v-col cols="4" class="py-0 text-right">
                      <span v-if="parameter.max" class="text-white text-caption font-weight-light">Long. max : {{ parameter.max || '∞' }}</span>
                    </v-col>
                  </v-row>
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
import { ref, watch, computed, defineProps, defineEmits } from 'vue';
import { useSettings } from '@/composables/useSettings';
import DocDisplay from './DocDisplay.vue';

const showDocDialog = ref(false);
const showPassword = ref(false);

const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show']);

const { updateSetting } = useSettings();

const editableValue = ref('');
const initialValue = ref('');

const hasSurcharge = computed(() => props.parameter.surcharge != null && props.parameter.surcharge !== '');
const isModified = computed(() => editableValue.value !== initialValue.value);

const currentLength = computed(() => editableValue.value.length);

const currentLengthColor = computed(() => {
  const len = currentLength.value;
  const min = props.parameter.min;
  const max = props.parameter.max;

  if (len === 0) return 'text-info';

  const isWithinMin = min === undefined || len >= min;
  const isWithinMax = max === undefined || len <= max;

  return (isWithinMin && isWithinMax) ? 'text-success' : 'text-error';
});

watch(() => props.show, (isVisible) => {
  if (isVisible && props.parameter) {
    const value = props.parameter.surcharge != null ? props.parameter.surcharge : '';
    editableValue.value = value;
    initialValue.value = value;
    showPassword.value = false; // Reset visibility on open
  }
}, { immediate: true });

const revertChanges = () => {
  editableValue.value = initialValue.value;
};

const removeSurcharge = () => {
  editableValue.value = '';
};

const closeDialog = () => {
  emit('update:show', false);
};

const save = async () => {
  try {
    const valueToSave = editableValue.value === '' ? null : editableValue.value;
    await updateSetting(props.groupPath, props.parameter.identifiant, valueToSave);
    closeDialog();
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du paramètre:", error);
  }
};
</script>
