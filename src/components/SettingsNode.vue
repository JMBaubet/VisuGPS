<template>
  <div v-if="node">
    <!-- Si le noeud est un groupe -->
    <v-list-group v-if="node.groupes || node.parametres">
      <template v-slot:activator="{ props }">
        <v-list-item v-bind="props" prepend-icon="mdi-folder">
          <v-list-item-title>{{ node.nom }}</v-list-item-title>
        </v-list-item>
      </template>

      <!-- Appel récursif pour les sous-groupes -->
      <SettingsNode v-for="(childGroup, index) in node.groupes" :key="`group-${index}`" :node="childGroup" />

      <!-- Affichage des paramètres -->
      <v-list-item v-for="(param, index) in node.parametres" :key="`param-${index}`" prepend-icon="mdi-file-cog-outline">
        <v-list-item-title>{{ param.nom }}</v-list-item-title>
        <v-list-item-subtitle>{{ param.description }}</v-list-item-subtitle>
      </v-list-item>

    </v-list-group>

  </div>
</template>

<script setup>
import { defineProps } from 'vue';

const props = defineProps({
  node: {
    type: Object,
    required: true
  }
});

</script>

<script>
// Nécessaire pour la récursivité
export default {
  name: 'SettingsNode'
}
</script>

<style scoped>
.v-list-item {
  margin-left: 16px;
}
</style>
