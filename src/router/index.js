import { createRouter, createWebHistory } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import MainView from '../views/MainView.vue'
import EditView from '../views/EditView.vue'
import VisualizeView from '../views/VisualizeView.vue'
import SettingsView from '../views/SettingsView.vue'

const routes = [
  {
    path: '/',
    name: 'Main',
    component: MainView
  },
  {
    path: '/edit/:circuitId',
    name: 'EditView', // Renommé pour être plus explicite
    component: EditView
  },
  {
    path: '/visualize/:circuitId',
    name: 'Visualize',
    component: () => import('../views/VisualizeView.vue'),
    props: route => ({ circuitId: route.params.circuitId, traceType: 'main' })
  },
  {
    path: '/settings',
    name: 'Settings',
    component: SettingsView
  },
  {
    path: '/debug-tracking/:circuitId',
    name: 'DebugTracking',
    component: () => import('../views/DebugTrackingView.vue')
  },
  {
    path: '/variant-trace/:circuitId',
    name: 'VariantTraceView',
    component: () => import('../views/VariantTraceView.vue'),
    props: true
  },
  {
    path: '/visualize-variant/:circuitId/:variantId?',
    name: 'VisualizeVariant',
    component: () => import('../views/VisualizeView.vue'),
    props: true
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})


let isRouterReady = false;

router.isReady().then(() => {
  isRouterReady = true;
});

router.afterEach(async (to) => {
  if (!isRouterReady) return; // Ignore initial navigation during setup

  try {
    await invoke('update_current_view', { newView: to.name || 'Main' });
  } catch (error) {
    console.warn("Mise à jour de la vue ignorée (backend non prêt ou erreur):", error);
  }
});

export default router