<template>
  <v-container>
    <v-row justify="center">
      <v-col cols="12" md="8" lg="6">
        <MapboxTokenEditor />
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup>
import MapboxTokenEditor from '../components/MapboxTokenEditor.vue';
</script>