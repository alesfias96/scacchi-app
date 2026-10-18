//! Il database delle partite e la coda dei salvataggi.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use game_session::SessionSnapshot;
use storage::Database;
use tokio::sync::{OnceCell, mpsc};

/// Il database delle partite.
///
/// Si apre la prima volta che serve (all'avvio, per riprendere la partita
/// lasciata a metà) e resta aperto. I salvataggi non si fanno subito: si
/// mettono in una **coda**, che un unico "lavoratore" svuota in ordine (vedi
/// `reactions::save_worker`). Così un salvataggio non blocca mai una mossa e
/// una versione vecchia della partita non può mai sovrascriverne una nuova.
#[derive(Debug)]
pub struct DatabaseState {
    /// Dove sta il file del database.
    path: PathBuf,
    /// Il database, una volta aperto.
    database: OnceCell<Database>,
    /// La coda dei salvataggi.
    saves: mpsc::UnboundedSender<SessionSnapshot>,
    /// La versione della partita messa in coda per ultima (vedi [`DatabaseState::claim_revision`]).
    last_revision: AtomicU64,
}

impl DatabaseState {
    /// Prepara il database (senza aprirlo) e la coda dei salvataggi.
    ///
    /// - Riceve: il percorso del file del database.
    /// - Restituisce: lo stato e l'uscita della coda, da consegnare al lavoratore.
    pub fn new(path: PathBuf) -> (Self, mpsc::UnboundedReceiver<SessionSnapshot>) {
        let (saves, queue) = mpsc::unbounded_channel();
        let state = Self {
            path,
            database: OnceCell::new(),
            saves,
            last_revision: AtomicU64::new(0),
        };
        (state, queue)
    }

    /// Il database, aprendolo se non è ancora aperto.
    ///
    /// # Errors
    ///
    /// Un messaggio leggibile se il file del database non si apre.
    pub async fn database(&self) -> Result<&Database, String> {
        self.database
            .get_or_try_init(|| Database::open(&self.path))
            .await
            .map_err(|error| error.to_string())
    }

    /// Segna come già salvata la versione `revision` della partita.
    ///
    /// Restituisce `true` se è una versione nuova (va salvata), `false` se era
    /// già stata messa in coda: ad esempio scorrendo le mosse la partita non cambia.
    pub fn claim_revision(&self, revision: u64) -> bool {
        self.last_revision.swap(revision, Ordering::Relaxed) != revision
    }

    /// Mette in coda il salvataggio di una partita.
    pub fn queue_save(&self, snapshot: SessionSnapshot) {
        // Se il lavoratore si è fermato (app in chiusura) il salvataggio va perso.
        let _ = self.saves.send(snapshot);
    }
}
