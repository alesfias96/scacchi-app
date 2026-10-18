/**
 * Configurazione di Vite, lo strumento che prepara l'interfaccia.
 *
 * Vite ha due compiti:
 * - in sviluppo (`npm run dev`) serve l'interfaccia su un indirizzo locale
 *   e la ricarica all'istante quando un file cambia;
 * - in produzione (`npm run build`) la impacchetta nella cartella `dist/`,
 *   che Tauri inserisce dentro l'app finale.
 *
 * I valori qui sotto sono quelli consigliati dalla documentazione di Tauri 2.
 */
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  // Abilita JSX/TSX e il ricaricamento rapido dei componenti React.
  plugins: [react()],

  // Non cancella il terminale: così restano visibili anche i messaggi di Rust.
  clearScreen: false,

  server: {
    // Porta fissa: deve coincidere con `build.devUrl` in src-tauri/tauri.conf.json.
    port: 1420,
    // Se la porta è occupata Vite si ferma invece di sceglierne un'altra,
    // altrimenti Tauri aprirebbe una finestra vuota.
    strictPort: true,
    watch: {
      // I file Rust sono gestiti da Tauri: Vite non deve osservarli.
      ignored: ['**/src-tauri/**', '**/crates/**', '**/target/**'],
    },
  },

  build: {
    // Il codice prodotto deve funzionare nei motori web di tutti e tre i sistemi:
    // WebView2 su Windows (Chromium), WKWebView su macOS (Safari), WebKitGTK su Linux.
    target: ['es2021', 'chrome105', 'safari15'],
  },
});
