<template>
  <v-dialog
    v-model="visible"
    max-width="800px"
    height="80vh"
    @click:outside="handleCloseAttempt"
  >
    <v-card class="pa-4 welcome-modal d-flex flex-column" height="100%">
      <v-card-title class="text-h5 d-flex justify-space-between align-center flex-none">
        <div class="d-flex align-center">
          <v-img src="/docs/images/logo.png" width="40" height="40" class="mr-3" alt="Logo"></v-img>
          {{ title }}
        </div>
        <v-btn icon variant="text" size="small" @click="visible = false">
          <v-icon>mdi-close</v-icon>
        </v-btn>
      </v-card-title>
      
      <v-card-text class="flex-grow-1 overflow-y-auto">
        <!-- Alerte si le token Mapbox n'est pas valide -->
        <v-alert
          v-if="!isMapboxValid"
          type="warning"
          variant="tonal"
          title="Token Mapbox manquant ou invalide"
          class="mb-4"
          border="start"
        >
          <div>
            L'affichage de la carte 3D nécessite un token Mapbox valide. 
          </div>
          <div class="mt-1 d-flex align-center">
            <span class="mr-2">Consultez la</span>
            <v-btn 
              variant="text" 
              color="info" 
              density="compact" 
              class="pa-0 text-none" 
              prepend-icon="mdi-book-open-page-variant-outline"
              @click="showDoc = true"
            >
              documentation pour obtenir un token
            </v-btn>
          </div>
          
          <v-text-field
            v-model="mapboxToken"
            label="Saisir votre token Mapbox"
            placeholder="pk.ey..."
            append-inner-icon="mdi-check-circle"
            @click:append-inner="checkToken"
            @keyup.enter="checkToken"
            :loading="checking"
            :error-messages="tokenError"
            class="mt-4"
            density="compact"
            variant="outlined"
            hide-details="auto"
          ></v-text-field>
        </v-alert>

        <v-alert
          v-else-if="isMapboxValid"
          type="success"
          variant="tonal"
          class="mb-4"
          density="compact"
        >
          <template v-slot:prepend>
            <v-icon>mdi-check-decagram</v-icon>
          </template>
          Token Mapbox valide !
        </v-alert>

        <v-divider class="mb-4"></v-divider>

        <!-- Contenu Markdown -->
        <div 
          v-if="mdContent" 
          v-html="compiledMarkdown" 
          class="markdown-body welcome-md-content"
        ></div>
        <div v-else class="text-center pa-10">
          <v-progress-circular indeterminate color="primary"></v-progress-circular>
        </div>
      </v-card-text>

      <v-card-actions class="justify-end px-4 pb-4 flex-none">
        <v-btn
          color="success"
          variant="elevated"
          @click="acquit"
          prepend-icon="mdi-checkbox-marked-circle-outline"
          class="mr-2"
          :disabled="!isMapboxValid"
        >
          {{ acquitButtonText }}
        </v-btn>
        
        <v-btn
          variant="outlined"
          color="primary"
          @click="visible = false"
        >
          Fermer
        </v-btn>
      </v-card-actions>
    </v-card>

    <!-- Sous-dialogue pour la documentation Mapbox -->
    <v-dialog v-model="showDoc" max-width="800px">
      <DocDisplay doc-path="/docs/DocAnnexe/obtenir_token_mapbox.md" @close="showDoc = false" />
    </v-dialog>
  </v-dialog>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue';
import { useSettings } from '@/composables/useSettings';
import { useServiceStatus } from '@/composables/useServiceStatus';
import { invoke } from '@tauri-apps/api/core';
import MarkdownIt from 'markdown-it';
import DocDisplay from '@/components/DocDisplay.vue';
import '@/components/DocDisplay.vue'; // Pour les styles CSS partagés si besoin

