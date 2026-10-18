//! Una mossa legale, descritta in modo semplice per l'interfaccia.

use shakmaty::uci::UciMove;

/// Una mossa che il giocatore di turno può fare.
///
/// Le case sono scritte in notazione algebrica (`"e2"`, `"e4"`).
/// L'arrocco è descritto come mossa del re (`"e1"` → `"g1"`), come
/// lo intende chi gioca.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct LegalMove {
    /// Casa di partenza, ad esempio `"e2"`.
    pub from: String,
    /// Casa di arrivo, ad esempio `"e4"`.
    pub to: String,
    /// La mossa in notazione UCI (`"e2e4"`, `"e7e8q"` per una promozione a donna).
    pub uci: String,
}

impl LegalMove {
    /// Crea una [`LegalMove`] da una mossa in formato UCI.
    ///
    /// - Riceve: una mossa UCI prodotta da `shakmaty`.
    /// - Restituisce: `Some(mossa)` per le mosse normali; `None` per i tipi
    ///   di mossa che negli scacchi classici non esistono (pezzi "lanciati"
    ///   sulla scacchiera o la mossa nulla).
    #[must_use]
    pub fn from_uci(uci_move: &UciMove) -> Option<Self> {
        match uci_move {
            UciMove::Normal { from, to, .. } => Some(Self {
                from: from.to_string(),
                to: to.to_string(),
                uci: uci_move.to_string(),
            }),
            UciMove::Put { .. } | UciMove::Null => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LegalMove;
    use shakmaty::uci::UciMove;

    #[test]
    fn describes_a_normal_move() {
        let uci: UciMove = "e2e4".parse().expect("mossa UCI valida");
        let legal_move = LegalMove::from_uci(&uci).expect("una mossa normale va convertita");

        assert_eq!(legal_move.from, "e2");
        assert_eq!(legal_move.to, "e4");
        assert_eq!(legal_move.uci, "e2e4");
    }

    #[test]
    fn keeps_the_promotion_piece_in_the_uci_text() {
        let uci: UciMove = "e7e8q".parse().expect("mossa UCI valida");
        let legal_move = LegalMove::from_uci(&uci).expect("una promozione va convertita");

        assert_eq!(legal_move.uci, "e7e8q");
    }

    #[test]
    fn ignores_the_null_move() {
        assert_eq!(LegalMove::from_uci(&UciMove::Null), None);
    }
}
