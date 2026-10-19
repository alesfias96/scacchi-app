//! Le letture per le statistiche: i dati grezzi di partite, mosse e puzzle.
//!
//! Qui non si calcola nulla: si leggono le righe che servono (filtrate per
//! periodo e avversario). I calcoli li fa la crate `stats`.

use app_settings::OpponentKind;
use chess_core::Side;
use sqlx::sqlite::SqliteRow;
use sqlx::{QueryBuilder, Row, Sqlite};

use crate::codes::{opponent_code, parse_opponent, parse_side};
use crate::database::Database;
use crate::storage_error::StorageError;

/// Le partite finite non ancora analizzate, dalla più vecchia.
const SELECT_UNANALYZED: &str = "
    SELECT id FROM games
    WHERE analyzed_at IS NULL AND result_score IS NOT NULL AND ply_count > 0
    ORDER BY started_at";
/// I tentativi dei puzzle da un certo istante, in ordine di tempo.
const SELECT_PUZZLES: &str = "
    SELECT puzzle_id, played_at, mode, solved, time_ms, rating_after
    FROM puzzle_attempts WHERE played_at >= ? ORDER BY id";

/// Quali dati leggere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StatsQuery {
    /// Solo da questo istante in poi (millesimi di secondo dal 1970); `None` = tutto.
    pub since_ms: Option<i64>,
    /// Solo le partite contro questo avversario; `None` = tutte.
    pub opponent: Option<OpponentKind>,
}

/// Una partita, per le statistiche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameStatRow {
    /// Il nome unico della partita.
    pub id: String,
    /// Quando è cominciata.
    pub started_at_ms: i64,
    /// Contro chi.
    pub opponent: OpponentKind,
    /// Il colore del computer, se c'era.
    pub computer_side: Option<Side>,
    /// Il livello del computer, se c'era.
    pub level: Option<u8>,
    /// `true` se la partita è finita.
    pub finished: bool,
    /// Chi ha vinto (`None` = patta o partita non finita).
    pub winner: Option<Side>,
    /// Il nome dell'apertura riconosciuta.
    pub opening_name: Option<String>,
    /// Il tempo iniziale per giocatore, se c'era l'orologio.
    pub time_initial_seconds: Option<u32>,
    /// `true` se la partita è stata analizzata.
    pub analyzed: bool,
}

/// Una mossa di una partita analizzata, per le statistiche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveStatRow {
    /// La partita.
    pub game_id: String,
    /// La mezza mossa.
    pub ply: u32,
    /// Chi l'ha giocata.
    pub side: Side,
    /// La mossa giocata (UCI).
    pub uci: String,
    /// La fase di gioco ('opening' / 'middlegame' / 'endgame').
    pub game_phase: Option<String>,
    /// La valutazione prima e dopo la mossa (per il Bianco): centipedoni o matto in N.
    pub eval_before_cp: Option<i32>,
    /// Vedi `eval_before_cp`.
    pub eval_before_mate: Option<i32>,
    /// Vedi `eval_before_cp`.
    pub eval_after_cp: Option<i32>,
    /// Vedi `eval_before_cp`.
    pub eval_after_mate: Option<i32>,
    /// La mossa migliore secondo il motore (UCI).
    pub best_move_uci: Option<String>,
    /// Il tempo che restava a chi ha mosso, dopo la mossa.
    pub clock_after_ms: Option<u32>,
}

/// Un tentativo di puzzle, per le statistiche.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuzzleStatRow {
    /// Il codice del puzzle.
    pub puzzle_id: String,
    /// Quando è stato giocato.
    pub played_at_ms: i64,
    /// La modalità ('depth', 'visualization', 'threat', 'review').
    pub mode: String,
    /// `true` se risolto senza errori né aiuti.
    pub solved: bool,
    /// Il tempo impiegato.
    pub time_ms: i64,
    /// Il rating dopo il tentativo.
    pub rating_after: i32,
}

/// Aggiunge i filtri di periodo e avversario (sulla tabella `games`, chiamata `g`).
fn push_filters(query: &mut QueryBuilder<Sqlite>, filter: &StatsQuery) {
    if let Some(since) = filter.since_ms {
        query.push(" AND g.started_at >= ").push_bind(since);
    }
    if let Some(opponent) = filter.opponent {
        query
            .push(" AND g.opponent = ")
            .push_bind(opponent_code(opponent));
    }
}

/// Un colore letto da una colonna che può essere vuota.
fn side_column(row: &SqliteRow, name: &str) -> Result<Option<Side>, StorageError> {
    Ok(row
        .try_get::<Option<String>, _>(name)?
        .as_deref()
        .and_then(parse_side))
}

