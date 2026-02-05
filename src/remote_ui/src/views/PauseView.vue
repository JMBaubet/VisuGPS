<template>
  <v-container class="fill-height d-flex flex-column" style="max-width: 600px; position: relative;">
    
    <!-- 1. Playback Controls & Speed (Shared Component) -->
    <PlaybackControls ref="playbackControls">
        <template #third-button>
             <v-btn 
                size="80" 
                rounded="circle"
                :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
                variant="text" 
                @click="triggerFinalView()"
                :disabled="isFlytoActive"
            >
                <v-icon icon="mdi-clock-end" size="64"></v-icon>
            </v-btn>
        </template>
    </PlaybackControls>


    
    <v-divider class="w-100 mb-4"></v-divider>

    <!-- 4. Camera Controls Grid -->
    <div class="d-flex flex-column flex-grow-1 w-100">
        
        <!-- Pan Area (Large square) -->
        <div class="flex-grow-1 mb-2">
            <CameraTouchPad 
                mode="pan" 
                class="w-100 h-100" 
                :sensitivityX="sensX" 
                :sensitivityY="sensY"
                @update="handleCameraUpdate"
                :disabled="isFlytoActive"
            >
                <v-icon icon="mdi-cursor-move" size="64"></v-icon>
            </CameraTouchPad>
        </div>

        <!-- Strip Controls -->
        <v-row dense style="height: 150px;" class="flex-grow-0">
            <v-col cols="3">
                <CameraTouchPad 
                    mode="zoom" 
                    class="w-100 h-100" 
                    :sensitivityY="sensZoom"
                    @update="handleCameraUpdate"
                    :disabled="isFlytoActive"
                >
                    <v-icon icon="mdi-magnify-plus-outline" size="48"></v-icon>
                </CameraTouchPad>
            </v-col>
            <v-col cols="6" class="d-flex flex-column justify-space-between align-center py-1">
                <!-- Cap (Top) -->
                <CameraTouchPad 
                    mode="bearing" 
                    class="w-100"
                    style="height: 70px;" 
                    :sensitivityX="sensBearing"
                     @update="handleCameraUpdate"
                     :disabled="isFlytoActive"
                >
                    <v-icon icon="mdi-compass-outline" size="48"></v-icon>
                </CameraTouchPad>

                <!-- Variant & Home Buttons (Bottom) -->
                <div class="d-flex align-center justify-center">
                    <!-- Home Button -->
                    <v-btn 
                        v-if="isFinished"
                        size="60" 
                        rounded="circle"
                        :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
                        variant="text" 
                        @click="goHome()"
                        title="Retour Vue Principale"
                        :disabled="isFlytoActive"
                    >
                        <v-icon icon="mdi-home" size="40"></v-icon>
                    </v-btn>

                    <!-- Variant Button -->
                    <v-btn 
                        v-if="variantCount > 0 || isVariantMode"
                        size="60" 
                        rounded="circle"
                        :color="isDark ? 'grey-lighten-1' : 'grey-darken-1'" 
                        variant="text" 
                        @click="triggerVariant()"
                        :title="isVariantMode ? 'Retour Trace Principale' : 'Choisir une variante'"
                        :disabled="isFlytoActive"
                    >
                        <v-icon :icon="isVariantMode ? 'mdi-map-marker-distance' : 'mdi-map-marker-path'" size="40"></v-icon>
                    </v-btn>
                </div>
            </v-col>
            <v-col cols="3">
                <CameraTouchPad 
                    mode="tilt" 
                    class="w-100 h-100" 
                    :sensitivityY="sensTilt"
                     @update="handleCameraUpdate"
                     :disabled="isFlytoActive"
                >
                    <v-icon icon="mdi-angle-acute" size="48"></v-icon>
                </CameraTouchPad>
            </v-col>
        </v-row>
    </div>

    <!-- NEW: Segment Bar (Bottom) -->
    <div v-if="isVariantMode" class="w-100 flex-grow-0">
        <v-divider class="w-75 mx-auto my-2"></v-divider>
        <SegmentBar 
            :segments="variantSegments"
            :current-index="currentSegmentIndex"
            @jump="onSegmentJump"
            :disabled="isFlytoActive"
        />
    </div>

    <!-- Fly-to Overlay -->
    <v-fade-transition>
      <div v-if="isFlytoActive" class="flyto-overlay">
        <v-card class="pa-4 d-flex align-center bg-black-opacity-70 text-white" rounded="lg">
          <v-progress-circular indeterminate color="primary" class="mr-4" size="24"></v-progress-circular>
          <span class="text-h6">Repositionnement...</span>
        </v-card>
      </div>
    </v-fade-transition>

    <VariantSelectorDialog
      v-model="showVariantDialog"
      :variants="variants"
      :is-dark="isDark"
      @select="onVariantSelect"
    />

  </v-container>
