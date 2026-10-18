//! Un aggiornamento dell'analisi in tempo reale, pronto per l'interfaccia.

use chess_core::{LineView, describe_line};

use crate::engine_info::EngineInfo;
use crate::evaluation::Evaluation;
use crate::search_position::SearchPosition;

/// Ciò che l'interfaccia mostra mentre il motore analizza: valutazione,
/// profondità raggiunta e variante migliore scritta come nei libri.
///
/// Ne arriva uno a ogni profondità completata; i primi arrivano in pochi
/// millesimi di secondo, poi sempre più distanziati, man mano che il motore
/// approfondisce.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct AnalysisUpdate {
    /// La profondità raggiunta, in mezze mosse.
    pub depth: u32,
    /// La valutazione, dal punto di vista del Bianco.
    pub evaluation: Evaluation,
    /// La variante migliore, a partire dalla posizione analizzata (la sua FEN
    /// permette all'interfaccia di scartare aggiornamenti di una posizione vecchia).
    pub line: LineView,
}

impl AnalysisUpdate {
    /// Costruisce l'aggiornamento da una riga `info` del motore.
    ///
    /// - Riceve: la posizione analizzata e la riga del motore.
    /// - Restituisce: l'aggiornamento; `None` se la riga non serve (senza
    ///   punteggio, con un punteggio solo provvisorio, senza variante, oppure
    ///   una variante diversa dalla migliore).
    #[must_use]
    pub fn from_info(position: &SearchPosition, info: &EngineInfo) -> Option<Self> {
        let score = info.score?;
        if info.bound || info.pv.is_empty() || info.multipv != 1 {
            return None;
        }
        let line = describe_line(&position.start_fen, &position.moves, &info.pv).ok()?;
        Some(Self {
            depth: info.depth,
            evaluation: Evaluation::from_score(score, line.turn),
            line,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::AnalysisUpdate;
    use crate::engine_info::EngineInfo;
    use crate::evaluation::Evaluation;
    use crate::score::Score;
    use crate::search_position::SearchPosition;

    fn after_e4() -> SearchPosition {
        SearchPosition {
            start_fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned(),
            moves: vec!["e2e4".to_owned()],
        }
    }

    fn info(score: Option<Score>, bound: bool) -> EngineInfo {
        EngineInfo {
            depth: 12,
            multipv: 1,
            score,
            bound,
            pv: vec!["c7c5".to_owned(), "g1f3".to_owned()],
        }
    }

    #[test]
    fn describes_the_line_and_gives_the_score_for_white() {
        let update =
            AnalysisUpdate::from_info(&after_e4(), &info(Some(Score::Centipawns(-25)), false))
                .expect("aggiornamento valido");

        assert_eq!(update.depth, 12);
        assert_eq!(update.evaluation, Evaluation::Centipawns(25));
        assert_eq!(update.line.san, ["c5", "Nf3"]);
    }

    #[test]
    fn skips_lines_without_a_final_score() {
        assert_eq!(
            AnalysisUpdate::from_info(&after_e4(), &info(None, false)),
            None
        );
        assert_eq!(
            AnalysisUpdate::from_info(&after_e4(), &info(Some(Score::Centipawns(10)), true)),
            None
        );
    }
}
