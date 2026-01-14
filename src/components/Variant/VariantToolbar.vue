<template>
  <v-toolbar density="compact" class="transition-swing">
    <v-btn icon @click="$emit('close')" class="ml-2 mr-2">
      <v-icon>mdi-home</v-icon>
    </v-btn>

    <v-toolbar-title class="font-weight-bold">
      {{ circuitName }}
    </v-toolbar-title>

    <v-spacer></v-spacer>

    <v-btn-toggle
      v-model="internalMode"
      mandatory
      rounded="lg"
      class="mr-4"
      variant="tonal"
      density="compact"
    >
      <v-btn value="DEPART" color="success">
        <v-icon start>mdi-ray-start-arrow</v-icon>
        Départ
      </v-btn>

      <v-btn value="SEGMENT" color="primary">
        <v-icon start>mdi-source-branch</v-icon>
        Segment
      </v-btn>

      <v-btn value="ARRIVEE" color="error">
        <v-icon start>mdi-ray-end-arrow</v-icon>
        Arrivée
      </v-btn>
    </v-btn-toggle>

    <v-spacer></v-spacer>

    <!-- Routing Profile Toggle -->
    <v-btn-toggle
      v-model="internalProfile"
      mandatory
      rounded="lg"
      class="mr-4"
      variant="outlined"
      density="compact"
      :color="toggleColor"
    >
      <v-btn value="car" title="Route + Pistes cyclables">
        <v-icon>mdi-bike</v-icon>
      </v-btn>
      <v-btn value="racingbike" title="Route uniquement">
        <v-icon>mdi-bike-fast</v-icon>
      </v-btn>
      <v-btn value="bike" title="VTT / Chemin">
        <v-icon>mdi-image-filter-hdr</v-icon>
      </v-btn>
    </v-btn-toggle>

    <v-btn icon @click="$emit('open-doc')" title="Documentation Route Builder" color="blue">
      <v-icon>mdi-book-open-page-variant-outline</v-icon>
    </v-btn>

  </v-toolbar>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from 'vuetify';

const props = defineProps({
  mode: {
    type: String,
    required: true,
    validator: v => ['DEPART', 'SEGMENT', 'ARRIVEE'].includes(v)
  },
  profile: {
    type: String,
    default: 'bike'
  },
  circuitName: {
    type: String,
    default: ''
  },
  errorProfile: {
    type: String,
    default: null
  }
});

const emit = defineEmits(['update:mode', 'update:profile', 'save', 'close', 'open-doc']);
const theme = useTheme();

const internalMode = computed({
  get: () => props.mode,
  set: (val) => emit('update:mode', val)
});

const internalProfile = computed({
  get: () => props.profile,
  set: (val) => emit('update:profile', val)
});

const toggleColor = computed(() => {
  return (props.errorProfile && props.errorProfile === internalProfile.value) ? 'error' : 'primary';
});
</script>

<style scoped>
.transition-swing {
  transition: background-color 0.3s cubic-bezier(0.25, 0.8, 0.5, 1);
}
</style>
