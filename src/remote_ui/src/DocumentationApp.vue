<template>
  <v-app>
    <v-app-bar :color="toolbarColor" density="compact">
      <v-avatar rounded="0" class="ml-2 mr-2" style="cursor: pointer" @click="goHome">
        <v-img src="/logo.png" alt="Logo"></v-img>
      </v-avatar>
      <v-app-bar-title style="min-width: 60%; flex-grow: 1">{{ currentDoc ? currentDocTitle : 'Documentation' }}</v-app-bar-title>
      <v-btn icon href="/" title="Retour télécommande">
         <v-icon>mdi-remote</v-icon>
      </v-btn>
    </v-app-bar>

    <v-main>
      <!-- Liste des documents -->
      <v-container v-if="!currentDoc">
        <div v-if="loading" class="d-flex justify-center mt-5">
          <v-progress-circular indeterminate></v-progress-circular>
        </div>
        
        <v-list v-else lines="two">
          <v-list-item
            v-for="doc in docs"
            :key="doc.filename"
            @click="openDoc(doc)"
            :title="doc.title"
            :subtitle="doc.filename"
            prepend-icon="mdi-file-document-outline"
          ></v-list-item>
        </v-list>
        
        <v-alert v-if="error" type="error" class="mt-4">{{ error }}</v-alert>
      </v-container>

      <!-- Lecture d'un document -->
      <v-container v-else class="doc-content">
        <div v-if="loadingContent" class="d-flex justify-center mt-5">
           <v-progress-circular indeterminate></v-progress-circular>
        </div>
        <div v-else class="markdown-body" v-html="renderedContent" @click="handleLinkClick"></div>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup>
import { ref, onMounted, computed, watch, nextTick } from 'vue'
import MarkdownIt from 'markdown-it'
import mermaid from 'mermaid'

// Initialize Mermaid
mermaid.initialize({
  startOnLoad: false,
  theme: 'default',
  securityLevel: 'loose',
});

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  breaks: true,
  highlight: function (str, lang) {
    if (lang === 'mermaid') {
      return `<div class="mermaid">${str}</div>`;
    }
    return ''; // ou gestion habituelle du code
  }
})

const docs = ref([])
const currentDoc = ref(null)
const currentDocTitle = ref('')
const docContent = ref('')
const loading = ref(false)
const loadingContent = ref(false)
const error = ref(null)

const isParameterDoc = computed(() => {
  if (!currentDoc.value) return false;
  const path = currentDoc.value.toLowerCase();
  // Détection basée sur le dossier ou le nom de fichier
  return path.includes('docparametrage') || path.includes('parametres.md');
});

const toolbarColor = computed(() => {
  return isParameterDoc.value ? 'orange-darken-3' : 'primary';
});

// Gestion du hash pour le routing simple
const handleHashChange = () => {
  const hash = window.location.hash
  if (hash.startsWith('#/read/')) {
    let fullPath = hash.replace('#/read/', '')
    let filename = fullPath;
    let anchor = null;

    // Support params ?anchor=...
    if (fullPath.includes('?anchor=')) {
        const parts = fullPath.split('?anchor=');
        filename = parts[0];
        anchor = parts[1];
    } else if (fullPath.includes('#')) {
        // Fallback si quelqu'un a mis #/read/file.md#anchor (bien que déconseillé)
        const parts = fullPath.split('#');
        filename = parts[0];
        anchor = parts[1];
    }

    // Nettoyer le filename (au cas où ./ ou / au début, mais on garde le chemin relatif)
    // On enlève juste le / initial s'il y en a un
    if (filename.startsWith('/')) filename = filename.substring(1);

    // Si on a déjà la liste, on peut retrouver le titre
    const found = docs.value.find(d => d.filename === filename)
    openDoc({ filename, title: found ? found.title : filename }, false, anchor)
  } else {
    currentDoc.value = null
  }
}

