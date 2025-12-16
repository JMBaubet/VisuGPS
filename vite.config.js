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
        fs.cpSync(srcDir, destDir, { recursive: true, force: true })
      }
    }
  }
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    copyDocs(),
    {
      name: 'serve-docs',
      configureServer(server) {
        server.middlewares.use('/docs', (req, res, next) => {
          const serve = fs.createReadStream(path.join(__dirname, 'docs', req.url));
          serve.on('error', () => next());
          serve.pipe(res);
        });
      }
    }
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
    },
  },
})