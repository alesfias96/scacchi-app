//! Un appunto: titolo, testo, etichette e, se c'è, il collegamento a una posizione.

/// La posizione (e, se c'è, la partita e la mossa) a cui l'appunto si riferisce.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, serde::Deserialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct NoteLink {
    /// La posizione, in notazione FEN.
    pub fen: String,
    /// Il nome unico della partita dell'archivio, se l'appunto è nato da una partita.
    pub game_id: Option<String>,
    /// La mezza mossa della partita dopo cui c'è la posizione (0 = partenza).
    pub ply: Option<u32>,
}

/// Un appunto come lo manda l'interfaccia quando lo si scrive.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Deserialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct NoteInput {
    /// Il nome unico; `None` per un appunto nuovo.
    pub id: Option<String>,
    /// Il titolo (può essere vuoto).
    pub title: String,
    /// Il testo.
    pub body: String,
    /// Le etichette, così come le ha scritte la persona.
    pub tags: Vec<String>,
    /// Il collegamento, se c'è.
    pub link: Option<NoteLink>,
}

/// Un appunto salvato.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct Note {
    /// Il nome unico.
    pub id: String,
    /// Il titolo, senza spazi in più (può essere vuoto: l'interfaccia mostra
    /// allora l'inizio del testo).
    pub title: String,
    /// Il testo.
    pub body: String,
    /// Le etichette, ripulite e senza doppioni.
    pub tags: Vec<String>,
    /// Il collegamento, se c'è.
    pub link: Option<NoteLink>,
    /// Quando è stato creato (millesimi di secondo dal 1970).
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub created_at_ms: i64,
    /// Quando è stato cambiato l'ultima volta.
    #[cfg_attr(feature = "bindings", ts(type = "number"))]
    pub updated_at_ms: i64,
}
