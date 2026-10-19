-- =============================================================================
-- Migrazione 5 — gli appunti personali
-- -----------------------------------------------------------------------------
-- Una riga per ogni appunto: titolo, testo, etichette e, se c'è, la posizione
-- a cui si riferisce (con la partita e la mossa, se l'appunto è nato da una
-- partita). Se la partita si elimina, l'appunto resta con la sua posizione:
-- solo il collegamento alla partita si svuota (lo fa delete_game.rs).
-- =============================================================================

CREATE TABLE notes (
    -- Il nome unico dell'appunto (es. 'nota-19a3f2c4b10-0').
    id TEXT PRIMARY KEY NOT NULL,
    -- Il titolo (può essere vuoto) e il testo.
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    -- Le etichette, una per riga (le etichette non contengono mai un a capo).
    tags TEXT NOT NULL,
    -- La posizione collegata (FEN), se c'è.
    fen TEXT,
    -- La partita dell'archivio e la mezza mossa, se l'appunto è nato da una partita.
    game_id TEXT,
    ply INTEGER,
    -- Quando è stato creato e cambiato l'ultima volta (millesimi di secondo dal 1970).
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
