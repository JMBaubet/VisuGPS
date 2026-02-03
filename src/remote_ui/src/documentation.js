/**
 * main.js - Point d'entrée de l'application de Documentation
 */

import { createApp } from 'vue'
import { createPinia } from 'pinia'

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { aliases, mdi } from 'vuetify/iconsets/mdi'

// Composants
import DocumentationApp from './DocumentationApp.vue'

const vuetify = createVuetify({
    components,
    directives,
    icons: {
        defaultSet: 'mdi',
        aliases,
        sets: {
            mdi,
        },
    },
    theme: {
        defaultTheme: 'dark'
    }
})

const app = createApp(DocumentationApp)

app.use(createPinia())
app.use(vuetify)

app.mount('#app')