const fetchDocs = async () => {
  loading.value = true
  try {
    const res = await fetch('/api/docs')
    if (!res.ok) throw new Error('Erreur chargement liste')
    docs.value = await res.json()
    
    // Vérifier si un hash est présent au chargement
    handleHashChange()
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

const openDoc = async (doc, updateHash = true, anchor = null) => {
  currentDoc.value = doc.filename
  currentDocTitle.value = doc.title
  loadingContent.value = true
  docContent.value = ''
  
  if (updateHash) {
    let newHash = `#/read/${doc.filename}`;
    if (anchor) newHash += `?anchor=${anchor}`;
    window.location.hash = newHash;
  }

  try {
    const res = await fetch(`/api/docs/${doc.filename}`)
    if (!res.ok) throw new Error('Erreur chargement contenu')
    docContent.value = await res.text()
    
    // Scroll to anchor after render
    if (anchor) {
        nextTick(() => {
            setTimeout(() => {
                scrollToAnchor(anchor);
            }, 500); // Délai pour laisser le temps au rendu Markdown/Mermaid
        });
    } else {
         nextTick(() => {
             window.scrollTo(0, 0);
         });
    }

  } catch (e) {
    docContent.value = `Erreur: ${e.message}`
  } finally {
    loadingContent.value = false
  }
}

function scrollToAnchor(id) {
    // Essayer de trouver l'élément par ID (markdown-it génère des id pour les titres souvent, sinon par name)
    // Les ancres générées par github sont souvent en minuscules avec tirets
    // Décodage au cas où
    try {
        id = decodeURIComponent(id);
    } catch(e) {}

    const element = document.getElementById(id) || document.getElementById(id.toLowerCase()); 
    if (element) {
        element.scrollIntoView({ behavior: 'smooth' });
    } else {
        // Recherche un peu plus large (par ex name=id)
        const named = document.getElementsByName(id);
        if (named.length > 0) named[0].scrollIntoView({ behavior: 'smooth' });
    }
}

const renderedContent = computed(() => {
  if (!docContent.value) return '';
  
  // Custom renderer rule to rewrite image paths
  // Save original rule
  const defaultRender = md.renderer.rules.image || function(tokens, idx, options, env, self) {
    return self.renderToken(tokens, idx, options);
  };
  
  md.renderer.rules.image = function (tokens, idx, options, env, self) {
    const token = tokens[idx];
    const srcIndex = token.attrIndex('src');
    if (srcIndex >= 0) {
      let src = token.attrs[srcIndex][1];
      // Si c'est un chemin relatif commençant par ../
      // Ex: ../images/logo.png -> /static-docs/images/logo.png
      if (src.startsWith('../')) {
         src = '/static-docs/' + src.substring(3);
         token.attrs[srcIndex][1] = src;
      } else if (src.startsWith('./')) {
         // Si c'est ./image.png dans DocUtilisateur -> /static-docs/DocUtilisateur/image.png
         src = '/static-docs/DocUtilisateur/' + src.substring(2);
         token.attrs[srcIndex][1] = src;
      } else if (!src.startsWith('/') && !src.startsWith('http')) {
         // Si c'est image.png -> /static-docs/DocUtilisateur/image.png
         src = '/static-docs/DocUtilisateur/' + src;
         token.attrs[srcIndex][1] = src;
      }
    }
    return defaultRender(tokens, idx, options, env, self);
  };

  let html = md.render(docContent.value);

  // Post-traitement pour les balises HTML brutes (ex: <img src="../images/...">)
  // Qui ne sont pas gérées par la règle markdown-it ci-dessus
  html = html.replace(/<img\s+[^>]*src="([^"]+)"[^>]*>/g, (match, src) => {
      let newSrc = src;
      if (src.startsWith('../')) {
         newSrc = '/static-docs/' + src.substring(3);
      } else if (src.startsWith('./')) {
         newSrc = '/static-docs/DocUtilisateur/' + src.substring(2);
      } else if (!src.startsWith('/') && !src.startsWith('http')) {
         newSrc = '/static-docs/DocUtilisateur/' + src;
      }
      
      if (newSrc !== src) {
          return match.replace(src, newSrc);
      }
      return match;
  });

  return html;
})

