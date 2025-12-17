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
      <div v-else v-html="compiledMarkdown" class="markdown-body" @click="handleLinkClick"></div>
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
import html from 'highlight.js/lib/languages/xml';

const theme = useTheme();

async function loadHighlightTheme(name) {
  if (name === 'dark') {
    await import('highlight.js/styles/github-dark.css');
  } else {
    await import('highlight.js/styles/github.css');
  }
}

watch(() => theme.global.name.value, (newTheme) => {
  loadHighlightTheme(newTheme);
}, { immediate: true });

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
const currentDocPath = ref(''); 

import { invoke } from '@tauri-apps/api/core';

// Fonction pour résoudre les chemins relatifs
function resolvePath(basePath, relativePath) {
  if (relativePath.startsWith('/')) return relativePath; // Chemin absolu
  if (relativePath.startsWith('http')) return relativePath; // URL externe

  const stack = basePath.split('/');
  // Retirer le nom du fichier actuel pour avoir le dossier parent
  stack.pop();

  const parts = relativePath.split('/');
  for (const part of parts) {
    if (part === '.') continue;
    if (part === '..') {
      if (stack.length > 0) stack.pop();
    } else {
      stack.push(part);
    }
  }
  return stack.join('/');
}

const md = new MarkdownIt({
  html: true,
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

// Custom renderer pour les images pour réécrire les src
const defaultImageRender = md.renderer.rules.image || function(tokens, idx, options, env, self) {
  return self.renderToken(tokens, idx, options);
};

md.renderer.rules.image = function (tokens, idx, options, env, self) {
  const token = tokens[idx];
  const srcIndex = token.attrIndex('src');
  if (srcIndex >= 0) {
    const src = token.attrs[srcIndex][1];
    // Si c'est un chemin relatif (ne commence pas par / ou http), on le résout
    if (!src.startsWith('/') && !src.startsWith('http')) {
        // En dev, on assume que les images sont servies depuis /docs/ si elles sont référencées dans la doc
        // Mais attention, "resolvePath" donne un chemin absolu par rapport à la racine "docs" du backend
        // Pour l'affichage frontend (<img>), il faut un chemin accessible par le navigateur.
        // Si on est en dev, `npm run tauri dev` sert le dossier `public` à la racine.
        // Mes docs sont dans /docs/...
        
        let resolved = resolvePath(currentDocPath.value, src);
        
        // Si le path résolu ne commence pas par /, on l'ajoute.
        // On suppose que resolvePath retourne un chemin basé sur la racine du serveur de dev
        // Ex: current = /docs/DocUtilisateur/index.md, src = ../images/logo.png
        // resolved = /docs/images/logo.png
        // Cela devrait fonctionner directement en dev car /docs est servi.
        
        token.attrs[srcIndex][1] = resolved;
    }
  }
  return defaultImageRender(tokens, idx, options, env, self);
};

// Remplacer aussi les balises <img> HTML brutes si nécessaire
function normalizePaths(markdown) {
  // Regex pour attraper les src="..." dans les balises img
  return markdown.replace(/<img\s+[^>]*src="([^"]+)"[^>]*>/g, (match, src) => {
    if (!src.startsWith('/') && !src.startsWith('http')) {
         const resolved = resolvePath(currentDocPath.value, src);
         return match.replace(src, resolved);
    }
    return match;
  });
}

const compiledMarkdown = computed(() =>
  md.render(normalizePaths(markdownContent.value))
);

async function fetchDocumentation(path) {
  loading.value = true;
  error.value = null;
  markdownContent.value = '';
  // S'assurer que le chemin commence par / pour la cohérence
  const cleanPath = path.startsWith('/') ? path : '/' + path;
  currentDocPath.value = cleanPath;
  
  try {
    // Le backend attend un chemin relatif sans le slash de début pour l'instant (voir lib.rs logic debug vs prod)
    // Mais attendons, lib.rs : project_root.join(&path). Si path commence par /, join peut revenir à la racine du disque.
    // Il faut probablement envoyer un chemin relatif pur.
    
    const relativePath = cleanPath.startsWith('/') ? cleanPath.substring(1) : cleanPath;
    const response = await invoke('get_doc_content', { path: relativePath });
    markdownContent.value = response.content;
    // Mise à jour du basePath si nécessaire
  } catch (e) {
    error.value = e;
  } finally {
    loading.value = false;
  }
}

  const handleLinkClick = (event) => {
    const target = event.target.closest('a');
    if (!target || !target.href) return;

    const hrefAttribute = target.getAttribute('href'); 
    console.log('Link clicked:', { href: target.href, attribute: hrefAttribute });

    // Handle external links
    if (hrefAttribute.startsWith('http')) {
      event.preventDefault();
      event.stopPropagation();
      if (window.__TAURI__ && window.__TAURI__.shell) {
        window.__TAURI__.shell.open(hrefAttribute);
      } else {
        window.open(hrefAttribute, '_blank');
      }
      return;
    }

    // Handle internal markdown links
    if (hrefAttribute.endsWith('.md')) {
      console.log('Intercepting markdown link:', hrefAttribute);
      event.preventDefault();
      event.stopPropagation();
      event.stopImmediatePropagation();
      
      const resolved = resolvePath(currentDocPath.value, hrefAttribute);
      console.log(`Navigating to doc: ${resolved}`);
      fetchDocumentation(resolved);
      return;
    }
  };

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

/* Citations / Notes */
.markdown-body blockquote {
  margin: 16px 0;
  padding: 0.5em 1em;
  border-left: 0.25em solid #dfe2e5;
  color: #6a737d;
  background-color: rgba(208, 215, 222, 0.2);
  border-radius: 0 3px 3px 0;
}

.theme-dark .markdown-body blockquote {
  border-left-color: #30363d;
  color: #8b949e;
  background-color: rgba(110, 118, 129, 0.2);
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

/* Inversion des icônes noires en mode sombre */
.theme-dark .markdown-body img[src*="api.iconify.design"]:not([src*="color="]) {
  filter: invert(1);
}

</style>