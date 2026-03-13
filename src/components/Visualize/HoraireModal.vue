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
                v-for="(group, index) in schedules"
                :key="index"
                class="schedule-item d-flex justify-space-between align-center px-6 py-4 mb-3"
              >
                <span class="group-name text-h6 font-weight-bold">{{ group.name }}</span>
                <span class="group-time text-h5 font-weight-black">{{ group.time }}</span>
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
  }
});

const emit = defineEmits(['update:modelValue']);

const model = computed({
  get: () => props.modelValue,
  set: (value) => emit('update:modelValue', value)
});

const schedules = [
  { name: 'Groupe 1', time: '09h30' },
  { name: 'Groupe 2', time: '09h30' },
  { name: 'Groupe 3', time: '09h10' },
  { name: 'Groupe 1', time: '09h30' },
  { name: 'Groupe 2', time: '09h30' },
  { name: 'Groupe 3', time: '09h10' }, 
  { name: 'Groupe 3', time: '09h10' }
];
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

.group-time {
  font-family: "Tauri", sans-serif;
  color: rgb(var(--v-theme-on-surface));
  letter-spacing: 1px;
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
