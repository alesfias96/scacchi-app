//! Comandi che riguardano la posizione sulla scacchiera.

use chess_core::PositionView;

/// Restituisce la posizione iniziale di una partita.
///
/// - Nome per l'interfaccia: `initial_position`.
/// - Riceve: niente.
/// - Restituisce: la [`PositionView`] della posizione iniziale.
#[tauri::command]
#[must_use]
pub fn initial_position() -> PositionView {
    chess_core::initial_position()
}
