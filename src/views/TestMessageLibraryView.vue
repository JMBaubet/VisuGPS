<template>
  <v-container fluid class="fill-height d-flex justify-center align-center">
    <v-btn @click="showModal = true">Ouvrir la Bibliothèque de Messages</v-btn>
    <MessageLibraryModal v-model="showModal" @select-message="handleSelectMessage" />
  </v-container>
</template>

<script setup>
import { ref } from 'vue';
import MessageLibraryModal from '@/components/MessageLibraryModal.vue';
import { useSnackbar } from '@/composables/useSnackbar';

const showModal = ref(false);
const { showSnackbar } = useSnackbar();

const handleSelectMessage = (messageId) => {
  showSnackbar(`Message sélectionné : ${messageId}`, 'success');
  showModal.value = false;
};
</script>
