import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],


  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  clearScreen: false,

  // Server configuration
  server: {
    port: 1420,
    strictPort: true,
    cors: true,
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, PATCH, OPTIONS',
      'Access-Control-Allow-Headers': 'X-Requested-With, content-type, Authorization',
    },
    proxy: {
      '/auth': {
        target: 'http://localhost:3030',
        changeOrigin: true,
        secure: false,
        ws: true
      },
      '/api': {
        target: 'http://localhost:3030',
        changeOrigin: true,
        secure: false,
        ws: true
      }
    },
    watch: {
      // Ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
    hmr: {
      protocol: 'ws',
      host: 'localhost',
      port: 1420
    }
  },

  // Optimize deps
  optimizeDeps: {
    include: ['react', 'react-dom'],
    esbuildOptions: {
      // Node.js global to browser globalThis
      define: {
        global: 'globalThis',
      },
    },
  },
});
