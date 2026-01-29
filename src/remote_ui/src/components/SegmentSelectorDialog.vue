<template>
  <v-dialog v-model="modelValue" max-height="80vh" scrollable>
    <v-card :theme="isDark ? 'dark' : 'light'" rounded="xl">
      <v-card-title class="d-flex align-center py-4 px-6">
        <v-icon icon="mdi-map-marker-radius-outline" class="mr-3" color="primary"></v-icon>
        <span class="text-h6 font-weight-bold">Navigation Segments</span>
        <v-spacer></v-spacer>
        <v-btn icon="mdi-close" variant="text" @click="modelValue = false"></v-btn>
      </v-card-title>
      
      <v-divider></v-divider>
      
      <v-card-text class="pa-2">
        <v-list class="bg-transparent">
          <v-list-item
            v-for="(segment, i) in segments"
            :key="i"
            @click="selectSegment(segment, i)"
            class="rounded-lg mb-1"
            link
          >
            <template v-slot:prepend>
              <v-icon 
                :icon="getIcon(segment.segmentType, segment.name)" 
                :color="getIconColor(segment.segmentType, segment.name)" 
                size="28"
                class="mr-4"
              ></v-icon>
            </template>
            
            <v-list-item-title class="font-weight-bold">
              {{ segment.name || `Segment ${i+1}` }}
            </v-list-item-title>
          </v-list-item>
        </v-list>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  modelValue: Boolean,
  segments: { type: Array, default: () => [] },
  isDark: Boolean
})

const emit = defineEmits(['update:modelValue', 'select'])

const modelValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

function selectSegment(segment, index) {
  emit('select', { segment, index })
  modelValue.value = false
}

function getIcon(type, name) {
  if (name === 'Vue Finale' || type === 'FIN') return 'mdi-clock-end'
  switch (type) {
    case 'DEPART': return 'mdi-ray-start-arrow'
    case 'ARRIVEE': return 'mdi-ray-end-arrow'
    default: return 'mdi-map-marker-path'
  }
}

function getIconColor(type, name) {
  if (name === 'Vue Finale' || type === 'FIN') return 'grey-darken-1'
  switch (type) {
    case 'DEPART': return 'success'
    case 'ARRIVEE': return 'error'
    default: return 'primary'
  }
}

function formatType(type) {
  switch (type) {
    case 'DEPART': return 'Départ'
    case 'ARRIVEE': return 'Arrivée'
    default: return 'Segment de variante'
  }
}
</script>
