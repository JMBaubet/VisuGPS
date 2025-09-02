<template>
  <div class="d-flex align-center">
    <v-icon color="yellow-darken-2">mdi-weather-sunny</v-icon>
    <v-switch
      v-model="isDarkTheme"
      hide-details
      inset
      color="primary"
      @change="toggleTheme"
      class="mx-2"
    ></v-switch>
    <v-icon>mdi-weather-night</v-icon>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useTheme } from 'vuetify';

const theme = useTheme();
const isDarkTheme = ref(false);

const toggleTheme = () => {
  theme.global.name.value = isDarkTheme.value ? 'dark' : 'light';
  localStorage.setItem('theme', theme.global.name.value);
};

onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme) {
    isDarkTheme.value = savedTheme === 'dark';
    theme.global.name.value = savedTheme;
  } else {
    // Default to light theme if no preference is saved
    isDarkTheme.value = false;
    theme.global.name.value = 'light';
  }
});
</script>

<style scoped>
/* Add any specific styles for the theme toggle here */
</style>
