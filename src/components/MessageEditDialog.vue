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
            <v-col cols="12">
              <v-select
                v-model="editableMessage.style.backgroundColor"
                :items="materialColors"
                label="Couleur de fond"
                :bg-color="toHex(editableMessage.style.backgroundColor)"
                :color="getContrastColor(editableMessage.style.backgroundColor)"
                class="color-select"
              >
                <template v-slot:selection="{ item }">
                  <span :style="{ color: getContrastColor(item.value) }">{{ item.value }}</span>
                </template>
                <template v-slot:item="{ props, item }">
                  <v-list-item v-bind="props" title="">
                    <div class="w-100 pa-4" :style="{ backgroundColor: toHex(item.value), border: '1px solid #9E9E9E' }"></div>
                  </v-list-item>
                </template>
              </v-select>
            </v-col>
            <v-col cols="12" v-if="isDevMode">
              <v-select
                v-model="target"
                :items="['Utilisateur', 'Production']"
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
import { useTheme } from 'vuetify';
import { useVuetifyColors } from '@/composables/useVuetifyColors'; // Import added

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
  },
});
const target = ref('Production'); // 'user' or 'default'

const theme = useTheme();
const { toHex, hexToRgb, getContrastColor } = useVuetifyColors(); // Initialize toHex, hexToRgb, and getContrastColor

const materialColors = [
  'red', 'pink', 'purple', 'deep-purple', 'indigo', 'blue', 'light-blue', 'cyan', 'teal', 'green', 'light-green', 'lime', 'yellow', 'amber', 'orange', 'deep-orange', 'brown', 'grey', 'blue-grey',
  'black', 'white'
];

const isDevMode = computed(() => import.meta.env.DEV);
const formTitle = computed(() => (editableMessage.value.id ? 'Modifier le Message' : 'Nouveau Message'));

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
    if (props.message) {
      // Deep copy to avoid modifying the original object directly
      editableMessage.value = JSON.parse(JSON.stringify(props.message));
      target.value = props.message.source || 'Production';
    } else {
      // Reset for new message
      editableMessage.value = {
        text: '',
        style: {
          backgroundColor: 'blue', // Default to 'blue'
          textColor: 'white',
        },
      };
      target.value = 'Production';
    }
  }
});

watch(() => editableMessage.value.style.backgroundColor, (newColorName) => {
  editableMessage.value.style.textColor = getContrastColor(newColorName);
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
