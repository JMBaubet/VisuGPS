import { computed } from 'vue';

export function useCommuneColor(progress) {
    const color = computed(() => {
        const p = progress.value;
        if (p <= 7) return 'red-darken-2';
        if (p <= 13) return 'orange-darken-2';
        if (p <= 25) return 'amber-darken-2';
        if (p <= 50) return 'yellow-darken-3';
        if (p <= 75) return 'light-green-darken-2';
        return 'green-darken-2';
    });
    return { color };
}
