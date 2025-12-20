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

      <!-- Appel récursif pour les sous-groupes -->
      <SettingsNode
        v-for="childGroup in node.groupes"
        :key="childGroup.identifiant"
        :node="childGroup"
        :currentPath="fullPath"
      />

      <!-- Affichage des paramètres -->
      <v-list-item
        v-for="param in node.parametres"
        :key="param.identifiant"
        @click="openEditDialog(param)"
        class="param-item"
      >
        <template v-slot:prepend>
          <v-icon :color="param.surcharge != null ? 'yellow' : undefined">mdi-file-cog-outline</v-icon>
          <v-btn
            v-if="param.doc"
            icon="mdi-book-open-page-variant-outline"
            color="info"
            variant="text"
            @click.stop="openDocDialog(param)"
            class="ml-1"
            title="Afficher la documentation"
          ></v-btn>
        </template>
        <v-list-item-title :class="{ 'text-warning': param.critique }">{{ param.libelle }}</v-list-item-title>
        <v-list-item-subtitle>{{ param.description }}</v-list-item-subtitle>
        <template v-slot:append>
            <v-avatar v-if="param.type === 'couleur'" :color="param.surcharge || param.defaut" size="24"></v-avatar>
            <v-chip v-else-if="param.type === 'entier' || param.type === 'reel' || param.type === 'list' || param.type === 'monitor'" size="small">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
            <v-chip v-else-if="param.type === 'directory'" size="small" class="text-truncate" style="max-width: 200px;">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
            <v-chip v-else-if="param.type === 'secret'" size="small">******</v-chip>
            <v-icon v-else-if="param.type === 'booleen'">{{ (param.surcharge != null ? param.surcharge : param.defaut) ? 'mdi-check' : 'mdi-close' }}</v-icon>
        </template>
      </v-list-item>

    </v-list-group>

    <!-- Composant de dialogue pour les strings -->
    <EditStringDialog
      v-if="selectedParameter && selectedParameter.type === 'string'"
      :show="isStringDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isStringDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les entiers -->
    <EditIntDialog
      v-if="selectedParameter && selectedParameter.type === 'entier'"
      :show="isIntDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isIntDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les booleens -->
    <EditBoolDialog
      v-if="selectedParameter && selectedParameter.type === 'booleen'"
      :show="isBoolDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isBoolDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les couleurs -->
    <EditColorDialog
      v-if="selectedParameter && selectedParameter.type === 'couleur'"
      :show="isColorDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      :material-design-strict="selectedParameter.materialDesignStrict"
      @update:show="isColorDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les réels -->
    <EditFloatDialog
      v-if="selectedParameter && selectedParameter.type === 'reel'"
      :show="isFloatDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isFloatDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les coordonnées -->
    <EditCoordDialog
      v-if="selectedParameter && selectedParameter.type === 'coord'"
      :show="isCoordDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isCoordDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les secrets -->
    <EditSecretDialog
      v-if="selectedParameter && selectedParameter.type === 'secret'"
      :show="isSecretDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isSecretDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les listes -->
    <EditListDialog
      v-if="selectedParameter && selectedParameter.type === 'list'"
      :show="isListDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isListDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les moniteurs -->
    <EditMonitorDialog
      v-if="selectedParameter && selectedParameter.type === 'monitor'"
      :show="isMonitorDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isMonitorDialogVisible = $event"
    />

    <!-- Composant de dialogue pour les dossiers -->
    <EditDirectoryDialog
      v-if="selectedParameter && selectedParameter.type === 'directory'"
      :show="isDirectoryDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isDirectoryDialogVisible = $event"
    />

    <!-- Composant de dialogue pour la documentation -->
    <v-dialog v-model="isDocDialogVisible" max-width="800px">
      <DocDisplay 
        v-if="selectedParameter" 
        :doc-path="selectedParameter.doc" 
        @close="isDocDialogVisible = false" 
      />
    </v-dialog>
  </div>
</template>

<script setup>
import { defineProps, computed, ref } from 'vue';
import { useSettings } from '@/composables/useSettings';
import EditStringDialog from './EditStringDialog.vue';
import EditIntDialog from './EditIntDialog.vue';
import EditBoolDialog from './EditBoolDialog.vue';
import EditColorDialog from './EditColorDialog.vue';
import EditFloatDialog from './EditFloatDialog.vue';
import EditCoordDialog from './EditCoordDialog.vue';
import EditSecretDialog from './EditSecretDialog.vue';
import EditListDialog from './EditListDialog.vue';
import EditMonitorDialog from './EditMonitorDialog.vue';
import EditDirectoryDialog from './EditDirectoryDialog.vue';
import DocDisplay from './DocDisplay.vue';

const { updateSetting } = useSettings();

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

const isStringDialogVisible = ref(false);
const isIntDialogVisible = ref(false);
const isBoolDialogVisible = ref(false);
const isColorDialogVisible = ref(false);
const isFloatDialogVisible = ref(false);
const isCoordDialogVisible = ref(false);
const isSecretDialogVisible = ref(false);
const isListDialogVisible = ref(false);
const isMonitorDialogVisible = ref(false);
const isDirectoryDialogVisible = ref(false);
const isDocDialogVisible = ref(false);
const selectedParameter = ref(null);

const openEditDialog = async (param) => {
  selectedParameter.value = param;
  if (param.type === 'string') {
    isStringDialogVisible.value = true;
  } else if (param.type === 'entier') {
    isIntDialogVisible.value = true;
  } else if (param.type === 'booleen') {
    isBoolDialogVisible.value = true;
  } else if (param.type === 'couleur') {
    isColorDialogVisible.value = true;
  } else if (param.type === 'reel') {
    isFloatDialogVisible.value = true;
  } else if (param.type === 'coord') {
    isCoordDialogVisible.value = true;
  } else if (param.type === 'secret') {
    isSecretDialogVisible.value = true;
  } else if (param.type === 'list') {
    isListDialogVisible.value = true;
  } else if (param.type === 'monitor') {
    isMonitorDialogVisible.value = true;
  } else if (param.type === 'directory') {
    isDirectoryDialogVisible.value = true;
  }
};

const openDocDialog = (param) => {
  selectedParameter.value = param;
  isDocDialogVisible.value = true;
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
