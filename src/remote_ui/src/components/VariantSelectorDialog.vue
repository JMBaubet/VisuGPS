<template>
  <v-dialog v-model="show" max-width="500px">
    <v-card :theme="isDark ? 'dark' : 'light'">
      <v-card-title class="text-h5 pa-4 text-primary font-weight-bold">
        Choisir une Variante
      </v-card-title>
      <v-divider></v-divider>
      <v-list class="pa-0">
        <v-list-item
          v-for="variant in variants"
          :key="variant.id"
          @click="selectVariant(variant.id)"
          class="rounded-lg mb-1"
          link
        >
          <template v-slot:prepend>
            <v-icon icon="mdi-map-marker-path" color="primary"></v-icon>
          </template>
          <v-list-item-title class="text-h6">{{ variant.name }}</v-list-item-title>
        </v-list-item>
      </v-list>
      <v-divider></v-divider>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="error" variant="text" @click="show = false">Fermer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  modelValue: Boolean,
  variants: {
    type: Array,
    default: () => []
  },
  isDark: Boolean
})

const emit = defineEmits(['update:modelValue', 'select'])

const show = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

function selectVariant(id) {
  emit('select', id)
  show.value = false
}
</script>
