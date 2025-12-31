<template>
  <v-card 
    :class="[theme.global.current.value.dark ? 'theme-dark' : 'theme-light', 'd-flex flex-column']" 
    style="height: 90vh; max-height: 90vh; overflow: hidden !important;"
  >
    <v-card-title 
      :class="['doc-header d-flex justify-space-between align-center flex-none', { 'header-parameters': isParameterDoc }]"
    >
      <div class="d-flex align-center">
        <v-btn
          icon
          variant="text"
          @click="goHome"
          class="mr-1"
          size="small"
          title="Accueil"
        >
          <v-icon>mdi-home</v-icon>
        </v-btn>
        <v-divider vertical class="mx-2" style="height: 24px"></v-divider>
        <v-btn
          icon
          variant="text"
          :disabled="!canGoBack"
          @click="goBack"
          class="mr-1"
          size="small"
          title="Précédent"
        >
          <v-icon>mdi-arrow-left</v-icon>
        </v-btn>
        <v-btn
          icon
          variant="text"
          :disabled="!canGoForward"
          @click="goForward"
          class="mr-4"
          size="small"
          title="Suivant"
        >
          <v-icon>mdi-arrow-right</v-icon>
        </v-btn>
        <span>{{ isParameterDoc ? 'Documentation des paramètres' : 'Manuel utilisateur' }}</span>
      </div>
      <v-btn icon variant="text" @click="$emit('close')">
        <v-icon>mdi-close</v-icon>
      </v-btn>
    </v-card-title>
    
    <!-- Zone de scroll séparée pour éviter les glitchs sur la toolbar -->
    <div class="flex-grow-1 overflow-y-auto pa-4" ref="scrollContainer">
      <div v-if="loading">Chargement de la documentation...</div>
      <div v-else-if="error">Erreur lors du chargement de la documentation: {{ error }}</div>
      <div v-else v-html="compiledMarkdown" class="markdown-body" @click="handleLinkClick" ref="contentRef"></div>
    </div>
  </v-card>
</template>

<script setup>
import { ref, watch, computed, onMounted, onUnmounted } from 'vue';
import { useTheme } from 'vuetify';
import { open as openUrl } from '@tauri-apps/plugin-shell';
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

// Gestion de l'historique de navigation
const history = ref([]);
const historyIndex = ref(-1);

const canGoBack = computed(() => historyIndex.value > 0);
const canGoForward = computed(() => historyIndex.value < history.value.length - 1);

const isParameterDoc = computed(() => {
  const path = currentDocPath.value.toLowerCase();
  return path.includes('parametres.md') || path.includes('docparametrage');
});

function goHome() {
  fetchDocumentation('/docs/DocUtilisateur/index.md');
}

function goBack() {
  if (canGoBack.value) {
    historyIndex.value--;
    fetchDocumentation(history.value[historyIndex.value], true);
  }
}

function goForward() {
  if (canGoForward.value) {
    historyIndex.value++;
    fetchDocumentation(history.value[historyIndex.value], true);
  }
}

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
        // Si on est en dev, `npm run tauri dev` serv le dossier `public` à la racine.
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

const compiledMarkdown = computed(() => {
  let html = md.render(normalizePaths(markdownContent.value));
  
  // Post-traitement pour transformer les blockquotes commençant par [!TYPE] en alertes stylisées
  html = html.replace(/<blockquote>\s*<p>\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*(?:<br>)?/gi, (match, type) => {
    const lowerType = type.toLowerCase();
    let displayTitle = type;
    
    // Traduction en français pour l'affichage
    switch(lowerType) {
      case 'note': displayTitle = 'Note'; break;
      case 'tip': displayTitle = 'Astuce'; break;
      case 'important': displayTitle = 'Important'; break;
      case 'warning': displayTitle = 'Attention'; break;
      case 'caution': displayTitle = 'Prudence'; break;
    }
    
    return `<blockquote class="markdown-alert markdown-alert-${lowerType}">
              <p class="markdown-alert-title">
                <span class="alert-icon-wrapper">${displayTitle}</span>
              </p>`;
  });
  
  return html;
});

