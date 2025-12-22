<template>
  <v-dialog v-model="dialog" max-width="800px">
    <v-card>
      <v-card-title class="headline d-flex align-center">
        <span>Bibliothèque de Messages</span>
        <v-spacer></v-spacer>
        <v-text-field
          v-model="filterText"
          label="Filtrer par texte"
          prepend-inner-icon="mdi-magnify"
          variant="solo-filled"
          density="compact"
          hide-details
          single-line
          class="ml-4"
          style="max-width: 250px;"
        ></v-text-field>
        <v-btn color="primary" class="ml-4" @click="openNewMessageDialog">Nouveau Message</v-btn>
      </v-card-title>
      <v-card-text>
        <v-progress-circular v-if="loading" indeterminate color="primary"></v-progress-circular>
        <v-list v-else>
          <v-list-item
            v-for="message in filteredMessages"
            :key="message.id"
          >
            <div
              :style="{
                backgroundColor: toHex(message.style.backgroundColor),
                color: getContrastColor(message.style.backgroundColor),
                padding: '4px 8px',
                borderRadius: '4px',
                display: 'inline-block'
              }"
            >
              {{ message.text }}
            </div>
            <template v-slot:append>
              <v-chip
                v-if="isDevMode"
                :color="message.source === 'user' ? 'primary' : 'warning'"
                class="ml-2"
                size="small"
              >
                {{ message.source === 'user' ? 'Utilisateur' : 'Production' }}
              </v-chip>
              <v-btn
                v-if="isDevMode || message.source !== 'default'"
                icon="mdi-pencil"
                variant="text"
                size="small"
                color="blue"
                @click.stop="openEditMessageDialog(message)"
              ></v-btn>
              <v-btn
                v-if="isDevMode || message.source !== 'default'"
                icon="mdi-delete"
                variant="text"
                size="small"
                color="red"
                @click.stop="confirmDelete(message)"
              ></v-btn>
              <v-btn
                color="primary"
                variant="tonal"
                size="small"
                class="ml-4"
                @click.stop="$emit('select-message', message.id)"
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
      :message="`Êtes-vous sûr de vouloir supprimer le message :<br>'${selectedMessage?.text}' ?`"
      @confirm="deleteMessage"
    />

    <v-dialog v-model="usageDialog" max-width="500px">
      <v-card>
        <v-card-title class="headline bg-warning text-white pa-4">
          <v-icon start color="white">mdi-alert</v-icon>
          Suppression impossible
        </v-card-title>
        <v-card-text class="pa-4">
          Ce message est actuellement utilisé dans les circuits suivants :
          <v-list density="compact" class="mt-2">
            <v-list-item v-for="(circuit, index) in usageList" :key="index">
              <v-list-item-title>• {{ circuit }}</v-list-item-title>
            </v-list-item>
          </v-list>
          <div v-if="usageList.length >= 10" class="text-caption text-grey mt-2">
            (Liste limitée aux 10 premiers circuits trouvés)
          </div>
          <p class="mt-4">
            Vous ne pouvez pas supprimer un message qui est en cours d'utilisation.
          </p>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="primary" @click="usageDialog = false">Compris</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-dialog>
</template>

<script setup>
import { ref, watch, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MessageEditDialog from './MessageEditDialog.vue';
import ConfirmationDialog from './ConfirmationDialog.vue';
import { useVuetifyColors } from '@/composables/useVuetifyColors';
import { useMessages } from '@/composables/useMessages'; // Import useMessages

const props = defineProps({
  modelValue: Boolean,
});

const emit = defineEmits(['update:modelValue', 'select-message']);

const dialog = ref(props.modelValue);
// const messages = ref([]); // Removed local state
const loading = ref(false);
const editDialog = ref(false);
const deleteDialog = ref(false);
const usageDialog = ref(false);
const usageList = ref([]);
const selectedMessage = ref(null);
const filterText = ref('');

const isDevMode = computed(() => import.meta.env.DEV);
const { toHex, getContrastColor } = useVuetifyColors();
const { messages, refreshMessages } = useMessages(); // Use shared state

const getTextForSorting = (text) => {
  const firstLetterMatch = text.match(/\p{L}/u);
  if (firstLetterMatch) {
    return text.substring(firstLetterMatch.index);
  }
  return text;
};

const filteredMessages = computed(() => {
  let msgs = messages.value; // Use shared messages
  if (filterText.value) {
    msgs = msgs.filter(message =>
      getTextForSorting(message.text).toLowerCase().includes(filterText.value.toLowerCase())
    );
  }
  return msgs.slice().sort((a, b) =>
      getTextForSorting(a.text).localeCompare(getTextForSorting(b.text))
  );
});

const fetchMessagesLocal = async () => {
  loading.value = true;
  await refreshMessages();
  loading.value = false;
};

const openNewMessageDialog = () => {
  selectedMessage.value = null;
  editDialog.value = true;
};

const openEditMessageDialog = (message) => {
  selectedMessage.value = message;
  editDialog.value = true;
};

const confirmDelete = async (message) => {
  selectedMessage.value = message;
  loading.value = true;
  try {
    const circuits = await invoke('check_message_usage', { messageId: message.id });
    if (circuits.length > 0) {
      usageList.value = circuits;
      usageDialog.value = true;
    } else {
      deleteDialog.value = true;
    }
  } catch (error) {
    console.error("Erreur lors de la vérification de l'usage du message:", error);
  } finally {
    loading.value = false;
  }
};

const saveMessage = async ({ message, target }) => {
  try {
    // Convert 'Utilisateur'/'Production' back to 'user'/'default' for backend
    const backendTarget = target === 'Utilisateur' ? 'user' : 'default';
    await invoke('save_message', { newMessage: message, target: backendTarget });
    fetchMessagesLocal(); // Refresh shared state
  } catch (error) {
    console.error("Erreur lors de la sauvegarde du message:", error);
  }
};

const deleteMessage = async () => {
  if (!selectedMessage.value) return;
  try {
    await invoke('delete_message', { id: selectedMessage.value.id, target: selectedMessage.value.source });
    fetchMessagesLocal(); // Refresh shared state
  } catch (error) {
    console.error("Erreur lors de la suppression du message:", error);
  }
};


watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    fetchMessagesLocal();
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
