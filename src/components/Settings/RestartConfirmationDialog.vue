<template>
  <v-dialog v-model="dialog" max-width="400">
    <v-card>
      <v-card-title class="bg-primary text-white px-4 py-2 d-flex align-center">
        <v-icon start icon="mdi-restart"></v-icon>
        {{ title }}
      </v-card-title>
      <v-card-text class="pa-4">{{ message }}</v-card-text>
      <v-card-actions class="pa-4 pt-0">
        <v-spacer></v-spacer>
        <v-btn variant="text" @click="cancel" v-if="showCancelButton">Annuler</v-btn>
        <v-btn color="primary" variant="flat" @click="confirm">Confirmer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
  modelValue: Boolean,
  title: String,
  message: String,
  showCancelButton: { // New prop
    type: Boolean,
    default: true,
  },
});

const emit = defineEmits(['update:modelValue', 'confirmed', 'cancelled']);

const dialog = ref(props.modelValue);

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
});

watch(dialog, (newVal) => {
  emit('update:modelValue', newVal);
});

const confirm = () => {
  emit('confirmed');
  dialog.value = false;
};

const cancel = () => {
  emit('cancelled');
  dialog.value = false;
};
</script>