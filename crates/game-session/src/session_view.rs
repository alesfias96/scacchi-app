//! La partita in corso come la mostra l'interfaccia.

use app_settings::{OpponentKind, TimeControl};
use chess_core::GameView;
use openings::Opening;

use crate::clock_view::ClockView;
use crate::computer_view::ComputerView;

/// Tutto ciò che l'interfaccia deve sapere sulla partita in corso: la
/// partita stessa, l'avversario, l'orologio e cosa si può fare adesso.
///
/// Le decisioni ("si può muovere?", "si può ritirare la mossa?") le prende
/// Rust: l'interfaccia si limita a seguirle.
// Clippy sconsiglia più di tre valori sì/no nella stessa struttura, perché di
// solito indicano uno "stato" da descrivere meglio con un elenco di casi.
// Qui invece sono risposte indipendenti a domande diverse dell'interfaccia.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct SessionView {
    /// Il nome unico della partita (con cui è salvata nell'archivio).
    pub id: String,
    /// La partita: posizione mostrata, mosse, risultato.
    pub game: GameView,
    /// L'apertura giocata, se riconosciuta.
    pub opening: Option<Opening>,
    /// Contro chi si gioca.
    pub opponent: OpponentKind,
    /// Il computer avversario, se si gioca contro il computer.
    pub computer: Option<ComputerView>,
    /// Il tempo scelto, se la partita ha l'orologio.
    pub time_control: Option<TimeControl>,
    /// I tempi dell'orologio, se la partita ha l'orologio.
    pub clock: Option<ClockView>,
    /// `true` se la persona può muovere un pezzo adesso.
    pub can_move: bool,
    /// `true` se si può ritirare la mossa.
    pub can_undo: bool,
    /// `true` se si può abbandonare (la partita non è ancora finita).
    pub can_resign: bool,
    /// `true` se si sta aspettando la mossa del computer.
    pub computer_thinking: bool,
}
