//! Una mossa già giocata nella partita.

use crate::side::Side;

/// Una mossa della partita, con tutto ciò che serve per mostrarla nella lista.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MoveRecord {
    /// Posizione della mossa nella partita, contando ogni mezza mossa: 1 è la prima.
    pub ply: usize,
    /// Il numero della mossa come nei libri (1, 2, 3, ...): bianco e nero condividono lo stesso numero.
    pub move_number: u32,
    /// Chi ha giocato la mossa.
    pub side: Side,
    /// La mossa in notazione algebrica standard internazionale (es. `"Nf3"`, `"O-O"`, `"e8=Q+"`).
    /// L'interfaccia la mostra nella notazione scelta nelle Impostazioni.
    pub san: String,
    /// La mossa in notazione UCI (es. `"g1f3"`).
    pub uci: String,
    /// Casa di partenza (es. `"g1"`).
    pub from: String,
    /// Casa di arrivo (es. `"f3"`; per l'arrocco è la casa d'arrivo del re).
    pub to: String,
    /// Vero se la mossa cattura un pezzo (presa en passant compresa).
    /// L'interfaccia lo usa per scegliere il suono della mossa.
    pub capture: bool,
}
