-- =============================================================================
-- Migrazione 2 — allenamento con i puzzle
-- -----------------------------------------------------------------------------
-- I tentativi di risolvere i puzzle (da cui si leggono il rating e le
-- statistiche) e il ripasso dei puzzle sbagliati. I puzzle stessi non sono
-- nel database: sono inclusi nell'app (crate training).
-- =============================================================================

-- Una riga per ogni puzzle affrontato.
CREATE TABLE puzzle_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- Il codice del puzzle su Lichess.
    puzzle_id TEXT NOT NULL,
    -- Quando è stato giocato (millesimi di secondo dal 1970).
    played_at INTEGER NOT NULL,
    -- La modalità: 'depth', 'visualization', 'threat' o 'review'.
    mode TEXT NOT NULL,
    -- 1 se risolto senza errori né aiuti, altrimenti 0.
    solved INTEGER NOT NULL,
    -- Quanto tempo è servito per decidere il risultato (millesimi di secondo).
    time_ms INTEGER NOT NULL,
    -- La difficoltà del puzzle e il rating di chi risolve dopo il tentativo.
    puzzle_rating INTEGER NOT NULL,
    rating_after INTEGER NOT NULL
);

CREATE INDEX puzzle_attempts_by_puzzle ON puzzle_attempts (puzzle_id);

-- Una riga per ogni puzzle da ripassare (metodo delle scatole).
CREATE TABLE puzzle_reviews (
    puzzle_id TEXT PRIMARY KEY NOT NULL,
    -- La scatola (0 = la prima, torna il giorno dopo).
    box_index INTEGER NOT NULL,
    -- Quando torna (millesimi di secondo dal 1970).
    due_at INTEGER NOT NULL
);

CREATE INDEX puzzle_reviews_by_due ON puzzle_reviews (due_at);
