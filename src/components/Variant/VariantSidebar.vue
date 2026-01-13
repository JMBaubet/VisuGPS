<template>
  <v-card class="fill-height" elevation="0" border>
    <v-card-text class="pa-0 d-flex flex-column" style="height: 100%;">
      <div class="px-2 py-1 bg-grey-lighten-4 border-b text-caption text-center text-grey-darken-1 font-weight-bold">
        Points d'édition
      </div>

      <!-- Modifications Section -->
      <div class="px-2 py-2 d-flex justify-space-between align-center">
        <div class="text-caption font-weight-bold uppercase">Points d'édition</div>
        <v-btn icon="mdi-refresh" variant="text" size="x-small" @click="$emit('reset')" title="Tout réinitialiser"></v-btn>
      </div>
      
      <div class="flex-grow-1 overflow-y-auto">
        <v-list density="compact" open-strategy="multiple" class="pa-0">
          <div v-if="modifications.length === 0" class="text-center py-8 text-grey text-caption">
            Cliquez sur la carte <br> pour ajouter des points
          </div>
          
          <v-list-group
            v-for="(mod, modIndex) in modifications"
            :key="modIndex"
            :value="modIndex"
            color="primary"
          >
            <template v-slot:activator="{ props: groupProps }">
              <v-list-item
                v-bind="groupProps"
                :prepend-icon="getModIcon(mod.type)"
                :title="getModTitle(mod, modIndex)"
                class="bg-grey-lighten-5"
                :class="{ 'border-s-4 border-primary': !mod.finalized && mod.type === activeMode }"
              >
                <template v-slot:append>
                    <v-btn 
                      v-if="!mod.finalized && (mod.type === 'DEPART' || mod.type === 'ARRIVEE')"
                      icon="mdi-check-circle-outline" 
                      size="x-small" 
                      variant="text" 
                      color="primary" 
                      class="mr-1"
                      @click.stop="$emit('finalize-mod', modIndex)"
                      title="Terminer cette modification"
                    ></v-btn>
                    <v-btn icon="mdi-delete" size="x-small" variant="text" color="error" @click.stop="$emit('delete-mod', modIndex)"></v-btn>
                </template>
              </v-list-item>
            </template>

            <v-list-item
              v-for="(point, pIndex) in mod.points"
              :key="pIndex"
              :prepend-icon="point.type === 'ANCHOR' ? 'mdi-anchor' : 'mdi-map-marker'"
              :title="getPointTitle(point, pIndex)"
              :subtitle="formatCoords(point.coords)"
              class="pl-8"
              :disabled="mod.finalized"
            >
              <template v-slot:append>
                <v-btn
                  v-if="!mod.finalized"
                  icon="mdi-delete"
                  size="x-small"
                  variant="text"
                  color="grey"
                  @click.stop="$emit('delete-point', modIndex, pIndex)"
                ></v-btn>
              </template>
            </v-list-item>
          </v-list-group>
        </v-list>

        <!-- Save Button moved here -->
        <div class="pa-4" v-if="modifications.length > 0">
          <v-btn
            block
            variant="flat"
            color="success"
            :disabled="!isValid"
            @click="$emit('save')"
          >
            <v-icon start>mdi-content-save</v-icon>
            Enregistrer Variante
          </v-btn>
        </div>
      </div>

      <div class="pa-2 border-t bg-white">
        <!-- New Saved Variants Section -->
        <div class="text-caption font-weight-bold mb-1 uppercase text-grey-darken-1">Variantes enregistrées</div>
        <v-list density="compact" class="pa-0 mb-4 bg-grey-lighten-4 rounded" style="max-height: 150px; overflow-y: auto;">
            <v-list-item v-if="savedVariants.length === 0" class="text-caption text-center py-2 text-grey">
                Aucune variante
            </v-list-item>
            <v-list-item
                v-for="v in savedVariants"
                :key="v.id"
                :title="v.name"
                :subtitle="new Date(v.creationDate).toLocaleDateString()"
                lines="one"
                class="border-b last-child-border-0"
            >
                <template v-slot:append>
                    <v-btn icon="mdi-delete" size="x-small" variant="text" color="error" @click.stop="$emit('delete-saved-variant', v.id)"></v-btn>
                </template>
            </v-list-item>
        </v-list>

        <v-btn
          block
          variant="tonal"
          color="primary"
          :disabled="!canGeneratePreview"
          @click="$emit('generate')"
        >
          <v-icon start>mdi-play</v-icon>
          Prévisualiser tout
        </v-btn>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
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
  isValid: Boolean
});

const emit = defineEmits(['update:config', 'generate', 'save', 'delete-point', 'delete-mod', 'finalize-mod', 'delete-saved-variant', 'reset']);

const routingProfiles = ['bike', 'mtb', 'racingbike', 'car', 'foot'];

// Profiles are now managed in the Toolbar, we keep this as a proxy if needed but most logic moved
const proxyRoutingProfile = computed({
  get: () => props.config.routingProfile,
  set: (val) => emit('update:config', { ...props.config, routingProfile: val })
});

const getModIcon = (type) => {
    if (type === 'DEPART') return 'mdi-ray-start-arrow';
    if (type === 'ARRIVEE') return 'mdi-ray-end-arrow';
    return 'mdi-vector-polyline';
};

const getModTitle = (mod, index) => {
    if (mod.type === 'DEPART') return 'Départ';
    if (mod.type === 'ARRIVEE') return 'Arrivée';
    return `Segment ${index + 1}`;
};

const getPointTitle = (point, pIndex) => {
    if (point.type === 'ANCHOR') return 'ANCRAGE (Trace)';
    return `Point via ${pIndex}`;
};

const formatCoords = (coords) => {
    return `${coords[1].toFixed(4)}, ${coords[0].toFixed(4)}`;
};
</script>
