import { ref } from 'vue';

const snackbar = ref({
  show: false,
  message: '',
  color: '',
  timeout: 5000, // Changed to 5000
});

export function useSnackbar() {
  const showSnackbar = (message, color = 'info', timeout = 5000) => { // Changed to 5000
    snackbar.value.show = true;
    snackbar.value.message = message;
    snackbar.value.color = color;
    snackbar.value.timeout = timeout;
  };

  return {
    snackbar,
    showSnackbar,
  };
}