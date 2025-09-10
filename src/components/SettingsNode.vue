<template>
  <div v-if="node">
    <!-- Si le noeud est un groupe -->
    <v-list-group v-if="node.groupes || node.parametres">
      <template v-slot:activator="{ props, isOpen }">
        <v-list-item v-bind="props" :prepend-icon="isOpen ? 'mdi-folder-open' : 'mdi-folder'" append-icon="">
          <v-list-item-title>{{ node.libelle }}</v-list-item-title>
        </v-list-item>
      </template>

      <!-- Affichage des paramètres -->
      <v-list-item v-for="param in node.parametres" :key="param.identifiant">
        <template v-slot:prepend>
          <v-icon :color="param.surcharge != null ? 'info' : undefined">mdi-file-cog-outline</v-icon>
        </template>
        <v-list-item-title :class="{ 'text-warning': param.critique }">{{ param.libelle }}</v-list-item-title>
        <v-list-item-subtitle>{{ param.description }}</v-list-item-subtitle>
      </v-list-item>

      <!-- Appel récursif pour les sous-groupes -->
      <SettingsNode v-for="childGroup in node.groupes" :key="childGroup.identifiant" :node="childGroup" />

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
