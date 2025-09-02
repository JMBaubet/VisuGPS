<template>
  <router-view />
  <MessageContainer />
</template>

<script setup>
import MessageContainer from './components/MessageContainer.vue';
// No script needed for a basic App.vue
</script>

<style>
/* Basic styling */
body {
  margin: 0;
  font-family: sans-serif;
}
html, body {
  overflow: hidden;
}
html, body, #app, .v-application {
  height: 100%;
  overflow: hidden !important; /* empêche le scroll */
}
</style>