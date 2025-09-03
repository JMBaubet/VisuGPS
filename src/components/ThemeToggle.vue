<template>
  <div class="d-flex align-center">
    <v-icon color="yellow-darken-2">mdi-weather-sunny</v-icon>
    <v-switch
      v-model="isDark"
      hide-details
      inset
      color="primary"
      class="mx-2"
    ></v-switch>
    <v-icon>mdi-weather-night</v-icon>
  </div>
</template>

<script setup>
import { computed, onMounted } from 'vue';
import { useTheme } from 'vuetify';

const theme = useTheme();

const isDark = computed({
  get: () => theme.global.name.value === 'dark',
  set: (val) => {
    const newTheme = val ? 'dark' : 'light';
    theme.global.name.value = newTheme;
    localStorage.setItem('theme', newTheme);
  }
});

onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme) {
    theme.global.name.value = savedTheme;
  }
});
</script>

<style scoped>
/* Add any specific styles for the theme toggle here */
</style>
