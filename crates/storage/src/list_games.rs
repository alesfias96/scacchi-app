//! L'elenco delle partite dell'archivio, con i filtri.

use app_settings::TimeControl;
use chess_core::GameResult;
use openings::Opening;
use sqlx::sqlite::SqliteRow;
use sqlx::{QueryBuilder, Row, Sqlite};

use crate::codes::{opponent_code, parse_opponent, parse_reason, parse_side};
use crate::database::Database;
use crate::game_filter::{GameFilter, OutcomeFilter};
use crate::game_summary::GameSummary;
use crate::storage_error::StorageError;

/// Quante partite mostrare al massimo (le più recenti).
const MAX_RESULTS: i64 = 500;

impl Database {
    /// Le partite salvate che rispettano i filtri, dalla più recente.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn list(&self, filter: &GameFilter) -> Result<Vec<GameSummary>, StorageError> {
        let mut query: QueryBuilder<Sqlite> = QueryBuilder::new(
            "SELECT id, started_at, opponent, computer_side, level, time_initial_seconds,
                    time_increment_seconds, result_score, result_winner, result_reason,
                    opening_eco, opening_name, ply_count
             FROM games WHERE 1 = 1",
        );
        if let Some(opponent) = filter.opponent {
            query
                .push(" AND opponent = ")
                .push_bind(opponent_code(opponent));
        }
        if let Some(outcome) = filter.outcome {
            query.push(match outcome {
                // Contro il computer: ha vinto chi non è il computer.
                OutcomeFilter::Won => {
                    " AND opponent = 'computer' AND result_winner IS NOT NULL AND result_winner <> computer_side"
                }
                OutcomeFilter::Lost => " AND opponent = 'computer' AND result_winner = computer_side",
                OutcomeFilter::Drawn => " AND result_score IS NOT NULL AND result_winner IS NULL",
                OutcomeFilter::Unfinished => " AND result_score IS NULL",
            });
        }
        if let Some(text) = filter
            .text
            .as_deref()
            .map(str::trim)
            .filter(|text| !text.is_empty())
        {
            let pattern = format!("%{text}%");
            query
                .push(" AND (opening_name LIKE ")
                .push_bind(pattern.clone())
                .push(" OR opening_eco LIKE ")
                .push_bind(pattern)
                .push(")");
        }
        query
            .push(" ORDER BY started_at DESC, rowid DESC LIMIT ")
            .push_bind(MAX_RESULTS);

        let rows = query.build().fetch_all(&self.pool).await?;
        rows.iter().map(summary_from_row).collect()
    }
}

/// Il riassunto di una partita, dalla sua riga.
fn summary_from_row(row: &SqliteRow) -> Result<GameSummary, StorageError> {
    let id: String = row.try_get("id")?;
    let time_control = row
        .try_get::<Option<u32>, _>("time_initial_seconds")?
        .zip(row.try_get::<Option<u32>, _>("time_increment_seconds")?)
        .map(|(initial_seconds, increment_seconds)| TimeControl {
            initial_seconds,
            increment_seconds,
        });
    let result = match row.try_get::<Option<String>, _>("result_score")? {
        Some(score) => {
            let reason = row.try_get::<Option<String>, _>("result_reason")?;
            Some(GameResult {
                winner: row
                    .try_get::<Option<String>, _>("result_winner")?
                    .as_deref()
                    .and_then(parse_side),
                reason: reason
                    .as_deref()
                    .and_then(parse_reason)
                    .ok_or_else(|| StorageError::Corrupted(id.clone()))?,
                score,
            })
        }
        None => None,
    };
    Ok(GameSummary {
        started_at_ms: row.try_get("started_at")?,
        opponent: parse_opponent(&row.try_get::<String, _>("opponent")?),
        computer_side: row
            .try_get::<Option<String>, _>("computer_side")?
            .as_deref()
            .and_then(parse_side),
        level: row.try_get("level")?,
        time_control,
        result,
        opening: row
            .try_get::<Option<String>, _>("opening_eco")?
            .zip(row.try_get::<Option<String>, _>("opening_name")?)
            .map(|(eco, name)| Opening { eco, name }),
        ply_count: row.try_get("ply_count")?,
        id,
    })
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use app_settings::{ColorChoice, GameSetup, OpponentKind};
    use chess_core::Side;
    use game_session::GameSession;

    use crate::database::Database;
    use crate::game_filter::{GameFilter, OutcomeFilter};

    /// Un archivio con tre partite, salvate in quest'ordine: una persa contro il
    /// computer (per abbandono), una a due giocatori non finita (Siciliana) e
    /// una a due giocatori vinta dal Bianco (abbandona il Nero).
    async fn sample_archive() -> (Database, [String; 3]) {
        let database = Database::open_in_memory().await.expect("database aperto");
        let now = Instant::now();
        let computer = GameSetup {
            opponent: OpponentKind::Computer,
            color: ColorChoice::White,
            level: 3,
            time_control: None,
        };

        // 1) Contro il computer, persa per abbandono.
        let mut lost = GameSession::start(&computer, Side::White, now);
        lost.play("d2d4", now).expect("mossa legale");
        lost.resign(now).expect("abbandono");

        // 2) A due giocatori, non finita, Siciliana.
        let mut sicilian = GameSession::default();
        sicilian.play("e2e4", now).expect("mossa legale");
        sicilian.play("c7c5", now).expect("mossa legale");

        // 3) A due giocatori, finita per abbandono del Nero.
        let mut won_by_white = GameSession::default();
        won_by_white.play("e2e4", now).expect("mossa legale");
        won_by_white.play("e7e5", now).expect("mossa legale");
        won_by_white.resign(now).expect("abbandono");

        for session in [&lost, &sicilian, &won_by_white] {
            database
                .save(&session.snapshot(now))
                .await
                .expect("salvataggio");
        }
        let ids = [lost.id(), sicilian.id(), won_by_white.id()].map(str::to_owned);
        (database, ids)
    }

    #[tokio::test]
    async fn lists_every_game_from_the_newest() {
        let (database, ids) = sample_archive().await;

        let all = database.list(&GameFilter::default()).await.expect("elenco");

        let listed: Vec<&str> = all.iter().map(|game| game.id.as_str()).collect();
        assert_eq!(listed, [ids[2].as_str(), ids[1].as_str(), ids[0].as_str()]);
        assert_eq!(all[1].ply_count, 2);
    }

    #[tokio::test]
    async fn filters_by_opponent_and_outcome() {
        let (database, ids) = sample_archive().await;

        let lost = database
            .list(&GameFilter {
                outcome: Some(OutcomeFilter::Lost),
                ..GameFilter::default()
            })
            .await
            .expect("elenco");
        let unfinished = database
            .list(&GameFilter {
                opponent: Some(OpponentKind::TwoPlayers),
                outcome: Some(OutcomeFilter::Unfinished),
                ..GameFilter::default()
            })
            .await
            .expect("elenco");

        assert_eq!(lost.len(), 1);
        assert_eq!(lost[0].id, ids[0]);
        assert_eq!(unfinished.len(), 1);
        assert_eq!(unfinished[0].id, ids[1]);
    }

    #[tokio::test]
    async fn searches_the_opening() {
        let (database, ids) = sample_archive().await;

        let found = database
            .list(&GameFilter {
                text: Some("sicilian".to_owned()),
                ..GameFilter::default()
            })
            .await
            .expect("elenco");

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].id, ids[1]);
        assert_eq!(found[0].opening.as_ref().expect("apertura").eco, "B20");
    }
}
