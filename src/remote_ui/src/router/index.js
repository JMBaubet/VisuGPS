import { createRouter, createWebHistory } from 'vue-router'
// Les vues seront créées plus tard, on met des placeholders pour le moment
import AccueilView from '@/views/AccueilView.vue'
// import AnimationView from '@/views/AnimationView.vue'
// import PauseView from '@/views/PauseView.vue'

const routes = [
    {
        path: '/',
        name: 'Home',
        component: AccueilView,
    },
    // D'autres routes pourront être ajoutées si nécessaire, 
    // mais l'architecture actuelle semble basée sur un switch de vue dynamique dans App.vue 
    // plutôt que sur le routing URL, car c'est une SPA pilotée par l'état.
    // Cependant, on garde le routeur pour la structure.
]

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes,
})

export default router