impl Database {
    /// Le partite del periodo, dalla più vecchia.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn stats_games(&self, filter: &StatsQuery) -> Result<Vec<GameStatRow>, StorageError> {
        let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT g.id, g.started_at, g.opponent, g.computer_side, g.level, g.result_score,
                    g.result_winner, g.opening_name, g.time_initial_seconds,
                    g.analyzed_at IS NOT NULL AS analyzed
             FROM games g WHERE g.ply_count > 0",
        );
        push_filters(&mut query, filter);
        query.push(" ORDER BY g.started_at");
        let rows = query.build().fetch_all(&self.pool).await?;
        rows.iter()
            .map(|row| {
                Ok(GameStatRow {
                    id: row.try_get("id")?,
                    started_at_ms: row.try_get("started_at")?,
                    opponent: parse_opponent(&row.try_get::<String, _>("opponent")?),
                    computer_side: side_column(row, "computer_side")?,
                    level: row.try_get("level")?,
                    finished: row.try_get::<Option<String>, _>("result_score")?.is_some(),
                    winner: side_column(row, "result_winner")?,
                    opening_name: row.try_get("opening_name")?,
                    time_initial_seconds: row.try_get("time_initial_seconds")?,
                    analyzed: row.try_get("analyzed")?,
                })
            })
            .collect()
    }

    /// Le mosse delle partite analizzate del periodo, in ordine di partita e di mossa.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn stats_moves(&self, filter: &StatsQuery) -> Result<Vec<MoveStatRow>, StorageError> {
        let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT m.game_id, m.ply, m.side, m.uci, m.game_phase, m.eval_before_cp,
                    m.eval_before_mate, m.eval_after_cp, m.eval_after_mate, m.best_move_uci,
                    m.clock_after_ms
             FROM moves m JOIN games g ON g.id = m.game_id
             WHERE g.analyzed_at IS NOT NULL",
        );
        push_filters(&mut query, filter);
        query.push(" ORDER BY g.started_at, m.game_id, m.ply");
        let rows = query.build().fetch_all(&self.pool).await?;
        rows.iter()
            .map(|row| {
                Ok(MoveStatRow {
                    game_id: row.try_get("game_id")?,
                    ply: row.try_get("ply")?,
                    side: side_column(row, "side")?.unwrap_or(Side::White),
                    uci: row.try_get("uci")?,
                    game_phase: row.try_get("game_phase")?,
                    eval_before_cp: row.try_get("eval_before_cp")?,
                    eval_before_mate: row.try_get("eval_before_mate")?,
                    eval_after_cp: row.try_get("eval_after_cp")?,
                    eval_after_mate: row.try_get("eval_after_mate")?,
                    best_move_uci: row.try_get("best_move_uci")?,
                    clock_after_ms: row.try_get("clock_after_ms")?,
                })
            })
            .collect()
    }

    /// I tentativi dei puzzle da un certo istante (`None` = tutti), in ordine di tempo.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn stats_puzzles(
        &self,
        since_ms: Option<i64>,
    ) -> Result<Vec<PuzzleStatRow>, StorageError> {
        let rows = sqlx::query(SELECT_PUZZLES)
            .bind(since_ms.unwrap_or(i64::MIN))
            .fetch_all(&self.pool)
            .await?;
        rows.iter()
            .map(|row| {
                Ok(PuzzleStatRow {
                    puzzle_id: row.try_get("puzzle_id")?,
                    played_at_ms: row.try_get("played_at")?,
                    mode: row.try_get("mode")?,
                    solved: row.try_get("solved")?,
                    time_ms: row.try_get("time_ms")?,
                    rating_after: row.try_get("rating_after")?,
                })
            })
            .collect()
    }

    /// Le partite finite non ancora analizzate, dalla più vecchia.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn unanalyzed_games(&self) -> Result<Vec<String>, StorageError> {
        Ok(sqlx::query_scalar(SELECT_UNANALYZED)
            .fetch_all(&self.pool)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use app_settings::OpponentKind;
    use game_session::GameSession;

    use super::StatsQuery;
    use crate::database::Database;
    use crate::move_analysis::MoveAnalysisRecord;
    use crate::puzzle_attempts::PuzzleAttemptRecord;

    /// Una partita a due giocatori finita per abbandono, salvata.
    async fn finished_game(database: &Database) -> String {
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        session.resign(now).expect("abbandono");
        database
            .save(&session.snapshot(now))
            .await
            .expect("salvataggio");
        session.id().to_owned()
    }

    #[tokio::test]
    async fn reads_games_moves_and_the_games_to_analyze() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let id = finished_game(&database).await;

        assert_eq!(
            database.unanalyzed_games().await.expect("lettura"),
            std::slice::from_ref(&id)
        );
        let games = database
            .stats_games(&StatsQuery::default())
            .await
            .expect("lettura");
        assert_eq!(games.len(), 1);
        assert!(games[0].finished && !games[0].analyzed);
        assert_eq!(
            database
                .stats_moves(&StatsQuery::default())
                .await
                .expect("lettura"),
            []
        );

        let record = MoveAnalysisRecord {
            ply: 1,
            eval_before_cp: Some(20),
            ..MoveAnalysisRecord::default()
        };
        database
            .save_analysis(&id, std::slice::from_ref(&record), 1)
            .await
            .expect("analisi");
        let moves = database
            .stats_moves(&StatsQuery::default())
            .await
            .expect("lettura");
        assert_eq!(moves[0].eval_before_cp, Some(20));
        assert_eq!(
            database.unanalyzed_games().await.expect("lettura"),
            Vec::<String>::new()
        );

        let computer_only = StatsQuery {
            opponent: Some(OpponentKind::Computer),
            ..StatsQuery::default()
        };
        assert_eq!(
            database.stats_games(&computer_only).await.expect("lettura"),
            []
        );
    }

    #[tokio::test]
    async fn reads_the_puzzle_attempts_of_the_period() {
        let database = Database::open_in_memory().await.expect("database aperto");
        for played_at_ms in [100, 300] {
            let attempt = PuzzleAttemptRecord {
                puzzle_id: "AAA".to_owned(),
                played_at_ms,
                mode: app_settings::TrainingMode::Depth,
                solved: true,
                time_ms: 5_000,
                puzzle_rating: 1_500,
                rating_after: 1_516,
            };
            database
                .record_puzzle_attempt(&attempt)
                .await
                .expect("salvataggio");
        }

        assert_eq!(
            database.stats_puzzles(None).await.expect("lettura").len(),
            2
        );
        assert_eq!(
            database
                .stats_puzzles(Some(200))
                .await
                .expect("lettura")
                .len(),
            1
        );
    }
}
