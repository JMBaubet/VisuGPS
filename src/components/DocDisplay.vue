<template>
  <v-card :class="theme.global.current.value.dark ? 'theme-dark' : 'theme-light'">
    <v-card-title class="d-flex justify-space-between align-center">
      <span>Documentation</span>
      <v-btn icon @click="$emit('close')">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-card-title>
    <v-card-text>
      <div v-if="loading">Chargement de la documentation...</div>
      <div v-else-if="error">Erreur lors du chargement de la documentation: {{ error }}</div>
      <div v-else v-html="renderedMarkdown" class="markdown-body"></div>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { useTheme } from 'vuetify';
import MarkdownIt from 'markdown-it';
import hljs from 'highlight.js/lib/core';
import javascript from 'highlight.js/lib/languages/javascript';
import typescript from 'highlight.js/lib/languages/typescript';
import bash from 'highlight.js/lib/languages/bash';
import powershell from 'highlight.js/lib/languages/powershell';
import html from 'highlight.js/lib/languages/xml'; // HTML is part of xml language file in highlight.js

// Import the themes to ensure they are bundled
import 'highlight.js/styles/atom-one-light.css';
import 'highlight.js/styles/github-dark.css';

const theme = useTheme();

// Register languages
hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('typescript', typescript);
hljs.registerLanguage('bash', bash);
hljs.registerLanguage('powershell', powershell);
hljs.registerLanguage('html', html);

const props = defineProps({
  docPath: String,
});

defineEmits(['close']);

const markdownContent = ref('');
const loading = ref(false);
const error = ref(null);

const md = new MarkdownIt({
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        // Return the highlighted code with the .hljs class
        return `<pre class="hljs"><code>${hljs.highlight(str, { language: lang, ignoreIllegals: true }).value}</code></pre>`;
      } catch (__) {}
    }
    // Return the code without highlighting but with the same structure
    return `<pre class="hljs"><code>${md.utils.escapeHtml(str)}</code></pre>`;
  }
});

const renderedMarkdown = computed(() => {
  return md.render(markdownContent.value);
});

const fetchDocumentation = async (path) => {
  loading.value = true;
  error.value = null;
  markdownContent.value = '';
  try {
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

<style>
/* Base styles for markdown content */
.markdown-body {
  line-height: 1.6;
}

.markdown-body h1, .markdown-body h2, .markdown-body h3 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
}

.markdown-body code {
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, Courier, monospace;
  font-size: 85%;
  padding: 0.2em 0.4em;
  margin: 0;
  border-radius: 3px;
}

.markdown-body pre {
  padding: 0;
  margin: 0;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  border-radius: 6px;
}

.markdown-body pre code {
  padding: 16px;
  display: block;
}

/* --- Theme-specific overrides --- */

/* Light Theme */
.theme-light .markdown-body pre.hljs {
  background-color: #fafafa; /* atom-one-light background */
}
.theme-light .markdown-body code:not(pre code) {
   background-color: rgba(27,31,35,0.05);
}

/* Dark Theme */
.theme-dark .markdown-body pre.hljs {
  background-color: #0d1117; /* github-dark background */
}
.theme-dark .markdown-body code:not(pre code) {
  background-color: #30363d;
  color: #c9d1d9;
}

/* The hljs class comes from highlight.js and contains all the specific token colors. 
   By importing the themes' CSS and only overriding the background, we keep the token colors. */
</style>