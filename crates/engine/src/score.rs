//! Il punteggio di una posizione, così come lo comunica il motore.

/// Il giudizio del motore su una posizione, dal punto di vista di **chi deve muovere**
/// (è la convenzione del protocollo UCI).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Score {
    /// Vantaggio in centesimi di pedone ("centipedoni"): +100 vale circa un pedone in più.
    Centipawns(i32),
    /// Scacco matto in tante mosse quante indicate: positivo se chi muove dà
    /// il matto, negativo se lo subisce.
    Mate(i32),
}
