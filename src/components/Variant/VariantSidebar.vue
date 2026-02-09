<template>
  <v-card class="fill-height" elevation="0" border>
    <v-card-text class="pa-0 d-flex flex-column" style="height: 100%;">

      <!-- Modifications Section -->
      <div class="px-2 py-2 d-flex justify-space-between align-center">
        <div class="text-caption font-weight-bold uppercase text-truncate" style="max-width: 250px;">
          {{ isEditing ? `Modification de ${variantName}` : "Points d'édition" }}
        </div>
        <div class="d-flex align-center">
          <v-btn 
            v-if="isEditing"
            icon="mdi-close" 
            variant="text" 
            size="x-small" 
            color="grey-darken-1" 
            class="mr-1"
            @click="$emit('reset')" 
            title="Fermer l'édition"
          ></v-btn>
          <v-btn 
            v-if="!isEditing"
            icon="mdi-delete" 
            variant="text" 
            size="x-small" 
            color="error" 
            @click="$emit('reset')" 
            title="Tout réinitialiser"
          ></v-btn>
        </div>
      </div>
      
      <div class="flex-grow-1 overflow-y-auto">
        <v-list density="compact" v-model:opened="openGroups" class="pa-0">
          <div v-if="modifications.length === 0" class="text-center py-8 text-grey text-caption">
            {{ isEditing ? 'Chargement...' : 'Cliquez sur la carte pour ajouter des points' }}
          </div>
          
          <v-list-group
            v-for="mod in sortedModifications"
            :key="mod.originalIndex"
            :value="mod.originalIndex"
            color="primary"
          >
            <!-- ... list content remains same ... -->
            <template v-slot:activator="{ props: groupProps }">
              <v-list-item
                v-bind="groupProps"
                :class="{ 'border-s-4 border-primary': !mod.finalized && mod.type === activeMode }"
              >
                <template v-slot:title>
                  <span :title="mod.routingService || 'Service par défaut'">{{ getModTitle(mod, mod.originalIndex) }}</span>
                </template>
                <template v-slot:subtitle>
                  <span class="text-caption text-grey-darken-1">{{ getModSubtitle(mod) }}</span>
                </template>
                <template v-slot:prepend>
                  <v-icon 
                    :color="getModIconColor(mod.type)" 
                    class="mr-2"
                    style="cursor: pointer;"
                    @click.stop="$emit('fly-to-mod', mod.originalIndex)"
                  >
                    {{ getModIcon(mod.type) }}
                  </v-icon>
                </template>
                <template v-slot:append>
                    <v-btn 
                      v-if="!mod.finalized && (mod.type === 'DEPART' || mod.type === 'ARRIVEE')"
                      icon="mdi-check-circle-outline" 
                      size="x-small" 
                      variant="text" 
                      color="primary" 
                      class="mr-1"
                      @click.stop="$emit('finalize-mod', mod.originalIndex)"
                      title="Terminer cette modification"
                    ></v-btn>
                    <v-btn 
                      v-if="mod.type === 'SEGMENT'"
                      icon="mdi-pencil" 
                      size="x-small" 
                      variant="text" 
                      color="primary" 
                      class="mr-1"
                      @click.stop.prevent="$emit('rename-mod', mod.originalIndex)"
                      title="Renommer ce segment"
                    ></v-btn>
                    <v-btn
                      :icon="getRoutingProfileIcon(mod.routingProfile)"
                      size="x-small"
                      variant="text"
                      :color="getModColor(mod)"
                      class="mr-1"
                      :disabled="mod.routingStatus === 'SUCCESS' && mod.routingProfile === config.routingProfile && mod.routingService === config.routingService && mod.finalized"
                      @click.stop="$emit('update-routing', mod.originalIndex)"
                      :title="`Appliquer le routage actuel (${mod.routingProfile || 'défaut'})`"
                    ></v-btn>
                    <v-btn icon="mdi-delete" size="x-small" variant="text" color="error" @click.stop="$emit('delete-mod', mod.originalIndex)"></v-btn>
                    <v-icon size="small" color="grey">{{ groupProps.appendIcon }}</v-icon>
                </template>
              </v-list-item>
            </template>

            <v-list-item
              v-for="(point, pIndex) in mod.points"
              :key="pIndex"
              :prepend-icon="point.type === 'ANCHOR' ? 'mdi-anchor' : 'mdi-map-marker'"
              :title="getPointTitle(point, pIndex)"
              :subtitle="getPointSubtitle(point)"
              class="pl-8"
            >
              <template v-slot:append>
                <v-btn
                  v-if="point.type !== 'ANCHOR' || (mod.type === 'SEGMENT' && pIndex > 0)"
                  icon="mdi-delete"
                  size="x-small"
                  variant="text"
                  color="grey"
                  @click.stop="$emit('delete-point', mod.originalIndex, pIndex)"
                  title="Supprimer ce point"
                ></v-btn>
              </template>
            </v-list-item>
          </v-list-group>
        </v-list>

        <!-- Stats Section -->
        <div class="pa-4" v-if="modifications.length > 0">
          <v-alert
            v-if="hasUnfinalizedStartOrEnd"
            density="compact"
            type="error"
            variant="tonal"
            class="mb-2 text-caption"
            icon="mdi-alert"
          >
            Veuillez valider le segment en cours d'édition pour l'enregistrer.
          </v-alert>

          <div v-if="projectedStats.total > 0" class="text-center text-caption font-weight-bold text-primary mb-1">
             <v-icon size="small" start>mdi-social-distance</v-icon>
             Distance totale : {{ projectedStats.total.toFixed(2) }} km
          </div>
        </div>
      </div>

      <div class="pa-2 border-t bg-surface">
        <!-- New Saved Variants Section -->
        <div class="text-caption font-weight-bold mb-1 uppercase text-disabled">
            <span v-if="savedVariants.length === 0">Aucune variante enregistrée</span>
            <span v-else>
                Variante{{ savedVariants.length > 1 ? 's' : '' }} enregistrée{{ savedVariants.length > 1 ? 's' : '' }} : 
                <span :class="savedVariants.length > 3 ? 'text-error font-weight-black' : ''">{{ savedVariants.length }}</span>
            </span>
        </div>
        <v-list density="compact" class="pa-0 mb-4 rounded border" style="max-height: 150px; overflow-y: auto;">
            <v-list-item v-if="savedVariants.length === 0" class="text-caption text-center py-2 text-grey">
                Aucune variante
            </v-list-item>
            <v-list-item
                v-for="v in savedVariants"
                :key="v.id"
                :title="v.name"
                :subtitle="`${v.stats?.totalDistance?.toFixed(1) || '0.0'} km; d+ ${v.stats?.totalAscent?.toFixed(0) || '0'}m`"
                lines="one"
                class="border-b last-child-border-0"
            >
                <template v-slot:prepend>
                    <v-btn
                      icon="mdi-information-outline"
                      size="x-small"
                      variant="text"
                      color="info"
                      @click.stop="openInfoDialog(v)"
                      title="Détails de la variante"
                    ></v-btn>
                </template>
                <template v-slot:append>
                    <v-btn
                      icon="mdi-eye"
                      size="x-small"
                      variant="text"
                      :color="getVariantEyeColor(v)"
                      class="mr-1"
                      :disabled="isValid && (!isEditing || isModified)"
                      @click.stop="$emit('load-variant', v.id)"
                      title="Charger cette variante pour édition"
                    ></v-btn>
                     <v-btn
                      icon="mdi-pencil"
                      size="x-small"
                      variant="text"
                      color="primary"
                      class="mr-1"
                      @click.stop.prevent="$emit('rename-saved-variant', v.id, v.name)"
                      title="Renommer cette variante"
                    ></v-btn>
                    <v-btn icon="mdi-delete" size="x-small" variant="text" color="error" @click.stop="$emit('delete-saved-variant', v.id, v.name)"></v-btn>
                </template>
            </v-list-item>
        </v-list>
      </div>
    </v-card-text>
  </v-card>

  <!-- Info Dialog for Variant Stats -->
  <v-dialog v-model="infoDialog" max-width="400">
    <v-card v-if="selectedVariant">
      <v-card-title class="bg-blue-darken-3 text-white d-flex align-center">
        <v-icon start>mdi-information</v-icon>
        Détails de la variante
      </v-card-title>
      <v-card-text class="pa-4">
        <div v-if="selectedVariant" class="mb-4">
          <div class="d-grid" style="grid-template-columns: 1fr auto; gap: 16px; align-items: baseline;">
            <!-- Ligne Circuit (Style léger) -->
            <div class="text-caption text-grey-darken-1 text-truncate" :title="circuitName">
              {{ circuitName }} :
            </div>
            <div class="text-caption text-grey-darken-1 text-right">
              Distance : {{ selectedVariant.stats?.masterDistance?.toFixed(1) || '0.0' }} km, 
              d+ : {{ selectedVariant.stats?.masterAscent?.toFixed(0) || '0' }} m
            </div>
            
            <!-- Ligne Variante (Style gras) -->
            <div class="text-body-2 font-weight-bold text-truncate" :title="selectedVariant.name">
              {{ selectedVariant.name }} :
            </div>
            <div class="text-body-2 font-weight-bold text-right">
              Distance : {{ selectedVariant.stats?.totalDistance?.toFixed(1) || '0.0' }} km, 
              d+ : {{ selectedVariant.stats?.totalAscent?.toFixed(0) || '0' }} m
            </div>
          </div>
        </div>
        
        <v-divider class="mb-4"></v-divider>
        
        <div v-if="selectedVariant" class="d-flex justify-space-between text-caption text-grey">
            <span>Créée le :</span>
            <span>{{ new Date(selectedVariant.creationDate).toLocaleDateString() }} à {{ new Date(selectedVariant.creationDate).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) }}</span>
        </div>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="primary" variant="text" @click="infoDialog = false">Fermer</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, computed, watch } from 'vue';

