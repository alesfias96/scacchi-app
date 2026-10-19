//! L'ora del calendario, come si salva nel database.

use std::time::{SystemTime, UNIX_EPOCH};

/// L'istante attuale in millesimi di secondo dal 1970.
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
        })
}
