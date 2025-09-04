import { ref, reactive, nextTick } from 'vue';

// Store state as a reactive object
const store = reactive({
  snackbar: {
    show: false,
    text: '',
    color: '',
    timeout: 3000,
  },
  queue: [],
});

// Private function to process the message queue
const processQueue = () => {
  if (store.queue.length > 0 && !store.snackbar.show) {
    const message = store.queue.shift();
    store.snackbar.text = message.text;
    store.snackbar.timeout = message.timeout || 3000;

    switch (message.type) {
      case 'success':
        store.snackbar.color = 'success';
        break;
      case 'warning':
        store.snackbar.color = 'warning';
        break;
      case 'error':
        store.snackbar.color = 'error';
        break;
      default:
        store.snackbar.color = 'info';
        break;
    }
    store.snackbar.show = true;
  }
};

// This function is called when the snackbar's v-model becomes false
const onHidden = () => {
  // We use nextTick to wait for the v-model to update before processing the next message.
  nextTick(() => {
    processQueue(); // Try to show the next message
  });
};

// The composable that components will use
export function useMessageStore() {
  const addMessage = (text, type = 'info', timeout = 3000) => {
    store.queue.push({ text, type, timeout });
    processQueue(); // Try to show immediately if nothing is displayed
  };

  const clearMessages = () => {
    store.queue = [];
    store.snackbar.show = false;
  };

  return {
    snackbar: store.snackbar,
    addMessage,
    clearMessages,
    onHidden,
  };
}
