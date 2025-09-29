<template>
    <v-card class="pa-2" elevation="2">
        <v-row align="center" no-gutters>
            <v-col cols="auto">
                <v-switch v-model="showEvents" label="Évènements" color="primary" hide-details inset></v-switch>
            </v-col>
            <v-col>
                <v-expand-transition>
                    <div v-if="showEvents">
                        <v-btn v-if="isPauseEvent" icon="mdi-delete" color="error" variant="text"
                            @click="onDeletePause" title="Supprimer la pause"></v-btn>
                    </div>
                </v-expand-transition>
            </v-col>
        </v-row>
    </v-card>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
    currentIncrement: {
        type: Number,
        required: true,
    },
    pauseEvents: {
        type: Array,
        required: true,
    },
});

const emit = defineEmits(['delete-pause']);

const showEvents = ref(false);

const isPauseEvent = computed(() => {
    return props.pauseEvents.includes(props.currentIncrement);
});

const onDeletePause = () => {
    emit('delete-pause');
};
</script>
