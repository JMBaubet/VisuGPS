<template>
  <div v-if="node" class="settings-node" :class="{ 'is-root': isRoot }">
    <!-- Cas 1 : Le groupe possède des sous-groupes -->
    <v-row v-if="node.groupes && node.groupes.length > 0" no-gutters class="nested-tabs-row">
      <v-col cols="3" md="2" lg="1" class="tabs-col">
        <v-tabs
          v-model="activeSubTab"
          direction="vertical"
          density="compact"
          color="primary"
          class="nested-tabs"
        >
          <!-- Onglet Général si des paramètres directs existent -->
          <v-tab
            v-if="node.parametres && node.parametres.length > 0"
            value="general"
            class="justify-start text-none sub-tab-item"
          >
            <v-icon start size="x-small">mdi-tune</v-icon>
            <span class="text-body-2 font-weight-medium">Général</span>
          </v-tab>

          <!-- Onglets pour chaque sous-groupe -->
          <v-tab
            v-for="subGroup in node.groupes"
            :key="subGroup.libelle"
            :value="subGroup.libelle"
            class="justify-start text-none sub-tab-item"
          >
            <v-icon start size="x-small">mdi-folder-outline</v-icon>
            <span class="text-body-2 font-weight-medium text-truncate">{{ subGroup.libelle }}</span>
            <v-tooltip activator="parent" location="end">{{ subGroup.libelle }}</v-tooltip>
          </v-tab>
        </v-tabs>
      </v-col>

      <v-col cols="9" md="10" lg="11">
        <v-window v-model="activeSubTab" class="nested-window">
          <!-- Contenu de l'onglet Général -->
          <v-window-item v-if="node.parametres && node.parametres.length > 0" value="general">
            <div class="params-container pa-4">
              <v-list dense bg-color="transparent">
                <v-list-item
                  v-for="param in node.parametres"
                  :key="param.identifiant"
                  @click="openEditDialog(param)"
                  class="param-item"
                >
                  <template v-slot:prepend>
                    <v-icon :color="getParamIconColor(param)">mdi-file-cog-outline</v-icon>
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
                  
                  <v-row no-gutters align="center" class="fill-height">
                    <v-col cols="6">
                      <v-list-item-title :class="{ 'text-warning': param.critique }">{{ param.libelle }}</v-list-item-title>
                      <v-list-item-subtitle class="text-truncate">{{ param.description }}</v-list-item-subtitle>
                    </v-col>
                    <v-col cols="6" class="d-flex align-center">
                        <v-avatar v-if="param.type === 'couleur'" :color="param.surcharge || param.defaut" size="24"></v-avatar>
                        <v-chip v-else-if="param.type === 'entier' || param.type === 'reel' || param.type === 'list' || param.type === 'monitor'" size="small">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
                        <v-chip v-else-if="param.type === 'directory'" size="small" class="text-truncate" style="max-width: 200px;">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
                        <div v-else-if="param.type === 'message' && getMessage(param.surcharge || param.defaut)"
                             :style="{
                                backgroundColor: toHex(getMessage(param.surcharge || param.defaut).style.backgroundColor),
                                color: getContrastColor(getMessage(param.surcharge || param.defaut).style.backgroundColor),
                                padding: '2px 8px',
                                borderRadius: '4px',
                                fontSize: '12px',
                                display: 'inline-block'
                             }">
                          {{ getMessage(param.surcharge || param.defaut).text }}
                        </div>
                        <v-chip v-else-if="param.type === 'message'" size="small" color="grey">Inconnu ({{ param.surcharge || param.defaut }})</v-chip>
                        <v-chip v-else-if="param.type === 'secret'" size="small">******</v-chip>
                        <v-chip v-else-if="param.type === 'heure'" size="small">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
                        <v-icon v-else-if="param.type === 'booleen'">{{ (param.surcharge != null ? param.surcharge : param.defaut) ? 'mdi-check' : 'mdi-close' }}</v-icon>
                    </v-col>
                  </v-row>
                </v-list-item>
              </v-list>
            </div>
          </v-window-item>

          <!-- Contenu des onglets de sous-groupes -->
          <v-window-item
            v-for="subGroup in node.groupes"
            :key="subGroup.libelle"
            :value="subGroup.libelle"
          >
            <SettingsNode 
              :node="subGroup" 
              :currentPath="fullPath" 
              @settings-updated="$emit('settings-updated')" 
            />
          </v-window-item>
        </v-window>
      </v-col>
    </v-row>

    <!-- Cas 2 : Le groupe possède uniquement des paramètres (pas de sous-groupes) -->
    <div v-else-if="node.parametres && node.parametres.length > 0" class="params-container pa-4">
      <v-list dense bg-color="transparent">
        <v-list-item
          v-for="param in node.parametres"
          :key="param.identifiant"
          @click="openEditDialog(param)"
          class="param-item"
        >
          <template v-slot:prepend>
            <v-icon :color="getParamIconColor(param)">mdi-file-cog-outline</v-icon>
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

          <v-row no-gutters align="center" class="fill-height">
            <v-col cols="6">
              <v-list-item-title :class="{ 'text-warning': param.critique }">{{ param.libelle }}</v-list-item-title>
              <v-list-item-subtitle class="text-truncate">{{ param.description }}</v-list-item-subtitle>
            </v-col>
            <v-col cols="6" class="d-flex align-center">
                <v-avatar v-if="param.type === 'couleur'" :color="param.surcharge || param.defaut" size="24"></v-avatar>
                <v-chip v-else-if="param.type === 'entier' || param.type === 'reel' || param.type === 'list' || param.type === 'monitor'" size="small">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
                <v-chip v-else-if="param.type === 'directory'" size="small" class="text-truncate" style="max-width: 200px;">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
                <div v-else-if="param.type === 'message' && getMessage(param.surcharge || param.defaut)"
                     :style="{
                        backgroundColor: toHex(getMessage(param.surcharge || param.defaut).style.backgroundColor),
                        color: getContrastColor(getMessage(param.surcharge || param.defaut).style.backgroundColor),
                        padding: '2px 8px',
                        borderRadius: '4px',
                        fontSize: '12px',
                        display: 'inline-block'
                     }">
                  {{ getMessage(param.surcharge || param.defaut).text }}
                </div>
                <v-chip v-else-if="param.type === 'message'" size="small" color="grey">Inconnu ({{ param.surcharge || param.defaut }})</v-chip>
                <v-chip v-else-if="param.type === 'secret'" size="small">******</v-chip>
                <v-chip v-else-if="param.type === 'heure'" size="small">{{ param.surcharge != null ? param.surcharge : param.defaut }}</v-chip>
                <v-icon v-else-if="param.type === 'booleen'">{{ (param.surcharge != null ? param.surcharge : param.defaut) ? 'mdi-check' : 'mdi-close' }}</v-icon>
            </v-col>
          </v-row>
        </v-list-item>
      </v-list>
    </div>

    <!-- Dialogues d'édition -->
    <EditStringDialog
      v-if="selectedParameter && selectedParameter.type === 'string'"
      :show="isStringDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isStringDialogVisible = $event"
    />
    <EditIntDialog
      v-if="selectedParameter && selectedParameter.type === 'entier'"
      :show="isIntDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isIntDialogVisible = $event"
    />
    <EditBoolDialog
      v-if="selectedParameter && selectedParameter.type === 'booleen'"
      :show="isBoolDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isBoolDialogVisible = $event"
    />
    <EditColorDialog
      v-if="selectedParameter && selectedParameter.type === 'couleur'"
      :show="isColorDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      :material-design-strict="selectedParameter.materialDesignStrict"
      @update:show="isColorDialogVisible = $event"
    />
    <EditFloatDialog
      v-if="selectedParameter && selectedParameter.type === 'reel'"
      :show="isFloatDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isFloatDialogVisible = $event"
    />
    <EditCoordDialog
      v-if="selectedParameter && selectedParameter.type === 'coord'"
      :show="isCoordDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isCoordDialogVisible = $event"
    />
    <EditSecretDialog
      v-if="selectedParameter && selectedParameter.type === 'secret'"
      :show="isSecretDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isSecretDialogVisible = $event"
    />
    <EditListDialog
      v-if="selectedParameter && selectedParameter.type === 'list'"
      :show="isListDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isListDialogVisible = $event"
    />
    <EditMonitorDialog
      v-if="selectedParameter && selectedParameter.type === 'monitor'"
      :show="isMonitorDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isMonitorDialogVisible = $event"
    />
    <EditDirectoryDialog
      v-if="selectedParameter && selectedParameter.type === 'directory'"
      :show="isDirectoryDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isDirectoryDialogVisible = $event"
    />
    <EditMessageDialog
      v-if="selectedParameter && selectedParameter.type === 'message'"
      :show="isMessageDialogVisible"
      :parameter="selectedParameter"
      :group-path="fullPath"
      @update:show="isMessageDialogVisible = $event"
    />
    <EditTimeDialog
        v-if="selectedParameter && selectedParameter.type === 'heure'"
        :show="isTimeDialogVisible"
        :parameter="selectedParameter"
        :group-path="fullPath"
        @update:show="isTimeDialogVisible = $event"
    />
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
import { computed, ref, onMounted } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useDebug } from '@/composables/useDebug';
import { useMessages } from '@/composables/useMessages';
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
import EditMessageDialog from './EditMessageDialog.vue';
import EditTimeDialog from './EditTimeDialog.vue';
import DocDisplay from '@/components/DocDisplay.vue';

