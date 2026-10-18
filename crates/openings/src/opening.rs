//! Un'apertura degli scacchi.

/// Un'apertura (o una sua variante), come la classificano i libri.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct Opening {
    /// Il codice ECO ("Encyclopaedia of Chess Openings"): una lettera da A a E
    /// e due cifre, ad esempio `"B90"`.
    pub eco: String,
    /// Il nome, in inglese come nell'elenco di Lichess (es. `"Sicilian Defense: Najdorf Variation"`).
    pub name: String,
}
