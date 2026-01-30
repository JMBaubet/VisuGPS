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
// --- Global Wake Lock / NoSleep Logic ---
const wakeLock = ref(null);
const noSleep = new NoSleep();
const usedNoSleepFallback = ref(false);

async function enableNoSleep() {
    // 1. Priority: Native Screen Wake Lock (No widget on iOS)
    // ONLY works on secure contexts (HTTPS) or localhost
    if ('wakeLock' in navigator) {
        try {
            wakeLock.value = await navigator.wakeLock.request('screen');
            console.log("[WakeLock] Native lock acquired");
            usedNoSleepFallback.value = false;
            
            wakeLock.value.addEventListener('release', () => {
                console.log("[WakeLock] Native lock released");
                wakeLock.value = null;
            });
            return;
        } catch (err) {
            console.warn("[WakeLock] Native lock failed (likely non-secure context):", err.message);
        }
    }

    // 2. Fallback: NoSleep.js (Video hack, works on HTTP)
    try {
        await noSleep.enable();
        usedNoSleepFallback.value = true;
        console.log("[WakeLock] NoSleep.js enabled as fallback");

        // Customize iOS Media Widget
        if ('mediaSession' in navigator) {
            navigator.mediaSession.metadata = new MediaMetadata({
                title: 'VisuGPS',
                artist: 'Maintien de l\'écran actif',
                album: 'Télécommande',
                artwork: [
                    { src: '/logo.png', sizes: '96x96', type: 'image/png' },
                    { src: '/logo.png', sizes: '128x128', type: 'image/png' },
                    { src: '/logo.png', sizes: '192x192', type: 'image/png' },
                    { src: '/logo.png', sizes: '256x256', type: 'image/png' },
                    { src: '/logo.png', sizes: '384x384', type: 'image/png' },
                    { src: '/logo.png', sizes: '512x512', type: 'image/png' },
                ]
            });
        }
    } catch (err) {
        console.error("[WakeLock] Fallback failed:", err);
    }
}

function disableNoSleep() {
    if (wakeLock.value) {
        wakeLock.value.release().then(() => {
            wakeLock.value = null;
            console.log("[WakeLock] Native lock released manually");
        });
    }
    if (usedNoSleepFallback.value) {
        noSleep.disable();
        usedNoSleepFallback.value = false;
        console.log("[WakeLock] NoSleep.js disabled");
    }
}

// Re-acquire wake lock when tab becomes visible again
document.addEventListener('visibilitychange', async () => {
    if (document.visibilityState === 'visible' && store.isConnected) {
        // Only re-acquire if we were supposed to have it
        await enableNoSleep();
    }
});

// Watch global connection status to auto-disable
watch(() => store.connectionStatus, (newStatus) => {
    if (newStatus === 'connected') {
        // ensure we attempt on connection (though gesture is needed)
    } else if (newStatus === 'disconnected') {
        disableNoSleep();
    }
});

// Provide to children
provide('enableNoSleep', enableNoSleep);

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
