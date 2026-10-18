//! Lo stile della scacchiera.

/// Il materiale della scacchiera, con la sua cornice.
///
/// Nel JSON diventa il nome in minuscolo (`"rovere"`, `"marmo"`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum BoardStyle {
    /// Legno di noce: scuro e caldo.
    Noce,
    /// Legno d'acero: chiaro e luminoso.
    Acero,
    /// Legno di rovere: tono intermedio, il più classico (predefinito).
    #[default]
    Rovere,
    /// Marmo bianco venato e verde salvia.
    Marmo,
    /// Ardesia grigio-blu opaca.
    Ardesia,
    /// Colori piatti verde e crema, come le scacchiere da gara.
    Torneo,
}
