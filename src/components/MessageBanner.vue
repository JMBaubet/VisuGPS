<template>
  <div class="alert-wrapper">
    <!-- Alerte principale avec transparence -->
    <v-alert
      :color="color"
      density="compact"
      class="pa-2"
      :style="{ opacity: opacity }"
    >
      <template v-slot:prepend>
        <v-icon :color="progressColor" :icon="icon" size="20"></v-icon>
      </template>
      <span class="text-body-2">{{ message }}</span>
    </v-alert>

    <!-- Progress bar en bas -->
    <v-progress-linear
      :model-value="progress"
      :color="progressColor"
      height="4"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from "vue";

const props = defineProps({
  message: { type: String, required: true },
  type: { type: String, default: "info" }, // success, info, warning, error
  duration: { type: Number, default: 5000 }, // 5s par défaut
  opacity: { type: Number, default: 0.85 }, // transparence globale (0 à 1)
});

const emit = defineEmits(["expired"]);

const progress = ref(100);
let intervalId = null;

const typeMap = {
  success: { color: "green-lighten-4", icon: "mdi-check-circle-outline", progressColor: "success" },
  info: { color: "blue-lighten-4", icon: "mdi-information-outline", progressColor: "info" },
  warning: { color: "orange-lighten-4", icon: "mdi-alert-outline", progressColor: "warning" },
  error: { color: "red-lighten-4", icon: "mdi-alert-circle-outline", progressColor: "error" },
};

const color = computed(() => typeMap[props.type]?.color || "blue-lighten-4");
const icon = computed(() => typeMap[props.type]?.icon || "mdi-information-outline");
const progressColor = computed(() => typeMap[props.type]?.progressColor || "info");

onMounted(() => {
  if (props.duration > 0) {
    const step = 100 / (props.duration / 100);
    intervalId = setInterval(() => {
      progress.value -= step;
      if (progress.value <= 0) {
        clearInterval(intervalId);
        emit("expired");
      }
    }, 100);
  }
});

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId);
});
</script>

<style scoped>
.alert-wrapper {
  display: inline-block;
  width: 100%;
  border-radius: 6px;
  overflow: hidden;
}
</style>
