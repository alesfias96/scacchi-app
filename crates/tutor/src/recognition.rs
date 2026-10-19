//! Ciò che il tutor riconosce nella posizione e che rimanda a una scheda.

use shakmaty::{Chess, Color, Position, Square};

use analysis::GamePhase;

use crate::topic::Topic;
use crate::{endgame_kind, middlegame_themes, principles, structures};

/// Una cosa riconosciuta nella posizione: una struttura pedonale, un tipo di
/// finale, un tema di mediogioco o un principio da ricordare.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recognition {
    /// L'argomento della scheda che spiega cosa è stato riconosciuto.
    pub topic: Topic,
    /// La chiave della scheda (es. `"isolated-queen-pawn"`).
    pub key: &'static str,
    /// Chi "ha" la cosa riconosciuta (es. chi ha il pedone isolato), se vale per uno solo.
    pub owner: Option<Color>,
    /// La frase che descrive la posizione concreta (es. "Il Bianco ha un pedone isolato in d4").
    pub text: String,
    /// Le case da evidenziare.
    pub squares: Vec<Square>,
}

/// Tutte le chiavi che il tutor sa riconoscere: ognuna deve avere la sua
/// scheda in `knowledge/` (lo controllano i test della base di conoscenza).
pub const KNOWN_KEYS: &[(Topic, &str)] = &[
    (Topic::Structure, structures::ISOLATED_QUEEN_PAWN),
    (Topic::Structure, structures::HANGING_PAWNS),
    (Topic::Structure, structures::CARLSBAD),
    (Topic::Structure, structures::MAROCZY_BIND),
    (Topic::Structure, structures::STONEWALL),
    (Topic::Structure, structures::FRENCH_CHAIN),
    (Topic::Structure, structures::KINGS_INDIAN_CHAIN),
    (Topic::Structure, structures::BENONI),
    (Topic::Structure, structures::OPEN_CENTER),
    (Topic::Endgame, endgame_kind::PAWN_ENDGAME),
    (Topic::Endgame, endgame_kind::ROOK_ENDGAME),
    (Topic::Endgame, endgame_kind::OPPOSITE_BISHOPS),
    (Topic::Endgame, endgame_kind::SAME_COLOR_BISHOPS),
    (Topic::Endgame, endgame_kind::KNIGHT_ENDGAME),
    (Topic::Endgame, endgame_kind::BISHOP_VS_KNIGHT),
    (Topic::Endgame, endgame_kind::QUEEN_ENDGAME),
    (Topic::Endgame, endgame_kind::ROOK_VS_MINOR),
    (Topic::Endgame, endgame_kind::MATE_WITH_QUEEN),
    (Topic::Endgame, endgame_kind::MATE_WITH_ROOK),
    (Topic::Middlegame, middlegame_themes::OPPOSITE_CASTLING),
    (Topic::Middlegame, middlegame_themes::OUTPOST),
    (Topic::Middlegame, middlegame_themes::SEVENTH_RANK),
    (Topic::Principle, principles::DEVELOPMENT),
    (Topic::Principle, principles::CASTLE_KING),
    (Topic::Principle, principles::CENTER),
    (Topic::Principle, principles::OPEN_FILE),
    (Topic::Principle, principles::BISHOP_PAIR),
    (Topic::Principle, principles::KING_ACTIVITY),
    (Topic::Principle, principles::PASSED_PAWN),
    (Topic::Principle, principles::TRADE_WHEN_AHEAD),
];

/// Tutto ciò che il tutor riconosce nella posizione, argomento per argomento.
#[must_use]
pub fn recognize(position: &Chess, phase: GamePhase) -> Vec<Recognition> {
    let mut found = structures::recognize_structures(position.board());
    found.extend(endgame_kind::recognize_endgame(position.board()));
    found.extend(middlegame_themes::recognize_themes(position, phase));
    found.extend(principles::recognize_principles(position, phase));
    found
}
