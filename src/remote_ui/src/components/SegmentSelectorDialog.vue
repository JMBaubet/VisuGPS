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
            :class="{ 'opacity-60': segment.segmentType === 'COMMON' && i !== currentIndex }"
            :link="segment.segmentType !== 'COMMON'"
          >
            <template v-slot:prepend>
              <v-icon 
                :icon="getIcon(segment.segmentType, segment.name)" 
                :color="getIconColor(segment.segmentType, segment.name, i)" 
                size="28"
                class="mr-4"
              ></v-icon>
            </template>
            
            <v-list-item-title 
              class="font-weight-bold"
              :class="{ 'text-white': i === currentIndex, 'text-grey': i !== currentIndex }"
            >
              {{ segment.name }}
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
  currentIndex: { type: Number, default: -1 },
  isDark: Boolean
})

const emit = defineEmits(['update:modelValue', 'select'])

const modelValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

function selectSegment(segment, index) {
  if (segment.segmentType === 'COMMON') return; // Disable click
  emit('select', { segment, index })
  modelValue.value = false
}

function getIcon(type, name) {
  if (name === 'Vue Finale' || type === 'FIN') return 'mdi-clock-end'
  switch (type) {
    case 'DEPART': return 'mdi-ray-start-arrow'
    case 'ARRIVEE': return 'mdi-ray-end-arrow'
    case 'COMMON': return 'mdi-link-variant'
    default: return 'mdi-map-marker-path'
  }
}

function getIconColor(type, name, index) {
  if (name === 'Vue Finale' || type === 'FIN') return 'grey'
  
  // Active Logic: Bright if active, defaults to Dark Logic
  const isActive = (index === props.currentIndex);
  
  if (isActive) {
      // ACTIVE: All White (user request for uniformity & visibility)
      return 'white';
  } else {
      // INACTIVE: Dark Colors
      if (type === 'COMMON') return 'grey-darken-2'; 
      switch (type) {
        case 'DEPART': return 'green-darken-4'
        case 'ARRIVEE': return 'red-darken-4'
        default: return 'blue-darken-4'
      }
  }
}

function formatType(type) {
  switch (type) {
    case 'DEPART': return 'Départ'
    case 'ARRIVEE': return 'Arrivée'
    case 'COMMON': return 'Tronçon Commun'
    default: return 'Segment de variante'
  }
}
</script>
