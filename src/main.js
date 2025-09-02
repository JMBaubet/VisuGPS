import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

// Vuetify
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import '@mdi/font/css/materialdesignicons.css'

const vuetify = createVuetify({
  theme: {
    defaultTheme: 'light',
    themes: {
      light: {
        colors: {
          background: '#FFFFFF', // Light background
          surface: '#FFFFFF',    // Light surface
          primary: '#1976D2',    // Blue
          secondary: '#424242',  // Grey
          error: '#FF5252',      // Red
          info: '#2196F3',       // Blue
          success: '#4CAF50',    // Green
          warning: '#FB8C00',    // Orange
        },
      },
      dark: {
        colors: {
          background: '#121212', // Dark background
          surface: '#212121',    // Dark surface
          primary: '#2196F3',    // Blue
          secondary: '#B0BEC5',  // Light grey
          error: '#FF5252',      // Red
          info: '#2196F3',       // Blue
          success: '#4CAF50',    // Green
          warning: '#FB8C00',    // Orange
        },
      },
    },
  },
  icons: {
    defaultSet: 'mdi',
  },
})

createApp(App).use(router).use(vuetify).mount('#app')
