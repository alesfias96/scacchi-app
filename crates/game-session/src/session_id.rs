//! Il nome unico di ogni partita e il momento in cui è cominciata.

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Un contatore che rende diversi due nomi creati nello stesso istante.
static SEQUENCE: AtomicU32 = AtomicU32::new(0);

/// I millesimi di secondo trascorsi dal 1° gennaio 1970 (il modo standard
/// con cui i computer scrivono una data e un'ora).
#[must_use]
pub fn now_unix_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
        })
}

/// Un nome unico per una nuova partita, con cui viene salvata nell'archivio.
///
/// È formato dall'istante di creazione (in millesimi di secondo) e da un
/// contatore, scritti in esadecimale: ad esempio `"19a3f2c4b10-0"`.
#[must_use]
pub fn new_session_id() -> String {
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!("{:x}-{sequence:x}", now_unix_ms())
}

#[cfg(test)]
mod tests {
    use super::{new_session_id, now_unix_ms};

    #[test]
    fn two_names_are_never_equal() {
        assert_ne!(new_session_id(), new_session_id());
    }

    #[test]
    fn the_date_is_after_2025() {
        // 1° gennaio 2025, in millesimi di secondo dal 1970.
        assert!(now_unix_ms() > 1_735_689_600_000);
    }
}
