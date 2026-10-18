/**
 * Configurazione di ESLint, il "correttore" del codice TypeScript/React.
 *
 * ESLint legge il codice senza eseguirlo e segnala errori probabili,
 * cattive pratiche e codice inutilizzato. Qui è impostato al livello
 * più severo (`strictTypeChecked`): usa anche le informazioni sui tipi
 * di TypeScript per scovare errori che un controllo semplice non vede.
 *
 * Comando: `npm run lint`
 */
import js from '@eslint/js';
import { defineConfig, globalIgnores } from 'eslint/config';
import prettierConfig from 'eslint-config-prettier';
import reactHooks from 'eslint-plugin-react-hooks';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default defineConfig([
  // Cartelle che non contengono codice dell'interfaccia o sono generate automaticamente
  // (src/bindings contiene i tipi generati da Rust).
  globalIgnores(['dist/', 'target/', 'node_modules/', 'src-tauri/', 'crates/', 'src/bindings/']),

  // Regole di base di JavaScript.
  js.configs.recommended,

  // Regole severe di TypeScript, con controllo dei tipi.
  tseslint.configs.strictTypeChecked,
  tseslint.configs.stylisticTypeChecked,

  // Dice a ESLint dove trovare le configurazioni TypeScript del progetto
  // (tsconfig.json e i file che richiama).
  {
    languageOptions: {
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },

  // Codice dell'interfaccia: gira nel browser e usa gli hook di React.
  {
    files: ['src/**/*.{ts,tsx}'],
    languageOptions: {
      globals: globals.browser,
    },
    plugins: {
      'react-hooks': reactHooks,
    },
    rules: {
      // Gli hook vanno chiamati sempre nello stesso ordine (regola fondamentale di React).
      'react-hooks/rules-of-hooks': 'error',
      // Ogni hook deve dichiarare tutte le dipendenze che usa.
      'react-hooks/exhaustive-deps': 'error',
    },
  },

  // File JavaScript di configurazione (come questo): girano in Node.js
  // e non fanno parte di nessun progetto TypeScript, quindi niente controllo dei tipi.
  {
    files: ['**/*.js'],
    extends: [tseslint.configs.disableTypeChecked],
    languageOptions: {
      globals: globals.node,
    },
  },

  // Va per ultimo: spegne le regole di stile che si sovrapporrebbero a Prettier.
  // ESLint controlla la correttezza, Prettier l'aspetto: nessuna sovrapposizione.
  prettierConfig,
]);
