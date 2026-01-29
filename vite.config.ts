import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  // Prevent vite from obscuring Rust errors
  clearScreen: false,
  // Tauri expects a fixed port
  server: {
    port: 5173,
    strictPort: true,
  },
  // Environment variables prefixed with TAURI_ will be available
  envPrefix: ['VITE_', 'TAURI_'],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
  },
})
