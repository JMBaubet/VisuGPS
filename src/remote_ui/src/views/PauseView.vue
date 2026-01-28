<template>
  <v-container class="fill-height d-flex flex-column" style="max-width: 600px;">
    
    <!-- 1. Playback Controls (Duplicated from AnimationView for consistency) -->
    <v-row class="w-100 flex-grow-0 mb-2" justify="space-between" align="center">
      <v-col cols="4" class="text-center">
        <v-btn 
            icon="mdi-rewind" 
            size="large" 
            color="secondary" 
            variant="tonal" 
            @mousedown="startRewind" 
            @mouseup="stopRewind"
            @mouseleave="stopRewind"
            @touchstart.prevent="startRewind" 
            @touchend.prevent="stopRewind"
        ></v-btn>
      </v-col>
      <v-col cols="4" class="text-center">
        <v-btn 
            :icon="isPlaying ? 'mdi-pause' : 'mdi-play'" 
            size="64" 
            color="primary" 
            elevation="4"
            @click="togglePlay()"
        ></v-btn>
      </v-col>
      <v-col cols="4" class="text-center">
        <v-btn icon="mdi-refresh" size="large" color="secondary" variant="tonal" @click="restart()"></v-btn>
      </v-col>
    </v-row>

    <!-- Speed (Compact row) -->
    <v-row dense class="w-100 flex-grow-0 mb-4 align-center">
        <v-col cols="2">
             <v-btn size="x-small" variant="text" color="primary" @click="resetSpeed()">x1</v-btn>
        </v-col>
        <v-col cols="10">
            <v-slider
                v-model="speedModel"
                :min="0"
                :max="100"
                :step="1"
                hide-details
                density="compact"
                color="primary"
                thumb-label
                @update:model-value="onSpeedChange"
            ></v-slider>
        </v-col>
    </v-row>

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
                <v-icon icon="mdi-cursor-move" class="mr-2"></v-icon> Point de Vue (Pan)
            </CameraTouchPad>
        </div>

        <!-- Strip Controls -->
        <v-row dense style="height: 100px;" class="flex-grow-0">
            <v-col cols="3">
                <CameraTouchPad 
                    mode="zoom" 
                    class="w-100 h-100" 
                    :sensitivityY="sensZoom"
                    @update="handleCameraUpdate"
                >
                    <div class="d-flex flex-column align-center">
                        <v-icon icon="mdi-magnify-plus-outline"></v-icon>
                        <span>Zoom</span>
                    </div>
                </CameraTouchPad>
            </v-col>
            <v-col cols="6">
                <CameraTouchPad 
                    mode="bearing" 
                    class="w-100 h-100" 
                    :sensitivityX="sensBearing"
                     @update="handleCameraUpdate"
                >
                    <div class="d-flex flex-column align-center">
                        <v-icon icon="mdi-compass-outline"></v-icon>
                        <span>Cap (Rotation)</span>
                    </div>
                </CameraTouchPad>
            </v-col>
            <v-col cols="3">
                <CameraTouchPad 
                    mode="tilt" 
                    class="w-100 h-100" 
                    :sensitivityY="sensTilt"
                     @update="handleCameraUpdate"
                >
                    <div class="d-flex flex-column align-center">
                        <v-icon icon="mdi-angle-acute"></v-icon>
                        <span>Pitch</span>
                    </div>
                </CameraTouchPad>
            </v-col>
        </v-row>
    </div>

  </v-container>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useRemoteStore } from '@/stores/remoteStore'
import CameraTouchPad from '@/components/CameraTouchPad.vue'
import VariantSegmentList from '@/components/VariantSegmentList.vue'

const store = useRemoteStore()

// --- Variant Logic ---
const isVariantMode = computed(() => store.appState?.viewName === 'VisualizeVariantView');
const variantSegments = computed(() => store.visualizeViewState?.segments || []); 

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


// --- Playback Logic (Copy from AnimationView) ---
const isPlaying = computed(() => store.visualizeViewState?.animationState === 'En_Animation');
const speedModel = ref(20) 

watch(() => store.visualizeViewState?.currentSpeed, (newSpeed) => {
    if (newSpeed !== undefined) {
        const val = (newSpeed - 0.5) / 0.05;
        if (Math.abs(speedModel.value - val) > 1) speedModel.value = val;
    }
}, { immediate: true })

function onSpeedChange(val) {
    store.sendCommand('update_speed', { speed: 0.5 + (val * 0.05) });
}
function resetSpeed() {
    speedModel.value = 10; onSpeedChange(10);
}
function togglePlay() { store.sendCommand('toggle_play'); }
function restart() { store.sendCommand('restart_animation'); }

const isRewinding = ref(false)
function startRewind() { isRewinding.value = true; store.sendCommand('start_rewind'); }
function stopRewind() { if (isRewinding.value) { isRewinding.value = false; store.sendCommand('stop_rewind'); } }

</script>
