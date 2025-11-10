import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { useSettings } from './composables/useSettings';

// Vuetify
import 'vuetify/styles';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

// ECharts
import ECharts from 'vue-echarts';
import 'echarts';

// Import de l'API shell de Tauri
import { open } from '@tauri-apps/plugin-shell';

const vuetify = createVuetify({
  components,
  directives,
});

async function startApp() {
  // Attendre que les paramètres soient initialisés
  const { initSettings } = useSettings();
  await initSettings();

  const app = createApp(App);

  app.use(router);
  app.use(vuetify);
  app.component('v-chart', ECharts);

  // Intercepter les clics sur les liens externes
  document.addEventListener('click', (ev) => {
    const a = ev.target.closest && ev.target.closest('a');
    if (!a) return;

    const href = a.getAttribute('href') || '';
    // filtre liens externes (https/http)
    if (href.startsWith('http://') || href.startsWith('https://')) {
      ev.preventDefault();       // empêche la navigation dans la webview
      open(href);               // ouvre dans le navigateur système
    }
  });

  app.mount('#app');
}

startApp();
