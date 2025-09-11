<template>
  <v-dialog v-model="dialog" max-width="600px">
    <v-card>
      <v-card-title class="headline">Gestion des modes d'exécution</v-card-title>
      <v-card-text>
        <v-list>
          <v-list-item
            v-for="mode in executionModes"
            :key="mode.name"
            :title="mode.name"
            :subtitle="mode.mode_type"
          ></v-list-item>
        </v-list>

        <v-divider class="my-4"></v-divider>

        <v-card-title class="headline">Créer un nouveau mode d'exécution</v-card-title>
        <v-text-field
          label="Nom du mode (ex: EVAL_MaFeature, TEST_MonTest)"
          v-model="newModeName"
          :rules="[rules.required, rules.modeNameFormat]"
        ></v-text-field>
        <v-textarea
          label="Description"
          v-model="newModeDescription"
          rows="2"
        ></v-textarea>
        <v-btn color="primary" @click="createMode">Créer</v-btn>

        <v-divider class="my-4"></v-divider>

        <v-card-title class="headline">Supprimer un mode d'exécution</v-card-title>
        <v-list>
          <v-list-item
            v-for="mode in deletableModes"
            :key="mode.name"
            :title="mode.name"
            :subtitle="mode.mode_type"
          >
            <template v-slot:append>
              <v-btn icon color="error" @click="deleteMode(mode.name)">
                <v-icon>mdi-delete</v-icon>
              </v-btn>
            </template>
          </v-list-item>
        </v-list>

        <v-divider class="my-4"></v-divider>

        <v-card-title class="headline">Sélectionner un mode d'exécution</v-card-title>
        <v-list>
          <v-list-item
            v-for="mode in executionModes"
            :key="mode.name"
            :title="mode.name"
            :subtitle="mode.mode_type"
          >
            <template v-slot:append>
              <v-btn icon color="success" @click="selectMode(mode.name)">
                <v-icon>mdi-check</v-icon>
              </v-btn>
            </template>
          </v-list-item>
        </v-list>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue darken-1" text @click="dialog = false">Fermer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import { useEnvironment } from '@/composables/useEnvironment';
import { confirm } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';

const props = defineProps({
  modelValue: Boolean,
});

const emit = defineEmits(['update:modelValue']);

const dialog = ref(props.modelValue);
const executionModes = ref([]);
const newModeName = ref('');
const newModeDescription = ref('');
const { showSnackbar } = useSnackbar();
const { appEnv } = useEnvironment();

const rules = {
  required: value => !!value || 'Requis.',
  modeNameFormat: value => {
    const evalRegex = /^EVAL_[a-zA-Z0-9_]+$/;
    const testRegex = /^TEST_[a-zA-Z0-9_]+$/;
    if (evalRegex.test(value) || testRegex.test(value)) {
      return true;
    }
    return 'Le nom doit commencer par EVAL_ ou TEST_ suivi de caractères alphanumériques ou _.';
  },
};

const deletableModes = computed(() => {
  return executionModes.value.filter(mode => mode.name !== 'OPE' && mode.name !== appEnv.value);
});

const fetchExecutionModes = async () => {
  try {
    executionModes.value = await invoke('list_execution_modes');
  } catch (error) {
    console.error("Error fetching execution modes:", error);
    showSnackbar('error', `Erreur lors du chargement des modes d'exécution: ${error}`);
  }
};

const createMode = async () => {
  if (!rules.required(newModeName.value) || !rules.modeNameFormat(newModeName.value)) {
    showSnackbar('error', 'Veuillez corriger les erreurs dans le nom du mode.');
    return;
  }

  if (newModeName.value.startsWith('TEST_') && process.env.TAURI_ENV !== 'dev') {
    showSnackbar('error', 'Les modes TEST ne peuvent être créés qu\'en environnement de développement.');
    return;
  }

  try {
    await invoke('create_execution_mode', { modeName: newModeName.value, description: newModeDescription.value });
    showSnackbar('success', `Mode ${newModeName.value} créé avec succès.`);
    newModeName.value = '';
    newModeDescription.value = '';
    fetchExecutionModes(); // Refresh the list

    const shouldRestart = await confirm(
      'Le nouveau mode d\'exécution a été créé. Voulez-vous redémarrer l\'application pour l\'activer ?',
      { title: 'Redémarrer l\'application' }
    );
    if (shouldRestart) {
      await relaunch();
    }

  } catch (error) {
    console.error("Error creating execution mode:", error);
    showSnackbar('error', `Erreur lors de la création du mode: ${error}`);
  }
};

const deleteMode = async (modeName) => {
  if (modeName.startsWith('TEST_') && process.env.TAURI_ENV !== 'dev') {
    showSnackbar('error', 'Les modes TEST ne peuvent être supprimés qu\'en environnement de développement.');
    return;
  }

  try {
    await invoke('delete_execution_mode', { modeName });
    showSnackbar('success', `Mode ${modeName} supprimé avec succès.`);
    fetchExecutionModes(); // Refresh the list
  } catch (error) {
    console.error("Error deleting execution mode:", error);
    showSnackbar('error', `Erreur lors de la suppression du mode: ${error}`);
  }
};

const selectMode = async (modeName) => {
  try {
    await invoke('select_execution_mode', { modeName });
    showSnackbar('success', `Mode ${modeName} sélectionné avec succès.`);

    const shouldRestart = await confirm(
      'Le mode d\'exécution a été modifié. Voulez-vous redémarrer l\'application pour prendre en compte le nouveau mode ?',
      { title: 'Redémarrer l\'application' }
    );
    if (shouldRestart) {
      await relaunch();
    }

  } catch (error) {
    console.error("Error selecting execution mode:", error);
    showSnackbar('error', `Erreur lors de la sélection du mode: ${error}`);
  }
};

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    fetchExecutionModes();
  }
});

watch(dialog, (newVal) => {
  emit('update:modelValue', newVal);
});
</script>

<style scoped>
/* Add any specific styles here */
</style>