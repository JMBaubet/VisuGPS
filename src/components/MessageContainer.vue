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
import MessageBanner from "./MessageBanner.vue";
import { useMessageStore } from '../composables/useMessageStore';

const { messages, removeMessage } = useMessageStore();
</script>

<style scoped>
.message-container {
  position: fixed;
  top: 64px; /* Adjusted from 16px */
  right: 16px;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 480px;
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