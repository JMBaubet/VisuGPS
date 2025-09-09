<template>
  <v-app>
    <v-container fluid class="fill-height pa-0 test-border">
      <router-view />
    </v-container>
  </v-app>
</template>

<script setup>
// No script needed for a basic App.vue
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
</style>