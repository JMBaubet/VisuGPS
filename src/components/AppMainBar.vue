<template>
  <v-toolbar flat class="pa-0">
    <!-- Chip for APP_ENV -->
    <v-chip
      v-if="showAppEnvChip"
      class="ma-2 blinking-chip"
      :color="chipColor"
      label
    >
      {{ appEnv }}
    </v-chip>

    <v-spacer></v-spacer> <!-- Spacer to push switch right -->

    <!-- Dark/Light Mode Switch -->
    <v-switch
      v-model="isDarkTheme"
      hide-details
      inset
      color="primary"
      class="pr-4"
    >
      <template v-slot:prepend>
        <v-icon :color="isDarkTheme ? 'primary' : 'grey'">mdi-weather-night</v-icon>
      </template>
      <template v-slot:append>
        <v-icon :color="isDarkTheme ? 'grey' : '#FFA000'">mdi-weather-sunny</v-icon> <!-- Darker yellow/amber -->
      </template>
    </v-switch>
  </v-toolbar>
</template>

<script setup>
import { computed } from 'vue';
import { useTheme } from 'vuetify';
import { useEnvironment } from '../composables/useEnvironment';

const theme = useTheme();
const { appEnv, executionMode } = useEnvironment();

const isDarkTheme = computed({
  get() {
    return theme.global.current.value.dark;
  },
  set(value) {
    theme.global.name.value = value ? 'dark' : 'light';
  }
});

const showAppEnvChip = computed(() => {
  return executionMode.value === 'EVAL' || executionMode.value === 'TEST';
});

const chipColor = computed(() => {
  if (executionMode.value === 'EVAL') {
    return 'orange';
  } else if (executionMode.value === 'TEST') {
    return 'red';
  }
  return 'primary'; // Default color if not EVAL or TEST
});
</script>

<style scoped>
@keyframes blink {
  0% { opacity: 1; }
  49% { opacity: 1; }
  50% { opacity: 0; }
  99% { opacity: 0; }
  100% { opacity: 1; }
}

.blinking-chip {
  animation: blink 1s linear infinite;
}
</style>