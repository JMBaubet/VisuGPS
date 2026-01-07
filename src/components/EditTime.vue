<template>
  <v-text-field
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    :label="label"
    type="time"
    :step="step"
    prepend-inner-icon="mdi-clock-outline"
    density="compact"
    variant="outlined"
    hide-details
    @keydown.up.prevent="adjustTime(5)"
    @keydown.down.prevent="adjustTime(-5)"
  ></v-text-field>
</template>

<script setup>
const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
  label: {
    type: String,
    default: 'Heure',
  },
  step: {
    type: [String, Number],
    default: "60"
  }
});

const emit = defineEmits(['update:modelValue']);

const adjustTime = (deltaMinutes) => {
    if (!props.modelValue) return;
    
    // Parse current 'HH:MM' or 'HH:MM:SS'
    let parts = props.modelValue.split(':');
    if (parts.length < 2) return;
    
    let h = parseInt(parts[0], 10);
    let m = parseInt(parts[1], 10);
    
    if (isNaN(h) || isNaN(m)) return;
    
    // Calculate total minutes
    let total = h * 60 + m + deltaMinutes;
    
    // Wrap around 24h
    const max = 24 * 60;
    if (total < 0) total += max;
    if (total >= max) total %= max;
    
    // Convert back
    const newH = Math.floor(total / 60);
    const newM = total % 60;
    
    const val = `${String(newH).padStart(2,'0')}:${String(newM).padStart(2,'0')}`;
    emit('update:modelValue', val);
};
</script>

<style scoped>
/* Custom styling if needed */
</style>
