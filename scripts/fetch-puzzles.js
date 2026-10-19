/**
 * Prepara i puzzle dell'allenamento a partire dal database di Lichess.
 *
 * Comando: `npm run puzzles` (serve solo per aggiornare i puzzle: il file
 * prodotto è salvato nel progetto, quindi di solito non va mai rifatto).
 *
 * Cosa fa, in ordine:
 * 1. scarica il database ufficiale dei puzzle di Lichess (licenza CC0, circa
 *    300 MB compressi, circa 5 milioni di puzzle) e lo legge **a flusso**:
 *    decomprime e analizza un pezzo alla volta, senza mai tenerlo tutto in memoria;
 * 2. tiene solo i puzzle di qualità: molto giocati, apprezzati e con una
 *    difficoltà (rating) affidabile;
 * 3. li distribuisce in modo uniforme tra difficoltà (da 400 a 2999, a fasce
 *    di 100) e profondità (da 1 a 6 mosse da trovare), così ogni livello e
 *    ogni profondità ne ha a sufficienza;
 * 4. scrive in `data/puzzles/lichess-puzzles.csv` i 100.000 scelti, solo con
 *    le colonne che servono all'app.
 *
 * Il file di Lichess è nel formato "zstd a blocchi" (pzstd): prima di ogni
 * blocco compresso c'è un piccolo "frame saltabile" con la sua lunghezza.
 * Lo zstd di Node non accetta questi frame, quindi qui vengono tolti prima
 * di decomprimere.
 *
 * Scritto in JavaScript (Node.js ≥ 22.15, che include zstd) e non in bash o
 * PowerShell, così funziona uguale su Windows, macOS e Linux (PROGETTO.md §5-bis).
 */
import { createWriteStream, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { createInterface } from 'node:readline';
import { Readable, Transform } from 'node:stream';
import { fileURLToPath } from 'node:url';
import { createZstdDecompress } from 'node:zlib';

/** Il database dei puzzle di Lichess. */
const SOURCE_URL = 'https://database.lichess.org/lichess_db_puzzle.csv.zst';
/** Il file prodotto. */
const OUTPUT = join(
  dirname(fileURLToPath(import.meta.url)),
  '..',
  'data',
  'puzzles',
  'lichess-puzzles.csv',
);

/** Quanti puzzle tenere in tutto. */
const TARGET = 100_000;
/** Quanti puzzle raccogliere al massimo per ogni casella (difficoltà × profondità). */
const CELL_CAP = 1_500;
/** Criteri di qualità. */
const MAX_RATING_DEVIATION = 90;
const MIN_POPULARITY = 85;
const MIN_PLAYS = 300;
/** Le fasce di difficoltà: da 400 a 2999, a fasce di 100. */
const MIN_RATING = 400;
const MAX_RATING = 2999;
/** La profondità massima (mosse da trovare). */
const MAX_DEPTH = 6;

/** Il "numero magico" dei frame saltabili di zstd (gli ultimi 4 bit possono variare). */
const SKIPPABLE_MAGIC = 0x184d2a50;

/**
 * Toglie i frame saltabili dal flusso e lascia passare solo i blocchi compressi.
 * Ogni frame saltabile del formato pzstd contiene 4 byte: la lunghezza del
 * blocco compresso che lo segue.
 */
class SkippableFrameStripper extends Transform {
  constructor() {
    super();
    /** I byte arrivati e non ancora usati. */
    this.pending = Buffer.alloc(0);
    /** Quanti byte del blocco compresso in corso mancano ancora. */
    this.remaining = 0;
  }

  _transform(chunk, _encoding, done) {
    this.pending = Buffer.concat([this.pending, chunk]);
    for (;;) {
      if (this.remaining > 0) {
        const take = Math.min(this.remaining, this.pending.length);
        if (take === 0) {
          break;
        }
        this.push(this.pending.subarray(0, take));
        this.pending = this.pending.subarray(take);
        this.remaining -= take;
        continue;
      }
      if (this.pending.length < 8) {
        break;
      }
      const magic = this.pending.readUInt32LE(0);
      if ((magic & 0xfffffff0) >>> 0 !== SKIPPABLE_MAGIC) {
        done(new Error('Formato del file dei puzzle inatteso (manca il frame saltabile).'));
        return;
      }
      const size = this.pending.readUInt32LE(4);
      if (this.pending.length < 8 + size) {
        break;
      }
      this.remaining = size >= 4 ? this.pending.readUInt32LE(8) : 0;
      this.pending = this.pending.subarray(8 + size);
    }
    done();
  }
}

/** La casella (fascia di difficoltà × profondità) di un puzzle, oppure `null` se è da scartare. */
function cellOf(rating, depth) {
  if (rating < MIN_RATING || rating > MAX_RATING || depth < 1 || depth > MAX_DEPTH) {
    return null;
  }
  return `${String(Math.floor(rating / 100))}-${String(depth)}`;
}

async function main() {
  console.log('Scarico e leggo il database dei puzzle di Lichess (circa 300 MB)…');
  const response = await fetch(SOURCE_URL);
  if (!response.ok || response.body === null) {
    throw new Error(`Download non riuscito: ${String(response.status)} ${response.statusText}.`);
  }
  const lines = createInterface({
    input: Readable.fromWeb(response.body)
      .pipe(new SkippableFrameStripper())
      .pipe(createZstdDecompress()),
    crlfDelay: Infinity,
  });

  /** Per ogni casella, le righe scelte (già nel formato di uscita). */
  const cells = new Map();
  let read = 0;
  let isHeader = true;
  for await (const line of lines) {
    if (isHeader) {
      isHeader = false;
      continue;
    }
    read += 1;
    if (read % 500_000 === 0) {
      console.log(`  ${String(read)} puzzle letti…`);
    }
    const [id, fen, moves, rating, deviation, popularity, plays, themes, , openings] =
      line.split(',');
    const moveList = moves.split(' ');
    const cell = cellOf(Number(rating), moveList.length / 2);
    if (
      cell === null ||
      moveList.length % 2 !== 0 ||
      Number(deviation) > MAX_RATING_DEVIATION ||
      Number(popularity) < MIN_POPULARITY ||
      Number(plays) < MIN_PLAYS
    ) {
      continue;
    }
    const chosen = cells.get(cell) ?? [];
    if (chosen.length < CELL_CAP) {
      chosen.push([id, fen, moves, rating, themes, openings ?? ''].join(','));
      cells.set(cell, chosen);
    }
  }

  // Quanti per casella, al massimo, per arrivare a TARGET in tutto: le caselle
  // con pochi puzzle li danno tutti, le altre si fermano alla stessa quota.
  const sizes = [...cells.values()].map((chosen) => chosen.length);
  let quota = CELL_CAP;
  while (quota > 0 && sizes.reduce((sum, size) => sum + Math.min(size, quota), 0) > TARGET) {
    quota -= 1;
  }

  mkdirSync(dirname(OUTPUT), { recursive: true });
  const output = createWriteStream(OUTPUT);
  output.write('id,fen,moves,rating,themes,openings\n');
  let written = 0;
  for (const key of [...cells.keys()].sort()) {
    for (const row of (cells.get(key) ?? []).slice(0, quota)) {
      output.write(`${row}\n`);
      written += 1;
    }
  }
  output.end();
  console.log(`Letti ${String(read)} puzzle, scelti ${String(written)}: salvati in ${OUTPUT}.`);
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
});
