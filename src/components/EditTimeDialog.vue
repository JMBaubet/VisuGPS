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
                  <v-label class="text-caption font-weight-light mb-1 text-white">Valeur par défault :&nbsp</v-label>
                  <span class="text-caption font-weight-light text-white">{{ parameter.defaut === '' ? '""' : parameter.defaut }}</span>
                </v-col>
                <v-col cols="11" class="pa-0">
                  <div class="d-flex justify-center pa-4">
                      <input 
                          type="time" 
                          v-model="editableValue"
                          style="font-size: 1.5rem; padding: 10px; border: 1px solid #ccc; border-radius: 4px; background: white; color: black; width: 100%;"
                      />
                  </div>
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

watch(() => props.show, (isVisible) => {
  if (isVisible && props.parameter) {
    const surcharge = props.parameter.surcharge;
    const defaut = props.parameter.defaut;
    // Use surcharge if exists, otherwise default. If both missing (shouldn't happen), empty string.
    // Note: The original logic in EditStringDialog used empty string if surcharge was null.
    // Here we want to edit specific surcharge if present, OR create a new one based on default.
    // But usually for "override" logic, if there is no surcharge, we show empty or the default?
    // EditStringDialog: `value = props.parameter.surcharge != null ? props.parameter.surcharge : '';` 
    // This implies if no surcharge, we start with empty? No, we likely want to start with the current effective value which is default if surcharge is null?
    // Actually, `EditStringDialog` initializes to empty string if no surcharge. 
    // But for a boolean or int, usually we show the current value.
    // Let's check `EditIntDialog`.
    
    // In `EditIntDialog` (assumed), we usually pre-fill with `surcharge ?? defaut`.
    // Let's assume we want to pre-fill with the CURRENT effective value to allow modification.
    // Or do we leave it empty to indicate "no override"?
    // A time input cannot be easily "empty" in some browsers, it defaults to --:--.
    
    // Let's stick to: use surcharge if present, otherwise use default to initialize the input,
    // BUT we need to know if we are creating a surcharge or editing one.
    // If we rely on `isModified`, we need `initialValue` to be the `surcharge` (or empty/null if none).
    
    // If I look at `EditStringDialog` again:
    // `const value = props.parameter.surcharge != null ? props.parameter.surcharge : '';`
    // It seems it forces the user to re-type if they want to overload? That seems annoying.
    // Let's assume standard behavior: load `surcharge` if exists, else load `defaut`.
    
    const value = props.parameter.surcharge != null ? props.parameter.surcharge : props.parameter.defaut;
    editableValue.value = value;
    initialValue.value = value;
  }
}, { immediate: true });

const revertChanges = () => {
  editableValue.value = initialValue.value;
};

const removeSurcharge = () => {
  // To remove surcharge, we should set it to null. 
  // But here we are editing the value that will be saved as surcharge.
  // If we want to simulate "removing", we might want to set `editableValue` to `defaut` 
  // AND ensure `save` handles it (e.g. if new value == default, maybe treat as no surcharge? 
  // OR explicitly handle remove).
  // The `EditStringDialog` has a direct `removeSurcharge` logic on the icon that calls this function?
  // Wait, `removeSurcharge` in `EditStringDialog` sets `editableValue = ''`.
  // And `save` converts `''` to `null`.
  
  // For time, "empty" might be weird. Let's just set it to default value? 
  // Or if we want to explicitly remove the surcharge key from JSON, we need to pass `null` to `updateSetting`.
  // Here we just update the UI state.
  editableValue.value = props.parameter.defaut;
};

const closeDialog = () => {
  emit('update:show', false);
};

const save = async () => {
  try {
    let valueToSave = editableValue.value;
    // If the value matches the default, we treat it as removing the surcharge (null)
    if (valueToSave === props.parameter.defaut) {
        valueToSave = null;
    }
    await updateSetting(props.groupPath, props.parameter.identifiant, valueToSave);
    closeDialog();
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du paramètre:", error);
  }
};
</script>

<style scoped>
/* Scoped styles */
</style>
