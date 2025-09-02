<!-- <template>
  <div>
    <h1>Vue Principale</h1>
  </div>
</template> -->

<template>
  <v-app>
    <v-main>
      <v-container>
        <v-btn color="green" @click="notify('Succès !', 'success')">Succès</v-btn>
        <v-btn color="blue" @click="notify('Information', 'info')">Info</v-btn>
        <v-btn color="orange" @click="notify('Attention', 'warning')">Warning</v-btn>
        <v-btn color="red" @click="notify('Erreur', 'error', 10000)">Erreur</v-btn>
      </v-container>

      <MessageContainer ref="notifier" />
    </v-main>
  </v-app>
</template>

<script setup>
import { ref } from "vue";
import MessageContainer from "../components/MessageContainer.vue";

const notifier = ref(null);

function notify(message, type, duration) {
  notifier.value.addMessage({ message, type, duration });
}
</script>

