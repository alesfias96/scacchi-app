//! Il ripasso dei puzzle sbagliati: quando torna ciascun puzzle.

use crate::database::Database;
use crate::storage_error::StorageError;

/// Legge il ripasso di un puzzle.
const SELECT_REVIEW: &str = "SELECT box_index, due_at FROM puzzle_reviews WHERE puzzle_id = ?";
/// Crea o aggiorna il ripasso di un puzzle.
const UPSERT_REVIEW: &str = "
    INSERT INTO puzzle_reviews (puzzle_id, box_index, due_at) VALUES (?, ?, ?)
    ON CONFLICT (puzzle_id) DO UPDATE SET box_index = excluded.box_index, due_at = excluded.due_at";
/// Toglie un puzzle dal ripasso.
const DELETE_REVIEW: &str = "DELETE FROM puzzle_reviews WHERE puzzle_id = ?";
/// I puzzle da ripassare entro un certo istante, dal più vecchio.
const SELECT_DUE: &str =
    "SELECT puzzle_id FROM puzzle_reviews WHERE due_at <= ? ORDER BY due_at, puzzle_id";

/// A che punto è il ripasso di un puzzle, come è salvato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoredReview {
    /// La scatola (0 = la prima).
    pub box_index: u8,
    /// Quando torna (millesimi di secondo dal 1970).
    pub due_at_ms: i64,
}

impl Database {
    /// Il ripasso di un puzzle: `None` se il puzzle non è da ripassare.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn puzzle_review(
        &self,
        puzzle_id: &str,
    ) -> Result<Option<StoredReview>, StorageError> {
        let row: Option<(i64, i64)> = sqlx::query_as(SELECT_REVIEW)
            .bind(puzzle_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|(box_index, due_at_ms)| StoredReview {
            box_index: u8::try_from(box_index).unwrap_or(0),
            due_at_ms,
        }))
    }

    /// Salva il ripasso di un puzzle; con `None` toglie il puzzle dal ripasso.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn set_puzzle_review(
        &self,
        puzzle_id: &str,
        review: Option<StoredReview>,
    ) -> Result<(), StorageError> {
        match review {
            Some(review) => {
                sqlx::query(UPSERT_REVIEW)
                    .bind(puzzle_id)
                    .bind(i64::from(review.box_index))
                    .bind(review.due_at_ms)
                    .execute(&self.pool)
                    .await?;
            }
            None => {
                sqlx::query(DELETE_REVIEW)
                    .bind(puzzle_id)
                    .execute(&self.pool)
                    .await?;
            }
        }
        Ok(())
    }

    /// I codici dei puzzle da ripassare entro l'istante indicato, dal più vecchio.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn due_puzzle_reviews(&self, now_ms: i64) -> Result<Vec<String>, StorageError> {
        Ok(sqlx::query_scalar(SELECT_DUE)
            .bind(now_ms)
            .fetch_all(&self.pool)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::StoredReview;
    use crate::database::Database;

    #[tokio::test]
    async fn saves_updates_and_removes_a_review() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let first = StoredReview {
            box_index: 0,
            due_at_ms: 100,
        };
        let second = StoredReview {
            box_index: 1,
            due_at_ms: 300,
        };

        database
            .set_puzzle_review("AAA", Some(first))
            .await
            .expect("salvataggio");
        assert_eq!(
            database.puzzle_review("AAA").await.expect("lettura"),
            Some(first)
        );

        database
            .set_puzzle_review("AAA", Some(second))
            .await
            .expect("aggiornamento");
        assert_eq!(
            database.puzzle_review("AAA").await.expect("lettura"),
            Some(second)
        );

        database
            .set_puzzle_review("AAA", None)
            .await
            .expect("eliminazione");
        assert_eq!(database.puzzle_review("AAA").await.expect("lettura"), None);
    }

    #[tokio::test]
    async fn lists_only_the_puzzles_already_due() {
        let database = Database::open_in_memory().await.expect("database aperto");
        for (id, due_at_ms) in [("LATE", 500), ("OLD", 100), ("NOW", 200)] {
            database
                .set_puzzle_review(
                    id,
                    Some(StoredReview {
                        box_index: 0,
                        due_at_ms,
                    }),
                )
                .await
                .expect("salvataggio");
        }

        assert_eq!(
            database.due_puzzle_reviews(200).await.expect("lettura"),
            ["OLD", "NOW"]
        );
    }
}
