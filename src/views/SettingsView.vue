<template>
  <v-container fluid>
    <SettingsToolbar />
    <v-row>
      <v-col cols="12" md="8" lg="6">
        <SettingsTree />
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup>
import SettingsToolbar from '@/components/SettingsToolbar.vue';
import SettingsTree from '@/components/SettingsTree.vue';
</script>

<style scoped>
/* Styles pour la vue des paramètres */
</style>
