import { ref } from 'vue';

const messages = ref([]);
let idCounter = 0;

export function useMessageStore() {
  const addMessage = ({ message, type = "info", duration = 5000 }) => {
    const id = ++idCounter;
    messages.value.push({ id, message, type, duration });
    return id;
  };

  const removeMessage = (id) => {
    messages.value = messages.value.filter((m) => m.id !== id);
  };

  return { messages, addMessage, removeMessage };
}
