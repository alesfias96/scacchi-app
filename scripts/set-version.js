/**
 * Cambia il numero di versione dell'app in tutti i file che lo contengono.
 *
 * Comando: `npm run set-version 0.11.0`
 *
 * La versione è scritta in quattro posti, che devono sempre coincidere:
 * - `package.json` e `package-lock.json` (l'interfaccia);
 * - `Cargo.toml` (tutte le crate Rust la ereditano da qui);
 * - `src-tauri/tauri.conf.json` (è quella che l'app mostra e che gli
 *   aggiornamenti confrontano con l'ultima pubblicata).
 * `Cargo.lock` si aggiorna da solo alla prossima compilazione.
 *
 * Il numero deve avere la forma "maggiore.minore.correzione" (es. 0.11.0).
 * È scritto in JavaScript (Node.js) e non in bash o PowerShell, così funziona
 * uguale su Windows, macOS e Linux (PROGETTO.md §5-bis).
 */
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

/** La cartella principale del progetto. */
const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..');

const version = process.argv[2];
if (version === undefined || !/^\d+\.\d+\.\d+$/.test(version)) {
  console.error('Uso: npm run set-version <numero>, ad esempio npm run set-version 0.11.0');
  process.exit(1);
}

/** Cambia un file JSON e lo riscrive con lo stesso aspetto (due spazi, a capo finale). */
function updateJson(path, change) {
  const full = join(ROOT, path);
  const data = JSON.parse(readFileSync(full, 'utf8'));
  change(data);
  writeFileSync(full, `${JSON.stringify(data, null, 2)}\n`);
  console.log(`${path}: ${version}`);
}

updateJson('package.json', (data) => {
  data.version = version;
});
updateJson('package-lock.json', (data) => {
  data.version = version;
  data.packages[''].version = version;
});
updateJson('src-tauri/tauri.conf.json', (data) => {
  data.version = version;
});

// In Cargo.toml la versione comune è la riga `version = "…"` della sezione [workspace.package].
const cargoPath = join(ROOT, 'Cargo.toml');
const cargo = readFileSync(cargoPath, 'utf8');
const updated = cargo.replace(
  /(\[workspace\.package\][^[]*?\nversion = ")[^"]*(")/,
  `$1${version}$2`,
);
if (updated === cargo && !cargo.includes(`version = "${version}"`)) {
  console.error('Cargo.toml: non ho trovato la versione in [workspace.package]');
  process.exit(1);
}
writeFileSync(cargoPath, updated);
console.log(`Cargo.toml: ${version}`);
