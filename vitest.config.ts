/**
 * Configurazione di Vitest, lo strumento che esegue i test dell'interfaccia.
 *
 * Riusa la configurazione di Vite (vite.config.ts), così i test vedono
 * il codice esattamente come lo vede l'app, e aggiunge solo le impostazioni
 * specifiche dei test.
 *
 * Comando: `npm run test`
 */
import { defineConfig, mergeConfig } from 'vitest/config';
import viteConfig from './vite.config';

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      // Simula una pagina web (jsdom), così i componenti React si possono
      // disegnare e controllare senza aprire l'app.
      environment: 'jsdom',
      // I test stanno accanto al file che verificano, con suffisso .test.ts/.test.tsx.
      include: ['src/**/*.test.{ts,tsx}'],
      // Preparazione comune: pulisce la pagina simulata dopo ogni test.
      setupFiles: ['src/test/setup.ts'],
      // Ripristina automaticamente eventuali simulazioni dopo ogni test.
      restoreMocks: true,
    },
  }),
);
