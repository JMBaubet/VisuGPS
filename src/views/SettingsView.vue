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
            <v-card-text>
              <v-treeview
                :items="settings"
                item-key="title"
                item-text="title"
                open-on-click
                activatable
                return-object
              >
                <template v-slot:label="{ item }">
                  {{ item.title }}
                </template>
                <template v-slot:append="{ item }">
                  <!-- Display parameters if they exist -->
                  <v-container v-if="item.parametres && item.parametres.length > 0">
                    <v-row v-for="parametre in item.parametres" :key="parametre.nom">
                      <v-col cols="12">
                        <v-card outlined class="mb-2">
                          <v-card-title>{{ parametre.nom }}</v-card-title>
                          <v-card-subtitle>{{ parametre.description }}</v-card-subtitle>
                          <v-card-text>
                            <!-- Dynamic input component based on type -->
                            <StringSetting
                              v-if="parametre.type === 'string'"
                              :label="parametre.nom"
                              :model-value="parametre.valeur_par_defaut"
                              @update:model-value="parametre.valeur_de_surcharge = $event"
                            />
                            <v-checkbox
                              v-else-if="parametre.type === 'boolean'"
                              :label="parametre.nom"
                              :model-value="parametre.valeur_par_defaut"
                              @update:model-value="parametre.valeur_de_surcharge = $event"
                            />
                            <v-text-field
                              v-else-if="parametre.type === 'number'"
                              :label="parametre.nom"
                              type="number"
                              :model-value="parametre.valeur_par_defaut"
                              @update:model-value="parametre.valeur_de_surcharge = $event"
                            />
                            <v-text-field
                              v-else-if="parametre.type === 'color'"
                              :label="parametre.nom"
                              type="color"
                              :model-value="parametre.valeur_par_defaut"
                              @update:model-value="parametre.valeur_de_surcharge = $event"
                            />
                            <div v-else class="text-caption text-disabled">
                              Éditeur pour le type '{{ parametre.type }}' non implémenté.
                            </div>
                          </v-card-text>
                        </v-card>
                      </v-col>
                    </v-row>
                  </v-container>
                </template>
              </v-treeview>
            </v-card-text>
          </v-card>
        </div>
      </v-col>
    </v-row>

  </v-container>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'; // Removed watch
import { invoke } from '@tauri-apps/api/core';
import { useMessageStore } from '../composables/useMessageStore.js';
// Removed SettingsTreeItem import
import StringSetting from '../components/StringSetting.vue'; // Still needed for parameter types
// Removed MapboxTokenEditor import (it was commented out in template anyway)

const { addMessage } = useMessageStore();

const settings = ref([]); // This will now hold the array of top-level nodes
const initialSettings = ref([]); // Keep for future save logic
const loading = ref(true);
const isSaving = ref(false);
const error = ref(null);

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

// isDirty and saveAllSettings are commented out as save_settings command is removed
const isDirty = computed(() => {
  // return JSON.stringify(settings.value) !== JSON.stringify(initialSettings.value);
  return false; // Always false for now
});

const saveAllSettings = async () => {
  isSaving.value = true;
  try {
    // await invoke('save_settings', { settings: settings.value }); 
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