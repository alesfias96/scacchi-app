//! Il salvataggio di una partita.

use game_session::{SessionSnapshot, now_unix_ms};

use crate::codes::{engine_code, opponent_code, reason_code, side_code};
use crate::database::Database;
use crate::storage_error::StorageError;

/// Scrive (o aggiorna) la riga della partita.
const UPSERT_GAME: &str = "
    INSERT INTO games (
        id, started_at, updated_at, opponent, computer_side, level,
        time_initial_seconds, time_increment_seconds, clock_white_ms, clock_black_ms,
        starting_fen, result_score, result_winner, result_reason,
        opening_eco, opening_name, ply_count, pgn, engine
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    ON CONFLICT (id) DO UPDATE SET
        updated_at = excluded.updated_at,
        opponent = excluded.opponent,
        computer_side = excluded.computer_side,
        level = excluded.level,
        time_initial_seconds = excluded.time_initial_seconds,
        time_increment_seconds = excluded.time_increment_seconds,
        clock_white_ms = excluded.clock_white_ms,
        clock_black_ms = excluded.clock_black_ms,
        starting_fen = excluded.starting_fen,
        result_score = excluded.result_score,
        result_winner = excluded.result_winner,
        result_reason = excluded.result_reason,
        opening_eco = excluded.opening_eco,
        opening_name = excluded.opening_name,
        ply_count = excluded.ply_count,
        pgn = excluded.pgn,
        engine = excluded.engine";

/// Scrive (o aggiorna) una mossa. Se nella stessa mezza mossa c'era già la
/// stessa mossa, i dati per le statistiche (valutazioni, …) restano; se la
/// mossa è cambiata (seguito sostituito), vengono svuotati.
const UPSERT_MOVE: &str = "
    INSERT INTO moves (
        game_id, ply, move_number, side, uci, san, from_square, to_square, capture,
        time_spent_ms, clock_after_ms
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    ON CONFLICT (game_id, ply) DO UPDATE SET
        eval_before_cp = CASE WHEN moves.uci = excluded.uci THEN moves.eval_before_cp END,
        eval_before_mate = CASE WHEN moves.uci = excluded.uci THEN moves.eval_before_mate END,
        eval_after_cp = CASE WHEN moves.uci = excluded.uci THEN moves.eval_after_cp END,
        eval_after_mate = CASE WHEN moves.uci = excluded.uci THEN moves.eval_after_mate END,
        best_move_uci = CASE WHEN moves.uci = excluded.uci THEN moves.best_move_uci END,
        centipawn_loss = CASE WHEN moves.uci = excluded.uci THEN moves.centipawn_loss END,
        game_phase = CASE WHEN moves.uci = excluded.uci THEN moves.game_phase END,
        tactical_themes = CASE WHEN moves.uci = excluded.uci THEN moves.tactical_themes END,
        missed_threats = CASE WHEN moves.uci = excluded.uci THEN moves.missed_threats END,
        move_number = excluded.move_number,
        side = excluded.side,
        uci = excluded.uci,
        san = excluded.san,
        from_square = excluded.from_square,
        to_square = excluded.to_square,
        capture = excluded.capture,
        time_spent_ms = excluded.time_spent_ms,
        clock_after_ms = excluded.clock_after_ms";

impl Database {
    /// Salva la partita: la crea se è nuova, altrimenti la aggiorna.
    ///
    /// Tutto avviene in una "transazione": o si salva tutto, o niente (se
    /// l'app si chiudesse a metà, il database resta com'era prima).
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn save(&self, snapshot: &SessionSnapshot) -> Result<(), StorageError> {
        let mut transaction = self.pool.begin().await?;
        let result = snapshot.result.as_ref();
        let ply_count = i64::try_from(snapshot.moves.len()).unwrap_or(i64::MAX);

        sqlx::query(UPSERT_GAME)
            .bind(&snapshot.id)
            .bind(snapshot.started_at_ms)
            .bind(now_unix_ms())
            .bind(opponent_code(snapshot.opponent))
            .bind(snapshot.computer_side.map(side_code))
            .bind(snapshot.level)
            .bind(snapshot.time_control.map(|control| control.initial_seconds))
            .bind(
                snapshot
                    .time_control
                    .map(|control| control.increment_seconds),
            )
            .bind(snapshot.clock_ms.map(|(white, _)| white))
            .bind(snapshot.clock_ms.map(|(_, black)| black))
            .bind(&snapshot.starting_fen)
            .bind(result.map(|result| result.score.as_str()))
            .bind(result.and_then(|result| result.winner).map(side_code))
            .bind(result.map(|result| reason_code(result.reason)))
            .bind(
                snapshot
                    .opening
                    .as_ref()
                    .map(|opening| opening.eco.as_str()),
            )
            .bind(
                snapshot
                    .opening
                    .as_ref()
                    .map(|opening| opening.name.as_str()),
            )
            .bind(ply_count)
            .bind(&snapshot.pgn)
            .bind(engine_code(snapshot.engine))
            .execute(&mut *transaction)
            .await?;

        for saved in &snapshot.moves {
            let record = &saved.record;
            sqlx::query(UPSERT_MOVE)
                .bind(&snapshot.id)
                .bind(i64::try_from(record.ply).unwrap_or(i64::MAX))
                .bind(record.move_number)
                .bind(side_code(record.side))
                .bind(&record.uci)
                .bind(&record.san)
                .bind(&record.from)
                .bind(&record.to)
                .bind(record.capture)
                .bind(saved.timing.time_spent_ms)
                .bind(saved.timing.clock_after_ms)
                .execute(&mut *transaction)
                .await?;
        }

        // Le mosse ritirate (oltre la fine della partita) non esistono più.
        sqlx::query("DELETE FROM moves WHERE game_id = ? AND ply > ?")
            .bind(&snapshot.id)
            .bind(ply_count)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;
        Ok(())
    }
}
