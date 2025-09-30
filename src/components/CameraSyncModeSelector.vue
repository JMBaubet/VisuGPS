<template>
  <div class="d-flex justify-center align-center">
    <span class="text-subtitle-1 font-weight-bold mr-2">Caméra :</span>
    <div class="d-flex justify-center" style="gap: 8px;">
      <v-btn 
          :variant="model === 'off' ? 'flat' : 'outlined'"
          :color="model === 'off' ? 'primary' : undefined"
          @click="model = 'off'"
          rounded="pill"
          size="small"
          class="flex-grow-1"
      >
          Fixe
      </v-btn>

      <v-btn 
          :variant="model === 'original' ? 'flat' : 'outlined'"
          :color="model === 'original' ? 'primary' : undefined"
          @click="model = 'original'"
          rounded="pill"
          size="small"
          class="flex-grow-1"
      >
          Calculée
      </v-btn>

      <v-btn 
          :variant="model === 'edited' ? 'flat' : 'outlined'"
          :color="model === 'edited' ? 'primary' : undefined"
          @click="model = 'edited'"
          rounded="pill"
          size="small"
          class="flex-grow-1"
      >
          Editée
      </v-btn>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  modelValue: String,
});

const emit = defineEmits(['update:modelValue']);

const model = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value),
});
</script>