async function fetchDocumentation(path, isHistoryAction = false, anchor = null) {
  loading.value = true;
  error.value = null;
  markdownContent.value = '';
  // S'assurer que le chemin commence par / pour la cohérence
  const cleanPath = path.startsWith('/') ? path : '/' + path;
  currentDocPath.value = cleanPath;

  // Mise à jour de l'historique s'il ne s'agit pas d'une action de navigation
  if (!isHistoryAction) {
    // Si on navigue vers un nouveau lien, on tronque l'historique futur
    if (historyIndex.value < history.value.length - 1) {
      history.value = history.value.slice(0, historyIndex.value + 1);
    }
    // On n'ajoute à l'historique que si le chemin est différent du dernier
    if (history.value[historyIndex.value] !== cleanPath) {
      history.value.push(cleanPath);
      historyIndex.value = history.value.length - 1;
    }
  }
  
  try {
    const relativePath = cleanPath.startsWith('/') ? cleanPath.substring(1) : cleanPath;
    const response = await invoke('get_doc_content', { path: relativePath });
    markdownContent.value = response.content;
  } catch (e) {
    error.value = e;
  } finally {
    loading.value = false;
    
    // On attend que Vue et le DOM soient prêts après loading = false
    if (anchor) {
      setTimeout(() => {
        scrollToAnchor(anchor);
      }, 300); // 300ms pour laisser le temps au rendu complexe et aux images
    } else {
      // Remonter en haut si pas d'ancre
      nextTick(() => {
        if (scrollContainer.value) scrollContainer.value.scrollTop = 0;
      });
    }
  }
}

