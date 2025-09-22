import { createRouter, createWebHistory } from 'vue-router'
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
    path: '/edit',
    name: 'Edit',
    component: EditView
  },
  {
    path: '/visualize',
    name: 'Visualize',
    component: VisualizeView
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

export default router