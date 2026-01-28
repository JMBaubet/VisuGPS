<template>
  <div v-if="segments && segments.length > 0" class="w-100 mb-4">
    <div class="text-caption text-disabled mb-1">Segments</div>
    <v-slide-group show-arrows>
      <v-slide-group-item
        v-for="(segment, i) in segments"
        :key="i"
        v-slot="{ isSelected, toggle }"
      >
        <v-btn
          class="ma-1"
          rounded
          :color="isSelected ? 'primary' : 'surface-light'"
          size="small"
          @click="selectSegment(segment, i)"
        >
          {{ segment.label || segment.name || `Segment ${i+1}` }}
        </v-btn>
      </v-slide-group-item>
    </v-slide-group>
  </div>
</template>

<script setup>
defineProps({
    segments: { type: Array, default: () => [] }
})

const emit = defineEmits(['select'])

function selectSegment(segment, index) {
    emit('select', { segment, index })
}
</script>
