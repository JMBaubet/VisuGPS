<template>
  <v-container>
    <v-row>
      <v-col cols="12">
        <div class="d-flex justify-space-between align-center mb-4">
          <h1 class="text-h4">Paramètres</h1>
          <v-btn 
            color="primary"
            @click="saveAllSettings"
            :disabled="!isDirty"
            :loading="isSaving"
          >
            Sauvegarder
          </v-btn>
        </div>
        <v-divider></v-divider>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12">
        <div v-if="loading" class="text-center">
          <v-progress-circular indeterminate color="primary"></v-progress-circular>
        </div>
        <div v-else-if="error" class="text-center">
          <v-alert type="error">{{ error }}</v-alert>
        </div>
        
        <div v-else>
          <v-card>
            <v-tabs v-model="activeTab" bg-color="primary" grow>
              <v-tab v-for="category in Object.keys(settingsTree)" :key="category" :value="category">
                {{ category }}
              </v-tab>
            </v-tabs>

            <v-window v-model="activeTab">
              <v-window-item 
                v-for="(category, index) in Object.keys(settingsTree)" 
                :key="category" 
                :value="category"
                class="ma-2"
              >
                <v-card-text>
                  <v-list :opened="openState[index]">
                    <SettingsTreeItem
                      v-for="node in settingsTree[category]"
                      :key="node.id"
                      :node="node"
                      :settings="settings"
                    />
                  </v-list>
                </v-card-text>
              </v-window-item>
            </v-window>
          </v-card>

          <v-card class="mt-6">
            <v-card-title>Token Mapbox</v-card-title>
            <v-card-text>
                <MapboxTokenEditor />
            </v-card-text>
          </v-card>
        </div>
      </v-col>
    </v-row>

  </v-container>
</template>

<script setup>
import { ref, onMounted, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useMessageStore } from '../composables/useMessageStore.js';
import SettingsTreeItem from '../components/SettingsTreeItem.vue';
import StringSetting from '../components/StringSetting.vue';
import MapboxTokenEditor from '../components/MapboxTokenEditor.vue';

const { addMessage } = useMessageStore();

const settings = ref([]);
const initialSettings = ref([]);
const loading = ref(true);
const isSaving = ref(false);
const error = ref(null);
const activeTab = ref(null);
const openState = ref([]);

const settingsTree = computed(() => {
  if (!settings.value || settings.value.length === 0) {
    return {};
  }

  const groupedByTopLevel = settings.value.reduce((acc, setting) => {
    const topLevel = setting.arbre.split('/')[0] || 'Général';
    if (!acc[topLevel]) {
      acc[topLevel] = [];
    }
    acc[topLevel].push(setting);
    return acc;
  }, {});

  const finalTree = {};

  for (const topLevel in groupedByTopLevel) {
    const settingsList = groupedByTopLevel[topLevel];
    const treeItems = [];
    const nodeMap = new Map();

    settingsList.sort((a, b) => a.arbre.localeCompare(b.arbre));

    settingsList.forEach(setting => {
      const pathSegments = setting.arbre.split('/').slice(1);
      let parentChildren = treeItems;
      let currentPath = topLevel;

      pathSegments.forEach(segment => {
        currentPath += `/${segment}`;
        let node = nodeMap.get(currentPath);
        if (!node) {
          node = {
            id: currentPath,
            title: segment,
            children: [],
          };
          nodeMap.set(currentPath, node);
          if (!parentChildren.some(child => child.id === node.id)) {
            parentChildren.push(node);
          }
        }
        parentChildren = node.children;
      });

      const settingIndex = settings.value.findIndex(s => s.nom === setting.nom);

      parentChildren.push({
        id: setting.nom,
        title: setting.nom,
        isSetting: true,
        settingIndex: settingIndex,
        ordre: setting.ordre, // Make ordre local to the node
      });
    });
    
    const sortNodesRecursively = (nodes) => {
      nodes.sort((a, b) => {
        const getOrder = (node) => {
          if (node.isSetting) {
            return node.ordre;
          }
          const findLowestOrder = (n) => {
            if (n.isSetting) return n.ordre;
            if (n.children && n.children.length > 0) {
              return n.children.map(findLowestOrder).sort((o1, o2) => o1.localeCompare(o2, undefined, { numeric: true }))[0];
            }
            return '999';
          };
          return findLowestOrder(node);
        };
        const orderA = getOrder(a);
        const orderB = getOrder(b);
        return orderA.localeCompare(orderB, undefined, { numeric: true });
      });
      nodes.forEach(node => {
        if (node.children) {
          sortNodesRecursively(node.children);
        }
      });
    };

    sortNodesRecursively(treeItems);
    finalTree[topLevel] = treeItems;
  }
  
  const sortedTopLevelKeys = Object.keys(finalTree).sort((a, b) => {
      const getLowestOrderInTab = (tabItems) => {
          return tabItems.map(item => {
              const getOrder = (node) => {
                  if (node.isSetting) return node.ordre;
                  if (node.children && node.children.length > 0) {
                      return node.children.map(getOrder).sort((o1, o2) => o1.localeCompare(o2, undefined, { numeric: true }))[0];
                  }
                  return '999';
              };
              return getOrder(item);
          }).sort((o1, o2) => o1.localeCompare(o2, undefined, { numeric: true }))[0] || '999';
      };
      const orderA = getLowestOrderInTab(finalTree[a]);
      const orderB = getLowestOrderInTab(finalTree[b]);
      return orderA.localeCompare(orderB, undefined, { numeric: true });
  });

  const orderedFinalTree = {};
  for (const key of sortedTopLevelKeys) {
      orderedFinalTree[key] = finalTree[key];
  }

  return orderedFinalTree;
});

watch(settings, (newSettings) => {
  if (!newSettings || newSettings.length === 0) {
    activeTab.value = null;
    return;
  }

  const tree = settingsTree.value;
  const categories = Object.keys(tree);

  if (categories.length > 0 && (!activeTab.value || !tree[activeTab.value])) {
    activeTab.value = categories[0];
  }

  // Pre-open all list groups
  const newOpenState = [];
  categories.forEach(() => {
    const ids = [];
    const traverse = (nodes) => {
        nodes.forEach(node => {
            if (node.children && node.children.length > 0) {
                ids.push(node.id);
                traverse(node.children);
            }
        });
    };
    traverse(tree[categories[newOpenState.length]]);
    newOpenState.push(ids);
  });
  openState.value = newOpenState;

}, { deep: true });

const loadSettings = async () => {
  const result = await invoke('get_settings');
  const parsedSettings = JSON.parse(result);
  
  settings.value = parsedSettings;
  initialSettings.value = JSON.parse(JSON.stringify(parsedSettings));
};

onMounted(async () => {
  loading.value = true;
  error.value = null;
  try {
    await loadSettings();
  } catch (e) {
    const errorMessage = `Erreur lors du chargement initial des paramètres: ${e}`;
    error.value = errorMessage;
    addMessage(errorMessage, 'error');
  } finally {
    loading.value = false;
  }
});

const isDirty = computed(() => {
  return JSON.stringify(settings.value) !== JSON.stringify(initialSettings.value);
});

const saveAllSettings = async () => {
  isSaving.value = true;
  try {
    await invoke('save_settings', { settings: settings.value });
    await new Promise(resolve => setTimeout(resolve, 100));
    await loadSettings();
    addMessage('Paramètres sauvegardés avec succès', 'success');
  } catch (e) {
    addMessage(`Une erreur est survenue lors de la sauvegarde ou de la relecture: ${e}`, 'error');
  } finally {
    isSaving.value = false;
  }
};
</script>