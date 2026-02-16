import { computed } from 'vue';
import { useSettings } from './useSettings';

export function useDebug() {
    const { getSettingValue } = useSettings();

    const isDebugMode = computed(() => {
        // True if in development environment OR if the Debug setting is enabled
        return import.meta.env.DEV || getSettingValue('Système.Debug') === true;
    });

    return {
        isDebugMode
    };
}
