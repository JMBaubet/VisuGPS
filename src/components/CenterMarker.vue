<template>
  <div class="center-marker-container">
    <v-icon 
      size="50"
      color="white"
      style="opacity: 0.5;"
    >mdi-target</v-icon>
  </div>
</template>

<style scoped>
.center-marker-container {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 1001; /* Above map, below top-level UI if needed */
  pointer-events: none; /* Allows map interaction underneath */
}
</style>
