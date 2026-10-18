//! La lettura di una partita salvata.

use app_settings::TimeControl;
use chess_core::{GameResult, MoveRecord};
use game_session::{MoveSnapshot, MoveTiming, SessionSnapshot};
use openings::Opening;
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::codes::{parse_opponent, parse_reason, parse_side};
use crate::database::Database;
use crate::storage_error::StorageError;

/// Legge una partita dato il suo nome unico.
const SELECT_BY_ID: &str = "
    SELECT id, started_at, opponent, computer_side, level,
           time_initial_seconds, time_increment_seconds, clock_white_ms, clock_black_ms,
           starting_fen, result_score, result_winner, result_reason, opening_eco, opening_name, pgn
    FROM games WHERE id = ?";

/// Legge la partita non finita salvata più di recente.
const SELECT_LATEST_UNFINISHED: &str = "
    SELECT id, started_at, opponent, computer_side, level,
           time_initial_seconds, time_increment_seconds, clock_white_ms, clock_black_ms,
           starting_fen, result_score, result_winner, result_reason, opening_eco, opening_name, pgn
    FROM games WHERE result_score IS NULL
    ORDER BY updated_at DESC, rowid DESC LIMIT 1";

impl Database {
    /// Legge una partita salvata.
    ///
    /// - Riceve: il nome unico della partita.
    /// - Restituisce: la sua "fotografia", da cui la partita si riprende;
    ///   `None` se non esiste.
    ///
    /// # Errors
    ///
    /// - [`StorageError::Database`] se la lettura non riesce;
    /// - [`StorageError::Corrupted`] se la partita salvata contiene valori senza senso.
    pub async fn load(&self, id: &str) -> Result<Option<SessionSnapshot>, StorageError> {
        let Some(row) = sqlx::query(SELECT_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
        else {
            return Ok(None);
        };
        self.snapshot_from_row(&row).await.map(Some)
    }

    /// La partita non finita salvata più di recente, da riprendere all'avvio.
    ///
    /// # Errors
    ///
    /// Gli stessi di [`Database::load`].
    pub async fn latest_unfinished(&self) -> Result<Option<SessionSnapshot>, StorageError> {
        let Some(row) = sqlx::query(SELECT_LATEST_UNFINISHED)
            .fetch_optional(&self.pool)
            .await?
        else {
            return Ok(None);
        };
        self.snapshot_from_row(&row).await.map(Some)
    }

    /// Ricostruisce la fotografia della partita dalla sua riga e dalle sue mosse.
    async fn snapshot_from_row(&self, row: &SqliteRow) -> Result<SessionSnapshot, StorageError> {
        let id: String = row.try_get("id")?;
        let corrupted = || StorageError::Corrupted(id.clone());

        let moves = sqlx::query(
            "SELECT ply, move_number, side, uci, san, from_square, to_square, capture,
                    time_spent_ms, clock_after_ms
             FROM moves WHERE game_id = ? ORDER BY ply",
        )
        .bind(&id)
        .fetch_all(&self.pool)
        .await?
        .iter()
        .map(|move_row| move_from_row(move_row).ok_or_else(corrupted))
        .collect::<Result<Vec<_>, _>>()?;

        let time_control = match (
            row.try_get::<Option<u32>, _>("time_initial_seconds")?,
            row.try_get::<Option<u32>, _>("time_increment_seconds")?,
        ) {
            (Some(initial_seconds), Some(increment_seconds)) => Some(TimeControl {
                initial_seconds,
                increment_seconds,
            }),
            _ => None,
        };
        let clock_ms = row
            .try_get::<Option<u32>, _>("clock_white_ms")?
            .zip(row.try_get::<Option<u32>, _>("clock_black_ms")?);

        let result = match row.try_get::<Option<String>, _>("result_score")? {
            Some(score) => {
                let reason = row.try_get::<Option<String>, _>("result_reason")?;
                let winner = row.try_get::<Option<String>, _>("result_winner")?;
                Some(GameResult {
                    winner: winner.as_deref().and_then(parse_side),
                    reason: reason
                        .as_deref()
                        .and_then(parse_reason)
                        .ok_or_else(corrupted)?,
                    score,
                })
            }
            None => None,
        };
        let opening = row
            .try_get::<Option<String>, _>("opening_eco")?
            .zip(row.try_get::<Option<String>, _>("opening_name")?)
            .map(|(eco, name)| Opening { eco, name });
        let computer_side = row
            .try_get::<Option<String>, _>("computer_side")?
            .as_deref()
            .and_then(parse_side);

        Ok(SessionSnapshot {
            started_at_ms: row.try_get("started_at")?,
            opponent: parse_opponent(&row.try_get::<String, _>("opponent")?),
            computer_side,
            level: row.try_get("level")?,
            time_control,
            clock_ms,
            starting_fen: row.try_get("starting_fen")?,
            moves,
            result,
            opening,
            pgn: row.try_get("pgn")?,
            id,
        })
    }
}

/// Una mossa salvata, dalla sua riga; `None` se la riga contiene valori senza senso.
fn move_from_row(row: &SqliteRow) -> Option<MoveSnapshot> {
    let ply: i64 = row.try_get("ply").ok()?;
    Some(MoveSnapshot {
        record: MoveRecord {
            ply: usize::try_from(ply).ok()?,
            move_number: row.try_get("move_number").ok()?,
            side: parse_side(&row.try_get::<String, _>("side").ok()?)?,
            san: row.try_get("san").ok()?,
            uci: row.try_get("uci").ok()?,
            from: row.try_get("from_square").ok()?,
            to: row.try_get("to_square").ok()?,
            capture: row.try_get("capture").ok()?,
        },
        timing: MoveTiming {
            time_spent_ms: row.try_get("time_spent_ms").ok()?,
            clock_after_ms: row.try_get("clock_after_ms").ok()?,
        },
    })
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use app_settings::{ColorChoice, GameSetup, OpponentKind, TimeControl};
    use chess_core::Side;
    use game_session::GameSession;

    use crate::database::Database;

    /// Una partita contro il computer, con l'orologio, dopo 1.e4 c5.
    fn sample_session() -> GameSession {
        let start = Instant::now();
        let setup = GameSetup {
            opponent: OpponentKind::Computer,
            color: ColorChoice::White,
            level: 5,
            time_control: Some(TimeControl {
                initial_seconds: 300,
                increment_seconds: 3,
            }),
        };
        let mut session = GameSession::start(&setup, Side::White, start);
        session
            .play("e2e4", start + Duration::from_secs(2))
            .expect("mossa legale");
        let request = session
            .engine_request(start + Duration::from_secs(2))
            .expect("tocca al computer");
        session
            .play_engine(request.token, "c7c5", start + Duration::from_secs(3))
            .expect("mossa del computer");
        session
    }

    #[tokio::test]
    async fn a_saved_game_comes_back_identical() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let snapshot = sample_session().snapshot(Instant::now());

        database.save(&snapshot).await.expect("salvataggio");
        let loaded = database
            .load(&snapshot.id)
            .await
            .expect("lettura")
            .expect("partita trovata");

        assert_eq!(loaded, snapshot);
    }

