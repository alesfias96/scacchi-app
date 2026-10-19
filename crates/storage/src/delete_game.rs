//! L'eliminazione di una partita dall'archivio.

use crate::database::Database;
use crate::storage_error::StorageError;

impl Database {
    /// Elimina una partita e tutte le sue mosse. Se non esiste non fa nulla.
    ///
    /// Gli appunti collegati alla partita restano, con la loro posizione:
    /// si toglie solo il collegamento alla partita.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn delete(&self, id: &str) -> Result<(), StorageError> {
        // Le mosse si cancellano da sole ("ON DELETE CASCADE" nello schema).
        sqlx::query("UPDATE notes SET game_id = NULL, ply = NULL WHERE game_id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
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
    use notes::{Note, NoteLink};

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

    #[tokio::test]
    async fn the_notes_of_a_deleted_game_keep_their_position() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        database
            .save(&session.snapshot(now))
            .await
            .expect("salvataggio");
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".to_owned();
        let note = Note {
            id: "nota".to_owned(),
            title: String::new(),
            body: "Apertura di re".to_owned(),
            tags: Vec::new(),
            link: Some(NoteLink {
                fen: fen.clone(),
                game_id: Some(session.id().to_owned()),
                ply: Some(1),
            }),
            created_at_ms: 1,
            updated_at_ms: 1,
        };
        database.save_note(&note).await.expect("appunto");

        database.delete(session.id()).await.expect("eliminazione");

        let kept = database
            .note_by_id("nota")
            .await
            .expect("lettura")
            .expect("appunto");
        assert_eq!(
            kept.link,
            Some(NoteLink {
                fen,
                game_id: None,
                ply: None
            })
        );
    }
}
