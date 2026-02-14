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
// ... existing component logic ...
const activeComponent = computed(() => {
    if (!store.isConnected) return AccueilView;

    const rawState = store.appState;
    const appView = (typeof rawState === 'string') ? rawState : (rawState?.viewName || '');
    
    // Check if we are in any visualization view
    const visualizationViews = ['Visualize', 'VisualizeVariant'];
    const isVisualizing = visualizationViews.some(v => appView === v);

    if (isVisualizing) {
        const animState = store.visualizeViewState?.animationState;
        // AnimationView if playing, PauseView for everything else (Pause, Termine, etc.)
        return (animState === 'En_Animation') ? AnimationView : PauseView;
    }
    
    return AccueilView;
})
</script>
