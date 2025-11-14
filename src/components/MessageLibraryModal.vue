<template>
  <v-dialog v-model="dialog" max-width="800px">
    <v-card>
      <v-card-title class="headline d-flex justify-space-between align-center">
        <span>Bibliothèque de Messages</span>
        <v-btn color="primary" @click="openNewMessageDialog">Nouveau Message</v-btn>
      </v-card-title>
      <v-card-text>
        <v-progress-circular v-if="loading" indeterminate color="primary"></v-progress-circular>
        <v-list v-else>
          <v-list-item
            v-for="message in messages"
            :key="message.id"
          >
            <v-list-item-title>{{ message.text }}</v-list-item-title>
            <template v-slot:append>
              <v-chip
                :color="message.style.backgroundColor"
                :text-color="message.style.textColor"
                class="ml-2"
              >
                {{ message.style.shape }}
              </v-chip>
              <v-chip
                :color="message.source === 'user' ? 'blue' : 'grey'"
                class="ml-2"
                size="small"
              >
                {{ message.source }}
              </v-chip>
              <v-btn
                icon="mdi-pencil"
                variant="text"
                size="small"
                @click="openEditMessageDialog(message)"
                :disabled="!isDevMode && message.source === 'default'"
              ></v-btn>
              <v-btn
                icon="mdi-delete"
                variant="text"
                size="small"
                @click="confirmDelete(message)"
                :disabled="!isDevMode && message.source === 'default'"
              ></v-btn>
              <v-btn
                color="primary"
                variant="tonal"
                size="small"
                class="ml-4"
                @click="$emit('select-message', message.id)"
              >
                Sélectionner
              </v-btn>
            </template>
          </v-list-item>
        </v-list>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="closeDialog">Fermer</v-btn>
      </v-card-actions>
    </v-card>

    <MessageEditDialog
      v-model="editDialog"
      :message="selectedMessage"
      @save="saveMessage"
    />

    <ConfirmationDialog
      v-model="deleteDialog"
      title="Confirmer la suppression"
      text="Êtes-vous sûr de vouloir supprimer ce message ?"
      @confirm="deleteMessage"
    />

  </v-dialog>
</template>

<script setup>
import { ref, watch, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MessageEditDialog from './MessageEditDialog.vue';
import ConfirmationDialog from './ConfirmationDialog.vue'; // Assuming this component exists

const props = defineProps({
  modelValue: Boolean,
});

const emit = defineEmits(['update:modelValue', 'select-message']);

const dialog = ref(props.modelValue);
const messages = ref([]);
const loading = ref(false);
const editDialog = ref(false);
const deleteDialog = ref(false);
const selectedMessage = ref(null);

const isDevMode = computed(() => import.meta.env.DEV);

const fetchMessages = async () => {
  loading.value = true;
  try {
    messages.value = await invoke('get_message_library');
  } catch (error) {
    console.error("Erreur lors de la récupération de la bibliothèque de messages:", error);
  } finally {
    loading.value = false;
  }
};

const openNewMessageDialog = () => {
  selectedMessage.value = null;
  editDialog.value = true;
};

const openEditMessageDialog = (message) => {
  selectedMessage.value = message;
  editDialog.value = true;
};

const confirmDelete = (message) => {
  selectedMessage.value = message;
  deleteDialog.value = true;
};

const saveMessage = async ({ message, target }) => {
  try {
    await invoke('save_message', { newMessage: message, target });
    fetchMessages(); // Refresh the list
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du message:", error);
  }
};

const deleteMessage = async () => {
  if (!selectedMessage.value) return;
  try {
    await invoke('delete_message', { id: selectedMessage.value.id, target: selectedMessage.value.source });
    fetchMessages(); // Refresh the list
  } catch (error) {
    console.error("Erreur lors de la suppression du message:", error);
  }
};


watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    fetchMessages();
  }
});

watch(dialog, (newVal) => {
  emit('update:modelValue', newVal);
});

onMounted(() => {
  if (dialog.value) {
    fetchMessages();
  }
});

const closeDialog = () => {
  dialog.value = false;
};
</script>

<style scoped>
/* Add any specific styles here */
</style>
