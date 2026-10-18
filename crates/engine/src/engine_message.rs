//! Lettura delle righe di testo che il motore invia.

use crate::engine_info::EngineInfo;
use crate::engine_option::EngineOption;
use crate::score::Score;

/// Una riga inviata dal motore, già interpretata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineMessage {
    /// `uciok`: il motore ha finito di presentarsi.
    UciOk,
    /// `readyok`: il motore è pronto a ricevere comandi.
    ReadyOk,
    /// `option ...`: un'impostazione disponibile.
    Option(EngineOption),
    /// `info ...`: informazioni sulla ricerca in corso.
    Info(EngineInfo),
    /// `bestmove ...`: la mossa scelta, in UCI; `None` se non ci sono mosse
    /// (il motore scrive `(none)` quando la partita è finita).
    BestMove(Option<String>),
    /// Qualsiasi altra riga (nome del motore, messaggi di servizio, ...): si ignora.
    Other,
}

impl EngineMessage {
    /// Interpreta una riga del motore.
    ///
    /// - Riceve: la riga, senza il carattere di fine riga.
    /// - Restituisce: il messaggio; [`EngineMessage::Other`] per le righe che non servono.
    #[must_use]
    pub fn parse(line: &str) -> Self {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("uciok") => Self::UciOk,
            Some("readyok") => Self::ReadyOk,
            Some("bestmove") => Self::BestMove(
                words
                    .next()
                    .filter(|uci| *uci != "(none)" && *uci != "0000")
                    .map(str::to_owned),
            ),
            Some("info") => parse_info(words).map_or(Self::Other, Self::Info),
            Some("option") => parse_option(words).map_or(Self::Other, Self::Option),
            _ => Self::Other,
        }
    }
}

/// Le parole di una riga `info` che sono seguite da un solo valore e che qui
/// non servono: vanno saltate insieme al loro valore.
const SINGLE_VALUE_KEYS: [&str; 10] = [
    "seldepth",
    "time",
    "nodes",
    "currmove",
    "currmovenumber",
    "hashfull",
    "nps",
    "tbhits",
    "sbhits",
    "cpuload",
];

/// Interpreta una riga `info`. Restituisce `None` per le righe senza
/// profondità (ad esempio `info string ...`, solo testo).
fn parse_info<'a>(mut words: impl Iterator<Item = &'a str>) -> Option<EngineInfo> {
    let mut info = EngineInfo {
        multipv: 1,
        ..EngineInfo::default()
    };
    let mut has_depth = false;
    while let Some(word) = words.next() {
        match word {
            "depth" => {
                info.depth = words.next()?.parse().ok()?;
                has_depth = true;
            }
            "multipv" => info.multipv = words.next()?.parse().ok()?,
            "score" => {
                let kind = words.next()?;
                let value: i32 = words.next()?.parse().ok()?;
                info.score = Some(match kind {
                    "mate" => Score::Mate(value),
                    _ => Score::Centipawns(value),
                });
            }
            "lowerbound" | "upperbound" => info.bound = true,
            "wdl" => {
                // Tre numeri: probabilità di vittoria, patta e sconfitta (non servono).
                words.nth(2)?;
            }
            // Il resto della riga è la variante principale.
            "pv" => info.pv = words.by_ref().map(str::to_owned).collect(),
            // Solo testo libero: non è un'informazione sulla ricerca.
            "string" => return None,
            key if SINGLE_VALUE_KEYS.contains(&key) => {
                words.next();
            }
            _ => {}
        }
    }
    has_depth.then_some(info)
}

/// Interpreta una riga `option name <nome> type <tipo> [min <a>] [max <b>] ...`.
fn parse_option<'a>(mut words: impl Iterator<Item = &'a str>) -> Option<EngineOption> {
    if words.next()? != "name" {
        return None;
    }
    // Il nome può contenere spazi: arriva fino alla parola "type".
    let mut name_parts = Vec::new();
    for word in words.by_ref() {
        if word == "type" {
            break;
        }
        name_parts.push(word);
    }
    let mut option = EngineOption {
        name: name_parts.join(" "),
        min: None,
        max: None,
    };
    while let Some(word) = words.next() {
        match word {
            "min" => option.min = words.next().and_then(|value| value.parse().ok()),
            "max" => option.max = words.next().and_then(|value| value.parse().ok()),
            _ => {}
        }
    }
    Some(option)
}

#[cfg(test)]
mod tests {
    use super::EngineMessage;
    use crate::engine_info::EngineInfo;
    use crate::engine_option::EngineOption;
    use crate::score::Score;

    #[test]
    fn reads_a_full_info_line() {
        let line = "info depth 14 seldepth 25 multipv 1 score cp -31 nodes 103508 nps 1067092 \
                    hashfull 33 tbhits 0 time 97 pv e7e5 g1f3 g8f6";

        assert_eq!(
            EngineMessage::parse(line),
            EngineMessage::Info(EngineInfo {
                depth: 14,
                multipv: 1,
                score: Some(Score::Centipawns(-31)),
                bound: false,
                pv: vec!["e7e5".to_owned(), "g1f3".to_owned(), "g8f6".to_owned()],
            })
        );
    }

    #[test]
    fn reads_a_mate_score_and_a_bound() {
        let EngineMessage::Info(info) =
            EngineMessage::parse("info depth 5 score mate -2 lowerbound wdl 0 0 1000 pv g6g7")
        else {
            panic!("doveva essere una riga info");
        };

        assert_eq!(info.score, Some(Score::Mate(-2)));
        assert!(info.bound);
        assert_eq!(info.pv, ["g6g7"]);
    }

    #[test]
    fn ignores_text_only_info_lines() {
        assert_eq!(
            EngineMessage::parse("info string NNUE evaluation using nn-1a298aa575a0.nnue"),
            EngineMessage::Other
        );
    }

    #[test]
    fn reads_the_best_move_or_its_absence() {
        assert_eq!(
            EngineMessage::parse("bestmove e7e5 ponder g1f3"),
            EngineMessage::BestMove(Some("e7e5".to_owned()))
        );
        assert_eq!(
            EngineMessage::parse("bestmove (none)"),
            EngineMessage::BestMove(None)
        );
    }

    #[test]
    fn reads_an_option_with_spaces_and_limits() {
        assert_eq!(
            EngineMessage::parse("option name Skill Level type spin default 20 min 0 max 20"),
            EngineMessage::Option(EngineOption {
                name: "Skill Level".to_owned(),
                min: Some(0),
                max: Some(20),
            })
        );
    }

    #[test]
    fn recognises_the_handshake_replies() {
        assert_eq!(EngineMessage::parse("uciok"), EngineMessage::UciOk);
        assert_eq!(EngineMessage::parse("readyok"), EngineMessage::ReadyOk);
        assert_eq!(
            EngineMessage::parse("id name Stockfish 19"),
            EngineMessage::Other
        );
    }
}
