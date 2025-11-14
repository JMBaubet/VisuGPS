<template>
  <v-dialog v-model="dialog" max-width="500px" persistent>
    <v-card>
      <v-card-title>
        <span class="headline">{{ formTitle }}</span>
      </v-card-title>

      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12">
              <v-text-field v-model="editableMessage.text" label="Texte du message"></v-text-field>
            </v-col>
            <v-col cols="12" sm="6">
              <v-text-field v-model="editableMessage.style.backgroundColor" label="Couleur de fond"></v-text-field>
            </v-col>
            <v-col cols="12" sm="6">
              <v-text-field v-model="editableMessage.style.textColor" label="Couleur du texte"></v-text-field>
            </v-col>
            <v-col cols="12">
              <v-select
                v-model="editableMessage.style.shape"
                :items="['skewed-rect', 'rounded-rect', 'circle']"
                label="Forme"
              ></v-select>
            </v-col>
            <v-col cols="12" v-if="isDevMode">
              <v-select
                v-model="target"
                :items="['user', 'default']"
                label="Destination"
                hint="Visible en mode DEV uniquement"
                persistent-hint
              ></v-select>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="close">Annuler</v-btn>
        <v-btn color="blue-darken-1" variant="text" @click="save">Sauvegarder</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed } from 'vue';

const props = defineProps({
  modelValue: Boolean,
  message: Object,
});

const emit = defineEmits(['update:modelValue', 'save']);

const dialog = ref(props.modelValue);
const editableMessage = ref({
  text: '',
  style: {
    backgroundColor: '',
    textColor: 'white',
    shape: 'skewed-rect',
  },
});
const target = ref('user'); // 'user' or 'default'

const isDevMode = computed(() => import.meta.env.DEV);
const formTitle = computed(() => (editableMessage.value.id ? 'Modifier le Message' : 'Nouveau Message'));

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    if (props.message) {
      // Deep copy to avoid modifying the original object directly
      editableMessage.value = JSON.parse(JSON.stringify(props.message));
      target.value = props.message.source || 'user';
    } else {
      // Reset for new message
      editableMessage.value = {
        text: '',
        style: {
          backgroundColor: 'blue-darken-2',
          textColor: 'white',
          shape: 'skewed-rect',
        },
      };
      target.value = 'user';
    }
  }
});

watch(dialog, (newVal) => {
  emit('update:modelValue', newVal);
});

const close = () => {
  dialog.value = false;
};

const save = () => {
  emit('save', { message: editableMessage.value, target: target.value });
  close();
};
</script>
