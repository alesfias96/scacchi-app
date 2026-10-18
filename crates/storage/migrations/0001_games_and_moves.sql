-- =============================================================================
-- Migrazione 1 — partite e mosse
-- -----------------------------------------------------------------------------
-- Crea le due tabelle dell'archivio. Questo file non si modifica mai dopo
-- essere stato rilasciato: ogni cambiamento dello schema va in un nuovo file
-- (0002_..., 0003_...), così i database già esistenti vengono aggiornati
-- senza perdere dati.
-- =============================================================================

-- Una riga per ogni partita (anche non finita).
CREATE TABLE games (
    -- Il nome unico della partita (creato da game-session).
    id TEXT PRIMARY KEY NOT NULL,
    -- Quando è cominciata e quando è stata salvata l'ultima volta (millesimi di secondo dal 1970).
    started_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    -- Contro chi: 'computer' oppure 'twoPlayers'.
    opponent TEXT NOT NULL,
    -- Contro il computer: il suo colore ('white' / 'black') e il livello (1–10).
    computer_side TEXT,
    level INTEGER,
    -- Il tempo di riflessione (vuoto = senza orologio) e i tempi rimasti all'ultimo salvataggio.
    time_initial_seconds INTEGER,
    time_increment_seconds INTEGER,
    clock_white_ms INTEGER,
    clock_black_ms INTEGER,
    -- La posizione di partenza (FEN).
    starting_fen TEXT NOT NULL,
    -- Il risultato (vuoto = partita in corso): punteggio, vincitore, motivo.
    result_score TEXT,
    result_winner TEXT,
    result_reason TEXT,
    -- L'apertura riconosciuta.
    opening_eco TEXT,
    opening_name TEXT,
    -- Quante mezze mosse sono state giocate.
    ply_count INTEGER NOT NULL,
    -- La partita completa in formato PGN.
    pgn TEXT NOT NULL
);

-- L'archivio si mostra dalla partita più recente.
CREATE INDEX games_by_start ON games (started_at DESC);

-- Una riga per ogni mossa di ogni partita.
CREATE TABLE moves (
    game_id TEXT NOT NULL REFERENCES games (id) ON DELETE CASCADE,
    -- La mezza mossa (1 = la prima) e il numero di mossa come nei libri.
    ply INTEGER NOT NULL,
    move_number INTEGER NOT NULL,
    -- Chi l'ha giocata ('white' / 'black').
    side TEXT NOT NULL,
    -- La mossa in notazione UCI e SAN, le sue case, se è una presa (0 / 1).
    uci TEXT NOT NULL,
    san TEXT NOT NULL,
    from_square TEXT NOT NULL,
    to_square TEXT NOT NULL,
    capture INTEGER NOT NULL,
    -- Quanto ha pensato chi l'ha giocata e quanto tempo gli restava (millesimi di secondo).
    time_spent_ms INTEGER,
    clock_after_ms INTEGER,
    -- Dati per le statistiche, riempiti dall'analisi automatica (Fase 8).
    -- Valutazione dal punto di vista del Bianco: in centipedoni oppure "matto in N".
    eval_before_cp INTEGER,
    eval_before_mate INTEGER,
    eval_after_cp INTEGER,
    eval_after_mate INTEGER,
    -- La mossa migliore secondo il motore e quanto si è perso rispetto a essa.
    best_move_uci TEXT,
    centipawn_loss INTEGER,
    -- Fase di gioco ('opening' / 'middlegame' / 'endgame').
    game_phase TEXT,
    -- Temi tattici presenti e minacce non viste (elenchi in formato JSON).
    tactical_themes TEXT,
    missed_threats TEXT,
    PRIMARY KEY (game_id, ply)
);
