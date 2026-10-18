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
    }
}
