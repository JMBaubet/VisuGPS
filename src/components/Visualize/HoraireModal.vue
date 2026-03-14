<template>
  <v-dialog
    v-model="model"
    max-width="1000"
    transition="dialog-bottom-transition"
    scrim="rgba(0,0,0,0.8)"
  >
    <v-card class="horaire-modal-card overflow-hidden" rounded="xl">
      <v-btn
        icon="mdi-close"
        variant="text"
        class="close-btn"
        @click="model = false"
      ></v-btn>

      <v-row no-gutters class="fill-height">
        <!-- Section Image -->
        <v-col cols="12" md="6" class="image-section d-none d-md-block">
          <v-img
            src="/cyclists.jpg"
            cover
            height="100%"
            class="fill-height"
          >
            <div class="image-overlay"></div>
          </v-img>
        </v-col>

        <!-- Section Contenu -->
        <v-col cols="12" md="6" class="content-section">
          <div class="content-wrapper pa-8 pa-md-12">
            <header class="mb-8">
              <h2 class="text-overline mb-2 d-flex align-center">
                <span class="accent-dot"></span>
                Horaires de départ
              </h2>
              <div class="divider mb-6"></div>
            </header>

            <div class="schedule-list">
              <div
                v-for="(group, index) in sortedSchedules"
                :key="index"
                class="schedule-item d-flex justify-space-between align-center px-6 py-4 mb-3"
              >
                <div class="d-flex flex-column">
                  <span 
                    class="group-name text-h6 font-weight-bold"
                    :class="{ 'variant-orange': group.isVariant }"
                  >
                    {{ group.name }}
                  </span>
                  <span v-if="group.isVariant" class="variant-subtitle text-caption">
                    {{ group.variantName }}
                  </span>
                </div>
                <span 
                  class="group-time text-h5 font-weight-black"
                  :class="{ 'variant-orange': group.isVariant }"
                >
                  {{ group.time }}
                </span>
              </div>
            </div>
          </div>
        </v-col>
      </v-row>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  modelValue: {
    type: Boolean,
    required: true
  },
  scenarios: {
    type: Array,
    default: () => []
  }
});

const emit = defineEmits(['update:modelValue']);

const model = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
});

// Helper pour parser l'heure "HHhMM" ou "HH:MM" en minutes pour le tri
const parseTimeToMinutes = (timeStr) => {
  if (!timeStr) return 0;
  // Gère 09h30, 9h30, 09:30, 9:30
  const match = timeStr.match(/(\d{1,2})[h:](\d{2})/i);
  if (match) {
    return parseInt(match[1]) * 60 + parseInt(match[2]);
  }
  return 0;
};

// Helper pour parser "Groupe X" et extraire le X pour le tri
const parseGroupNumber = (groupName) => {
  if (!groupName) return 0;
  const match = groupName.replace(/-/g, ' ').match(/(?:groupe|gr|g)\s*(\d+)/i);
  if (match) return parseInt(match[1]);
  // Si juste un nombre comme nom
  const justNum = groupName.match(/^(\d+)$/);
  if (justNum) return parseInt(justNum[1]);
  return 999; // Fallback pour les groupes non standards
};

const sortedSchedules = computed(() => {
  if (!props.scenarios || props.scenarios.length === 0) return [];
  
  // Transforme les scenarios en liste d'horaires
  const schedulesList = props.scenarios.map(s => {
    return {
      name: s.nom || 'Groupe',
      time: s.heureDepart || '00h00',
      timeMinutes: parseTimeToMinutes(s.heureDepart),
      groupNum: parseGroupNumber(s.nom),
      isVariant: !!s.variantId,
      variantName: s.variantName
    };
  });

  // Tri chronologique, puis par numéro de groupe
  return schedulesList.sort((a, b) => {
    if (a.timeMinutes !== b.timeMinutes) {
      return a.timeMinutes - b.timeMinutes; // Plus tôt en premier
    }
    return a.groupNum - b.groupNum; // Plus petit groupe en premier
  });
});
</script>

<style scoped>
.horaire-modal-card {
  background: rgb(var(--v-theme-surface));
  min-height: 500px;
  position: relative;
}

.close-btn {
  position: absolute;
  top: 10px;
  right: 10px;
  z-index: 10;
  color: rgb(var(--v-theme-on-surface));
}

.image-section {
  position: relative;
}

.image-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(90deg, transparent 50%, rgb(var(--v-theme-surface)) 100%);
}

.content-section {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.accent-dot {
  width: 10px;
  height: 10px;
  background-color: #f97316; /* Orange accent */
  border-radius: 50%;
  margin-right: 12px;
}

.divider {
  height: 2px;
  width: 60px;
  background: #f97316;
  opacity: 0.6;
}

.schedule-item {
  background: rgba(var(--v-theme-on-surface), 0.05);
  border-radius: 16px;
  border: 1px solid rgba(var(--v-theme-on-surface), 0.05);
  transition: all 0.3s ease;
}

.v-theme--light .schedule-item {
  background: rgba(var(--v-theme-on-surface), 0.03);
}

.schedule-item:hover {
  background: rgba(249, 115, 22, 0.1);
  border-color: rgba(249, 115, 22, 0.3);
  transform: translateX(8px);
}

.group-name {
  font-family: "Tauri", sans-serif;
  color: rgb(var(--v-theme-on-surface));
  opacity: 0.9;
}

.variant-orange {
  color: #f97316 !important;
}

.variant-subtitle {
  color: rgb(var(--v-theme-on-surface));
  opacity: 0.8;
  font-size: 0.75rem !important;
  margin-top: -4px;
}

/* Typographie spécifique si disponible dans le projet */
h2 {
  font-family: "Tauri", sans-serif;
  letter-spacing: 4px !important;
  color: rgb(var(--v-theme-on-surface));
  opacity: 0.9;
  font-size: 1.5rem !important; /* Environ deux fois plus grand */
  line-height: 1.2;
}
</style>
