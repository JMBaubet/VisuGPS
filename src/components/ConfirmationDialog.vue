<template>
  <v-dialog v-model="dialog" max-width="500px">
    <v-card>
      <v-card-title :class="`bg-${color} text-white px-4 py-2 d-flex align-center`">
        <v-icon v-if="icon" start :icon="icon"></v-icon>
        {{ title }}
      </v-card-title>
      <v-card-text class="pa-4" v-html="message"></v-card-text>
      <v-card-actions class="pa-4 pt-0">
        <v-spacer></v-spacer>
        <v-btn variant="text" @click="cancel">{{ cancelText }}</v-btn>
        <v-btn :color="color" variant="flat" @click="confirm">{{ confirmText }}</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
  modelValue: Boolean,
  title: { type: String, default: 'Confirmation' },
  message: { type: String, default: 'Êtes-vous sûr ?' },
  confirmText: { type: String, default: 'Confirmer' },
  cancelText: { type: String, default: 'Annuler' },
  color: { type: String, default: 'error' },
  icon: { type: String, default: 'mdi-alert-circle-outline' },
});

const emit = defineEmits(['update:modelValue', 'confirm', 'cancel']);

const dialog = ref(props.modelValue);

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
});

watch(dialog, (newVal) => {
  emit('update:modelValue', newVal);
});

const confirm = () => {
  emit('confirm');
  dialog.value = false;
};

const cancel = () => {
  emit('cancel');
  dialog.value = false;
};
</script>

<style scoped>
</style>
