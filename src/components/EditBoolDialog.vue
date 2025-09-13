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
          <v-row align="center" no-gutters>
            <v-col cols="11">
              <div class="text-caption font-weight-light mb-2">Valeur par défaut : {{ parameter.defaut ? 'Activé' : 'Désactivé' }}</div>
              <v-switch
                v-model="editableValue"
                :label="editableValue ? 'Activé' : 'Désactivé'"
                color="success"
                base-color="error"
                inset
                hide-details
              ></v-switch>
            </v-col>
            <v-col cols="1" class="text-center">
              <v-icon v-if="isModified" @click="revertChanges" title="Annuler les modifications" color="info">mdi-undo</v-icon>
              <v-icon v-if="hasSurcharge && !isModified" @click="removeSurcharge" title="Supprimer la surcharge" color="info">mdi mdi-format-color-marker-cancel</v-icon>
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
const surchargeRemoved = ref(false);

const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show']);

const { updateSetting } = useSettings();

const editableValue = ref(false);

const hasSurcharge = computed(() => props.parameter?.surcharge != null);
const isModified = computed(() => {
  if (surchargeRemoved.value) return true;
  return editableValue.value !== (props.parameter?.surcharge ?? props.parameter?.defaut);
});

watch(() => props.parameter, (param) => {
  if (param) {
    editableValue.value = param.surcharge ?? param.defaut;
    surchargeRemoved.value = false;
  }
}, { immediate: true, deep: true });

const revertChanges = () => {
  editableValue.value = props.parameter.surcharge ?? props.parameter.defaut;
  surchargeRemoved.value = false;
};

const removeSurcharge = () => {
  surchargeRemoved.value = true;
  editableValue.value = props.parameter.defaut;
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
    // TODO: Utiliser un snackbar pour informer l'utilisateur de l'erreur
  }
};
</script>
