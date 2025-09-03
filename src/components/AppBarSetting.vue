<template>
  <v-app-bar app density="compact">
    <v-toolbar-title>Paramètres</v-toolbar-title>
    <v-spacer></v-spacer>
    <v-btn icon @click="goToMainView">
      <v-icon>mdi-arrow-left</v-icon>
    </v-btn>
  </v-app-bar>
</template>

<script setup>
import { useRouter } from 'vue-router';

const router = useRouter();

const goToMainView = () => {
  router.push('/');
};
</script>
