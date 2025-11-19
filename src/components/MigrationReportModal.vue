<template>
  <v-dialog v-model="dialog" max-width="800px" persistent scrollable>
    <v-card>
      <v-card-title class="text-h5 bg-primary text-white">
        Mise à jour de la configuration
      </v-card-title>

      <v-card-text class="pa-4" style="max-height: 60vh; overflow-y: auto;">
        <!-- We render markdown as HTML if possible, or just text -->
        <div v-html="renderedMarkdown" class="markdown-body"></div>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="primary" variant="elevated" @click="close">
          Compris
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import MarkdownIt from 'markdown-it';

const md = new MarkdownIt();

const props = defineProps({
  modelValue: Boolean,
  report: String,
});

const emit = defineEmits(['update:modelValue']);

const dialog = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val),
});

const renderedMarkdown = computed(() => {
  if (!props.report) return '';
  try {
      return md.render(props.report);
  } catch (e) {
      console.warn("Markdown rendering failed", e);
      return props.report.replace(/\n/g, '<br>');
  }
});

const close = () => {
  dialog.value = false;
};
</script>

<style scoped>
.markdown-body {
  font-family: sans-serif;
  line-height: 1.6;
}
.markdown-body :deep(h1) {
  font-size: 1.5em;
  border-bottom: 1px solid #eaecef;
  padding-bottom: 0.3em;
  margin-top: 0;
}
.markdown-body :deep(h2) {
  font-size: 1.3em;
  border-bottom: 1px solid #eaecef;
  padding-bottom: 0.3em;
  margin-top: 1.5em;
}
.markdown-body :deep(ul) {
  padding-left: 2em;
}
.markdown-body :deep(li) {
  margin-bottom: 0.5em;
}
.markdown-body :deep(strong) {
  font-weight: bold;
}
</style>
