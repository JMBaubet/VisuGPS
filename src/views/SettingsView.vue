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
      <v-col cols="12" md="8" lg="9">
        <v-card class="mb-6">
          <v-card-title>Paramètres de l'application</v-card-title>
          <v-card-text>
            <div v-if="loading" class="text-center">
              <v-progress-circular indeterminate color="primary"></v-progress-circular>
            </div>
            <div v-else-if="error" class="text-center">
              <v-alert type="error">{{ error }}</v-alert>
            </div>
            <div v-else class="d-flex flex-column ga-4">
              <template v-for="(setting, index) in settings" :key="`${templateKey}-${setting.nom}`">
                <!-- Group Header -->
                <h2 v-if="isNewGroup(index)" class="text-h6 mt-6 mb-2 text-grey-darken-1">{{ setting.arbre }}</h2>

                <!-- Setting Editors -->
                <StringSetting 
                  v-if="setting.type === 'string'"
                  v-model="settings[index]"
                />
                
                <div v-if="setting.type !== 'string'" class="text-caption text-disabled ml-12">Éditeur pour le type '{{ setting.type }}' non implémenté.</div>
              </template>
            </div>
          </v-card-text>
        </v-card>

        <v-card>
           <v-card-title>Token Mapbox</v-card-title>
           <v-card-text>
              <MapboxTokenEditor />
           </v-card-text>
        </v-card>

      </v-col>

      <v-col cols="12" md="4" lg="3">
        <!-- Potential help/info column -->
      </v-col>
    </v-row>

  </v-container>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useMessageStore } from '../composables/useMessageStore.js';
import StringSetting from '../components/StringSetting.vue';
import MapboxTokenEditor from '../components/MapboxTokenEditor.vue';

const { addMessage } = useMessageStore();

const settings = ref([]);
const initialSettings = ref([]); // To track changes
const loading = ref(true);
const isSaving = ref(false);
const error = ref(null);
const templateKey = ref(0);

// This function now throws errors to be caught by the caller.
const loadSettings = async () => {
  const result = await invoke('get_settings');
  const parsedSettings = JSON.parse(result);
  
  parsedSettings.sort((a, b) => (a.arbre || '').localeCompare(b.arbre || ''));

  settings.value = parsedSettings;
  initialSettings.value = JSON.parse(JSON.stringify(parsedSettings));
};

// Initial load on component mount, with its own error handling.
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

const isNewGroup = (index) => {
  if (index === 0) return true;
  return settings.value[index].arbre !== settings.value[index - 1].arbre;
};

// Unified save and reload logic with single point of error handling.
const saveAllSettings = async () => {
  isSaving.value = true;
  try {
    await invoke('save_settings', { settings: settings.value });
    
    // Wait a fraction of a second to ensure the file system has caught up.
    await new Promise(resolve => setTimeout(resolve, 100));

    await loadSettings();
    templateKey.value++;

    // Only show success if both save and reload are successful.
    addMessage('Paramètres sauvegardés avec succès', 'success');
  } catch (e) {
    addMessage(`Une erreur est survenue lors de la sauvegarde ou de la relecture: ${e}`, 'error');
  } finally {
    isSaving.value = false;
  }
};

</script>
