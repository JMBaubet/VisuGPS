<template>
  <div v-if="node">
    <!-- Si le noeud est un groupe -->
    <v-list-group v-if="node.groupes || node.parametres">
      <template v-slot:activator="{ props, isOpen }">
        <v-list-item v-bind="props" :prepend-icon="isOpen ? 'mdi-folder-open' : 'mdi-folder'" append-icon="">
          <v-list-item-title>{{ node.libelle }}</v-list-item-title>
          <v-tooltip activator="parent" location="top">/{{ fullPath }}</v-tooltip>
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
      <SettingsNode v-for="childGroup in node.groupes" :key="childGroup.identifiant" :node="childGroup" :currentPath="fullPath" />

    </v-list-group>

  </div>
</template>

<script setup>
import { defineProps, computed } from 'vue';

const props = defineProps({
  node: {
    type: Object,
    required: true
  },
  currentPath: {
    type: String,
    default: ''
  }
});

const fullPath = computed(() => {
  if (props.currentPath) {
    return `${props.currentPath}/${props.node.libelle}`;
  }
  return props.node.libelle;
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
