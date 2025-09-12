<template>
  <v-card>
    <v-card-title class="d-flex justify-space-between align-center">
      <span>Documentation</span>
      <v-btn icon @click="$emit('close')">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-card-title>
    <v-card-text>
      <div v-if="loading">Chargement de la documentation...</div>
      <div v-else-if="error">Erreur lors du chargement de la documentation: {{ error }}</div>
      <div v-else v-html="renderedMarkdown"></div>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { marked } from 'marked'; // Assuming 'marked' library is available or will be installed

const props = defineProps({
  docPath: String,
});

const emit = defineEmits(['close']);

const markdownContent = ref('');
const loading = ref(false);
const error = ref(null);

const renderedMarkdown = computed(() => {
  return marked(markdownContent.value);
});

const fetchDocumentation = async (path) => {
  loading.value = true;
  error.value = null;
  markdownContent.value = '';
  try {
    // Assuming the .md files are directly accessible via HTTP from the /public directory
    const response = await fetch(path);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    markdownContent.value = await response.text();
  } catch (e) {
    error.value = e.message;
    console.error("Failed to fetch documentation:", e);
  } finally {
    loading.value = false;
  }
};

watch(() => props.docPath, (newPath) => {
  if (newPath) {
    fetchDocumentation(newPath);
  }
}, { immediate: true });
</script>
