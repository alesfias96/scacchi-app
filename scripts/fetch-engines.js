/**
 * Scarica Stockfish, il motore di scacchi, per il sistema in uso.
 *
 * Comando: `npm run engines` (parte da solo anche dopo `npm install` e
 * all'inizio di `npm run check`).
 *
 * Cosa fa, in ordine:
 * 1. capisce per quale sistema si sta compilando (il "target" di Rust, es.
 *    `x86_64-pc-windows-msvc`), chiedendolo a `rustc`;
 * 2. se il motore giusto è già presente, non fa nulla;
 * 3. altrimenti scarica l'archivio ufficiale di Stockfish da GitHub e ne
 *    controlla l'impronta SHA-256: se non coincide con quella scritta qui
 *    sotto (file rovinato o manomesso) si ferma con un errore;
 * 4. estrae l'eseguibile e lo salva in `src-tauri/binaries/` con il nome che
 *    Tauri si aspetta (`stockfish-<target>`, con `.exe` su Windows), insieme
 *    alla licenza di Stockfish (GPL-3.0).
 *
 * È scritto in JavaScript (Node.js) e non in bash o PowerShell, così funziona
 * uguale su Windows, macOS e Linux (PROGETTO.md §5-bis). L'eseguibile è
 * pesante (circa 100 MB) e non entra nella cronologia git: si riscarica quando serve.
 *
 * Opzioni: `--target <target>` per scaricare il motore di un altro sistema;
 * `--force` per riscaricarlo anche se è già presente.
 */
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { chmodSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { gunzipSync, unzipSync } from 'fflate';

/** La versione di Stockfish usata dall'app (nome della release su GitHub). */
const STOCKFISH_RELEASE = 'sf_19';

/** Da dove si scaricano gli archivi ufficiali. */
const DOWNLOAD_BASE = `https://github.com/official-stockfish/Stockfish/releases/download/${STOCKFISH_RELEASE}`;

/**
 * Per ogni sistema: l'archivio ufficiale e la sua impronta SHA-256 (copiata
 * dalla pagina della release). Il Mac ha un solo archivio "universal" che
 * funziona sia sui processori Apple (M1, M2, …) sia sugli Intel.
 */
const ARCHIVES = {
  'x86_64-pc-windows-msvc': {
    file: 'stockfish-windows-x86-64-universal.zip',
    sha256: '3c8bf1f9ea66a09350a40df4f632288285ac206d99f33ab5842c408fc30b48a7',
  },
  'aarch64-pc-windows-msvc': {
    file: 'stockfish-windows-arm64-universal.zip',
    sha256: '8372ad3f0d7276deb2c70f801f541ec7db463219fc6d9c7592864e542aa4f401',
  },
  'aarch64-apple-darwin': {
    file: 'stockfish-macos-universal.tar.gz',
    sha256: 'a1f0e3bcc5a6927a11fe6fc8e54a779754645f3c2bae2cf13420fd1957adaa77',
  },
  'x86_64-apple-darwin': {
    file: 'stockfish-macos-universal.tar.gz',
    sha256: 'a1f0e3bcc5a6927a11fe6fc8e54a779754645f3c2bae2cf13420fd1957adaa77',
  },
  'x86_64-unknown-linux-gnu': {
    file: 'stockfish-linux-x86-64-universal.tar.gz',
    sha256: '9defc0d4e55d49c65a6d042f3e571a39fcea499ade6dbe741b53b8c65e03611f',
  },
  'aarch64-unknown-linux-gnu': {
    file: 'stockfish-linux-arm64-universal.tar.gz',
    sha256: 'fe26cfd1d9db4c8af3d21e24d9ff34cacb31c1f940085a7583da11796f2bac01',
  },
};

/** La cartella dove Tauri cerca i programmi esterni ("sidecar"). */
const BINARIES_DIR = join(dirname(fileURLToPath(import.meta.url)), '..', 'src-tauri', 'binaries');

/**
 * @returns il target di Rust indicato con `--target`, oppure quello del
 * computer in uso (riga `host:` di `rustc -vV`).
 */
function detectTarget() {
  const index = process.argv.indexOf('--target');
  if (index !== -1 && process.argv[index + 1] !== undefined) {
    return process.argv[index + 1];
  }
  const info = execFileSync('rustc', ['-vV'], { encoding: 'utf8' });
  const host = /^host: (\S+)$/m.exec(info);
  if (host === null) {
    throw new Error('Non riesco a capire il sistema in uso da `rustc -vV`.');
  }
  return host[1];
}

/**
 * Legge un archivio .tar (già decompresso) e restituisce i file che contiene.
 * Il formato tar è semplice: per ogni file un'intestazione di 512 byte (nome,
 * dimensione, tipo) seguita dal contenuto, arrotondato a multipli di 512.
 *
 * @param {Uint8Array} tar - il contenuto dell'archivio.
 * @returns {Record<string, Uint8Array>} nome del file → contenuto.
 */
function readTar(tar) {
  const decoder = new TextDecoder();
  /** Legge un campo di testo dell'intestazione, fino al primo byte zero. */
  const field = (start, length) =>
    decoder.decode(tar.subarray(start, start + length)).replace(/\0.*$/s, '');
  const files = {};
  let offset = 0;
  while (offset + 512 <= tar.length && tar[offset] !== 0) {
    const prefix = field(offset + 345, 155);
    const name = prefix === '' ? field(offset, 100) : `${prefix}/${field(offset, 100)}`;
    const size = Number.parseInt(field(offset + 124, 12).trim() || '0', 8);
    const type = field(offset + 156, 1);
    // '0' o vuoto = file normale; il resto (cartelle, collegamenti, ...) non serve.
    if (type === '0' || type === '') {
      files[name] = tar.subarray(offset + 512, offset + 512 + size);
    }
    offset += 512 + Math.ceil(size / 512) * 512;
  }
  return files;
}

/**
 * @param {string} archiveName - il nome dell'archivio (decide come aprirlo).
 * @param {Uint8Array} archive - il contenuto scaricato.
 * @returns {Record<string, Uint8Array>} nome del file → contenuto.
 */
function extractArchive(archiveName, archive) {
  return archiveName.endsWith('.zip') ? unzipSync(archive) : readTar(gunzipSync(archive));
}

async function main() {
  const target = detectTarget();
  const archive = ARCHIVES[target];
  if (archive === undefined) {
    throw new Error(`Stockfish non è disponibile per il sistema ${target}.`);
  }
  const extension = target.includes('windows') ? '.exe' : '';
  const binaryPath = join(BINARIES_DIR, `stockfish-${target}${extension}`);
  /** Ricorda quale archivio ha prodotto l'eseguibile presente. */
  const stampPath = join(BINARIES_DIR, `stockfish-${target}.version`);
  const stamp = `${STOCKFISH_RELEASE} ${archive.sha256}`;

  const upToDate =
    existsSync(binaryPath) &&
    existsSync(stampPath) &&
    readFileSync(stampPath, 'utf8').trim() === stamp;
  if (upToDate && !process.argv.includes('--force')) {
    console.log(`Stockfish (${STOCKFISH_RELEASE}) per ${target} è già presente.`);
    return;
  }

  console.log(`Scarico Stockfish (${STOCKFISH_RELEASE}) per ${target}: circa 80 MB…`);
  const response = await fetch(`${DOWNLOAD_BASE}/${archive.file}`);
  if (!response.ok) {
    throw new Error(`Download non riuscito: ${response.status} ${response.statusText}.`);
  }
  const data = new Uint8Array(await response.arrayBuffer());

  const digest = createHash('sha256').update(data).digest('hex');
  if (digest !== archive.sha256) {
    throw new Error(
      `L'archivio scaricato non è quello ufficiale (impronta ${digest}): non lo uso.`,
    );
  }

  const files = extractArchive(archive.file, data);
  /** L'eseguibile è l'unico file `stockfish/stockfish-…` nella cartella principale. */
  const binaryName = Object.keys(files).find((name) => /^stockfish\/stockfish-[^/]+$/.test(name));
  const license = files['stockfish/Copying.txt'];
  if (binaryName === undefined || license === undefined) {
    throw new Error("Nell'archivio non trovo l'eseguibile di Stockfish o la sua licenza.");
  }

  mkdirSync(BINARIES_DIR, { recursive: true });
  writeFileSync(binaryPath, files[binaryName]);
  // Su macOS e Linux un file si può avviare solo se è segnato come "eseguibile".
  chmodSync(binaryPath, 0o755);
  writeFileSync(join(BINARIES_DIR, 'STOCKFISH-COPYING.txt'), license);
  writeFileSync(stampPath, `${stamp}\n`);
  console.log(`Stockfish salvato in ${binaryPath}.`);
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
});
