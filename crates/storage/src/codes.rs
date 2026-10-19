//! Come i valori della partita vengono scritti come testo nel database.
//!
//! Il database contiene solo testi e numeri: colori, avversario, motore e
//! motivo della fine diventano parole fisse (`"white"`, `"computer"`, `"maia"`,
//! `"checkmate"`…).
//! Queste parole **non vanno mai cambiate**: i database già salvati le usano.

use app_settings::{ComputerEngine, OpponentKind};
use chess_core::{GameEndReason, Side};

/// Il testo di un colore.
#[must_use]
pub fn side_code(side: Side) -> &'static str {
    match side {
        Side::White => "white",
        Side::Black => "black",
    }
}

/// Il colore scritto nel testo, se è riconosciuto.
#[must_use]
pub fn parse_side(code: &str) -> Option<Side> {
    match code {
        "white" => Some(Side::White),
        "black" => Some(Side::Black),
        _ => None,
    }
}

/// Il testo dell'avversario.
#[must_use]
pub fn opponent_code(opponent: OpponentKind) -> &'static str {
    match opponent {
        OpponentKind::Computer => "computer",
        OpponentKind::TwoPlayers => "twoPlayers",
    }
}

/// L'avversario scritto nel testo (se non è riconosciuto: due giocatori).
#[must_use]
pub fn parse_opponent(code: &str) -> OpponentKind {
    if code == "computer" {
        OpponentKind::Computer
    } else {
        OpponentKind::TwoPlayers
    }
}

/// Il testo del motore del computer.
#[must_use]
pub fn engine_code(engine: ComputerEngine) -> &'static str {
    match engine {
        ComputerEngine::Stockfish => "stockfish",
        ComputerEngine::Maia => "maia",
    }
}

/// Il motore scritto nel testo (se non è riconosciuto: Stockfish, come le partite
/// salvate prima che ci fosse Maia).
#[must_use]
pub fn parse_engine(code: &str) -> ComputerEngine {
    if code == "maia" {
        ComputerEngine::Maia
    } else {
        ComputerEngine::Stockfish
    }
}

/// Tutti i motivi di fine partita con il loro testo.
const REASONS: [(GameEndReason, &str); 8] = [
    (GameEndReason::Checkmate, "checkmate"),
    (GameEndReason::Stalemate, "stalemate"),
    (GameEndReason::InsufficientMaterial, "insufficientMaterial"),
    (GameEndReason::ThreefoldRepetition, "threefoldRepetition"),
    (GameEndReason::FiftyMoveRule, "fiftyMoveRule"),
    (GameEndReason::Timeout, "timeout"),
    (
        GameEndReason::TimeoutVsInsufficientMaterial,
        "timeoutVsInsufficientMaterial",
    ),
    (GameEndReason::Resignation, "resignation"),
];

/// Il testo di un motivo di fine partita.
#[must_use]
pub fn reason_code(reason: GameEndReason) -> &'static str {
    REASONS
        .iter()
        .find(|(known, _)| *known == reason)
        .map_or("unknown", |(_, code)| code)
}

/// Il motivo scritto nel testo, se è riconosciuto.
#[must_use]
pub fn parse_reason(code: &str) -> Option<GameEndReason> {
    REASONS
        .iter()
        .find(|(_, known)| *known == code)
        .map(|(reason, _)| *reason)
}

#[cfg(test)]
mod tests {
    use super::{
        REASONS, engine_code, opponent_code, parse_engine, parse_opponent, parse_reason,
        parse_side, reason_code, side_code,
    };
    use app_settings::{ComputerEngine, OpponentKind};
    use chess_core::Side;

    #[test]
    fn every_value_comes_back_from_its_code() {
        for side in [Side::White, Side::Black] {
            assert_eq!(parse_side(side_code(side)), Some(side));
        }
        for opponent in [OpponentKind::Computer, OpponentKind::TwoPlayers] {
            assert_eq!(parse_opponent(opponent_code(opponent)), opponent);
        }
        for engine in [ComputerEngine::Stockfish, ComputerEngine::Maia] {
            assert_eq!(parse_engine(engine_code(engine)), engine);
        }
        for (reason, _) in REASONS {
            assert_eq!(parse_reason(reason_code(reason)), Some(reason));
        }
    }

    #[test]
    fn unknown_codes_are_not_invented() {
        assert_eq!(parse_side("green"), None);
        assert_eq!(parse_reason("boredom"), None);
    }
}
