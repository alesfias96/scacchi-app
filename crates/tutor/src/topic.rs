//! Gli argomenti della base di conoscenza e i "quando vale" delle schede.

/// L'argomento di una scheda (è anche la sua cartella in `knowledge/`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub enum Topic {
    /// Le aperture (cartella `openings/`).
    Opening,
    /// Le strutture pedonali (cartella `structures/`).
    Structure,
    /// I temi del mediogioco (cartella `middlegame/`).
    Middlegame,
    /// I tipi di finale (cartella `endgames/`).
    Endgame,
    /// I principi generali (cartella `principles/`).
    Principle,
    /// I motivi tattici e il metodo di pensiero (cartella `tactics/`).
    Tactic,
}

impl Topic {
    /// Tutti gli argomenti, nell'ordine in cui il tutor li mostra.
    pub const ALL: [Self; 6] = [
        Self::Opening,
        Self::Structure,
        Self::Middlegame,
        Self::Endgame,
        Self::Principle,
        Self::Tactic,
    ];

    /// La parola con cui l'argomento si scrive nelle schede (`"structure:…"`).
    #[must_use]
    pub fn code(self) -> &'static str {
        match self {
            Self::Opening => "opening",
            Self::Structure => "structure",
            Self::Middlegame => "middlegame",
            Self::Endgame => "endgame",
            Self::Principle => "principle",
            Self::Tactic => "tactic",
        }
    }

    /// La cartella delle schede di questo argomento.
    #[must_use]
    pub fn folder(self) -> &'static str {
        match self {
            Self::Opening => "openings",
            Self::Structure => "structures",
            Self::Middlegame => "middlegame",
            Self::Endgame => "endgames",
            Self::Principle => "principles",
            Self::Tactic => "tactics",
        }
    }
}

/// Un "quando vale" di una scheda, ad esempio `structure:isolated-queen-pawn`
/// (vale quando c'è un pedone isolato di donna) oppure `opening:Italian Game`
/// (vale quando l'apertura giocata è la Partita Italiana).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trigger {
    /// L'argomento.
    pub topic: Topic,
    /// La chiave: il nome di una struttura, di un principio, di un finale…
    /// oppure, per le aperture, il nome di Lichess (in inglese).
    pub key: String,
}

impl Trigger {
    /// Legge un "quando vale" scritto come `argomento:chiave`.
    ///
    /// Restituisce `None` se l'argomento non esiste o la chiave è vuota.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let (code, key) = text.split_once(':')?;
        let topic = Topic::ALL
            .into_iter()
            .find(|topic| topic.code() == code.trim())?;
        let key = key.trim();
        (!key.is_empty()).then(|| Self {
            topic,
            key: key.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Topic, Trigger};

    #[test]
    fn reads_a_trigger() {
        assert_eq!(
            Trigger::parse("opening:Queen's Gambit Declined"),
            Some(Trigger {
                topic: Topic::Opening,
                key: "Queen's Gambit Declined".to_owned()
            })
        );
    }

    #[test]
    fn rejects_unknown_topics_and_empty_keys() {
        assert_eq!(Trigger::parse("weather:sunny"), None);
        assert_eq!(Trigger::parse("structure:"), None);
        assert_eq!(Trigger::parse("senza due punti"), None);
    }
}
