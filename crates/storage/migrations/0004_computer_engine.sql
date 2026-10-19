-- =============================================================================
-- Migrazione 4 — il motore del computer
-- -----------------------------------------------------------------------------
-- Contro il computer si può giocare con Stockfish o con Maia (il computer
-- "umano", Fase 10-A). Il livello da solo non basta più: il livello 5 di
-- Stockfish e il livello 5 di Maia sono diversi.
-- Le partite salvate prima di questa migrazione erano tutte contro Stockfish.
-- =============================================================================

-- Il motore: 'stockfish' o 'maia' (anche a due giocatori vale 'stockfish', ma non conta).
ALTER TABLE games ADD COLUMN engine TEXT NOT NULL DEFAULT 'stockfish';
