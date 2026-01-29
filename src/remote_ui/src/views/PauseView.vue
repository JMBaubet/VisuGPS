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
                @click="triggerFinalView()"
                :disabled="isFinished"
            >
                <v-icon icon="mdi-skip-forward" size="64"></v-icon>
            </v-btn>
        </template>
    </PlaybackControls>

    <!-- 3. Variant Segments (Phase 6) -->
    <VariantSegmentList 
        v-if="isVariantMode" 
        :segments="variantSegments" 
        @select="onSegmentSelect"
    />
    
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
                    size="60" 
                    rounded="circle"
                    :color="isDark ? 'white' : 'grey-darken-3'" 
                    variant="text" 
                    @click="triggerVariant()"
                    :disabled="!hasVariants"
                >
                    <v-icon icon="mdi-source-branch" size="40"></v-icon>
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

  </v-container>
</template>

<script setup>
import { ref, computed } from 'vue' // removed watch
import { useTheme } from 'vuetify'
import { useRemoteStore } from '@/stores/remoteStore'
import CameraTouchPad from '@/components/CameraTouchPad.vue'
import VariantSegmentList from '@/components/VariantSegmentList.vue'
import PlaybackControls from '@/components/PlaybackControls.vue'

const store = useRemoteStore()
const theme = useTheme()
const isDark = computed(() => theme.global.current.value.dark)

// --- Variant Logic ---
const isVariantMode = computed(() => store.appState?.viewName === 'VisualizeVariantView');
const variantSegments = computed(() => store.visualizeViewState?.segments || []); 
const hasVariants = computed(() => store.visualizeViewState?.hasVariants ?? false); 

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
function triggerVariant() { store.sendCommand('trigger_variant_selection'); }

</script>
