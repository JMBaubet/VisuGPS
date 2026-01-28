<template>
  <div 
    class="touch-pad d-flex align-center justify-center border rounded bg-surface-variant elevation-2"
    :class="{ 'active': active }"
    @mousedown.prevent="start"
    @mousemove.prevent="move"
    @mouseup.prevent="end"
    @mouseleave.prevent="end"
    @touchstart.prevent="start"
    @touchmove.prevent="move"
    @touchend.prevent="end"
  >
    <div class="text-caption font-weight-bold text-uppercase pointer-events-none user-select-none">
        <slot></slot>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
    mode: { type: String, required: true }, // 'pan', 'zoom', 'tilt', 'bearing'
    sensitivityX: { type: Number, default: 1 },
    sensitivityY: { type: Number, default: 1 }
})

const emit = defineEmits(['update'])

const active = ref(false)
const lastX = ref(0)
const lastY = ref(0)

function start(e) {
    active.value = true
    const { clientX, clientY } = getPoint(e)
    lastX.value = clientX
    lastY.value = clientY
}

function move(e) {
    if (!active.value) return
    const { clientX, clientY } = getPoint(e)
    
    const dx = (clientX - lastX.value) * props.sensitivityX
    const dy = (clientY - lastY.value) * props.sensitivityY
    
    lastX.value = clientX
    lastY.value = clientY
    
    if (dx !== 0 || dy !== 0) {
        emit('update', { type: props.mode, dx, dy })
    }
}

function end() {
    active.value = false
}

function getPoint(e) {
    if (e.touches && e.touches.length > 0) {
        return { clientX: e.touches[0].clientX, clientY: e.touches[0].clientY }
    }
    return { clientX: e.clientX, clientY: e.clientY }
}

</script>

<style scoped>
.touch-pad {
    cursor: grab;
    touch-action: none;
    transition: background-color 0.2s;
}
.touch-pad:active, .touch-pad.active {
    cursor: grabbing;
    background-color: var(--v-primary-base) !important; /* Vuetify variable? Use class logic instead */
     border-color: rgb(var(--v-theme-primary)) !important;
}
.pointer-events-none {
    pointer-events: none;
}
.user-select-none {
    user-select: none;
}
</style>
