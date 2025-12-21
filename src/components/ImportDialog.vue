<template>
  <v-dialog v-model="dialog" max-width="800px" @click:outside="close" @keydown.esc="close">
    <v-card>
      <v-card-title class="d-flex justify-space-between align-center">
        <span>{{ title }}</span>
        <v-btn icon="mdi-close" variant="text" @click="close"></v-btn>
      </v-card-title>
      
      <v-card-text>
        <!-- Path Navigation / Breadcrumbs could go here if needed, but path display is enough -->
        <div class="d-flex align-center mb-2 text-caption text-grey">
            <v-icon icon="mdi-folder-open" size="small" class="mr-2"></v-icon>
            <span class="text-truncate">{{ currentPath }}</span>
        </div>

        <v-text-field
          v-model="filterText"
          label="Filtrer"
          prepend-inner-icon="mdi-magnify"
          variant="outlined"
          hide-details
          clearable
          density="compact"
          class="mb-4"
        ></v-text-field>

        <div v-if="error" class="text-error mb-2">{{ error }}</div>

        <v-list density="compact" class="border rounded" style="max-height: 400px; overflow-y: auto;">
            <!-- Go Up Directory -->
            <v-list-item
                v-if="canGoUp"
                @click="goUp"
                prepend-icon="mdi-arrow-up-bold-box-outline"
                title=".."
            ></v-list-item>

            <v-list-item
                v-for="item in filteredItems"
                :key="item.path"
                @click="handleItemClick(item)"
                :title="item.name"
                :value="item"
            >
                <template v-slot:prepend>
                    <v-icon :icon="item.isDir ? 'mdi-folder' : 'mdi-file'" :color="item.isDir ? 'yellow-darken-2' : 'blue'"></v-icon>
                </template>
                <template v-slot:append>
                    <span class="text-caption text-grey">{{ formatSize(item.size) }}</span>
                </template>
            </v-list-item>
            
            <v-list-item v-if="filteredItems.length === 0 && !loading">
                <v-list-item-title class="text-center text-grey">Aucun fichier trouvé</v-list-item-title>
            </v-list-item>
        </v-list>
        
        <div v-if="loading" class="d-flex justify-center mt-4">
            <v-progress-circular indeterminate color="primary"></v-progress-circular>
        </div>

      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  modelValue: Boolean,
  title: {
      type: String,
      default: 'Importer un fichier'
  },
  extensions: {
      type: Array,
      default: () => [] 
  }
});

const emit = defineEmits(['update:modelValue', 'select']);

const dialog = ref(props.modelValue);
const currentPath = ref('');
const items = ref([]);
const loading = ref(false);
const error = ref(null);
const filterText = ref('');
// Navigation History could be added here

const filteredItems = computed(() => {
    if (!filterText.value) return items.value;
    const lowerFilter = filterText.value.toLowerCase();
    return items.value.filter(item => item.name.toLowerCase().includes(lowerFilter));
});

const canGoUp = computed(() => {
    // Simple check: if current path is not empty and likely has a parent
    // OS dependent logic might be tricky, but basic availability of parent is safer handled by ".." entry in backend? 
    // Actually backend doesn't return ".." 
    // We can simulate it. 
    return currentPath.value && currentPath.value.length > 1; // Very rough check
});

watch(() => props.modelValue, (newVal) => {
  dialog.value = newVal;
  if (newVal) {
      // Load initial path (DEFAULT_IMPORT logic handled by backend if we pass specific magic string or None)
      loadDirectory('DEFAULT_IMPORT');
      filterText.value = '';
  }
});

watch(dialog, (newVal) => {
  emit('update:modelValue', newVal);
});

async function loadDirectory(path) {
    loading.value = true;
    error.value = null;
    items.value = [];
    
    try {
        const result = await invoke('list_import_files', { 
            path: path,
            extensions: props.extensions 
        });
        currentPath.value = result.path;
        items.value = result.entries;
    } catch (e) {
        error.value = `Erreur: ${e}`;
        console.error(e);
    } finally {
        loading.value = false;
    }
}

function handleItemClick(item) {
    if (item.isDir) {
        loadDirectory(item.path);
    } else {
        emit('select', item.path);
        close();
    }
}

function goUp() {
    // Basic string manipulation to go up. 
    // Ideally we use a 'dirname' logic.
    // Since we don't have Rust 'dirname' exposed directly other than via list_files which takes a path...
    // We can try to guess parent.
    // Or better: ask backend to resolve parent? 
    // For now: naive implementation for UNIX/Windows commonality
    // But separators differ.
    // Safer: loadDirectory with 'path/..' -> Backend handles it? 
    // Rust PathBuf handles '..' but canonicalization might be needed.
    // Let's try sending currentPath + "/.."
    
    // BUT checking for root is hard.
    // Let's assume separator.
    let parent = "";
    if (currentPath.value.includes("\\")) {
         // Windows
         parent = currentPath.value.substring(0, currentPath.value.lastIndexOf("\\"));
         if (parent.endsWith(":")) parent += "\\"; // Drive root C:\
    } else {
         // Unix
         parent = currentPath.value.substring(0, currentPath.value.lastIndexOf("/"));
         if (parent === "") parent = "/";
    }
    
    if (parent) {
        loadDirectory(parent);
    }
}

function close() {
  dialog.value = false;
}

function formatSize(bytes) {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

</script>
