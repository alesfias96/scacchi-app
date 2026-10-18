//! Lo stile dei pezzi.

/// Il disegno dei pezzi sulla scacchiera.
///
/// Nel JSON diventa il nome in minuscolo (`"icona"`, `"gettoni"`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum PieceStyle {
    /// Forme geometriche con contorno netto (predefinito).
    #[default]
    Icona,
    /// Le stesse forme senza contorno, con volume e ombra.
    Morbido,
    /// Bosso chiaro ed ebano.
    Legno,
    /// Argento e ottone.
    Metallo,
    /// Dischi con l'iniziale italiana del pezzo.
    Gettoni,
    /// Il set tradizionale in stile Staunton.
    Classico,
}
