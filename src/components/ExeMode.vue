<template>
  <v-dialog v-model="dialog" max-width="600px">
    <v-card>
      <v-card-title class="headline">Gestion des modes d'exécution</v-card-title>
      <v-card-text>
        <v-list lines="one" class="mb-4">
            <v-list-item
                v-for="mode in sortedAndStyledModes"
                :key="mode.name"
            >
                <template v-slot:title>
                    <span :class="`text-${mode.color} font-weight-bold`">{{ mode.name }}</span>
                    <div v-if="mode.description" class="text-caption text-grey">{{ mode.description }}</div>
                </template>

                <template v-slot:append>
                    <v-chip v-if="mode.name === appEnv" color="primary" variant="elevated" size="small" class="ml-2">Actif</v-chip>

                    <!-- Delete Button -->
                    <v-btn
                        v-if="deletableModes.some(m => m.name === mode.name)"
                        icon="mdi-delete"
                        variant="text"
                        color="error"
                        @click="deleteMode(mode.name)"
                        class="ml-2"
                        title="Supprimer ce mode"
                    ></v-btn>

                    <!-- Select Button -->
                    <v-btn
                        v-if="mode.name !== appEnv"
                        icon="mdi-check"
                        variant="text"
                        color="success"
                        @click="selectMode(mode.name)"
                        class="ml-2"
                        title="Sélectionner ce mode"
                    ></v-btn>
                </template>
            </v-list-item>
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
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue darken-1" text @click="dialog = false">Fermer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <RestartConfirmationDialog
    v-model="showRestartDialog"
    :title="restartDialogTitle"
    :message="restartDialogMessage"
    @confirmed="handleRestartConfirmed"
    @cancelled="handleRestartCancelled"
  />

  <ConfirmationDialog
    v-model="showDeleteConfirmDialog"
    title="Supprimer le mode"
    :message="`Êtes-vous sûr de vouloir supprimer le mode d'exécution <strong>${modeToDelete}</strong> ?<br>Cette action est irréversible.`"
    confirmText="Supprimer"
    cancelText="Annuler"
    @confirm="confirmDeleteMode"
  />
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSnackbar } from '@/composables/useSnackbar';
import { useEnvironment } from '@/composables/useEnvironment';
import { relaunch } from '@tauri-apps/plugin-process';
import RestartConfirmationDialog from './RestartConfirmationDialog.vue';
import ConfirmationDialog from './ConfirmationDialog.vue';

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

const showRestartDialog = ref(false);
const restartDialogTitle = ref('');
const restartDialogMessage = ref('');
let restartPromiseResolve = null;

const showDeleteConfirmDialog = ref(false);
const modeToDelete = ref('');

const rules = {
  required: value => !!value || 'Requis.',
  modeNameFormat: value => {
    const evalRegex = /^EVAL_[a-zA-Z0-9_]+$/;
    const testRegex = /^TEST_[a-zA-Z0-9_]+$/;

    if (import.meta.env.PROD) {
      if (evalRegex.test(value)) {
        return true;
      }
      return 'En mode production, le nom doit commencer par EVAL_ suivi de caractères alphanumériques ou _.';
    } else { // DEV mode
      if (evalRegex.test(value) || testRegex.test(value)) {
        return true;
      }
      return 'Le nom doit commencer par EVAL_ ou TEST_ suivi de caractères alphanumériques ou _.';
    }
  },
};

const deletableModes = computed(() => {
  return executionModes.value.filter(mode => mode.name !== 'OPE' && mode.name !== appEnv.value);
});

const sortedAndStyledModes = computed(() => {
  if (!executionModes.value) return [];

  const getModeStyle = (modeName) => {
    if (modeName === 'OPE') {
      return { color: 'success', order: 1 };
    }
    if (modeName.startsWith('EVAL_')) {
      return { color: 'info', order: 2 };
    }
    if (modeName.startsWith('TEST_')) {
      return { color: 'warning', order: 3 };
    }
    return { color: undefined, order: 4 }; // Default
  };

  let modes = [...executionModes.value]; // Create a shallow copy

  // In production, filter out TEST modes
  if (import.meta.env.PROD) {
    modes = modes.filter(mode => !mode.name.startsWith('TEST_'));
  }

  return modes
    .map(mode => ({
      ...mode,
      ...getModeStyle(mode.name),
    }))
    .sort((a, b) => {
      if (a.order !== b.order) {
        return a.order - b.order;
      }
      return a.name.localeCompare(b.name); // Alphabetical sort within the same type
    });
});

