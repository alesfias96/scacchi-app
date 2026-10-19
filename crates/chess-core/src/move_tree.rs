//! L'albero delle mosse: la linea principale e le varianti, pronto per l'interfaccia.

use crate::move_record::MoveRecord;

/// Una mossa dell'albero, con le mosse che la seguono.
///
/// Il primo seguito (`children[0]`) è la continuazione principale; gli altri
/// sono varianti, nell'ordine in cui sono state giocate.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MoveNode {
    /// Il numero che identifica la mossa nell'albero (resta lo stesso finché la mossa esiste).
    pub id: u32,
    /// La mossa, con la sua notazione.
    pub record: MoveRecord,
    /// La mossa precedente (`None` per le mosse dalla posizione di partenza).
    pub parent: Option<u32>,
    /// Le mosse che la seguono: prima la continuazione principale, poi le varianti.
    pub children: Vec<u32>,
    /// `true` se la mossa fa parte della linea principale della partita.
    pub mainline: bool,
}

/// L'albero delle mosse della partita.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MoveTree {
    /// Tutte le mosse dell'albero, in ordine di lettura (linea principale e varianti).
    pub nodes: Vec<MoveNode>,
    /// Le prime mosse dalla posizione di partenza: la principale, poi le varianti.
    pub roots: Vec<u32>,
    /// La mossa che porta alla posizione mostrata (`None` = posizione di partenza).
    pub current: Option<u32>,
}