async function renderMermaidDiagrams() {
  const mermaidDivs = document.querySelectorAll('.mermaid');
  if (mermaidDivs.length === 0) return;

  try {
    await mermaid.run({
       nodes: mermaidDivs,
    });
  } catch (err) {
    console.error('Mermaid rendering error:', err);
  }
}

// Trigger mermaid render after content update
watch(renderedContent, () => {
  nextTick(() => {
    renderMermaidDiagrams();
  });
});

function resolvePath(basePath, relativePath) {
  if (relativePath.startsWith('/')) return relativePath.substring(1); 
  if (relativePath.startsWith('http')) return relativePath; 

  // Normalisation
  const stack = basePath.split('/');
  // On enlève le nom du fichier courant pour avoir le dossier courant
  stack.pop();

  const parts = relativePath.split('/');
  for (const part of parts) {
    if (part === '.') continue;
    if (part === '') continue; // double slash protection
    if (part === '..') {
      if (stack.length > 0) stack.pop();
    } else {
      stack.push(part);
    }
  }
  return stack.join('/');
}

const goHome = () => {
  // Navigation vers l'index de la documentation utilisateur
  window.location.hash = '#/read/DocUtilisateur/index.md';
};

const handleLinkClick = (event) => {
  const target = event.target.closest('a');
  if (!target || !target.getAttribute('href')) return;

  const href = target.getAttribute('href');

  // Si c'est un lien vers un fichier markdown (interne)
  if (href.includes('.md')) {
    event.preventDefault();
    
    // Gestion avec ancre potentielle
    // ex: ./upload.md#1-intro
    const parts = href.split('#');
    let pathPart = parts[0];
    const anchorPart = parts[1] || null;

    // Nettoyer le chemin (./p.md -> p.md)
    let filename = pathPart;
    // Résolution relative basée sur le document courant
    if (currentDoc.value) {
        filename = resolvePath(currentDoc.value, filename);
    } else {
        // Fallback si pas de doc courant (peu probable ici)
        if (filename.startsWith('./')) filename = filename.substring(2);
        if (filename.startsWith('/')) filename = filename.substring(1);
    }
    
    // Mise à jour du hash pour déclencher la navigation
    let newHash = `#/read/${filename}`;
    if (anchorPart) {
        newHash += `?anchor=${anchorPart}`;
    }
    
    window.location.hash = newHash;
  }
};

onMounted(() => {
  fetchDocs()
  window.addEventListener('hashchange', handleHashChange)
})
</script>

<style>
/* Styles basiques pour le markdown */
.doc-content {
  max-width: 800px;
  margin: 0 auto;
}
.markdown-body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
  line-height: 1.6;
}
/* Curseur pointer pour les liens */
.markdown-body a {
  cursor: pointer;
  color: #1976D2;
  text-decoration: none;
}
.markdown-body a:hover {
  text-decoration: underline;
}
.markdown-body h1, .markdown-body h2, .markdown-body h3 {
  margin-top: 1.5em;
  margin-bottom: 0.5em;
  font-weight: 600;
}
.markdown-body h1 { border-bottom: 1px solid #444; padding-bottom: 0.3em; }
.markdown-body p { margin-bottom: 1em; }
.markdown-body code {
  background-color: rgba(128, 128, 128, 0.2);
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
}
.markdown-body pre {
  background-color: #1e1e1e;
  padding: 1em;
  border-radius: 6px;
  overflow-x: auto;
}
.markdown-body pre code {
  background-color: transparent;
  padding: 0;
}
.markdown-body img {
  max-width: 100%;
  border-radius: 4px;
}
.markdown-body blockquote {
  border-left: 4px solid #666;
  padding-left: 1em;
  color: #aaa;
  margin-left: 0;
}
</style>
