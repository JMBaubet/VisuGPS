import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useEnvironment() {
  const appEnv = ref(null)
  const executionMode = ref(null)
  const appEnvPath = ref(null)
  const isLoading = ref(true)
  const error = ref(null)

  async function fetchEnvironment() {
    try {
      const state = await invoke('get_app_state')
      appEnv.value = state.app_env
      executionMode.value = state.execution_mode
      appEnvPath.value = state.app_env_path
    } catch (e) {
      error.value = e
    } finally {
      isLoading.value = false
    }
  }

  onMounted(fetchEnvironment)

  return {
    appEnv,
    executionMode,
    appEnvPath,
    isLoading,
    error,
    fetchEnvironment
  }
}