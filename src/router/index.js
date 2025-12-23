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
    component: VisualizeView,
    props: true
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
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.afterEach(async (to) => {
  try {
    await invoke('update_current_view', { newView: to.name || 'Main' });
  } catch (error) {
    console.error("Erreur lors de la mise à jour de la vue courante dans le backend:", error);
  }
});

export default router