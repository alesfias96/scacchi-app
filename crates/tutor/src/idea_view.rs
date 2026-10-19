//! L'idea di una variante, pronta per l'interfaccia.

use analysis::FactArrow;
use chess_core::{LegalMove, Side};
use engine::Evaluation;

use crate::explanation_view::CardView;

/// Un momento chiave della variante, con la sua spiegazione.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct IdeaMoment {
    /// La mezza mossa della variante dopo cui succede (0 = la prima mossa).
    pub ply: u32,
    /// La frase.
    pub text: String,
    /// Le case da evidenziare.
    pub squares: Vec<String>,
    /// Le frecce da disegnare.
    pub arrows: Vec<FactArrow>,
    /// La scheda della base di conoscenza che approfondisce, se c'è.
    pub card_id: Option<String>,
}

/// Una posizione della variante, da mostrare sulla scacchiera mentre la si guarda.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct IdeaFrame {
    /// La disposizione dei pezzi (la prima parte della FEN).
    pub board_fen: String,
    /// Chi deve muovere.
    pub turn: Side,
    /// `true` se il re di chi muove è sotto scacco.
    pub in_check: bool,
    /// La mossa appena giocata (`None` nella posizione di partenza).
    pub last_move: Option<LegalMove>,
}

/// Un motivo tattico della variante (es. "Deviazione"), con la sua scheda e il tema dei puzzle.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MotifView {
    /// La chiave del motivo (es. `"deflection"`).
    pub key: String,
    /// Il nome in italiano.
    pub name: String,
    /// Il tema dei puzzle di Lichess con questo motivo, se c'è (es. `"deflection"`).
    pub puzzle_theme: Option<String>,
    /// La scheda del motivo (es. `"tactics/deflection"`).
    pub card_id: String,
}

/// L'idea di una variante: cosa vuole ottenere chi la gioca, guardando lontano.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct IdeaView {
    /// Di chi è l'idea.
    pub side: Side,
    /// La frase principale (es. "L'idea: il cavallo va d2–f1–g3–f5… — si vede tra 3 mosse").
    /// Non contiene mosse: la mossa da cui comincia l'idea è `key_ply`.
    pub headline: String,
    /// La mezza mossa della variante in cui l'idea principale si vede (0 = la prima), se c'è.
    pub key_ply: Option<u32>,
    /// La valutazione di Stockfish alla fine della sua analisi, se c'è.
    pub evaluation: Option<Evaluation>,
    /// Le mosse della variante, in notazione algebrica internazionale.
    pub moves: Vec<String>,
    /// Il numero della mossa da cui comincia la variante.
    pub first_move_number: u32,
    /// Chi gioca la prima mossa della variante.
    pub first_side: Side,
    /// I momenti chiave, in ordine di mossa.
    pub moments: Vec<IdeaMoment>,
    /// Le posizioni: quella di partenza e quella dopo ogni mossa della variante.
    pub frames: Vec<IdeaFrame>,
    /// Le schede della base di conoscenza citate dai momenti chiave, pronte da aprire.
    pub cards: Vec<CardView>,
    /// I motivi tattici della variante, nell'ordine in cui compaiono.
    pub motifs: Vec<MotifView>,
}

/// Le idee della posizione: le migliori varianti di Stockfish, ognuna spiegata.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct PositionIdeas {
    /// La posizione spiegata, in notazione FEN.
    pub fen: String,
    /// Le idee, dalla migliore secondo Stockfish.
    pub ideas: Vec<IdeaView>,
    /// Perché la prima idea è preferibile alle altre, se ce ne sono più d'una.
    pub comparison: Option<String>,
    /// I piani della strategia (le loro schede, es. `"middlegame/minority-attack"`) che
    /// la variante migliore di Stockfish conferma.
    pub confirmed_plans: Vec<String>,
}

/// L'idea della mossa appena giocata e, se Stockfish preferiva altro, quella migliore.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MoveIdea {
    /// L'idea della mossa giocata (la variante comincia con la mossa stessa).
    pub played: IdeaView,
    /// L'idea della mossa migliore, se era un'altra e valeva di più.
    pub better: Option<IdeaView>,
    /// Il confronto tra le due, se c'è la migliore (la mossa migliore è la
    /// prima di `better`: l'interfaccia la scrive nella sua notazione).
    pub comparison: Option<String>,
    /// Se c'è la migliore: quale passo del metodo «Prima di muovere» l'avrebbe trovata.
    pub lesson: Option<String>,
}

/// Le strade dell'avversario: cosa potrebbe fare se toccasse a lui.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct OpponentPaths {
    /// Di chi sono le strade (l'avversario di chi muove).
    pub side: Side,
    /// La frase d'introduzione: quanto sono pericolose.
    pub intro: String,
    /// Le strade, dalla più forte secondo Stockfish (frasi al condizionale).
    pub paths: Vec<IdeaView>,
}
