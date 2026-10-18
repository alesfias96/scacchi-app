//! Una riga di informazioni inviata dal motore mentre pensa.

use crate::score::Score;

/// Ciò che il motore racconta mentre pensa (riga `info ...` del protocollo UCI).
///
/// Il motore approfondisce la ricerca un livello ("profondità") alla volta e,
/// a ogni livello completato, comunica il punteggio e la sequenza di mosse
/// migliore che ha trovato (la "variante principale").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EngineInfo {
    /// La profondità raggiunta, in mezze mosse.
    pub depth: u32,
    /// Il numero della variante (1 = la migliore), se il motore ne calcola più d'una.
    pub multipv: u32,
    /// Il punteggio, se la riga lo contiene.
    pub score: Option<Score>,
    /// `true` se il punteggio è solo un limite provvisorio (`lowerbound` o
    /// `upperbound`): conviene aspettare la riga successiva.
    pub bound: bool,
    /// La variante principale, in notazione UCI (es. `["e2e4", "e7e5"]`).
    pub pv: Vec<String>,
}
