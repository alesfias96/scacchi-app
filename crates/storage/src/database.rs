//! L'apertura del database.

use std::path::Path;
use std::str::FromStr;

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use crate::storage_error::StorageError;

/// Il database delle partite: un file SQLite.
///
/// Si apre una volta all'avvio dell'app e si usa da qualsiasi compito: tutte
/// le operazioni sono asincrone (non bloccano mai l'interfaccia). Le singole
/// operazioni (salvare, cercare, …) stanno ognuna nel suo file di questa crate.
#[derive(Debug, Clone)]
pub struct Database {
    /// Le connessioni aperte con il file (un piccolo "parco" riutilizzato).
    pub(crate) pool: SqlitePool,
}

impl Database {
    /// Apre il database nel file indicato, creandolo se non esiste, e porta lo
    /// schema all'ultima versione (applica le migrazioni mancanti).
    ///
    /// # Errors
    ///
    /// - [`StorageError::Database`] se il file non si può aprire o creare;
    /// - [`StorageError::Migration`] se lo schema non si può aggiornare.
    pub async fn open(path: &Path) -> Result<Self, StorageError> {
        if let Some(folder) = path.parent() {
            std::fs::create_dir_all(folder).map_err(sqlx::Error::Io)?;
        }
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            // "WAL": le letture non aspettano le scritture; più sicuro se l'app si chiude di colpo.
            .journal_mode(SqliteJournalMode::Wal)
            .foreign_keys(true);
        let pool = SqlitePoolOptions::new().connect_with(options).await?;
        Self::migrate(pool).await
    }

    /// Apre un database temporaneo, solo in memoria (per i test).
    ///
    /// # Errors
    ///
    /// Gli stessi di [`Database::open`].
    pub async fn open_in_memory() -> Result<Self, StorageError> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")?.foreign_keys(true);
        // In memoria ogni connessione avrebbe un database diverso: ne basta una.
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        Self::migrate(pool).await
    }

    /// Applica le migrazioni della cartella `migrations/` (incluse nell'app).
    async fn migrate(pool: SqlitePool) -> Result<Self, StorageError> {
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}

#[cfg(test)]
mod tests {
    use super::Database;

    #[tokio::test]
    async fn opens_an_empty_database_with_the_schema() {
        let database = Database::open_in_memory().await.expect("database aperto");

        let tables: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('games', 'moves')",
        )
        .fetch_one(&database.pool)
        .await
        .expect("lettura dello schema");

        assert_eq!(tables, 2);
    }

    #[tokio::test]
    async fn creates_the_file_and_its_folder() {
        let folder = std::env::temp_dir().join(format!("scacchi-storage-{}", std::process::id()));
        let path = folder.join("dati").join("partite.sqlite");

        Database::open(&path).await.expect("database aperto");

        assert!(path.exists());
        let _ = std::fs::remove_dir_all(folder);
    }
}
