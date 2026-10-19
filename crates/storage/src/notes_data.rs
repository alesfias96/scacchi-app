//! Il salvataggio e la lettura degli appunti personali.

use notes::{Note, NoteLink};
use sqlx::Row;
use sqlx::sqlite::SqliteRow;

use crate::database::Database;
use crate::storage_error::StorageError;

/// Scrive (o aggiorna) un appunto.
const UPSERT_NOTE: &str = "
    INSERT INTO notes (id, title, body, tags, fen, game_id, ply, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    ON CONFLICT (id) DO UPDATE SET
        title = excluded.title,
        body = excluded.body,
        tags = excluded.tags,
        fen = excluded.fen,
        game_id = excluded.game_id,
        ply = excluded.ply,
        updated_at = excluded.updated_at";

/// Legge un appunto dato il suo nome unico.
const SELECT_NOTE: &str = "
    SELECT id, title, body, tags, fen, game_id, ply, created_at, updated_at
    FROM notes WHERE id = ?";

/// Legge tutti gli appunti, dal più recente.
const SELECT_ALL_NOTES: &str = "
    SELECT id, title, body, tags, fen, game_id, ply, created_at, updated_at
    FROM notes ORDER BY updated_at DESC, rowid DESC";

/// Un appunto da una riga della tabella.
fn note_from_row(row: &SqliteRow) -> Result<Note, StorageError> {
    let tags: String = row.try_get("tags")?;
    let link = row
        .try_get::<Option<String>, _>("fen")?
        .map(|fen| NoteLink {
            fen,
            game_id: row.try_get("game_id").ok().flatten(),
            ply: row.try_get("ply").ok().flatten(),
        });
    Ok(Note {
        id: row.try_get("id")?,
        title: row.try_get("title")?,
        body: row.try_get("body")?,
        tags: tags.lines().map(str::to_owned).collect(),
        link,
        created_at_ms: row.try_get("created_at")?,
        updated_at_ms: row.try_get("updated_at")?,
    })
}

impl Database {
    /// Salva un appunto (lo crea o lo aggiorna).
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn save_note(&self, note: &Note) -> Result<(), StorageError> {
        let link = note.link.as_ref();
        sqlx::query(UPSERT_NOTE)
            .bind(&note.id)
            .bind(&note.title)
            .bind(&note.body)
            .bind(note.tags.join("\n"))
            .bind(link.map(|link| link.fen.as_str()))
            .bind(link.and_then(|link| link.game_id.as_deref()))
            .bind(link.and_then(|link| link.ply))
            .bind(note.created_at_ms)
            .bind(note.updated_at_ms)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Un appunto dato il suo nome unico, se esiste.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn note_by_id(&self, id: &str) -> Result<Option<Note>, StorageError> {
        let row = sqlx::query(SELECT_NOTE)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        row.as_ref().map(note_from_row).transpose()
    }

    /// Tutti gli appunti, dal più recente.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn all_notes(&self) -> Result<Vec<Note>, StorageError> {
        let rows = sqlx::query(SELECT_ALL_NOTES).fetch_all(&self.pool).await?;
        rows.iter().map(note_from_row).collect()
    }

    /// Elimina un appunto. Se non esiste non fa nulla.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn delete_note(&self, id: &str) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use notes::{Note, NoteLink};

    use crate::database::Database;

    fn sample(id: &str, updated: i64) -> Note {
        Note {
            id: id.to_owned(),
            title: "Idea".to_owned(),
            body: "Spingere f5\ncon calma".to_owned(),
            tags: vec!["Attacco".to_owned(), "Finali di torre".to_owned()],
            link: Some(NoteLink {
                fen: "8/8/8/8/8/8/8/K6k w - - 0 1".to_owned(),
                game_id: Some("partita-1".to_owned()),
                ply: Some(12),
            }),
            created_at_ms: 1,
            updated_at_ms: updated,
        }
    }

    #[tokio::test]
    async fn a_saved_note_comes_back_identical() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let note = sample("a", 5);

        database.save_note(&note).await.expect("salvataggio");

        assert_eq!(database.note_by_id("a").await.expect("lettura"), Some(note));
        assert_eq!(database.note_by_id("nessuno").await.expect("lettura"), None);
    }

    #[tokio::test]
    async fn notes_are_updated_listed_and_deleted() {
        let database = Database::open_in_memory().await.expect("database aperto");
        database
            .save_note(&sample("a", 5))
            .await
            .expect("salvataggio");
        database
            .save_note(&sample("b", 9))
            .await
            .expect("salvataggio");
        let mut changed = sample("a", 20);
        changed.link = None;
        database.save_note(&changed).await.expect("aggiornamento");

        let all = database.all_notes().await.expect("lettura");
        assert_eq!(
            all.iter().map(|note| note.id.as_str()).collect::<Vec<_>>(),
            ["a", "b"]
        );
        assert_eq!(all[0].link, None);

        database.delete_note("a").await.expect("eliminazione");
        assert_eq!(database.all_notes().await.expect("lettura").len(), 1);
    }
}
