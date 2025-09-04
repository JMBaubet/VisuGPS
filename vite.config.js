import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vuetify from 'vite-plugin-vuetify'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vuetify({ autoImport: true }),
  ],
  server: {
    watch: {
      ignored: ['**/.env'],
    },
  },
  resolve: {
    alias: {
      '@tauri-apps/plugin-opener': 'E:/Loisirs/VisuGPS/node_modules/@tauri-apps/plugin-opener/dist-js/index.js',
    },
  },
})