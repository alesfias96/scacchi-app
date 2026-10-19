//! I puzzle nati dagli errori gravi delle proprie partite.

use crate::database::Database;
use crate::storage_error::StorageError;

/// Salva un puzzle, se non c'è già.
const INSERT_PUZZLE: &str = "
    INSERT OR IGNORE INTO own_puzzles (id, game_id, ply, fen, moves, rating, created_at)
    VALUES (?, ?, ?, ?, ?, ?, ?)";
/// Legge un puzzle.
const SELECT_PUZZLE: &str =
    "SELECT id, game_id, ply, fen, moves, rating, created_at FROM own_puzzles WHERE id = ?";

/// Un puzzle nato da un errore grave, come è salvato.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct OwnPuzzleRecord {
    /// Il codice del puzzle.
    pub id: String,
    /// La partita dell'errore.
    pub game_id: String,
    /// La mezza mossa dell'errore.
    pub ply: u32,
    /// La posizione prima della mossa dell'avversario (FEN).
    pub fen: String,
    /// La mossa dell'avversario e la mossa migliore, in UCI separate da spazi.
    pub moves: String,
    /// La difficoltà data al puzzle.
    pub rating: u32,
    /// Quando è stato creato (millesimi di secondo dal 1970).
    pub created_at: i64,
}

impl Database {
    /// Salva un puzzle.
    ///
    /// Restituisce `true` se è nuovo, `false` se c'era già (la stessa partita
    /// analizzata due volte non crea doppioni).
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn save_own_puzzle(&self, puzzle: &OwnPuzzleRecord) -> Result<bool, StorageError> {
        let result = sqlx::query(INSERT_PUZZLE)
            .bind(&puzzle.id)
            .bind(&puzzle.game_id)
            .bind(i64::from(puzzle.ply))
            .bind(&puzzle.fen)
            .bind(&puzzle.moves)
            .bind(i64::from(puzzle.rating))
            .bind(puzzle.created_at)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// Un puzzle, dato il suo codice: `None` se non esiste (ad esempio se la
    /// sua partita è stata eliminata).
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn own_puzzle(&self, id: &str) -> Result<Option<OwnPuzzleRecord>, StorageError> {
        Ok(sqlx::query_as(SELECT_PUZZLE)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use game_session::GameSession;

    use super::OwnPuzzleRecord;
    use crate::database::Database;

    #[tokio::test]
    async fn saves_a_puzzle_once_and_forgets_it_with_its_game() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        database
            .save(&session.snapshot(now))
            .await
            .expect("salvataggio");
        let puzzle = OwnPuzzleRecord {
            id: "own-1".to_owned(),
            game_id: session.id().to_owned(),
            ply: 2,
            fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_owned(),
            moves: "e7e5 g1f3".to_owned(),
            rating: 1500,
            created_at: 1_000,
        };

        assert!(
            database
                .save_own_puzzle(&puzzle)
                .await
                .expect("salvataggio")
        );
        assert!(
            !database
                .save_own_puzzle(&puzzle)
                .await
                .expect("salvataggio")
        );
        assert_eq!(
            database.own_puzzle("own-1").await.expect("lettura"),
            Some(puzzle)
        );

        database.delete(session.id()).await.expect("eliminazione");
        assert_eq!(database.own_puzzle("own-1").await.expect("lettura"), None);
    }
}