const fetchExecutionModes = async () => {
  try {
    executionModes.value = await invoke('list_execution_modes');
  } catch (error) {
    console.error("Error fetching execution modes:", error);
    showSnackbar(`Erreur lors du chargement des modes d'exécution: ${error.message || error}`, 'error');
  }
};

const createMode = async () => {
  if (!rules.required(newModeName.value) || !rules.modeNameFormat(newModeName.value)) {
    showSnackbar('Veuillez corriger les erreurs dans le nom du mode.', 'error');
    return;
  }

  if (newModeName.value.startsWith('TEST_') && !import.meta.env.DEV) {
    showSnackbar('Les modes TEST ne peuvent être créés qu\'en environnement de développement.', 'error');
    return;
  }

  try {
    await invoke('create_execution_mode', { modeName: newModeName.value, description: newModeDescription.value });
    const createdModeName = newModeName.value;
    newModeName.value = '';
    newModeDescription.value = '';
    fetchExecutionModes(); // Refresh the list
    showSnackbar(`Pour utiliser le mode d'exécution '${createdModeName}' veuillez le sélectionner dans la liste.`, 'info');

   

    const shouldRestart = await new Promise(resolve => {
      restartPromiseResolve = resolve;
    });

    if (shouldRestart) {
      if (import.meta.env.DEV) {
        showSnackbar('Redémarrage requis. En mode DEV, veuillez relancer la commande "npm run tauri dev".', 'error', 30000);
        setTimeout(async () => {
             await exit(0);
        }, 30000);
      } else {
        await relaunch();
      }
    }

  } catch (error) {
    console.error("Error creating execution mode:", error);
    showSnackbar(`Erreur lors de la création du mode: ${error.message || error}`, 'error');
  }
};

const deleteMode = async (modeName) => {
  if (modeName.startsWith('TEST_') && !import.meta.env.DEV) {
    showSnackbar('Les modes TEST ne peuvent être supprimés qu\'en environnement de développement.', 'error');
    return;
  }

  modeToDelete.value = modeName;
  showDeleteConfirmDialog.value = true;
};

const confirmDeleteMode = async () => {
  const modeName = modeToDelete.value;

  try {
    await invoke('delete_execution_mode', { modeName });
    showSnackbar(`Le mode d'exécution '${modeName}' a été supprimé avec succès.`, 'success');
    fetchExecutionModes(); // Refresh the list
  } catch (error) {
    console.error("Error deleting execution mode:", error);
    showSnackbar(`Erreur lors de la suppression du mode: ${error.message || error}`, 'error');
  }
};

const selectMode = async (modeName) => {
  try {
    await invoke('select_execution_mode', { modeName });
    showSnackbar(`Mode ${modeName} sélectionné avec succès.`, 'success');

    restartDialogTitle.value = 'Redémarrer l\'application';
    restartDialogMessage.value = 'Le mode d\'exécution est en cours de modification. Vous devez redémarrer l\'application pour prendre en compte le nouveau mode. Confirmez-vous le changement de mode et le redémarrage ?';
    showRestartDialog.value = true;

    const shouldRestart = await new Promise(resolve => {
      restartPromiseResolve = resolve;
    });

    if (shouldRestart) {
      if (import.meta.env.DEV) {
        showSnackbar('Redémarrage requis. En mode DEV, veuillez relancer la commande "npm run tauri dev".', 'error', 30000);
        setTimeout(async () => {
             await exit(0);
        }, 30000);
      } else {
        await relaunch();
      }
    }

  } catch (error) {
    console.error("Error selecting execution mode:", error);
    showSnackbar(`Erreur lors de la sélection du mode: ${error.message || error}`, 'error');
  }
};

const handleRestartConfirmed = () => {
  if (restartPromiseResolve) {
    restartPromiseResolve(true);
  }
};

const handleRestartCancelled = () => {
  if (restartPromiseResolve) {
    restartPromiseResolve(false);
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