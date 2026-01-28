<template>
  <v-container class="fill-height d-flex flex-column align-center justify-center text-center">
    
    <!-- Header Logo, Title & Theme -->
    <v-row class="w-100 flex-grow-0 mb-6 px-4" align="center" no-gutters>
        <!-- Logo Left -->
        <v-col cols="auto">
             <v-img src="/logo.png" width="120" height="120" class="mr-4"></v-img>
        </v-col>
        
        <!-- Right Column: Title + Theme Switch -->
        <v-col class="d-flex flex-column align-center justify-center">
            <div class="text-h4 font-weight-bold mb-2">VisuGPS</div>
            
            <!-- Compact Theme Switch -->
            <v-row align="center" no-gutters>
                <v-icon size="small" class="mr-2" :color="isDarkTheme ? 'blue-grey-lighten-3' : 'blue-grey-darken-3'">mdi-weather-night</v-icon>
                <v-switch
                    v-model="isDarkTheme"
                    hide-details
                    color="primary"
                    density="compact"
                    inset
                ></v-switch>
                <v-icon size="small" class="ml-2" :color="isDarkTheme ? 'yellow' : 'yellow-darken-3'">mdi-white-balance-sunny</v-icon>
            </v-row>
        </v-col>
    </v-row>

    <!-- Connection Status Card -->
    <v-card width="100%" max-width="400" variant="tonal" :color="statusColor" class="mb-6">
      <v-card-text>
        <div class="text-h6 font-weight-bold mb-1">
          {{ store.statusMessage }}
        </div>
        <div v-if="store.errorMessage" class="text-caption text-error">
          {{ store.errorMessage }}
        </div>
        
        <!-- Pairing Code Display -->
        <div v-if="store.isPairing" class="mt-4 pa-4 bg-surface rounded text-h3 font-weight-black text-primary letter-spacing-4">
          {{ store.pairingCode }}
        </div>
      </v-card-text>
    </v-card>

    <!-- Controls -->
    <div class="d-flex flex-column gap-4 w-100 mw-400">
      
      <!-- Connect / Disconnect Buttons -->
      <!-- Connect / Disconnect Buttons -->
      <v-btn 
        v-if="store.connectionStatus === 'disconnected'"
        color="primary" 
        size="x-large" 
        block
        prepend-icon="mdi-wifi"
        @click="handleConnect()"
        :loading="store.connectionStatus === 'connecting'"
      >
        Connexion
      </v-btn>

      <v-btn 
        v-else
        color="error" 
        variant="outlined"
        size="large" 
        block
        prepend-icon="mdi-power"
        @click="handleDisconnect()"
      >
        Déconnexion
      </v-btn>

      <!-- Options moved to header -->
    </div>

    <!-- Debug Toggle (Easter Egg or Small Button) -->
    <div class="mt-auto pt-8">
        <v-btn variant="text" size="small" density="compact" color="grey" @click="showDebug = !showDebug">
            {{ showDebug ? 'Masquer Debug' : 'Afficher Debug' }}
        </v-btn>
    </div>

    <!-- Debug Panel -->
    <v-dialog v-model="showDebug" fullscreen transition="dialog-bottom-transition">
        <v-card>
            <v-toolbar color="primary">
                <v-btn icon="mdi-close" @click="showDebug = false"></v-btn>
                <v-toolbar-title>Debug Panel</v-toolbar-title>
                <v-spacer></v-spacer>
            </v-toolbar>
            <v-list class="bg-black text-green-accent-3 font-monospace text-caption">
                <v-list-subheader class="text-white">État Store</v-list-subheader>
                <v-list-item min-height="20">
                    <div>Status: {{ store.connectionStatus }}</div>
                    <div>ClientId: {{ store.clientId }}</div>
                </v-list-item>
                
                <v-divider class="my-2 border-white"></v-divider>
                <v-list-subheader class="text-white">Settings Reçus</v-list-subheader>
                 <pre class="pa-4 text-wrap">{{ JSON.stringify(store.remoteSettings, null, 2) }}</pre>

                <v-divider class="my-2 border-white"></v-divider>
                 <v-list-subheader class="text-white">Logs SSE ({{ store.sseDebugLog.length }})</v-list-subheader>
                 <v-list-item v-for="(log, i) in store.sseDebugLog" :key="i" density="compact" min-height="auto">
                    <span class="text-grey">{{ log.timestamp }}</span>
                    <strong class="mx-2 text-primary">[{{ log.type }}]</strong>
                    <span class="text-wrap">{{ log.data }}</span>
                 </v-list-item>
            </v-list>
        </v-card>
    </v-dialog>

  </v-container>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { useRemoteStore } from '@/stores/remoteStore'
import { useTheme } from 'vuetify'
import NoSleep from 'nosleep.js'

const store = useRemoteStore()
const theme = useTheme()
const showDebug = ref(false)
const noSleepEnabled = ref(false)
const noSleep = new NoSleep()

// Theme Logic
const isDarkTheme = computed({
    get: () => theme.global.current.value.dark,
    set: (val) => {
        theme.global.name.value = val ? 'dark' : 'light'
    }
})

// Status Color Logic
const statusColor = computed(() => {
    switch (store.connectionStatus) {
        case 'connected': return 'success'
        case 'pairing': return 'info'
        case 'connecting': return 'warning'
        case 'refused': return 'error'
        default: return 'grey'
    }
})

// NoSleep Logic
function toggleNoSleep(val) {
    noSleepEnabled.value = val; // Update UI state
    if (val) {
        noSleep.enable().then(() => {
            console.log("NoSleep enabled")
        }).catch(err => {
            console.error("NoSleep error", err)
            noSleepEnabled.value = false // Revert if failed
        })
    } else {
        noSleep.disable()
        console.log("NoSleep disabled")
    }
}

// Watch for disconnection to disable NoSleep automatically
watch(() => store.connectionStatus, (newStatus) => {
    if (newStatus === 'disconnected' && noSleepEnabled.value) {
        toggleNoSleep(false)
    }
})

// Enhanced Connect Action (Enable NoSleep on user gesture)
function handleConnect() {
    toggleNoSleep(true); // Enable immediately on click (gesture)
    store.connect();
}

function handleDisconnect() {
    store.disconnect();
    // Watcher will handle NoSleep disable
}

// Auto-connect disabled to ensure user gesture for NoSleep
// onMounted(() => {
//     store.connect()
// })

</script>

<style scoped>
.letter-spacing-4 {
    letter-spacing: 4px;
}
.font-monospace {
    font-family: monospace;
}
.mw-400 {
    max-width: 400px;
}
.gap-4 {
    gap: 16px;
}
</style>