const infoDialog = ref(false);
const selectedVariant = ref(null);

const openInfoDialog = (variant) => {
    selectedVariant.value = variant;
    infoDialog.value = true;
};

const getDeltaColor = (delta, lowerIsBetter = true) => {
    if (!delta || Math.abs(delta) < 0.01) return 'text-grey';
    if (lowerIsBetter) {
        return delta < 0 ? 'text-success' : 'text-error';
    }
    return delta > 0 ? 'text-success' : 'text-error';
};

const formatDelta = (delta, decimals = 1) => {
    if (!delta || Math.abs(delta) < 0.01) return '--';
    const sign = delta > 0 ? '+' : '';
    return `${sign}${delta.toFixed(decimals)}`;
};

const props = defineProps({
  circuitName: String,
  activeMode: String,
  config: Object,
  modifications: {
    type: Array,
    default: () => []
  },
  savedVariants: {
    type: Array,
    default: () => []
  },
  canGeneratePreview: Boolean,
  isValid: Boolean,
  isEditing: Boolean,
  isModified: Boolean,
  variantName: String,
  trackingPoints: {
    type: Array,
    default: () => []
  },
  segmentLength: {
    type: Number,
    default: 100
  },
  projectedStats: {
    type: Object,
    default: () => ({ total: 0, current: 0 })
  }
});

