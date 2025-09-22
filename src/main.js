import { createApp } from 'vue'
import { createVuetify } from 'vuetify'
import 'vuetify/styles'
import App from './App.vue'
import router from './router'
import { useSettings } from './composables/useSettings'

import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const vuetify = createVuetify({
  components,
  directives,
})

async function startApp() {
  const { initSettings } = useSettings();
  await initSettings();

  createApp(App)
    .use(router)
    .use(vuetify)
    .mount('#app');
}

startApp();