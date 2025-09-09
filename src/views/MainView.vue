<template>
  <v-container fluid class="fill-height d-flex flex-column align-center pa-0">
    <AppMainBar /> <!-- Added AppMainBar -->
    <v-btn color="primary">
      Aller aux Paramètres
    </v-btn>
  </v-container>
</template>

<script setup>
import AppMainBar from '../components/AppMainBar.vue'; // Import AppMainBar
</script>