</template>

<script setup>
import { ref, computed, watch } from 'vue' // removed watch
import { useTheme } from 'vuetify'
import { useRemoteStore } from '@/stores/remoteStore'
import CameraTouchPad from '@/components/CameraTouchPad.vue'
import PlaybackControls from '@/components/PlaybackControls.vue'
import VariantSelectorDialog from '@/components/VariantSelectorDialog.vue'
import SegmentSelectorDialog from '@/components/SegmentSelectorDialog.vue'
import SegmentBar from '@/components/SegmentBar.vue'

const store = useRemoteStore()
const theme = useTheme()
const isDark = computed(() => theme.global.current.value.dark)

// --- Variant Logic ---
const isVariantMode = computed(() => store.visualizeViewState?.isVariantTrace ?? false);
const variantSegments = computed(() => store.visualizeViewState?.segments || []); 
const currentSegmentIndex = computed(() => store.visualizeViewState?.currentSegmentIndex ?? -1);
const hasVariants = computed(() => store.visualizeViewState?.hasVariants ?? false); 
const variantCount = computed(() => store.visualizeViewState?.variantCount || 0);
const variants = computed(() => store.visualizeViewState?.variants || []);
const isFlytoActive = computed(() => store.visualizeViewState?.isFlytoActive ?? false);
const showVariantDialog = ref(false);

// Close dialogs when view changes (e.g. selection made on desktop)
watch(isVariantMode, (newVal) => {
    if (newVal) {
        showVariantDialog.value = false;
    }
});

function onSegmentJump(index) {
    store.sendCommand('jump_to_segment', { index });
}

const playbackControls = ref(null)
function resetSpeed() {
    playbackControls.value?.resetSpeed()
}

// --- Settings ---
// Default values if settings not loaded
// settingsDefault.json path: 
// sensibilitePointDeVueX/Y: groups[0].groupes[0].parametres... actually it was in separate file.
// In settingsDefault root: data.groupes[...]
// Need to find IDs: 'sensibilitePointDeVueX', 'sensibilitePointDeVueY', 'sensibiliteZoom', 'sensibiliteCap', 'sensibiliteTilt'
// The backend likely sends a flat map or the structured object?
// existing logic: `data.settings` passed to `applyRemoteSettings`.
// Let's assume `store.remoteSettings` is a Key-Value map of identifiants -> values.
// because backend usually flattens config for usage.
// If it sends the raw hierarchy, I need to search.
// BUT `remote-client-v2.js` does: `if (window.applyRemoteSettings) window.applyRemoteSettings(data.settings);`
// and `remote-camera.js` used: `settings.sensibilitePointDeVueX` directly.
// So it seems it is a flat object.

const sensX = computed(() => store.remoteSettings?.sensibilitePointDeVueX ?? 1.5)
const sensY = computed(() => store.remoteSettings?.sensibilitePointDeVueY ?? 1.5)
const sensZoom = computed(() => store.remoteSettings?.sensibiliteZoom ?? 0.1)
const sensBearing = computed(() => store.remoteSettings?.sensibiliteCap ?? 0.5)
const sensTilt = computed(() => store.remoteSettings?.sensibiliteTilt ?? 0.5)

// --- Camera Action ---
function handleCameraUpdate(payload) {
    // payload: { type, dx, dy }
    // We send it to backend
    store.sendCommand('update_camera', payload)
}


// --- Playback Logic (Refactored to PlaybackControls) ---
const isFinished = computed(() => store.visualizeViewState?.animationState === 'Termine' || store.visualizeViewState?.animationState === 'Vol_Final');

function triggerVariant() { 
    if (isVariantMode.value) {
        store.sendCommand('return_to_main_trace');
    } else if (variantCount.value === 1 && variants.value.length > 0) {
        // Direct select if only one
        store.sendCommand('select_variant', { variantId: variants.value[0].id });
    } else if (variantCount.value > 1) {
        showVariantDialog.value = true;
    }
}

function onVariantSelect(variantId) {
    store.sendCommand('select_variant', { variantId });
}

function triggerFinalView() {
    store.sendCommand('trigger_final_view');
}

function goHome() {
    store.sendCommand('go_home');
}

</script>

<style scoped>
.flyto-overlay {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 5000;
  pointer-events: none;
  width: 90%;
  max-width: 400px;
}
.bg-black-opacity-70 {
  background-color: rgba(0, 0, 0, 0.7) !important;
}
</style>
