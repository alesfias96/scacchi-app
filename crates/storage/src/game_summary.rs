//! Una partita come compare nell'elenco dell'archivio.

use app_settings::{OpponentKind, TimeControl};
use chess_core::{GameResult, Side};
use openings::Opening;

/// Il riassunto di una partita salvata: quanto basta per mostrarla in una
/// riga dell'archivio (le mosse si leggono solo quando la si apre).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct GameSummary {
    /// Il nome unico della partita.
    pub id: String,
    /// Quando è cominciata, in millesimi di secondo dal 1970.
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub started_at_ms: i64,
    /// Contro chi si è giocato.
    pub opponent: OpponentKind,
    /// Il colore del computer, se si è giocato contro il computer.
    pub computer_side: Option<Side>,
    /// Il livello del computer (1–10), se si è giocato contro il computer.
    pub level: Option<u8>,
    /// Il tempo di riflessione, se la partita aveva l'orologio.
    pub time_control: Option<TimeControl>,
    /// Il risultato; `None` se la partita non è finita.
    pub result: Option<GameResult>,
    /// L'apertura riconosciuta.
    pub opening: Option<Opening>,
    /// Quante mezze mosse sono state giocate.
    pub ply_count: u32,
    /// `true` se la partita è già stata analizzata (Fase 8).
    pub analyzed: bool,
}
