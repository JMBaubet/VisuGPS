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
      <div v-else v-html="renderedMarkdown" class="markdown-body" ref="markdownBodyRef"></div>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { ref, watch, computed, onMounted, onBeforeUnmount } from 'vue';
import { useTheme } from 'vuetify';
import MarkdownIt from 'markdown-it';
import hljs from 'highlight.js/lib/core';
import javascript from 'highlight.js/lib/languages/javascript';
import typescript from 'highlight.js/lib/languages/typescript';
import bash from 'highlight.js/lib/languages/bash';
import powershell from 'highlight.js/lib/languages/powershell';
import html from 'highlight.js/lib/languages/xml';

const theme = useTheme();

async function loadHighlightTheme(name) {
  if (name === 'dark') {
    await import('highlight.js/styles/github-dark.css');
  } else {
    await import('highlight.js/styles/github.css');
  }
}

const markdownBodyRef = ref(null); // Référence au div markdown-body

const handleLinkClick = (event) => {
  const target = event.target;
  if (target.tagName === 'A' && target.href) {
    const url = new URL(target.href);
    if (url.protocol.startsWith('http') && url.host !== window.location.host) {
      event.preventDefault();
      if (window.__TAURI__ && window.__TAURI__.shell) {
        window.__TAURI__.shell.open(target.href);
      } else {
        window.open(target.href, '_blank');
      }
    }
  }
};

onMounted(() => {
  loadHighlightTheme(theme.global.name.value);
  if (markdownBodyRef.value) {
    markdownBodyRef.value.addEventListener('click', handleLinkClick);
  }
});

onBeforeUnmount(() => {
  if (markdownBodyRef.value) {
    markdownBodyRef.value.removeEventListener('click', handleLinkClick);
  }
});

watch(() => theme.global.name.value, (newTheme) => {
  loadHighlightTheme(newTheme);
});

// Register languages
hljs.registerLanguage('javascript', javascript);
hljs.registerLanguage('typescript', typescript);
hljs.registerLanguage('bash', bash);
hljs.registerLanguage('powershell', powershell);
hljs.registerLanguage('html', html);

const props = defineProps({ docPath: String });
defineEmits(['close']);

const markdownContent = ref('');
const loading = ref(false);
const error = ref(null);

const md = new MarkdownIt({
  html: true, // Added this line to enable HTML rendering
  breaks: true,
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return `<pre class="hljs"><code>${hljs.highlight(str, { language: lang, ignoreIllegals: true }).value}</code></pre>`;
      } catch {}
    }
    return `<pre class="hljs"><code>${md.utils.escapeHtml(str)}</code></pre>`;
  }
});

const renderedMarkdown = computed(() => md.render(markdownContent.value));

async function fetchDocumentation(path) {
  loading.value = true;
  error.value = null;
  markdownContent.value = '';
  try {
    const response = await fetch(path);
    if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
    markdownContent.value = await response.text();
  } catch (e) {
    error.value = e.message;
  } finally {
    loading.value = false;
  }
}

watch(() => props.docPath, (newPath) => {
  if (newPath) fetchDocumentation(newPath);
}, { immediate: true });
</script>

<style>
/* Base styles for markdown content */
.markdown-body {
  line-height: 1.6;
  font-family: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
}

/* Titres */
.markdown-body h1,
.markdown-body h2,
.markdown-body h3 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
}

/* Code inline */
.markdown-body code {
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, Courier, monospace;
  font-size: 85%;
  padding: 0.2em 0.4em;
  margin: 0;
  border-radius: 3px;
  white-space: pre-wrap; /* ✅ garde indentation inline */
}

/* Bloc de code */
.markdown-body pre {
  margin: 1em 0;
  overflow-x: auto;
  font-size: 85%;
  line-height: 1.45;
  border-radius: 6px;
  white-space: pre-wrap; /* ✅ conserve indentation et retour ligne */
  word-break: break-word; /* ✅ évite débordement */
}

.markdown-body pre code {
  padding: 16px;
  display: block;
}

/* --- Theme-specific overrides --- */

/* Light Theme */
.theme-light .markdown-body pre.hljs {
  background-color: #fafafa; /* fond clair github-like */
}

.theme-light .markdown-body code:not(pre code) {
  background-color: rgba(27, 31, 35, 0.05);
  color: #24292f;
}

/* Dark Theme */
.theme-dark .markdown-body pre.hljs {
  background-color: #0d1117; /* fond sombre github-dark */
}

.theme-dark .markdown-body code:not(pre code) {
  background-color: #30363d;
  color: #c9d1d9;
}

/* Tableaux */
.markdown-body table {
  border-collapse: collapse;
  width: 100%;
  margin: 1em 0;
}

.markdown-body th,
.markdown-body td {
  border: 1px solid rgba(0, 0, 0, 0.1);
  padding: 6px 13px;
}

.theme-dark .markdown-body th,
.theme-dark .markdown-body td {
  border-color: rgba(255, 255, 255, 0.1);
}

/* Listes */
.markdown-body ul,
.markdown-body ol {
  padding-left: 1.5em;
  margin: 0.5em 0;
}

/* Responsiveness */
@media (max-width: 768px) {
  .markdown-body pre code {
    font-size: 80%;
    padding: 12px;
  }
}

/* --- Amélioration globale du contraste --- */

/* Texte principal */
.theme-light .markdown-body {
  color: #1a1a1a; /* plus sombre que le gris par défaut */
}

.theme-dark .markdown-body {
  color: #e6e6e6; /* plus clair et lisible sur fond sombre */
}

/* Titres - plus de contraste et meilleure hiérarchie */
.theme-light .markdown-body h1,
.theme-light .markdown-body h2,
.theme-light .markdown-body h3 {
  color: #000000; /* noir pur en clair pour bien ressortir */
}

.theme-dark .markdown-body h1,
.theme-dark .markdown-body h2,
.theme-dark .markdown-body h3 {
  color: #ffffff; /* blanc pur en sombre */
}

/* Listes et paragraphes - garder une bonne lisibilité */
.theme-light .markdown-body p,
.theme-light .markdown-body ul,
.theme-light .markdown-body li {
  color: #000000;
}

.theme-dark .markdown-body p,
.theme-dark .markdown-body li {
  color: #e6e6e6;
}

/* Lien - améliorer la visibilité */
.theme-light .markdown-body a {
  color: #0366d6; /* bleu GitHub */
}

.theme-dark .markdown-body a {
  color: #58a6ff; /* bleu GitHub dark */
}

</style>