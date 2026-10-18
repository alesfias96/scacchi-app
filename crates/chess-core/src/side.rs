//! Il colore dei pezzi: bianco o nero.

/// Uno dei due giocatori.
///
/// Nel JSON inviato all'interfaccia diventa `"white"` o `"black"`:
/// sono gli stessi valori usati dalla scacchiera grafica, così non serve
/// nessuna conversione dall'altra parte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "lowercase"),
    ts(export)
)]
pub enum Side {
    /// Il giocatore con i pezzi bianchi (muove per primo).
    White,
    /// Il giocatore con i pezzi neri.
    Black,
}

impl Side {
    /// L'altro giocatore: il bianco per il nero e viceversa.
    #[must_use]
    pub fn opponent(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl From<Side> for shakmaty::Color {
    /// Converte il nostro colore in quello della libreria `shakmaty`.
    fn from(side: Side) -> Self {
        match side {
            Side::White => Self::White,
            Side::Black => Self::Black,
        }
    }
}

impl From<shakmaty::Color> for Side {
    /// Converte il colore della libreria `shakmaty` nel nostro tipo.
    fn from(color: shakmaty::Color) -> Self {
        match color {
            shakmaty::Color::White => Self::White,
            shakmaty::Color::Black => Self::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Side;

    #[test]
    fn converts_both_colors_from_shakmaty() {
        assert_eq!(Side::from(shakmaty::Color::White), Side::White);
        assert_eq!(Side::from(shakmaty::Color::Black), Side::Black);
        assert_eq!(shakmaty::Color::from(Side::White), shakmaty::Color::White);
    }

    #[test]
    fn the_opponent_is_the_other_color() {
        assert_eq!(Side::White.opponent(), Side::Black);
        assert_eq!(Side::Black.opponent(), Side::White);
    }
}
