<template>
  <v-row align="center" no-gutters>
    <v-col cols="auto">
      <v-icon>mdi-weather-night</v-icon>
    </v-col>
    <v-col cols="auto" class="mx-2">
      <v-switch
        v-model="isDarkTheme"
        hide-details
        color="primary"
        @change="toggleTheme"
      >
      </v-switch>
    </v-col>
    <v-col cols="auto">
      <v-icon :color="isDarkTheme ? 'yellow' : 'yellow-darken-3'">mdi-white-balance-sunny</v-icon>
    </v-col>
  </v-row>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useTheme } from 'vuetify';

const theme = useTheme();
const isDarkTheme = ref(false);

const toggleTheme = () => {
  theme.change(isDarkTheme.value ? 'dark' : 'light');
  localStorage.setItem('theme', isDarkTheme.value ? 'dark' : 'light'); // Update localStorage with the new theme name
};

onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme) {
    theme.change(savedTheme);
    isDarkTheme.value = savedTheme === 'dark';
  } else {
    // Default to dark theme if no preference is saved
    theme.change('dark');
    isDarkTheme.value = true;
  }
});
</script>