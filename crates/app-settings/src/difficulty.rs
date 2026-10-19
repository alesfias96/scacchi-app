//! Quanto difficili devono essere i puzzle, rispetto al proprio livello.

/// La difficoltà dei puzzle proposti, rispetto al proprio rating dei puzzle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum Difficulty {
    /// Un po' sotto il proprio livello.
    Easier,
    /// Poco sopra il proprio livello: il punto giusto per migliorare (predefinito).
    #[default]
    Normal,
    /// Decisamente sopra il proprio livello.
    Harder,
}
