<template>
  <v-dialog :model-value="show" @update:model-value="$emit('update:show', $event)" persistent max-width="600px">
    <v-card v-if="parameter">
      <v-card-title>
        <span class="text-h5" :class="{ 'text-warning': parameter.critique }">Modifier: {{ parameter.libelle }}</span>
      </v-card-title>
      <v-card-subtitle>{{ parameter.description }}</v-card-subtitle>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12">
              <v-row class="ma-0" align="start">
                <v-col cols="11" class="pa-0">
                  <v-text-field
                    label="Valeur"
                    v-model="editableValue"
                    :hint="`Défaut: ${parameter.defaut}`"
                    persistent-hint
                    required
                    autofocus
                  ></v-text-field>
                </v-col>
                <v-col cols="1" class="d-flex justify-center mt-3">
                  <v-icon v-if="isModified" @click="revertChanges" title="Annuler les modifications">mdi-undo</v-icon>
                  <v-icon v-if="hasSurcharge && !isModified" @click="removeSurcharge" title="Supprimer la surcharge">mdi-eraser</v-icon>
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
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed, defineProps, defineEmits } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show', 'saved']);

const editableValue = ref('');
const initialValue = ref('');

const hasSurcharge = computed(() => props.parameter.surcharge != null && props.parameter.surcharge !== '');
const isModified = computed(() => editableValue.value !== initialValue.value);

watch(() => props.show, (isVisible) => {
  if (isVisible && props.parameter) {
    const value = props.parameter.surcharge != null ? props.parameter.surcharge : '';
    editableValue.value = value;
    initialValue.value = value;
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
    // Si la valeur est une chaîne vide, on envoie `null` pour supprimer la surcharge.
    const valueToSave = editableValue.value === '' ? null : editableValue.value;
    await invoke('update_setting', {
      groupPath: props.groupPath,
      paramId: props.parameter.identifiant,
      newValue: valueToSave,
    });
    emit('saved');
    closeDialog();
  } catch (error) {
    console.error("Erreur lors de la mise à jour du paramètre:", error);
    // TODO: Utiliser un snackbar pour informer l'utilisateur
  }
};
</script>
