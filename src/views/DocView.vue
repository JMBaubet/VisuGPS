<template>
  <v-container fluid>
    <v-card>
      <v-card-title class="d-flex align-center">
        <v-icon left>mdi-book-open-page-variant-outline</v-icon>
        <span class="ml-2">Documentation</span>
      </v-card-title>
      <v-divider></v-divider>
      <v-card-text>
        <div v-if="loading" class="text-center">
          <v-progress-circular indeterminate color="primary"></v-progress-circular>
          <p>Chargement...</p>
        </div>
        <div v-else-if="error" class="text-error">
          <p>Erreur lors du chargement de la documentation :</p>
          <pre>{{ error }}</pre>
        </div>
        <!-- La classe "prose" peut être utilisée pour styliser le contenu HTML -->
        <div v-else v-html="docContent" class="prose"></div>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { convertFileSrc } from '@tauri-apps/api/core';

const route = useRoute();
const docContent = ref('');
const loading = ref(true);
const error = ref(null);

onMounted(async () => {
  const docFile = route.params.docFile;
  if (!docFile) {
    error.value = "Aucun fichier de documentation n'a été spécifié.";
    loading.value = false;
    return;
  }

  try {
    const assetPath = `assets/docs/${docFile}`;
    const assetUrl = convertFileSrc(assetPath);

    const response = await fetch(assetUrl);
    if (!response.ok) {
      throw new Error(`Le fichier de documentation '${docFile}' est introuvable ou une erreur est survenue.`);
    }
    docContent.value = await response.text();

  } catch (e) {
    error.value = e.toString();
  } finally {
    loading.value = false;
  }
});
</script>

<style scoped>
/* Styles pour le contenu HTML injecté. 
   Vous pouvez personnaliser cela pour que ça corresponde à votre thème. */
.prose {
  line-height: 1.6;
}

.prose :deep(h1), .prose :deep(h2), .prose :deep(h3) {
  margin-top: 1.5em;
  margin-bottom: 0.5em;
  font-weight: 600;
}

.prose :deep(code) {
  background-color: rgba(var(--v-theme-on-surface), 0.1);
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
}

.prose :deep(pre) {
  background-color: rgba(var(--v-theme-on-surface), 0.1);
  padding: 1em;
  border-radius: 5px;
  overflow-x: auto;
}
</style>
