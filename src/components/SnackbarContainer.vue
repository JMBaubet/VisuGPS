<template>
  <v-snackbar
    v-model="snackbar.show"
    :color="snackbar.color"
    :timeout="snackbar.timeout"
    location="bottom left"
  >
    {{ snackbar.message }}
    
  </v-snackbar>
</template>

<script setup>
import { useSnackbar } from '../composables/useSnackbar';

const { snackbar } = useSnackbar();
</script>