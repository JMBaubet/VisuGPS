<template>
  <v-menu
    v-model="menu"
    :close-on-content-click="false"
    location="end"
  >
    <template v-slot:activator="{ props }">
      <v-text-field
        v-bind="props"
        :model-value="formattedDate"
        :label="label"
        prepend-icon="mdi-calendar"
        readonly
        hide-details
        density="compact"
        variant="underlined"
        class="mb-2"
      ></v-text-field>
    </template>
    <v-card min-width="290">
      <v-toolbar density="compact" color="primary">
        <v-toolbar-title>{{ label }}</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn icon @click="menu = false">
            <v-icon>mdi-close</v-icon>
        </v-btn>
      </v-toolbar>
      <v-card-text class="pa-0">
          <!-- Mobile-friendly date input using standard input type="date" -->
          <div class="d-flex justify-center pa-4">
              <input 
                  type="date" 
                  :value="modelValue" 
                  @input="$emit('update:modelValue', $event.target.value)"
                  style="font-size: 1.2rem; padding: 10px; border: 1px solid #ccc; border-radius: 4px; background: white; color: black;"
              />
          </div>
      </v-card-text>
    </v-card>
  </v-menu>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  modelValue: {
    type: String,
    required: true,
  },
  label: {
    type: String,
    default: 'Date',
  },
});

const emit = defineEmits(['update:modelValue']);

const menu = ref(false);

const formattedDate = computed(() => {
    if (!props.modelValue) return '';
    const part = props.modelValue.split('-');
    if (part.length !== 3) return props.modelValue;
    // Converts YYYY-MM-DD to DD/MM/YYYY for display
    return `${part[2]}/${part[1]}/${part[0]}`;
});
</script>

<style scoped>
</style>
