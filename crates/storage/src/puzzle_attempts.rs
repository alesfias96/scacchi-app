//! I tentativi di risolvere i puzzle: salvataggio, rating e statistiche.

use app_settings::TrainingMode;

use crate::database::Database;
use crate::storage_error::StorageError;

/// Salva un tentativo.
const INSERT_ATTEMPT: &str = "
    INSERT INTO puzzle_attempts (puzzle_id, played_at, mode, solved, time_ms, puzzle_rating, rating_after)
    VALUES (?, ?, ?, ?, ?, ?, ?)";
/// Il rating dopo l'ultimo tentativo.
const SELECT_LATEST_RATING: &str =
    "SELECT rating_after FROM puzzle_attempts ORDER BY id DESC LIMIT 1";
/// I puzzle già affrontati almeno una volta.
const SELECT_ATTEMPTED_IDS: &str = "SELECT DISTINCT puzzle_id FROM puzzle_attempts";
/// Quanti puzzle nuovi sono stati affrontati e quanti risolti.
const SELECT_TOTALS: &str =
    "SELECT COUNT(*), COALESCE(SUM(solved), 0) FROM puzzle_attempts WHERE mode <> 'review'";

/// Un tentativo da salvare.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuzzleAttemptRecord {
    /// Il codice del puzzle.
    pub puzzle_id: String,
    /// Quando è stato giocato (millesimi di secondo dal 1970).
    pub played_at_ms: i64,
    /// La modalità di allenamento.
    pub mode: TrainingMode,
    /// `true` se risolto senza errori né aiuti.
    pub solved: bool,
    /// Quanto tempo è servito (millesimi di secondo).
    pub time_ms: i64,
    /// La difficoltà del puzzle.
    pub puzzle_rating: u32,
    /// Il rating di chi risolve dopo il tentativo.
    pub rating_after: i32,
}

/// Quanti puzzle nuovi sono stati affrontati e quanti risolti (i ripassi non contano).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PuzzleTotals {
    /// I puzzle affrontati.
    pub attempted: u32,
    /// I puzzle risolti senza errori né aiuti.
    pub solved: u32,
}

/// Il testo fisso della modalità nel database (non si cambia mai: i database
/// già salvati lo usano).
fn mode_code(mode: TrainingMode) -> &'static str {
    match mode {
        TrainingMode::Depth => "depth",
        TrainingMode::Visualization => "visualization",
        TrainingMode::Threat => "threat",
        TrainingMode::Review => "review",
    }
}

impl Database {
    /// Salva un tentativo.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn record_puzzle_attempt(
        &self,
        record: &PuzzleAttemptRecord,
    ) -> Result<(), StorageError> {
        sqlx::query(INSERT_ATTEMPT)
            .bind(&record.puzzle_id)
            .bind(record.played_at_ms)
            .bind(mode_code(record.mode))
            .bind(record.solved)
            .bind(record.time_ms)
            .bind(i64::from(record.puzzle_rating))
            .bind(record.rating_after)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Il rating dei puzzle attuale: `None` se non si è ancora fatto nessun puzzle.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn latest_puzzle_rating(&self) -> Result<Option<i32>, StorageError> {
        Ok(sqlx::query_scalar(SELECT_LATEST_RATING)
            .fetch_optional(&self.pool)
            .await?)
    }

    /// I codici dei puzzle già affrontati (per non riproporli come nuovi).
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn attempted_puzzle_ids(&self) -> Result<Vec<String>, StorageError> {
        Ok(sqlx::query_scalar(SELECT_ATTEMPTED_IDS)
            .fetch_all(&self.pool)
            .await?)
    }

    /// Quanti puzzle nuovi sono stati affrontati e quanti risolti.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn puzzle_totals(&self) -> Result<PuzzleTotals, StorageError> {
        let (attempted, solved): (i64, i64) =
            sqlx::query_as(SELECT_TOTALS).fetch_one(&self.pool).await?;
        Ok(PuzzleTotals {
            attempted: u32::try_from(attempted).unwrap_or(u32::MAX),
            solved: u32::try_from(solved).unwrap_or(u32::MAX),
        })
    }
}

#[cfg(test)]
mod tests {
    use app_settings::TrainingMode;

    use super::{PuzzleAttemptRecord, PuzzleTotals};
    use crate::database::Database;

    fn record(
        puzzle_id: &str,
        mode: TrainingMode,
        solved: bool,
        rating_after: i32,
    ) -> PuzzleAttemptRecord {
        PuzzleAttemptRecord {
            puzzle_id: puzzle_id.to_owned(),
            played_at_ms: 1_000,
            mode,
            solved,
            time_ms: 12_000,
            puzzle_rating: 1_600,
            rating_after,
        }
    }

    #[tokio::test]
    async fn an_empty_database_has_no_rating_and_no_puzzles() {
        let database = Database::open_in_memory().await.expect("database aperto");

        assert_eq!(
            database.latest_puzzle_rating().await.expect("lettura"),
            None
        );
        assert_eq!(
            database.attempted_puzzle_ids().await.expect("lettura"),
            Vec::<String>::new()
        );
        assert_eq!(
            database.puzzle_totals().await.expect("lettura"),
            PuzzleTotals::default()
        );
    }

    #[tokio::test]
    async fn remembers_the_latest_rating_and_the_puzzles_played() {
        let database = Database::open_in_memory().await.expect("database aperto");
        for attempt in [
            record("AAA", TrainingMode::Depth, true, 1_516),
            record("BBB", TrainingMode::Threat, false, 1_500),
            record("AAA", TrainingMode::Review, true, 1_500),
        ] {
            database
                .record_puzzle_attempt(&attempt)
                .await
                .expect("salvataggio");
        }

        assert_eq!(
            database.latest_puzzle_rating().await.expect("lettura"),
            Some(1_500)
        );
        let mut ids = database.attempted_puzzle_ids().await.expect("lettura");
        ids.sort();
        assert_eq!(ids, ["AAA", "BBB"]);
        assert_eq!(
            database.puzzle_totals().await.expect("lettura"),
            PuzzleTotals {
                attempted: 2,
                solved: 1
            }
        );
    }
}
