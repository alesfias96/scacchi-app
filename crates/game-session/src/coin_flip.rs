//! Il "lancio della moneta" per estrarre a sorte il colore.

use std::time::{SystemTime, UNIX_EPOCH};

use chess_core::Side;

/// Estrae a sorte un colore.
///
/// Usa i nanosecondi dell'orologio di sistema: per scegliere il colore di
/// una partita non serve un generatore di numeri casuali "da crittografia".
#[must_use]
pub fn coin_flip() -> Side {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.subsec_nanos());
    if nanos.is_multiple_of(2) {
        Side::White
    } else {
        Side::Black
    }
}
