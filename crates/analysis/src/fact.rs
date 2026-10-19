//! Un fatto trovato nella posizione.

use chess_core::Side;
use shakmaty::{Color, Square};

/// Il gruppo a cui appartiene un fatto (diventa un titolo nella scheda "Osservazioni").
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub enum FactCategory {
    /// Matti in una mossa e pezzi in presa.
    Threats,
    /// Forchette, inchiodature, infilate, attacchi di scoperta, sovraccarico.
    Tactics,
    /// La struttura dei pedoni.
    Pawns,
    /// La sicurezza dei re.
    Kings,
    /// L'attività dei pezzi.
    Activity,
    /// Il materiale.
    Material,
}

/// Una freccia da disegnare sulla scacchiera (es. dal pezzo che attacca a quello attaccato).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct FactArrow {
    /// La casa di partenza (es. `"e5"`).
    pub from: String,
    /// La casa di arrivo.
    pub to: String,
}

/// Un fatto verificato sulla posizione, pronto da mostrare.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct Fact {
    /// Il gruppo del fatto.
    pub category: FactCategory,
    /// Il giocatore a cui il fatto si riferisce: chi ne approfitta (una
    /// forchetta, una minaccia) o chi ne ha il problema (un pedone isolato).
    pub side: Side,
    /// La frase in italiano, ad esempio "Il cavallo in e5 attacca la donna e la torre".
    pub text: String,
    /// Le case da evidenziare con un cerchio.
    pub squares: Vec<String>,
    /// Le frecce da disegnare.
    pub arrows: Vec<FactArrow>,
    /// Le mosse citate dal fatto, in notazione algebrica internazionale (es.
    /// `["Qh4#"]`). L'interfaccia le scrive dopo la frase, nella notazione
    /// scelta nelle Impostazioni.
    pub moves: Vec<String>,
}

impl Fact {
    /// Un fatto senza case né frecce.
    #[must_use]
    pub fn new(category: FactCategory, color: Color, text: String) -> Self {
        Self {
            category,
            side: Side::from(color),
            text,
            squares: Vec::new(),
            arrows: Vec::new(),
            moves: Vec::new(),
        }
    }

    /// Aggiunge una mossa citata (in notazione algebrica internazionale).
    #[must_use]
    pub fn with_move(mut self, san: String) -> Self {
        self.moves.push(san);
        self
    }

    /// Aggiunge le case da evidenziare.
    #[must_use]
    pub fn with_squares(mut self, squares: impl IntoIterator<Item = Square>) -> Self {
        self.squares
            .extend(squares.into_iter().map(|square| square.to_string()));
        self
    }

    /// Aggiunge una freccia.
    #[must_use]
    pub fn with_arrow(mut self, from: Square, to: Square) -> Self {
        self.arrows.push(FactArrow {
            from: from.to_string(),
            to: to.to_string(),
        });
        self
    }
}
