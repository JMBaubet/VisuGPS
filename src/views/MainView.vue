<!-- <template>
  <div>
    <h1>Vue Principale</h1>
  </div>
</template> -->

<template>
  <v-app>
    <AppBarMain />
    <v-main>
      <!-- Content of MainView goes here -->
      <v-container>
        <h1>Bienvenue sur VisuGPS</h1>
        <p>Utilisez la barre d'application ci-dessus pour naviguer.</p>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup>
import AppBarMain from "../components/AppBarMain.vue";
// MessageContainer and notify function are no longer needed here
</script>