//! Il giudizio su una mossa: ottima, buona, imprecisione, errore, errore grave.

/// Il giudizio su una mossa.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub enum MoveQuality {
    /// La mossa migliore secondo il motore (o quasi uguale).
    Best,
    /// Una buona mossa: perde poco.
    Good,
    /// Imprecisione (?!): perde almeno 10 punti di probabilità di vittoria.
    Inaccuracy,
    /// Errore (?): perde almeno 20 punti.
    Mistake,
    /// Errore grave (??): perde almeno 30 punti.
    Blunder,
}

/// Da quanti punti di probabilità persi la mossa smette di essere "ottima".
const BEST_LIMIT: f64 = 2.0;
/// I limiti delle imprecisioni, degli errori e degli errori gravi (gli stessi di Lichess).
const INACCURACY_LIMIT: f64 = 10.0;
const MISTAKE_LIMIT: f64 = 20.0;
const BLUNDER_LIMIT: f64 = 30.0;

impl MoveQuality {
    /// Il giudizio di una mossa.
    ///
    /// - Riceve: i punti di probabilità di vittoria persi e se la mossa è
    ///   quella migliore secondo il motore.
    #[must_use]
    pub fn judge(chance_lost: f64, is_best: bool) -> Self {
        match chance_lost {
            _ if is_best || chance_lost < BEST_LIMIT => Self::Best,
            _ if chance_lost < INACCURACY_LIMIT => Self::Good,
            _ if chance_lost < MISTAKE_LIMIT => Self::Inaccuracy,
            _ if chance_lost < BLUNDER_LIMIT => Self::Mistake,
            _ => Self::Blunder,
        }
    }

    /// Il nome in italiano, con la maiuscola.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Best => "Ottima mossa",
            Self::Good => "Buona mossa",
            Self::Inaccuracy => "Imprecisione",
            Self::Mistake => "Errore",
            Self::Blunder => "Errore grave",
        }
    }

    /// `true` per imprecisioni, errori ed errori gravi.
    #[must_use]
    pub fn is_error(self) -> bool {
        self >= Self::Inaccuracy
    }
}

#[cfg(test)]
mod tests {
    use super::MoveQuality;

    #[test]
    fn judges_by_the_chance_lost() {
        assert_eq!(MoveQuality::judge(15.0, true), MoveQuality::Best);
        assert_eq!(MoveQuality::judge(5.0, false), MoveQuality::Good);
        assert_eq!(MoveQuality::judge(12.0, false), MoveQuality::Inaccuracy);
        assert_eq!(MoveQuality::judge(25.0, false), MoveQuality::Mistake);
        assert_eq!(MoveQuality::judge(45.0, false), MoveQuality::Blunder);
        assert!(MoveQuality::Mistake.is_error());
        assert!(!MoveQuality::Good.is_error());
    }
}
