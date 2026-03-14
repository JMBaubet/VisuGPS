<template>
  <v-container class="fill-height d-flex flex-column" style="max-width: 600px; position: relative;">
    
    <!-- 1. Top Buttons Row (Sizes mathching PlaybackControls) -->
    <v-row class="w-100 flex-grow-0 mb-4" justify="space-between" align="center">
      <!-- MDI Home -->
      <v-col cols="4" class="text-center">
        <v-btn 
            size="80" 
            rounded="circle"
            :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
            variant="text" 
            @click="goHome()"
            title="Retour Vue Principale"
        >
            <v-icon icon="mdi-home" size="64"></v-icon>
        </v-btn>
      </v-col>
      <!-- Horaires -->
      <v-col cols="4" class="text-center">
        <v-btn 
            size="80" 
            rounded="circle"
            color="deep-purple"
            variant="text" 
            @click="toggleHoraires()"
            title="Afficher/Masquer les horaires"
        >
            <v-icon icon="mdi-routes-clock" size="64"></v-icon>
        </v-btn>
      </v-col>
      <!-- Reload -->
      <v-col cols="4" class="text-center">
        <v-btn 
            size="80" 
            rounded="circle"
            :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
            variant="text" 
            @click="reloadAnimation()"
            title="Relancer l'animation"
        >
            <v-icon icon="mdi-reload" size="64"></v-icon>
        </v-btn>
      </v-col>
    </v-row>

    <!-- 1bis. Second Buttons Row (Variants & Recenter) -->
    <v-row class="w-100 flex-grow-0 mb-4" justify="start" align="center">
        <!-- Recenter -->
        <v-col cols="4" class="text-center">
            <v-btn 
                size="80" 
                rounded="circle"
                :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
                variant="text" 
                @click="recenterTrace()"
                title="Recentrer la trace"
            >
                <v-icon icon="mdi-image-filter-center-focus" size="64"></v-icon>
            </v-btn>
        </v-col>

        <!-- Return to Main Trace (Only if in Variant) -->
        <v-col cols="4" class="text-center" v-if="isVariantMode">
            <v-btn 
                size="80" 
                rounded="circle"
                color="green" 
                variant="text" 
                @click="triggerMainTrace()"
                title="Retour Trace Principale"
            >
                <v-icon icon="mdi-map-marker-distance" size="64"></v-icon>
            </v-btn>
        </v-col>

        <!-- Variants (If main trace and variants exist, OR if variant mode and multiple variants exist) -->
        <v-col cols="4" class="text-center" v-if="(!isVariantMode && variantCount > 0) || (isVariantMode && variantCount > 1)">
            <v-btn 
                size="80" 
                rounded="circle"
                color="blue" 
                variant="text" 
                @click="triggerVariant()"
                title="Choisir une variante"
            >
                <v-icon icon="mdi-map-marker-path" size="64"></v-icon>
            </v-btn>
        </v-col>
    </v-row>

    <v-divider class="w-100 mb-4"></v-divider>

    <!-- 2. Main Camera Controls Grid -->
    <div class="d-flex flex-column flex-grow-1 w-100">
        
        <!-- Pan Area & Zoom (Row 1) -->
        <v-row class="flex-grow-1 mb-2" dense>
            <v-col cols="3" class="h-100 pb-0">
                <CameraTouchPad 
                    mode="zoom" 
                    class="w-100 h-100" 
                    :sensitivityY="sensZoom"
                    @update="handleCameraUpdate"
                >
                    <v-icon icon="mdi-magnify-plus-outline" size="48"></v-icon>
                </CameraTouchPad>
            </v-col>
            <v-col cols="9" class="h-100 pb-0">
                <CameraTouchPad 
                    mode="pan" 
                    class="w-100 h-100" 
                    :sensitivityX="sensX" 
                    :sensitivityY="sensY"
                    @update="handleCameraUpdate"
                >
                    <v-icon icon="mdi-cursor-move" size="64"></v-icon>
                </CameraTouchPad>
            </v-col>
        </v-row>
    </div>

    <!-- Variant Selector Dialog -->
    <VariantSelectorDialog
      v-model="showVariantDialog"
      :variants="variants"
      :is-dark="isDark"
      @select="onVariantSelect"
    />
  </v-container>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useTheme } from 'vuetify'
import { useRemoteStore } from '@/stores/remoteStore'
import CameraTouchPad from '@/components/CameraTouchPad.vue'
import VariantSelectorDialog from '@/components/VariantSelectorDialog.vue'

const store = useRemoteStore()
const theme = useTheme()
const isDark = computed(() => theme.global.current.value.dark)

// View State Bindings
const isVariantMode = computed(() => store.visualizeViewState?.isVariantTrace ?? false);
const variantCount = computed(() => store.visualizeViewState?.variantCount || 0);
const variants = computed(() => store.visualizeViewState?.variants || []);
const showVariantDialog = ref(false);

// Settings
const sensX = computed(() => store.remoteSettings?.sensibilitePointDeVueX ?? 1.5)
const sensY = computed(() => store.remoteSettings?.sensibilitePointDeVueY ?? 1.5)
const sensZoom = computed(() => store.remoteSettings?.sensibiliteZoom ?? 0.1)

// Actions
function goHome() {
    store.sendCommand('go_home');
}

function reloadAnimation() {
    store.sendCommand('restart_animation'); // Uses existing reset command
}

function toggleHoraires() {
    store.sendCommand('toggle_horaires');
}

function recenterTrace() {
    store.sendCommand('recenter_trace');
}

function triggerMainTrace() {
    store.sendCommand('return_to_main_trace');
}

function triggerVariant() { 
    if (variantCount.value === 1 && variants.value.length > 0) {
        store.sendCommand('select_variant', { variantId: variants.value[0].id });
    } else if (variantCount.value > 1) {
        showVariantDialog.value = true;
    }
}

function onVariantSelect(variantId) {
    store.sendCommand('select_variant', { variantId });
}

function handleCameraUpdate(payload) {
    store.sendCommand('update_camera', payload)
}
</script>
