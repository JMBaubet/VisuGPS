<template>
  <v-snackbar
    v-model="snackbar.show"
    :color="snackbar.color"
    :timeout="snackbar.timeout"
    location="bottom right"
  >
    {{ snackbar.message }}
    <template v-slot:actions>
      <v-btn
        color="white"
        variant="text"
        @click="snackbar.show = false"
      >
        Fermer
      </v-btn>
    </template>
  </v-snackbar>
</template>

<script setup>
import { useSnackbar } from '../composables/useSnackbar';

const { snackbar } = useSnackbar();
</script>