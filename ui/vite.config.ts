import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import { resolve } from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:5150',
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: '../assets/static',
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
})
