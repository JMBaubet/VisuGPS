<template>
  <v-app>
    <AppBarSetting />
    <v-main>
      <v-container>
        <v-row justify="center">
          <v-col cols="12" md="8" lg="6">
            <MapboxTokenEditor />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup>
import AppBarSetting from '../components/AppBarSetting.vue';
import MapboxTokenEditor from '../components/MapboxTokenEditor.vue';
</script>