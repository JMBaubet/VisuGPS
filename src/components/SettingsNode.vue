<template>
  <div v-if="node">
    <!-- Si le noeud est un groupe -->
    <v-list-group v-if="node.groupes || node.parametres">
      <template v-slot:activator="{ props, isOpen }">
        <v-list-item v-bind="props" :prepend-icon="isOpen ? 'mdi-folder-open' : 'mdi-folder'" append-icon="">
          <v-list-item-title>{{ node.libelle }}</v-list-item-title>
          <v-tooltip v-if="isDev" activator="parent" location="end">/{{ fullPath }}</v-tooltip>
        </v-list-item>
      </template>

      <!-- Affichage des paramètres -->
      <v-list-item
        v-for="param in node.parametres"
        :key="param.identifiant"
        @click="openEditDialog(param)"
        class="param-item"
      >
        <template v-slot:prepend>
          <v-icon :color="param.surcharge != null ? 'info' : undefined">mdi-file-cog-outline</v-icon>
        </template>
        <v-list-item-title :class="{ 'text-warning': param.critique }">{{ param.libelle }}</v-list-item-title>
        <v-list-item-subtitle>{{ param.description }}</v-list-item-subtitle>
      </v-list-item>

      <!-- Appel récursif pour les sous-groupes -->
      <SettingsNode
        v-for="childGroup in node.groupes"
        :key="childGroup.identifiant"
        :node="childGroup"
        :currentPath="fullPath"
        @settings-updated="$emit('settings-updated')"
      />

    </v-list-group>

    <!-- Composant de dialogue pour les strings -->
    <EditStringDialog
      v-if="selectedParameter"
      :show="isStringDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isStringDialogVisible = $event"
      @saved="onSettingsUpdated"
    />
  </div>
</template>

<script setup>
import { defineProps, computed, ref, defineEmits } from 'vue';
import EditStringDialog from './EditStringDialog.vue';

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

const emit = defineEmits(['settings-updated']);

const isStringDialogVisible = ref(false);
const selectedParameter = ref(null);

const openEditDialog = (param) => {
  // Pour l'instant, on ne gère que le type 'string'
  if (param.type === 'string') {
    selectedParameter.value = param;
    isStringDialogVisible.value = true;
  }
  // On pourra ajouter des else if pour d'autres types ici
};

const onSettingsUpdated = () => {
  emit('settings-updated');
};

const fullPath = computed(() => {
  if (props.currentPath) {
    return `${props.currentPath}/${props.node.libelle}`;
  }
  return props.node.libelle;
});

const isDev = computed(() => {
  return process.env.NODE_ENV === 'development';
});

</script>

<script>
// Nécessaire pour la récursivité
import SettingsNode from './SettingsNode.vue';
export default {
  name: 'SettingsNode'
}
</script>

<style scoped>
.param-item {
  cursor: pointer;
}
.param-item:hover {
  background-color: rgba(var(--v-theme-on-surface), 0.04);
}
.v-list-item {
  margin-left: 16px;
}
</style>
