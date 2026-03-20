<template>
  <v-toolbar density="compact" class="transition-swing">
    <v-btn icon @click="$emit('close')" class="ml-2 mr-2">
      <v-icon>mdi-home</v-icon>
    </v-btn>

    <v-toolbar-title class="font-weight-bold">
      {{ circuitName }}
    </v-toolbar-title>

    <v-spacer></v-spacer>

    <!-- Mode Trace Toggle (DEPART / SEGMENT / ARRIVEE) -->
    <v-btn-toggle
      v-model="internalTraceMode"
      rounded="lg"
      class="mr-2"
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

    <!-- Waypoints Menu -->
    <v-menu location="bottom center" :close-on-content-click="true">
      <template v-slot:activator="{ props: menuProps }">
        <v-btn
          v-bind="menuProps"
          :color="activeWaypointColor"
          :variant="isWaypointMode ? 'flat' : 'tonal'"
          rounded="lg"
          class="mr-4"
          density="compact"
          append-icon="mdi-menu-down"
          :title="isWaypointMode ? `Mode actif : ${activeWaypointLabel}` : 'Ajouter un point d\'intérêt (Waitpoint)'"
        >
          <v-icon start>{{ activeWaypointIcon }}</v-icon>
          {{ isWaypointMode ? activeWaypointLabel : 'Waitpoint' }}
        </v-btn>
      </template>
      <v-list density="compact" nav min-width="200">
        <v-list-subheader>Points d'intérêt (Waitpoints)</v-list-subheader>
        <v-list-item
          v-for="wp in waypointTypes"
          :key="wp.value"
          :value="wp.value"
          :active="internalMode === wp.value"
          active-color="primary"
          rounded="lg"
          @click="selectWaypointMode(wp.value)"
        >
          <template v-slot:prepend>
            <v-icon :color="wp.color" size="small">{{ wp.icon }}</v-icon>
          </template>
          <v-list-item-title>{{ wp.label }}</v-list-item-title>
          <v-list-item-subtitle class="text-caption">{{ wp.sublabel }}</v-list-item-subtitle>
        </v-list-item>
      </v-list>
    </v-menu>

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

<script>
const WAYPOINT_MODES = ['WATER', 'MEETING SPOT', 'DANGER', 'OVERLOOK', 'TOILET', 'INFO', 'SUMMIT', 'TUNNEL', 'FOOD'];
const TRACE_MODES = ['DEPART', 'SEGMENT', 'ARRIVEE'];

export default {
  inheritAttrs: false
}
</script>

<script setup>
import { computed } from 'vue';
import { useTheme } from 'vuetify';

const waypointTypes = [
  { value: 'WATER',         label: 'Eau',           sublabel: 'Point d\'eau potable', icon: 'mdi-water',              color: 'blue' },
  { value: 'FOOD',          label: 'Ravito',        sublabel: 'Ravitaillement',        icon: 'mdi-food-fork-drink',         color: 'orange' },
  { value: 'MEETING SPOT',  label: 'Rdv',           sublabel: 'Point de rendez-vous', icon: 'mdi-account-group',      color: 'purple' },
  { value: 'DANGER',        label: 'Danger',        sublabel: 'Zone de danger',        icon: 'mdi-alert-octagon',      color: 'red' },
  { value: 'OVERLOOK',      label: 'Vue',           sublabel: 'Point de vue',          icon: 'mdi-camera',   color: 'green' },
  { value: 'TOILET',        label: 'WC',            sublabel: 'Toilettes',             icon: 'mdi-human-male-female',  color: 'brown' },
  { value: 'INFO',          label: 'Info',          sublabel: 'Information',           icon: 'mdi-information',        color: 'cyan' },
  { value: 'SUMMIT',        label: 'Sommet',        sublabel: 'Sommet / Col',           icon: 'mdi-image-filter-hdr',           color: 'blue-grey' },
  { value: 'TUNNEL',        label: 'Tunnel',        sublabel: 'Passage en tunnel',     icon: 'mdi-tunnel',             color: 'black' },
];

const props = defineProps({
  mode: {
    type: String,
    required: true,
    validator: v => [...TRACE_MODES, ...WAYPOINT_MODES].includes(v)
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

// Computed: internal mode for trace toggle
const internalTraceMode = computed({
  get: () => TRACE_MODES.includes(props.mode) ? props.mode : null,
  set: (val) => {
    if (val) emit('update:mode', val);
  }
});

// Full internal mode proxy
const internalMode = computed({
  get: () => props.mode,
  set: (val) => emit('update:mode', val)
});

const internalProfile = computed({
  get: () => props.profile,
  set: (val) => emit('update:profile', val)
});

const isWaypointMode = computed(() => WAYPOINT_MODES.includes(props.mode));

const activeWaypointDef = computed(() => waypointTypes.find(w => w.value === props.mode) || null);
const activeWaypointLabel = computed(() => activeWaypointDef.value?.label || 'Waitpoint');
const activeWaypointIcon = computed(() => activeWaypointDef.value?.icon || 'mdi-map-marker-plus');
const activeWaypointColor = computed(() => isWaypointMode.value ? (activeWaypointDef.value?.color || 'secondary') : 'secondary');

const selectWaypointMode = (val) => {
  // Toggle: if same mode clicked again, return to SEGMENT
  if (props.mode === val) {
    emit('update:mode', 'SEGMENT');
  } else {
    emit('update:mode', val);
  }
};

const toggleColor = computed(() => {
  return (props.errorProfile && props.errorProfile === internalProfile.value) ? 'error' : 'primary';
});
</script>

<style scoped>
.transition-swing {
  transition: background-color 0.3s cubic-bezier(0.25, 0.8, 0.5, 1);
}
</style>
