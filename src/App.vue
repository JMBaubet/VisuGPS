<template>
  <v-app>
    <v-container :class="[{ 'app-frame': showFrame }, frameColorClass]" fluid>
      <router-view />
    </v-container>
  </v-app>
</template>

<script setup>
import { computed } from 'vue';
import { useEnvironment } from './composables/useEnvironment';

const { executionMode } = useEnvironment();

const showFrame = computed(() => {
  return executionMode.value !== 'OPE';
});

const frameColorClass = computed(() => {
  if (executionMode.value === 'EVAL') {
    return 'app-frame-eval';
  } else if (executionMode.value === 'TEST') {
    return 'app-frame-test';
  }
  return '';
});
</script>

<style>
/* Basic styling */
body {
  margin: 0;
  font-family: sans-serif;
}
html, body, #app {
  height: 100%;
  margin: 0;
  padding: 0;
}

.prod-border {
  border: 0px solid rgba(255, 255, 255, 0);
}
.sandbox-border {
  border: 5px solid orange;
}

.test-border {
  border: 5px solid red;
}

.app-frame {
  border-width: 5px; /* Changed to 5px */
  border-style: solid;
  /* Removed border-radius */
  height: 100%; /* Added for full height */
}

.app-frame-eval {
  border-color: orange;
  /* Removed box-shadow */
}

.app-frame-test {
  border-color: red;
  /* Removed box-shadow */
}
</style>