const emit = defineEmits(['update:config', 'generate', 'save', 'delete-point', 'delete-mod', 'finalize-mod', 'rename-mod', 'update-routing', 'fly-to-mod', 'load-variant', 'delete-saved-variant', 'rename-saved-variant', 'reset']);

const openGroups = ref([]);

// Auto-expand new modifications
watch(() => props.modifications.length, (newLength, oldLength) => {
    if (newLength > oldLength) {
        // New item added, find its index and open it
        // We open ALL to be sure, or just the new one.
        // Actually, sortedModifications uses originalIndex as value.
        // Let's just ensure all indices are in openGroups if the user wants "visualize all points by default"
        const allIndices = props.modifications.map((_, i) => i);
        // Merge with existing open groups to not close what user might have closed?
        // User asked "par defaut on puisse visualiser tous les points".
        // Let's force open all on change, or just add new ones.
        // Safer: add new ones.
        // But since we can't easily know WHICH one is new without diffing, let's just Open All on load/change if simplistic.
        // Better: just add all current indices to openGroups.
        openGroups.value = [...new Set([...openGroups.value, ...allIndices])];
    }
}, { immediate: true });

// Also watch when entering editing mode to expand existing mods
watch(() => props.isEditing, (newVal) => {
    if (newVal) {
         const allIndices = props.modifications.map((_, i) => i);
         openGroups.value = [...new Set([...openGroups.value, ...allIndices])];
    }
});

