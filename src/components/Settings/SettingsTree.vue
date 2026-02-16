<template>
  <v-card color="transparent" flat class="settings-container">
    <v-row no-gutters class="fill-height">
      <!-- Onglets verticaux du premier niveau (Racines) -->
      <v-col cols="2" md="2" lg="1">
        <v-tabs
          v-model="activeTab"
          direction="vertical"
          color="primary"
          class="settings-tabs"
        >
          <template v-if="settings && settings.data && settings.data.groupes">
            <v-tab
              v-for="group in settings.data.groupes"
              :key="group.libelle"
              :value="group.libelle"
              class="justify-start text-none tab-item"
            >
              <v-icon start size="small">mdi-folder-outline</v-icon>
              <span class="text-body-2 font-weight-medium text-truncate">{{ group.libelle }}</span>
              <v-tooltip activator="parent" location="end">{{ group.libelle }}</v-tooltip>
            </v-tab>
          </template>
        </v-tabs>
      </v-col>

      <!-- Contenu des onglets -->
      <v-col cols="10" md="10" lg="11">
        <v-window v-model="activeTab" class="settings-window">
          <template v-if="settings && settings.data && settings.data.groupes">
            <v-window-item
              v-for="group in settings.data.groupes"
              :key="group.libelle"
              :value="group.libelle"
              class="fill-height"
            >
              <div class="scrolling-content pa-0">
                <SettingsNode 
                  :node="group" 
                  currentPath="" 
                  :is-root="true"
                  @settings-updated="initSettings"
                />
              </div>
            </v-window-item>
          </template>
          <v-window-item v-else value="loading">
             <div class="pa-4">Chargement des paramètres...</div>
          </v-window-item>
        </v-window>
      </v-col>
    </v-row>
  </v-card>
</template>

<script setup>
import { ref, watch } from 'vue';
import { useSettings } from '@/composables/useSettings';
import SettingsNode from './SettingsNode.vue';

const { settings, initSettings } = useSettings();
const activeTab = ref(null);

</script>

<style scoped>
.settings-container {
  height: 80vh;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 8px;
  overflow: hidden;
}

.settings-tabs {
  border-right: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  height: 100%;
  overflow-y: auto;
}

.settings-window {
  height: 100%;
}

.scrolling-content {
  height: 100%;
  overflow-y: auto;
}

.tab-item {
  min-width: 0;
  padding: 0 16px;
  height: 56px !important;
  border-bottom: 1px solid rgba(var(--v-border-color), 0.05);
}

:deep(.v-tab__slider) {
  width: 4px;
}

:deep(.v-tab--selected) {
  background-color: rgba(var(--v-theme-primary), 0.08);
}
</style>