const { status, settings, updateReferenceField, updateSetting, getSettingValue } = useSettings();
const { serviceStatus, checkAllServices } = useServiceStatus();
const visible = ref(false);
const showDoc = ref(false);
const mdContent = ref('');
const md = new MarkdownIt({ html: true, breaks: true });
const mapboxToken = ref('');
const checking = ref(false);
const tokenError = ref('');

const isMapboxValid = computed(() => serviceStatus.value === 'connected');

const title = computed(() => {
  if (status.value === 'Update') return 'Mise à jour VisuGPS';
  return 'Bienvenue sur VisuGPS';
});

const acquitButtonText = computed(() => {
  if (status.value === 'Update') return 'Acquitter le message de mise à jour';
  return 'Acquitter le message de bienvenue';
});

const canAcquit = computed(() => {
  return isMapboxValid.value && (status.value === 'MapBoxOK' || status.value === 'Update');
});

const compiledMarkdown = computed(() => {
  if (!mdContent.value) return '';
  return md.render(mdContent.value);
});

// Charger le contenu MD en fonction du statut
const loadContent = async () => {
  let path = 'docs/DocStatus/start.md';
  if (status.value === 'Update') {
    path = 'docs/DocStatus/update.md';
  }
  
  try {
    const response = await invoke('get_doc_content', { path });
    // Optionnel : remplacer les placeholders
    let content = response.content;
    
    // Remplacement du dossier de téléchargement
    const downloadDir = getSettingValue('Importation/ImportDir');
    content = content.replace('DEFAULT_DOWNLOADS', downloadDir || 'votre dossier de téléchargement');
    
    mdContent.value = content;
  } catch (error) {
    console.error("Erreur chargement welcome MD:", error);
    mdContent.value = "# Bienvenue\n\nErreur lors du chargement de la documentation.";
  }
};

const checkToken = async () => {
  if (!mapboxToken.value) return;
  checking.value = true;
  tokenError.value = '';
  try {
    // On met simplement à jour le setting, useServiceStatus réagira au changement du token et appellera check_mapbox_status
    await updateSetting('Système/Tokens', 'mapbox', mapboxToken.value);
    
    // On attend un peu que useServiceStatus fasse son travail ou on force l'appel
    await checkAllServices();
    
    if (serviceStatus.value !== 'connected') {
      tokenError.value = "Token invalide ou services inaccessibles.";
    }
  } catch (error) {
    tokenError.value = "Erreur lors de la mise à jour : " + error;
  } finally {
    checking.value = false;
  }
};

const acquit = async () => {
  await updateReferenceField('Status', 'Acq');
  visible.value = false;
};

const handleCloseAttempt = () => {
};

// Récupération initiale du token pour l'affichage du champ
const initMapboxValue = () => {
  const storedToken = settings.value?.data?.groupes
    ?.find(g => g.libelle === 'Système')?.groupes
    ?.find(g => g.libelle === 'Tokens')?.parametres
    ?.find(p => p.identifiant === 'mapbox')?.surcharge 
    || settings.value?.data?.groupes
    ?.find(g => g.libelle === 'Système')?.groupes
    ?.find(g => g.libelle === 'Tokens')?.parametres
    ?.find(p => p.identifiant === 'mapbox')?.defaut;

  if (storedToken) {
    mapboxToken.value = storedToken;
  }
};

onMounted(async () => {
  if (settings.value) {
    initMapboxValue();
    if (status.value !== 'Acq') {
      visible.value = true;
      loadContent();
    }
  }
});

watch(status, (newStatus) => {
  if (newStatus && newStatus !== 'Acq') {
    visible.value = true;
    loadContent();
  } else {
    visible.value = false;
  }
});

// Si les settings changent (chargement initial), on réagit
watch(settings, async (newSettings) => {
    if (newSettings) {
        initMapboxValue();
        if (status.value !== 'Acq') {
            visible.value = true;
            loadContent();
        }
    }
}, { immediate: true });

</script>

<style scoped>
.welcome-modal {
  border-radius: 12px;
}
.welcome-md-content {
  font-size: 0.95rem;
}
</style>
