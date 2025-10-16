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


  app.mount('#app');
}

startApp();
