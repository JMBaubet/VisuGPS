<template>
  <div class="message-container">
    <transition-group name="fade" tag="div">
      <MessageBanner
        v-for="msg in messages"
        :key="msg.id"
        :message="msg.message"
        :type="msg.type"
        :duration="msg.duration"
        @expired="removeMessage(msg.id)"
      />
    </transition-group>
  </div>
</template>

<script setup>
import { ref } from "vue";
import MessageBanner from "./MessageBanner.vue";

const messages = ref([]);

let idCounter = 0;

// Fonction publique pour ajouter un message
function addMessage({ message, type = "info", duration = 5000 }) {
  const id = ++idCounter;
  messages.value.push({ id, message, type, duration });
  return id;
}

// Supprimer un message
function removeMessage(id) {
  messages.value = messages.value.filter((m) => m.id !== id);
}

// On expose addMessage au parent (ex: App.vue)
defineExpose({ addMessage });
</script>

<style scoped>
.message-container {
  position: fixed;
  top: 16px;
  right: 16px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 640px;
}

.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}
.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
