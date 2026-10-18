//! Gli errori possibili nel database.

/// Il motivo per cui un'operazione sul database non è riuscita.
///
/// Ogni variante contiene un messaggio leggibile da mostrare all'utente.
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    /// Il database non si apre, non si legge o non si scrive.
    #[error("errore del database delle partite: {0}")]
    Database(#[from] sqlx::Error),
    /// Lo schema del database non si è potuto aggiornare.
    #[error("non riesco ad aggiornare il database delle partite: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    /// Una partita salvata contiene un valore che non ha senso (file rovinato).
    #[error("la partita salvata «{0}» è rovinata")]
    Corrupted(String),
}
