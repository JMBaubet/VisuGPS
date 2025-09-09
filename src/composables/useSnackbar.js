import { ref } from 'vue';

const snackbar = ref({
  show: false,
  message: '',
  color: '',
  timeout: 3000,
});

export function useSnackbar() {
  const showSnackbar = (message, color = 'info', timeout = 3000) => {
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