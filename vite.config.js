import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],

  // This section is crucial for Tauri development
  // 1. Prevents Vite from hiding Rust errors
  clearScreen: false,
  // 2. Tauri expects a fixed port, so we'll enforce that
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. Tells Vite to ignore the Rust backend folder
      ignored: ["**/src-tauri/**"],
    },
  },
});