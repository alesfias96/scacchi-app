//! L'eliminazione di una partita dall'archivio.

use crate::database::Database;
use crate::storage_error::StorageError;

impl Database {
    /// Elimina una partita e tutte le sue mosse. Se non esiste non fa nulla.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn delete(&self, id: &str) -> Result<(), StorageError> {
        // Le mosse si cancellano da sole ("ON DELETE CASCADE" nello schema).
        sqlx::query("DELETE FROM games WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use game_session::GameSession;

    use crate::database::Database;

    #[tokio::test]
    async fn removes_the_game_and_its_moves() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        database
            .save(&session.snapshot(now))
            .await
            .expect("salvataggio");

        database.delete(session.id()).await.expect("eliminazione");

        assert_eq!(database.load(session.id()).await.expect("lettura"), None);
        let moves: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM moves")
            .fetch_one(&database.pool)
            .await
            .expect("conteggio");
        assert_eq!(moves, 0);
    }
}