const { updateSetting } = useSettings();
const { isDebugMode } = useDebug();
const { getMessage, toHex, getContrastColor, fetchMessages } = useMessages();

fetchMessages();

const props = defineProps({
  node: {
    type: Object,
    required: true
  },
  currentPath: {
    type: String,
    default: ''
  },
  isRoot: {
    type: Boolean,
    default: false
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
const isMessageDialogVisible = ref(false);
const isTimeDialogVisible = ref(false);
const isDocDialogVisible = ref(false);
const selectedParameter = ref(null);
const activeSubTab = ref(null);

const openEditDialog = async (param) => {
  selectedParameter.value = param;
  if (param.type === 'string') isStringDialogVisible.value = true;
  else if (param.type === 'entier') isIntDialogVisible.value = true;
  else if (param.type === 'booleen') isBoolDialogVisible.value = true;
  else if (param.type === 'couleur') isColorDialogVisible.value = true;
  else if (param.type === 'reel') isFloatDialogVisible.value = true;
  else if (param.type === 'coord') isCoordDialogVisible.value = true;
  else if (param.type === 'secret') isSecretDialogVisible.value = true;
  else if (param.type === 'list') isListDialogVisible.value = true;
  else if (param.type === 'monitor') isMonitorDialogVisible.value = true;
  else if (param.type === 'directory') isDirectoryDialogVisible.value = true;
  else if (param.type === 'message') isMessageDialogVisible.value = true;
  else if (param.type === 'heure') isTimeDialogVisible.value = true;
};

const openDocDialog = (param) => {
  selectedParameter.value = param;
  isDocDialogVisible.value = true;
};

const getParamIconColor = (param) => {
  if (param.type === 'secret') {
    // Si pas de surcharge pour un secret, c'est une anomalie -> Orange
    // Si surcharge présente, c'est normal -> Pas de signalement spécial
    return (param.surcharge == null) ? 'orange' : undefined;
  }
  // Logique standard pour les autres paramètres
  return param.surcharge != null ? (param.critique ? 'red' : 'yellow') : undefined;
};

const fullPath = computed(() => {
  if (props.currentPath) {
    return `${props.currentPath}/${props.node.libelle}`;
  }
  return props.node.libelle;
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
.settings-node {
  width: 100%;
}

.nested-tabs-row {
  border-bottom: 1px solid rgba(var(--v-border-color), 0.1);
}

.tabs-col {
  border-right: 1px solid rgba(var(--v-border-color), 0.05);
}

.nested-tabs {
  height: 100%;
}

.sub-tab-item {
  min-height: 40px !important;
  height: auto !important;
  padding: 8px 12px;
}

.param-item {
  cursor: pointer;
  border-radius: 4px;
  margin-bottom: 4px;
}

.param-item:hover {
  background-color: rgba(var(--v-theme-on-surface), 0.04);
}

.params-container {
  background-color: rgba(var(--v-theme-on-surface), 0.01);
}

.nested-window {
  min-height: 100px;
}
</style>
