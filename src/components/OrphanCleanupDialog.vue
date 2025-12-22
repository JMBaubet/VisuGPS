<template>
  <v-dialog v-model="visible" max-width="700px" persistent>
    <v-card>
      <v-card-title class="headline grey lighten-2 d-flex align-center">
        <v-icon start color="warning">mdi-broom</v-icon>
        Nettoyage des données orphelines
      </v-card-title>

      <v-card-text class="pa-4">
        <div class="mb-4">
          Certaines informations ne sont plus utilisées par aucun circuit. 
          Celles liées au circuit qui vient d'être supprimé sont <span class="text-primary font-weight-bold">mises en évidence</span> et sélectionnées par défaut.
        </div>

        <v-list v-if="hasOrphans" lines="one">
          <!-- Villes -->
          <template v-if="orphans.villes.length > 0">
            <v-list-subheader>Villes orphelines</v-list-subheader>
            <v-list-item v-for="ville in orphans.villes" :key="ville.id" :class="{'circuit-related': isRelated(ville.id, 'ville')}">
              <template v-slot:prepend>
                <v-checkbox-btn v-model="selectedVilles" :value="ville.id" :color="isRelated(ville.id, 'ville') ? 'primary' : ''"></v-checkbox-btn>
              </template>
              <v-list-item-title :class="{'text-primary font-weight-bold': isRelated(ville.id, 'ville')}">
                {{ ville.nom }}
              </v-list-item-title>
            </v-list-item>
          </template>

          <!-- Traceurs -->
          <template v-if="orphans.traceurs.length > 0">
            <v-list-subheader>Traceurs orphelins</v-list-subheader>
            <v-list-item v-for="traceur in orphans.traceurs" :key="traceur.id" :class="{'circuit-related': isRelated(traceur.id, 'traceur')}">
              <template v-slot:prepend>
                <v-checkbox-btn v-model="selectedTraceurs" :value="traceur.id" :color="isRelated(traceur.id, 'traceur') ? 'primary' : ''"></v-checkbox-btn>
              </template>
              <v-list-item-title :class="{'text-primary font-weight-bold': isRelated(traceur.id, 'traceur')}">
                {{ traceur.nom }}
              </v-list-item-title>
            </v-list-item>
          </template>

          <!-- Messages -->
          <template v-if="orphans.messages.length > 0">
            <v-list-subheader>Messages orphelins</v-list-subheader>
            <v-list-item v-for="msg in orphans.messages" :key="msg.id" :class="{'circuit-related': isRelated(msg.id, 'message')}">
              <template v-slot:prepend>
                <v-checkbox-btn v-model="selectedMessages" :value="msg.id" :color="isRelated(msg.id, 'message') ? 'primary' : ''"></v-checkbox-btn>
              </template>
              <v-list-item-title :class="{'text-primary font-weight-bold': isRelated(msg.id, 'message')}">
                {{ msg.text }}
              </v-list-item-title>
            </v-list-item>
          </template>
        </v-list>
        
        <div v-else class="text-center py-4 text-grey">
          Aucune donnée orpheline détectée.
        </div>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="grey" variant="text" @click="close">Fermer sans supprimer</v-btn>
        <v-btn color="error" :disabled="!anySelected" @click="handleDelete">
          Supprimer la sélection
        </v-btn>
      </v-card-actions>
    </v-card>

    <!-- Confirmation pour les éléments additionnels -->
    <v-dialog v-model="showConfirmExtra" max-width="400px">
      <v-card>
        <v-card-title class="headline">Confirmation</v-card-title>
        <v-card-text>
          Vous avez sélectionné des éléments qui ne sont pas directement liés au circuit supprimé. 
          Voulez-vous vraiment les supprimer définitivement ?
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="grey" variant="text" @click="showConfirmExtra = false">Annuler</v-btn>
          <v-btn color="error" @click="confirmDelete">Confirmer la suppression</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  modelValue: Boolean,
  orphans: {
    type: Object,
    default: () => ({ villes: [], traceurs: [], messages: [] })
  },
  relatedIds: {
    type: Object,
    default: () => ({ ville: null, traceur: null, messages: [] })
  }
});

const emit = defineEmits(['update:modelValue', 'cleaned']);

const visible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
});

const selectedVilles = ref([]);
const selectedTraceurs = ref([]);
const selectedMessages = ref([]);
const showConfirmExtra = ref(false);

const hasOrphans = computed(() => {
  return props.orphans.villes.length > 0 || 
         props.orphans.traceurs.length > 0 || 
         props.orphans.messages.length > 0;
});

const anySelected = computed(() => {
  return selectedVilles.value.length > 0 || 
         selectedTraceurs.value.length > 0 || 
         selectedMessages.value.length > 0;
});

const isRelated = (id, type) => {
  if (type === 'ville') return id === props.relatedIds.ville;
  if (type === 'traceur') return id === props.relatedIds.traceur;
  if (type === 'message') return props.relatedIds.messages.includes(id);
  return false;
};

// Initialiser les sélections avec les éléments liés
watch(() => props.modelValue, (newVal) => {
  if (newVal) {
    selectedVilles.value = props.orphans.villes
      .filter(v => isRelated(v.id, 'ville'))
      .map(v => v.id);
    selectedTraceurs.value = props.orphans.traceurs
      .filter(t => isRelated(t.id, 'traceur'))
      .map(t => t.id);
    selectedMessages.value = props.orphans.messages
      .filter(m => isRelated(m.id, 'message'))
      .map(m => m.id);
  }
});

const handleDelete = () => {
  // Vérifier s'il y a des sélections "extra"
  const hasExtra = selectedVilles.value.some(id => !isRelated(id, 'ville')) ||
                   selectedTraceurs.value.some(id => !isRelated(id, 'traceur')) ||
                   selectedMessages.value.some(id => !isRelated(id, 'message'));

  if (hasExtra) {
    showConfirmExtra.value = true;
  } else {
    confirmDelete();
  }
};

const confirmDelete = async () => {
  try {
    await invoke('delete_orphans', {
      villeIds: selectedVilles.value,
      traceurIds: selectedTraceurs.value,
      messageIds: selectedMessages.value
    });
    showConfirmExtra.value = false;
    emit('cleaned');
    close();
  } catch (error) {
    console.error('Erreur lors de la suppression des orphelins:', error);
  }
};

const close = () => {
  visible.value = false;
};
</script>

<style scoped>
.circuit-related {
  background-color: rgba(var(--v-theme-primary), 0.05);
}
</style>
