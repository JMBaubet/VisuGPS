import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'
import fs from 'node:fs'

// Plugin personnalisé pour copier le dossier docs vers dist/docs
const copyDocs = () => {
  return {
    name: 'copy-docs',
    closeBundle() {
      const srcDir = path.resolve(__dirname, 'docs')
      const destDir = path.resolve(__dirname, 'dist/docs')

      if (fs.existsSync(srcDir)) {
        // Crée le dossier de destination s'il n'existe pas (nécessite Node.js 16.7+)
        fs.cpSync(srcDir, destDir, { recursive: true, force: true })
        console.log(`[copy-docs] Documents copiés vers ${destDir}`)
      }
    }
  }
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    copyDocs()
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
})