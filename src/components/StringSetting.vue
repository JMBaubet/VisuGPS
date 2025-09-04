<template>
  <div class="setting-row d-flex align-center">
    <!-- Label Column -->
    <div class="setting-label-container mr-4">
      <v-tooltip location="top" max-width="400px">
        <template v-slot:activator="{ props: tooltipProps }">
          <v-label v-bind="tooltipProps">{{ modelValue.nom }}</v-label>
        </template>
        <div><b>{{ modelValue.nom }}</b></div>
        <div class="text-caption">{{ modelValue.arbre }}</div>
        <v-divider class="my-2"></v-divider>
        <div>{{ modelValue.description }}</div>
        <div v-if="modelValue.valeur_min != null || modelValue.valeur_max != null" class="mt-2 pt-2 border-t">
          <span v-if="modelValue.valeur_min != null">Min: <b>{{ modelValue.valeur_min }}</b></span>
          <span v-if="modelValue.valeur_max != null" class="ml-2">Longueur maximale de la chaine : <b>{{ modelValue.valeur_max }}</b></span>
        </div>
      </v-tooltip>
    </div>

    <!-- Editor Column -->
    <div class="d-flex flex-grow-1 align-center">
       <v-tooltip location="top">
        <template v-slot:activator="{ props: tooltipProps }">
          <v-icon v-bind="tooltipProps" :color="modelValue.critique ? 'warning' : 'grey-darken-1'" class="mr-2">
            {{ modelValue.critique ? 'mdi-alert-circle' : 'mdi-cog' }}
          </v-icon>
        </template>
        <span v-if="modelValue.critique"><b><u>Attention:</u></b> paramètre critique</span>
        <span v-else>Paramètre standard</span>
      </v-tooltip>

      <!-- Container for documentation icon -->
      <div class="reset-button-container mr-2">
        <v-btn
          v-if="modelValue.documentation"
          icon="mdi-book-open-page-variant-outline"
          variant="text"
          size="small"
          color="blue"
          @click="openDocumentation"
        >
          <v-icon>mdi-book-open-page-variant-outline</v-icon>
          <v-tooltip activator="parent" location="top">Ouvrir la documentation</v-tooltip>
        </v-btn>
      </div>

      <!-- Fixed-width container for Reset Button -->
      <div class="reset-button-container mr-2">
        <v-btn
          v-show="modelValue.valeur_de_surcharge !== null"
          icon="mdi-backup-restore"
          variant="text"
          size="small"
          @click="resetToDefault"
        >
          <v-icon>mdi-backup-restore</v-icon>
          <v-tooltip activator="parent" location="top">Réinitialiser à la valeur par défaut</v-tooltip>
        </v-btn>
      </div>

      <v-text-field
        v-model="editableValue"
        :placeholder="String(modelValue.valeur_par_defaut)"
        :counter="typeof modelValue.valeur_max === 'number' ? modelValue.valeur_max : undefined"
        :maxlength="modelValue.valeur_max"
        variant="outlined"
        dense
        hide-details="auto"
        class="flex-grow-1"
        :rules="lengthRules"
      ></v-text-field>

      <div class="ml-2 d-flex align-center" style="width: 40px; height: 40px;">
        <!-- Undo Change Button -->
        <v-btn v-if="isDirty" icon="mdi-undo" variant="text" size="small" @click="resetValue">
          <v-icon>mdi-undo</v-icon>
          <v-tooltip activator="parent" location="top">Annuler la modification</v-tooltip>
        </v-btn>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { openUrl } from '@tauri-apps/plugin-opener';

const props = defineProps({
  modelValue: {
    type: Object,
    required: true,
  },
});

const emit = defineEmits(['update:modelValue']);

// Store the initial value on component setup. This will only be re-evaluated if the component is re-created (e.g. with a new :key)
const initialValue = props.modelValue.valeur_de_surcharge;

// This is the writable computed property that powers the v-model
const editableValue = computed({
  get() {
    return props.modelValue.valeur_de_surcharge;
  },
  set(newValue) {
    // When the value is changed, emit an update to the parent
    emit('update:modelValue', { ...props.modelValue, valeur_de_surcharge: newValue });
  }
});

const isDirty = computed(() => {
  return props.modelValue.valeur_de_surcharge !== initialValue;
});

const resetValue = () => {
  // This triggers the 'set' on the computed property, which emits the update
  editableValue.value = initialValue; 
};

const resetToDefault = () => {
  // Set the override value to null, which makes the app use the default value
  editableValue.value = null;
};

const openDocumentation = async () => {
  if (props.modelValue.documentation) {
    await openUrl(props.modelValue.documentation);
  }
};

const lengthRules = computed(() => {
  const rules = [];
  if (typeof props.modelValue.valeur_max === 'number') {
    rules.push(
      v => (v || '').length <= props.modelValue.valeur_max ||
      `La longueur maximale est de ${props.modelValue.valeur_max} caractères.`
    );
  }
  return rules;
});
</script>

<style scoped>
.setting-label-container {
  width: 250px; /* Approx 32 chars, provides alignment */
  flex-shrink: 0;
  text-align: right;
  font-weight: 500;
}
.v-label {
  cursor: help;
}
.reset-button-container {
  width: 40px; /* Fixed width for the button slot */
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>

