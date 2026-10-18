//! Come e perché una partita è finita.

use crate::side::Side;

/// Il motivo per cui la partita è finita.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub enum GameEndReason {
    /// Il re di chi deve muovere è sotto scacco e non ha mosse per salvarsi.
    Checkmate,
    /// Chi deve muovere non ha mosse legali ma non è sotto scacco: patta.
    Stalemate,
    /// Nessuno dei due ha abbastanza pezzi per dare scacco matto: patta.
    InsufficientMaterial,
    /// La stessa posizione si è presentata tre volte: patta.
    ThreefoldRepetition,
    /// Cinquanta mosse per parte senza catture né mosse di pedone: patta.
    FiftyMoveRule,
    /// Un giocatore ha finito il tempo sull'orologio: vince l'avversario.
    Timeout,
    /// Un giocatore ha finito il tempo, ma l'avversario non ha pezzi
    /// sufficienti per dare scacco matto: per regolamento è patta.
    TimeoutVsInsufficientMaterial,
    /// Un giocatore ha abbandonato: vince l'avversario.
    Resignation,
}

/// Il risultato di una partita finita.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct GameResult {
    /// Chi ha vinto; `None` se la partita è patta.
    pub winner: Option<Side>,
    /// Perché la partita è finita.
    pub reason: GameEndReason,
    /// Il risultato scritto come nei libri e nei file PGN: `"1-0"`, `"0-1"` o `"1/2-1/2"`.
    pub score: String,
}

impl GameResult {
    /// Una vittoria di `winner` per il motivo indicato.
    #[must_use]
    pub fn decisive(winner: Side, reason: GameEndReason) -> Self {
        let score = match winner {
            Side::White => "1-0",
            Side::Black => "0-1",
        };
        Self {
            winner: Some(winner),
            reason,
            score: score.to_owned(),
        }
    }

    /// Una patta per il motivo indicato.
    #[must_use]
    pub fn draw(reason: GameEndReason) -> Self {
        Self {
            winner: None,
            reason,
            score: "1/2-1/2".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GameEndReason, GameResult};
    use crate::side::Side;

    #[test]
    fn writes_the_score_like_a_pgn_file() {
        assert_eq!(
            GameResult::decisive(Side::White, GameEndReason::Checkmate).score,
            "1-0"
        );
        assert_eq!(
            GameResult::decisive(Side::Black, GameEndReason::Checkmate).score,
            "0-1"
        );
        assert_eq!(GameResult::draw(GameEndReason::Stalemate).score, "1/2-1/2");
    }
}