    #[tokio::test]
    async fn saving_again_updates_the_game_and_drops_taken_back_moves() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let mut session = GameSession::default();
        let now = Instant::now();
        session.play("e2e4", now).expect("mossa legale");
        session.play("e7e5", now).expect("mossa legale");
        database
            .save(&session.snapshot(now))
            .await
            .expect("salvataggio");

        session.undo(now).expect("si può ritirare");
        database
            .save(&session.snapshot(now))
            .await
            .expect("salvataggio");

        let loaded = database
            .load(session.id())
            .await
            .expect("lettura")
            .expect("partita trovata");
        assert_eq!(loaded.moves.len(), 1);
    }

    #[tokio::test]
    async fn finds_the_latest_unfinished_game() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let now = Instant::now();
        let mut finished = GameSession::default();
        finished.play("e2e4", now).expect("mossa legale");
        finished.resign(now).expect("abbandono");
        let unfinished = sample_session();
        database
            .save(&finished.snapshot(now))
            .await
            .expect("salvataggio");
        database
            .save(&unfinished.snapshot(now))
            .await
            .expect("salvataggio");

        let latest = database
            .latest_unfinished()
            .await
            .expect("lettura")
            .expect("partita trovata");

        assert_eq!(latest.id, unfinished.id());
    }

    #[tokio::test]
    async fn an_unknown_game_is_not_found() {
        let database = Database::open_in_memory().await.expect("database aperto");

        assert_eq!(database.load("nessuna").await.expect("lettura"), None);
    }
}
