<template>
  <div class="center-marker">
    <div class="marker-line horizontal" :style="{ backgroundColor: color }"></div>
    <div class="marker-line vertical" :style="{ backgroundColor: color }"></div>
  </div>
</template>

<script setup>
defineProps({
  color: {
    type: String,
    default: 'white',
  },
});
</script>

<style scoped>
.center-marker {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 20px;
  height: 20px;
  pointer-events: none; /* Important pour ne pas intercepter les clics de la carte */
  z-index: 10; /* S'assurer qu'il est au-dessus de la carte */
}

.marker-line {
  position: absolute;
  background-color: white;
}

.horizontal {
  top: 50%;
  left: 0;
  width: 100%;
  height: 2px;
  transform: translateY(-50%);
}

.vertical {
  top: 0;
  left: 50%;
  width: 2px;
  height: 100%;
  transform: translateX(-50%);
}
</style>