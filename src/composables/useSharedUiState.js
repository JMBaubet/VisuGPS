import { ref } from 'vue';

// State shared across components
const isBackButtonVisible = ref(true);

export function useSharedUiState() {

  const toggleBackButtonVisibility = () => {
    isBackButtonVisible.value = !isBackButtonVisible.value;
  };

  return {
    isBackButtonVisible,
    toggleBackButtonVisibility,
  };
}
