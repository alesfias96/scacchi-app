-- =============================================================================
-- Migrazione 3 — analisi delle partite
-- -----------------------------------------------------------------------------
-- Quando una partita è stata analizzata (i dati di ogni mossa stanno già
-- nelle colonne della tabella moves, previste dalla migrazione 1) e i puzzle
-- nati dagli errori gravi delle proprie partite.
-- =============================================================================

-- Quando la partita è stata analizzata (millesimi di secondo dal 1970); vuoto = mai.
ALTER TABLE games ADD COLUMN analyzed_at INTEGER;

-- Una riga per ogni puzzle nato da un errore grave.
CREATE TABLE own_puzzles (
    -- Il codice del puzzle (es. 'own-<partita>-<mezza mossa>').
    id TEXT PRIMARY KEY NOT NULL,
    -- La partita e la mezza mossa dell'errore (se la partita si elimina, il puzzle sparisce).
    game_id TEXT NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    ply INTEGER NOT NULL,
    -- La posizione prima della mossa dell'avversario (FEN).
    fen TEXT NOT NULL,
    -- La mossa dell'avversario e la mossa migliore, in UCI separate da spazi.
    moves TEXT NOT NULL,
    -- La difficoltà data al puzzle (il rating di chi l'ha sbagliato).
    rating INTEGER NOT NULL,
    created_at INTEGER NOT NULL
);
