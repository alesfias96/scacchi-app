//! Gli errori possibili leggendo o scrivendo il file delle impostazioni.

/// Il motivo per cui non è stato possibile leggere o salvare le impostazioni.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Il file o la sua cartella non sono accessibili (permessi, disco pieno, ...).
    #[error("impossibile accedere al file delle impostazioni: {0}")]
    Io(#[from] std::io::Error),
    /// Il file esiste ma il suo contenuto non è un JSON di impostazioni valido.
    #[error("il file delle impostazioni non è valido: {0}")]
    Format(#[from] serde_json::Error),
}
