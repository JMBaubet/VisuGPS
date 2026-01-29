<template>
  <v-container class="fill-height d-flex flex-column" style="max-width: 600px;">
    
    <!-- 1. Playback Controls & Speed (Shared Component) -->
    <PlaybackControls>
        <template #third-button>
             <v-btn 
                size="80" 
                rounded="circle"
                :color="isDark ? 'white' : 'grey-darken-3'" 
                variant="text" 
                @click="isVariantMode ? showSegmentDialog = true : triggerFinalView()"
                :disabled="isFinished && !isVariantMode"
            >
                <v-icon :icon="isVariantMode ? 'mdi-map-marker-radius-outline' : 'mdi-clock-end'" size="64"></v-icon>
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
                >
                    <v-icon icon="mdi-compass-outline" size="48"></v-icon>
                </CameraTouchPad>

                <!-- Variant Button (Bottom) -->
                 <v-btn 
                    v-if="variantCount > 0 || isVariantMode"
                    size="60" 
                    rounded="circle"
                    :color="isDark ? 'white' : 'grey-darken-3'" 
                    variant="text" 
                    @click="triggerVariant()"
                    :title="isVariantMode ? 'Retour Trace Principale' : 'Choisir une variante'"
                >
                    <v-icon :icon="isVariantMode ? 'mdi-map-marker-distance' : 'mdi-map-marker-path'" size="40"></v-icon>
                </v-btn>
            </v-col>
            <v-col cols="3">
                <CameraTouchPad 
                    mode="tilt" 
                    class="w-100 h-100" 
                    :sensitivityY="sensTilt"
                     @update="handleCameraUpdate"
                >
                    <v-icon icon="mdi-angle-acute" size="48"></v-icon>
                </CameraTouchPad>
            </v-col>
        </v-row>
    </div>

    <VariantSelectorDialog
      v-model="showVariantDialog"
      :variants="variants"
      :is-dark="isDark"
      @select="onVariantSelect"
    />

    <SegmentSelectorDialog
      v-model="showSegmentDialog"
      :segments="variantSegments"
      :current-index="currentSegmentIndex"
      :is-dark="isDark"
      @select="onSegmentSelect"
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

const store = useRemoteStore()
const theme = useTheme()
const isDark = computed(() => theme.global.current.value.dark)

// --- Variant Logic ---
const isVariantMode = computed(() => {
    const rawState = store.appState;
    const appView = (typeof rawState === 'string') ? rawState : (rawState?.viewName || '');
    return appView === 'VisualizeVariant' || appView === 'VisualizeVariantView';
});
const variantSegments = computed(() => store.visualizeViewState?.segments || []); 
const currentSegmentIndex = computed(() => store.visualizeViewState?.currentSegmentIndex ?? -1);
const hasVariants = computed(() => store.visualizeViewState?.hasVariants ?? false); 
const variantCount = computed(() => store.visualizeViewState?.variantCount || 0);
const variants = computed(() => store.visualizeViewState?.variants || []);
const showVariantDialog = ref(false);
const showSegmentDialog = ref(false);

// Close dialogs when view changes (e.g. selection made on desktop)
watch(isVariantMode, (newVal) => {
    if (newVal) {
        showVariantDialog.value = false;
    } else {
        showSegmentDialog.value = false;
    }
});

function onSegmentSelect({ segment, index }) {
    store.sendCommand('jump_to_segment', { index });
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

function triggerFinalView() { store.sendCommand('trigger_final_view'); }

function triggerVariant() { 
    if (isVariantMode.value) {
        store.sendCommand('return_to_main_trace');
    } else if (variantCount.value === 1) {
        store.sendCommand('trigger_variant_selection'); 
    } else if (variantCount.value > 1) {
        showVariantDialog.value = true;
    }
}

function onVariantSelect(variantId) {
    store.sendCommand('select_variant', { variantId });
}

</script>