const routingProfiles = ['bike', 'mtb', 'racingbike', 'car', 'foot'];

// Profiles are now managed in the Toolbar, we keep this as a proxy if needed but most logic moved
const proxyRoutingProfile = computed({
  get: () => props.config.routingProfile,
  set: (val) => emit('update:config', { ...props.config, routingProfile: val })
});

const hasUnfinalizedStartOrEnd = computed(() => {
  return props.modifications.some(m => 
    (m.type === 'DEPART' || m.type === 'ARRIVEE') && !m.finalized
  );
});

const sortedModifications = computed(() => {
  return [...props.modifications]
    .map((mod, index) => ({ ...mod, originalIndex: index }))
    .sort((a, b) => {
      const order = { 'DEPART': 0, 'SEGMENT': 1, 'ARRIVEE': 2 };
      return order[a.type] - order[b.type];
    });
});

const getModIcon = (type) => {
    if (type === 'DEPART') return 'mdi-ray-start-arrow';
    if (type === 'ARRIVEE') return 'mdi-ray-end-arrow';
    return 'mdi-source-branch';
};

const getModIconColor = (type) => {
    if (type === 'DEPART') return 'success';
    if (type === 'ARRIVEE') return 'error';
    return 'primary';
};

const getModColor = (mod) => {
    // 1. Priorité aux statuts d'erreur ou d'alerte
    if (mod.routingStatus === 'ROUTE_FAIL') return 'error'; // Rouge
    if (mod.routingStatus === 'ALT_FAIL') return 'warning';   // Orange

    // 2. Si le segment n'est pas synchronisé avec les paramètres globaux (et n'a pas d'erreur critique)
    if (mod.routingProfile !== props.config.routingProfile || mod.routingService !== props.config.routingService) {
         return 'primary'; // Bleu: Needs refresh/Sync
    }

    // 3. Succès et synchronisé
    if (mod.routingStatus === 'SUCCESS') return 'success';   // Vert
    
    return 'primary'; // Bleu (par défaut)
};

const getVariantEyeColor = (variant) => {
    const status = variant.globalStatus;
    if (status === 'ROUTE_FAIL') return 'error';
    if (status === 'ALT_FAIL') return 'warning';
    if (status === 'SUCCESS') return 'success';
    return 'primary';
};

const getModTitle = (mod, index) => {
    if (mod.name) return mod.name;
    if (mod.type === 'DEPART') return 'Départ';
    if (mod.type === 'ARRIVEE') return 'Arrivée';
    return `Segment ${index + 1}`;
};

const getModSubtitle = (mod) => {
    if (mod.type === 'SEGMENT' && mod.length) {
        const lengthMeters = mod.length * 1000;
        const modulo = lengthMeters % 100;
        return `Modulo : ${modulo.toFixed(0)}m`;
    }
    return '';
};

const getPointTitle = (point, pIndex) => {
    if (point.type === 'ANCHOR') return 'ANCRAGE (Trace)';
    return `Point via ${pIndex}`;
};

const getPointSubtitle = (point) => {
    // New points use 'index', loaded/saved points use 'index_on_master'
    const masterIdx = point.index ?? point.index_on_master;

    if (point.type === 'ANCHOR' && masterIdx !== undefined) {
        // Calculate distance from master increment: dist = (idx * segmentLength) / 1000
        const distKm = (Number(masterIdx) * props.segmentLength) / 1000.0;
        return `Position : ${distKm.toFixed(1)} km`;
    }
    return ''; 
};

const getRoutingProfileIcon = (profile) => {
    const map = {
        'bike': 'mdi-image-filter-hdr',     // VTT / Chemin
        'mtb': 'mdi-terrain',
        'racingbike': 'mdi-bike-fast',      // Route uniquement
        'car': 'mdi-bike',                  // Route + Pistes
        'foot': 'mdi-walk'
    };
    return map[profile] || 'mdi-help-circle-outline';
};

const getServiceIcon = (service) => {
    if (service === 'GraphHopper') return 'mdi-VectorLine'; // Or something similar
    if (service === 'OpenRouteService') return 'mdi-map-marker-path';
    return 'mdi-server';
};
</script>
