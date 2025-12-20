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
              <v-row class="ma-0" align="center">
                <v-col cols="12" class="pa-0 mb-2">
                  <v-label class="text-caption font-weight-light mb-1 text-white">Valeur par défaut :&nbsp;</v-label>
                  <span class="text-caption font-weight-light text-white">{{ parameter.defaut }}</span>
                </v-col>
                
                <v-col cols="10" class="pa-0">
                  <v-text-field
                    label="Dossier sélectionné"
                    v-model="editableValue"
                    readonly
                    hide-details
                    variant="outlined"
                    density="compact"
                  ></v-text-field>
                </v-col>
                <v-col cols="2" class="d-flex justify-center align-center">
                   <v-btn icon="mdi-folder-open" variant="text" @click="selectDirectory" title="Choisir un dossier"></v-btn>
                </v-col>

                <v-col cols="12" class="d-flex justify-end mt-3 px-0">
                  <v-icon v-if="isModified" @click="revertChanges" title="Annuler les modifications" color="info" class="mr-2">mdi-undo</v-icon>
                  <v-icon v-if="hasSurcharge && !isModified" @click="removeSurcharge" title="Revenir à la valeur par défaut" color="info">mdi-format-color-marker-cancel</v-icon>
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
        <v-btn color="blue-darken-1" variant="text" @click="save" :disabled="!isModified && !isRemoval">
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
import { open } from '@tauri-apps/plugin-dialog';
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
// Flag to indicate if we are in a state where we just want to remove the surcharge
const isRemoval = ref(false);

const hasSurcharge = computed(() => props.parameter.surcharge != null && props.parameter.surcharge !== '');
const isModified = computed(() => editableValue.value !== initialValue.value);

watch(() => props.show, (isVisible) => {
  if (isVisible && props.parameter) {
    // If surcharge exists, show it. Otherwise show default.
    // However, logic in settings usually implies: if surcharge is null, we use default.
    // Here we want to edit the *surcharge*.
    const value = props.parameter.surcharge != null ? props.parameter.surcharge : props.parameter.defaut;
    editableValue.value = value;
    initialValue.value = value;
    isRemoval.value = false;
  }
}, { immediate: true });

const selectDirectory = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: editableValue.value || undefined,
      });
      
      if (selected) {
        editableValue.value = selected;
        isRemoval.value = false;
      }
    } catch (error) {
      console.error("Erreur lors de la sélection du dossier:", error);
    }
};

const revertChanges = () => {
  editableValue.value = initialValue.value;
  isRemoval.value = false;
};

const removeSurcharge = () => {
   // To remove surcharge, we effectively want to save 'null'.
   // But for display in this dialog, we might want to show the default value pending save.
   editableValue.value = props.parameter.defaut;
   isRemoval.value = true;
};

const closeDialog = () => {
  emit('update:show', false);
};

const save = async () => {
  try {
    let valueToSave;
    if (isRemoval.value) {
        valueToSave = null;
    } else {
        valueToSave = editableValue.value;
        // If the value matches the default, we could logically save null, 
        // but explicit saving is usually safer unless "removeSurcharge" was explicitly called.
        // However, standard behavior: if value == default, it's effectively same as no surcharge?
        // Let's stick to: if isRemoval is true => null. Else => value.
        // Wait, if users selects the default folder manually, it becomes a surcharge equal to default.
    }

    await updateSetting(props.groupPath, props.parameter.identifiant, valueToSave);
    closeDialog();
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du paramètre:", error);
  }
};
</script>
