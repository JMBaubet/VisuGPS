<template>
  <v-app>
    <v-main>
      <component :is="activeComponent" />
    </v-main>
  </v-app>
</template>

<script setup>
import { computed, provide, ref, watch } from 'vue'
import { useRemoteStore } from '@/stores/remoteStore'
import AccueilView from '@/views/AccueilView.vue'
import AnimationView from '@/views/AnimationView.vue'
import PauseView from '@/views/PauseView.vue'
import NoSleep from 'nosleep.js'

const store = useRemoteStore()
const noSleep = new NoSleep()
const noSleepEnabled = ref(false)

// --- Global NoSleep Logic ---
function enableNoSleep() {
    noSleep.enable().then(() => {
        console.log("Global NoSleep enabled")
        noSleepEnabled.value = true
    }).catch(err => {
        console.error("Global NoSleep error", err)
        noSleepEnabled.value = false
    })
}

function disableNoSleep() {
    noSleep.disable()
    console.log("Global NoSleep disabled")
    noSleepEnabled.value = false
}

// Watch global connection status to auto-disable
watch(() => store.connectionStatus, (newStatus) => {
    // If disconnected AND we think it's enabled, turn it off
    if (newStatus === 'disconnected' && noSleepEnabled.value) {
        disableNoSleep()
    }
})

// Provide to children (specifically AccueilView for the Click Trigger)
provide('enableNoSleep', enableNoSleep)

// ... existing component logic ...
const activeComponent = computed(() => {
    // Logic from requirements:
    // "Quand les vues VisalizeView.vue et VisualzeVariantView.vue seront sélectionnées sur l'application de bureau, 
    // on affichera AnimationView ou PauseView en fonction du status de isPaused des vues Visualize*View du bureau.
    // Sinon on affichera la vue AccueilView."
    
    // appState.viewName usually holds 'MainView', 'VisualizeView', 'VisualizeVariantView' etc.
    // visualizeViewState holds the specific state of the visualization (isPaused, etc.)
    
    // Robust check: appState might be { viewName: '...' } or just '...'
    const rawState = store.appState;
    const appView = (typeof rawState === 'string') ? rawState : (rawState?.viewName || '');

    // Backend sends 'Visualize' or 'VisualizeVariant' (based on doc/observation)
    // We also keep the 'View' suffix check just in case legacy or future changes use it.
    const isVisualizing = ['Visualize', 'VisualizeVariant', 'VisualizeView', 'VisualizeVariantView'].some(v => appView.includes(v));
    
    // Also check if we are actually connected/paired. If not, show AccueilView (which has connection controls)
    if (!store.isConnected) return AccueilView;

    if (isVisualizing) {
        // Check pause state
        // visualizeViewState might be null initially
        const isPaused = store.visualizeViewState?.animationState === 'En_Pause' || store.visualizeViewState?.animationState === 'Termine'; 
        // Note: 'Termine' might show PauseView too or special end state? Req says "AnimationView or PauseView".
        // Let's assume 'En_Animation' -> AnimationView, others -> PauseView?
        // Wait, req says: "AnimationView ou PauseView en fonction du status de isPaused"
        // Let's use the explicit 'animationState' from our store logic
        
        const animState = store.visualizeViewState?.animationState;
        
        if (animState === 'En_Animation') {
            return AnimationView;
        } else {
            // 'En_Pause', 'Termine', or undefined (default to Pause view for safety/control?)
            return PauseView;
        }
    }
    
    return AccueilView;
})
</script>
