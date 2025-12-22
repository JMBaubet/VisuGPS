<template>
  <v-dialog :model-value="show" @update:model-value="$emit('update:show', $event)" persistent max-width="600px">
    <v-card v-if="parameter">
      <v-card-title class="d-flex justify-space-between align-center">
        <span class="text-h5" :class="{ 'text-warning': parameter.critique }">{{ parameter.libelle }}</span>
        <v-icon
          v-if="parameter.doc"
          color="info"
          @click="showDocDialog = true"
          title="Afficher la documentation"
        >mdi-book-open-page-variant-outline</v-icon>
      </v-card-title>
      <v-card-subtitle>{{ parameter.description }}</v-card-subtitle>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="12">
              <v-row class="ma-0" align="center">
                
                <!-- Default Value -->
                <v-col cols="12" class="pa-0 mb-4">
                  <v-label class="text-caption font-weight-light mb-1 text-white d-block">Valeur par défaut :</v-label>
                  <div v-if="defaultMessage" 
                       :style="{
                         backgroundColor: toHex(defaultMessage.style.backgroundColor),
                         color: getContrastColor(defaultMessage.style.backgroundColor),
                         padding: '4px 8px',
                         borderRadius: '4px',
                         display: 'inline-block'
                       }">
                    {{ defaultMessage.text }}
                  </div>
                  <span v-else class="text-caption font-italic text-grey">Message introuvable (ID: {{ parameter.defaut }})</span>
                </v-col>

                <!-- Current Value -->
                <v-col cols="10" class="pa-0">
                  <v-label class="text-caption font-weight-light mb-1 text-white d-block">Valeur actuelle :</v-label>
                  <div v-if="currentMessage" 
                       :style="{
                         backgroundColor: toHex(currentMessage.style.backgroundColor),
                         color: getContrastColor(currentMessage.style.backgroundColor),
                         padding: '4px 8px',
                         borderRadius: '4px',
                         display: 'inline-block'
                       }">
                    {{ currentMessage.text }}
                  </div>
                  <span v-else class="text-caption font-italic text-grey">Message introuvable (ID: {{ currentValue }})</span>
                </v-col>

                 <!-- Actions -->
                <v-col cols="2" class="d-flex justify-center align-center">
                   <v-btn icon="mdi-pencil" variant="text" color="primary" @click="openLibrary"></v-btn>
                   <v-icon v-if="hasSurcharge" @click="removeSurcharge" title="Supprimer la surcharge" color="info" class="ml-2">mdi-format-color-marker-cancel</v-icon>
                </v-col>

              </v-row>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue-darken-1" variant="text" @click="closeDialog">
          Fermer
        </v-btn>
      </v-card-actions>
    </v-card>

    <!-- Message Library Modal -->
    <MessageLibraryModal
      v-model="isLibraryVisible"
      @select-message="handleMessageSelect"
    />

    <!-- Doc Dialog -->
    <v-dialog v-model="showDocDialog" max-width="800px">
      <DocDisplay :doc-path="parameter.doc" @close="showDocDialog = false" />
    </v-dialog>
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed, defineProps, defineEmits, onMounted } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useMessages } from '@/composables/useMessages';
import MessageLibraryModal from './MessageLibraryModal.vue';
import DocDisplay from './DocDisplay.vue';

const props = defineProps({
  show: Boolean,
  parameter: Object,
  groupPath: String,
});

const emit = defineEmits(['update:show']);
const { updateSetting } = useSettings();
const { fetchMessages, getMessage, toHex, getContrastColor } = useMessages();

const showDocDialog = ref(false);
const isLibraryVisible = ref(false);

const hasSurcharge = computed(() => props.parameter.surcharge != null && props.parameter.surcharge !== '');
const currentValue = computed(() => props.parameter.surcharge || props.parameter.defaut);

const defaultMessage = computed(() => getMessage(props.parameter.defaut));
const currentMessage = computed(() => getMessage(currentValue.value));

onMounted(() => {
    fetchMessages();
});

const openLibrary = () => {
    isLibraryVisible.value = true;
};

const handleMessageSelect = async (messageId) => {
    isLibraryVisible.value = false;
     try {
        await updateSetting(props.groupPath, props.parameter.identifiant, messageId);
      } catch (error) {
        console.error("Erreur lors de la sauvegarde du paramètre:", error);
      }
};

const removeSurcharge = async () => {
     try {
        await updateSetting(props.groupPath, props.parameter.identifiant, null);
      } catch (error) {
        console.error("Erreur lors de la suppression de la surcharge:", error);
      }
};

const closeDialog = () => {
  emit('update:show', false);
};

</script>