function scrollToAnchor(anchorName) {
  if (!scrollContainer.value || !contentRef.value) return;
  
  const decodedAnchor = decodeURIComponent(anchorName).toLowerCase().replace(/^#/, '');
  console.log(`Searching for anchor: ${decodedAnchor}`);

  // 1. Recherche par ID
  let element = document.getElementById(decodedAnchor);
  
  if (!element) {
    // 2. Recherche par texte de titre ou slug
    const headers = contentRef.value.querySelectorAll('h1, h2, h3, h4, h5, h6');
    
    // On essaie plusieurs stratégies de matching
    for (const h of headers) {
      const text = h.innerText.toLowerCase().trim();
      
      // Stratégie A : Slugification standard (on retire tout sauf alphanum et tirets)
      const slug = text
        .replace(/[^\w\s-à-ÿ]/g, '')
        .replace(/\s+/g, '-')
        .replace(/-+/g, '-');
      
      // Stratégie B : Nettoyage radical (chiffres et lettres seulement) pour les chapitres comme "5.3."
      const cleanText = text.replace(/[^\wà-ÿ]/g, '');
      const cleanAnchor = decodedAnchor.replace(/[^\wà-ÿ]/g, '');

      if (slug === decodedAnchor || 
          slug.includes(decodedAnchor) || 
          (cleanAnchor.length > 2 && cleanText.includes(cleanAnchor)) ||
          text.includes(decodedAnchor)) {
        element = h;
        break;
      }
    }
  }

  if (element) {
    console.log('Element found, scrolling to offsetTop:', element.offsetTop);
    scrollContainer.value.scrollTo({
      top: element.offsetTop - 20,
      behavior: 'smooth'
    });
  } else {
    console.warn(`Anchor not found: ${decodedAnchor}`);
    scrollContainer.value.scrollTop = 0;
  }
}

  const handleLinkClick = (event) => {
    const target = event.target.closest('a');
    if (!target || !target.href) return;

    const hrefAttribute = target.getAttribute('href'); 
    console.log('Link clicked:', { href: target.href, attribute: hrefAttribute });

    if (!hrefAttribute) return;

    // Handle external links
    if (hrefAttribute.startsWith('http')) {
      event.preventDefault();
      event.stopPropagation();
      openUrl(hrefAttribute);
      return;
    }

    // Handle internal markdown links (with or without anchor)
    if (hrefAttribute.includes('.md')) {
      console.log('Intercepting markdown link:', hrefAttribute);
      event.preventDefault();
      event.stopPropagation();
      
      const [path, anchor] = hrefAttribute.split('#');
      // Decode path to handle %20 and other encoded characters
      const decodedPath = decodeURIComponent(path);
      const resolved = resolvePath(currentDocPath.value, decodedPath);
      console.log(`Navigating to doc: ${resolved}${anchor ? ' #'+anchor : ''}`);
      fetchDocumentation(resolved, false, anchor);
      return;
    }

    // Handle scroll to anchor in the SAME document
    if (hrefAttribute.startsWith('#')) {
      event.preventDefault();
      event.stopPropagation();
      const anchor = hrefAttribute.substring(1);
      scrollToAnchor(anchor);
      return;
    }
  };

watch(() => props.docPath, (newPath) => {
  if (newPath) fetchDocumentation(newPath);
}, { immediate: true });

const contentRef = ref(null);
const scrollContainer = ref(null);

const handleGlobalWheel = (event) => {
  // If we have content and the mouse is NOT over the content (to avoid double scroll)
  if (contentRef.value) {
    const isHoveringContent = contentRef.value.contains(event.target);
    
    // Si la souris n'est pas déjà sur le contenu, on scrolle manuellement
    if (!isHoveringContent) {
      // On peut ajouter une condition pour vérifier si le dialog est bien visible/au premier plan si nécessaire
      // Mais comme le composant est monté uniquement quand le dialog est ouvert (v-if), ça devrait aller.
      
      contentRef.value.scrollTop += event.deltaY;
    }
  }
};

onMounted(() => {
  window.addEventListener('wheel', handleGlobalWheel);
});

onUnmounted(() => {
  window.removeEventListener('wheel', handleGlobalWheel);
});
</script>

<style>
/* Base styles for markdown content */
.markdown-body {
  line-height: 1.6;
  font-family: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
  position: relative; /* ✅ Nécessaire pour que offsetTop soit relatif à ce conteneur */
}

/* Fix header colors to prevent transparency glitches */
.doc-header {
  flex: 0 0 auto; /* ✅ Fixe la hauteur */
  z-index: 10;
  border-bottom: 1px solid rgba(var(--v-border-color), 0.12);
}

.theme-light.v-card {
  background-color: #ffffff !important;
}
.theme-dark.v-card {
  background-color: #1e1e1e !important;
}

.theme-light .doc-header {
  background-color: #ffffff !important; /* ✅ Blanc pur, pas de transparence */
  color: #000000 !important;
}
.theme-dark .doc-header {
  background-color: #2a2a2a !important; /* Gris plus clair pour trancher */
  color: #ffffff !important;
}

/* Style spécifique pour la doc des paramètres */
.header-parameters {
  transition: background-color 0.3s ease, color 0.3s ease;
}

.theme-light .header-parameters {
  background-color: #FFF3E0 !important; /* Ambre très clair */
  border-bottom: 2px solid #FF9800 !important;
  color: #E65100 !important;
}

.theme-dark .header-parameters {
  background-color: #382b1a !important; /* Marron/Orange un peu plus clair */
  border-bottom: 2px solid #FF9800 !important;
  color: #FFB74D !important;
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

/* --- GitHub-style Alerts (Callouts) --- */
.markdown-body .markdown-alert {
  padding: 8px 16px;
  margin-bottom: 16px;
  border-left: 0.25em solid;
  border-radius: 0 6px 6px 0;
}

.markdown-body .markdown-alert-title {
  display: flex;
  align-items: center;
  font-size: 14px;
  font-weight: 600;
  line-height: 1.5;
  margin-bottom: 4px;
}

/* Types d'alertes */
.markdown-body .markdown-alert-note {
  border-left-color: #2196F3;
  background-color: rgba(33, 150, 243, 0.1);
}
.markdown-body .markdown-alert-note .markdown-alert-title { color: #2196F3; }

.markdown-body .markdown-alert-tip {
  border-left-color: #4CAF50;
  background-color: rgba(76, 175, 80, 0.1);
}
.markdown-body .markdown-alert-tip .markdown-alert-title { color: #4CAF50; }

.markdown-body .markdown-alert-important {
  border-left-color: #7c4dff;
  background-color: rgba(124, 77, 255, 0.1);
}
.markdown-body .markdown-alert-important .markdown-alert-title { color: #7c4dff; }

.markdown-body .markdown-alert-warning {
  border-left-color: #FF9800;
  background-color: rgba(255, 152, 0, 0.1);
}
.markdown-body .markdown-alert-warning .markdown-alert-title { color: #FF9800; }

.markdown-body .markdown-alert-caution {
  border-left-color: #F44336;
  background-color: rgba(244, 67, 54, 0.1);
}
.markdown-body .markdown-alert-caution .markdown-alert-title { color: #F44336; }

/* Adaptation au mode sombre */
.theme-dark .markdown-body .markdown-alert-note { background-color: rgba(33, 150, 243, 0.15); }
.theme-dark .markdown-body .markdown-alert-tip { background-color: rgba(76, 175, 80, 0.15); }
.theme-dark .markdown-body .markdown-alert-important { background-color: rgba(124, 77, 255, 0.15); }
.theme-dark .markdown-body .markdown-alert-warning { background-color: rgba(255, 152, 0, 0.15); }
.theme-dark .markdown-body .markdown-alert-caution { background-color: rgba(244, 67, 54, 0.15); }

</style>