//! I motivi tattici di una variante: come è fatta una combinazione.
//!
//! I rilevatori (crate `analysis`) vedono i motivi "fermi" in una posizione:
//! forchette, inchiodature, infilate, attacchi di scoperta, sovraccarichi.
//! Qui si riconoscono quelli che si vedono solo **mossa dopo mossa**, lungo
//! la variante rigiocata (vedi `line_walk`), dal punto di vista di chi attacca:
//! - **attrazione**: un sacrificio attira il re o la donna su una casa dove
//!   diventano un bersaglio (la mossa dopo li colpisce);
//! - **deviazione**: un pezzo che difendeva una casa è costretto a spostarsi,
//!   e subito dopo quella casa cade;
//! - **eliminazione del difensore**: si cattura il pezzo che difendeva, poi
//!   si prende ciò che difendeva;
//! - **mossa intermedia**: invece di riprendere subito, prima uno scacco o una
//!   cattura, poi la ripresa;
//! - **scacco doppio** e **scacco di scoperta**;
//! - **matto sull'ultima traversa** e **matto affogato**;
//! - **liberazione di casa**: un pezzo si sposta e un altro usa subito la sua casa;
//! - **promozione**.
//!
//! Ogni regola è verificabile sulla scacchiera: se un caso è dubbio, non si dice niente.

use analysis::wording::piece_phrase;
use shakmaty::{Board, Chess, Color, Piece, Position, Role, Square};

use crate::idea_events::{IdeaEvent, IdeaKind};
use crate::idea_view::MotifView;
use crate::line_walk::LineStep;
use crate::topic::Topic;

/// Un motivo tattico: ha una scheda in `knowledge/tactics/` e, se c'è, un tema dei puzzle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Motif {
    /// Un pezzo attacca due bersagli insieme.
    Fork,
    /// Un pezzo non può muoversi perché scoprirebbe uno più prezioso dietro.
    Pin,
    /// Il pezzo davanti, più prezioso, si sposta e si prende quello dietro.
    Skewer,
    /// Spostando un pezzo si scopre l'attacco di un altro.
    DiscoveredAttack,
    /// Due pezzi danno scacco insieme.
    DoubleCheck,
    /// Un difensore deve proteggere troppe cose.
    Overload,
    /// Un sacrificio attira un pezzo su una casa dove diventa un bersaglio.
    Attraction,
    /// Un difensore è costretto a spostarsi.
    Deflection,
    /// Si cattura il pezzo che difende.
    RemovingDefender,
    /// Prima di riprendere, una mossa più forte.
    Intermezzo,
    /// Il matto sull'ultima traversa.
    BackRank,
    /// Il matto di cavallo con il re circondato dai suoi pezzi.
    SmotheredMate,
    /// Un pezzo libera una casa per un altro.
    Clearance,
    /// Un pedone arriva a promozione.
    Promotion,
}

impl Motif {
    /// Tutti i motivi.
    pub const ALL: [Self; 14] = [
        Self::Fork,
        Self::Pin,
        Self::Skewer,
        Self::DiscoveredAttack,
        Self::DoubleCheck,
        Self::Overload,
        Self::Attraction,
        Self::Deflection,
        Self::RemovingDefender,
        Self::Intermezzo,
        Self::BackRank,
        Self::SmotheredMate,
        Self::Clearance,
        Self::Promotion,
    ];

    /// La chiave della scheda (`tactic:<chiave>`, file `knowledge/tactics/<chiave>.toml`).
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::Fork => "fork",
            Self::Pin => "pin",
            Self::Skewer => "skewer",
            Self::DiscoveredAttack => "discovered-attack",
            Self::DoubleCheck => "double-check",
            Self::Overload => "overload",
            Self::Attraction => "attraction",
            Self::Deflection => "deflection",
            Self::RemovingDefender => "removing-defender",
            Self::Intermezzo => "intermezzo",
            Self::BackRank => "back-rank",
            Self::SmotheredMate => "smothered-mate",
            Self::Clearance => "clearance",
            Self::Promotion => "promotion",
        }
    }

    /// Il nome in italiano.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Fork => "Forchetta",
            Self::Pin => "Inchiodatura",
            Self::Skewer => "Infilata",
            Self::DiscoveredAttack => "Attacco di scoperta",
            Self::DoubleCheck => "Scacco doppio",
            Self::Overload => "Sovraccarico",
            Self::Attraction => "Attrazione",
            Self::Deflection => "Deviazione",
            Self::RemovingDefender => "Eliminazione del difensore",
            Self::Intermezzo => "Mossa intermedia",
            Self::BackRank => "Matto sull'ultima traversa",
            Self::SmotheredMate => "Matto affogato",
            Self::Clearance => "Liberazione di casa",
            Self::Promotion => "Promozione",
        }
    }

    /// Il tema dei puzzle di Lichess con questo motivo, se c'è.
    #[must_use]
    pub fn puzzle_theme(self) -> Option<&'static str> {
        match self {
            Self::Fork => Some("fork"),
            Self::Pin => Some("pin"),
            Self::Skewer => Some("skewer"),
            Self::DiscoveredAttack => Some("discoveredAttack"),
            Self::DoubleCheck => Some("doubleCheck"),
            Self::Attraction => Some("attraction"),
            Self::Deflection => Some("deflection"),
            Self::RemovingDefender => Some("capturingDefender"),
            Self::Intermezzo => Some("intermezzo"),
            Self::BackRank => Some("backRankMate"),
            Self::SmotheredMate => Some("smotheredMate"),
            Self::Clearance => Some("clearance"),
            Self::Promotion => Some("promotion"),
            // Lichess non ha un tema per il sovraccarico.
            Self::Overload => None,
        }
    }

    /// Il nome della scheda (es. `"tactics/fork"`).
    #[must_use]
    pub fn card_id(self) -> String {
        format!("{}/{}", Topic::Tactic.folder(), self.key())
    }

    /// Il motivo di una scheda, dal suo nome.
    #[must_use]
    pub fn from_card_id(card_id: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|motif| motif.card_id() == card_id)
    }
}

/// Un motivo pronto per l'interfaccia.
#[must_use]
pub fn motif_view(motif: Motif) -> MotifView {
    MotifView {
        key: motif.key().to_owned(),
        name: motif.name().to_owned(),
        puzzle_theme: motif.puzzle_theme().map(str::to_owned),
        card_id: motif.card_id(),
    }
}

/// La chiave della scheda del metodo di pensiero («Prima di muovere»).
pub const THINKING_METHOD: &str = "thinking-method";

/// Le chiavi delle schede di tattica (per i test della base di conoscenza).
pub const TACTIC_KEYS: [(Topic, &str); 15] = [
    (Topic::Tactic, "fork"),
    (Topic::Tactic, "pin"),
    (Topic::Tactic, "skewer"),
    (Topic::Tactic, "discovered-attack"),
    (Topic::Tactic, "double-check"),
    (Topic::Tactic, "overload"),
    (Topic::Tactic, "attraction"),
    (Topic::Tactic, "deflection"),
    (Topic::Tactic, "removing-defender"),
    (Topic::Tactic, "intermezzo"),
    (Topic::Tactic, "back-rank"),
    (Topic::Tactic, "smothered-mate"),
    (Topic::Tactic, "clearance"),
    (Topic::Tactic, "promotion"),
    (Topic::Tactic, THINKING_METHOD),
];

/// Il motivo di un fatto dei rilevatori, dalle parole con cui è scritto.
#[must_use]
pub fn fact_motif(text: &str) -> Option<Motif> {
    let lower = text.to_lowercase();
    [
        ("forchetta", Motif::Fork),
        ("inchiod", Motif::Pin),
        ("infilata", Motif::Skewer),
        ("scoperta", Motif::DiscoveredAttack),
        ("sovraccaric", Motif::Overload),
    ]
    .into_iter()
    .find(|(word, _)| lower.contains(word))
    .map(|(_, motif)| motif)
}

/// La traversa vista da `color` (0 = la sua prima traversa, 7 = l'ultima).
fn relative_rank(square: Square, color: Color) -> u32 {
    let rank = u32::from(square.rank());
    if color == Color::White {
        rank
    } else {
        7 - rank
    }
}

/// La posizione prima della mezza mossa `ply`.
fn before<'a>(start: &'a Chess, steps: &'a [LineStep], ply: usize) -> &'a Chess {
    if ply == 0 {
        start
    } else {
        &steps[ply - 1].after
    }
}

/// Un motivo trovato, come evento della variante.
fn motif_event(motif: Motif, ply: usize, text: String, squares: Vec<Square>) -> IdeaEvent {
    IdeaEvent {
        kind: IdeaKind::Tactic,
        ply,
        text,
        squares,
        arrows: Vec::new(),
        card_id: Some(motif.card_id()),
    }
}

/// `true` se la mossa `step` è "forte": cattura, scacco o matto.
fn is_forcing(step: &LineStep) -> bool {
    step.capture.is_some() || step.after.is_check()
}

/// Attrazione: `side` mette un pezzo in `S` (in mossa `ply`), l'avversario lo
/// prende con il re o la donna, e la mossa dopo colpisce quel pezzo (scacco o attacco).
fn attraction(steps: &[LineStep], side: Color, ply: usize) -> Option<IdeaEvent> {
    let (bait, taken, hit) = (steps.get(ply)?, steps.get(ply + 1)?, steps.get(ply + 2)?);
    let lured = taken.capture.is_some()
        && taken.to == bait.to
        && matches!(taken.role, Role::King | Role::Queen);
    let target = taken.to;
    let strikes = hit.after.is_check() || hit.after.board().attacks_from(hit.to).contains(target);
    (bait.mover == side && hit.mover == side && lured && strikes).then(|| {
        let who = if taken.role == Role::King {
            "il re avversario"
        } else {
            "la donna avversaria"
        };
        motif_event(
            Motif::Attraction,
            ply,
            format!(
                "un sacrificio in {target} attira lì {who}: su quella casa diventa un bersaglio"
            ),
            vec![target],
        )
    })
}

/// Il pezzo dell'avversario di `side` in `square` che attaccava (difendeva) `target`.
fn defender_of(board: &Board, square: Square, target: Square, side: Color) -> Option<Piece> {
    board
        .piece_at(square)
        .filter(|piece| piece.color == side.other())
        .filter(|_| board.attacks_from(square).contains(target))
}

/// Deviazione: dopo la mossa di `side`, un pezzo avversario che difendeva la
/// casa `T` si sposta, e subito `side` cattura in `T` (o dà matto).
fn deflection(start: &Chess, steps: &[LineStep], side: Color, ply: usize) -> Option<IdeaEvent> {
    let (lure, moved, strike) = (steps.get(ply)?, steps.get(ply + 1)?, steps.get(ply + 2)?);
    let target = strike.to;
    let gains = strike.capture.is_some() || strike.after.is_checkmate();
    let defender = defender_of(before(start, steps, ply).board(), moved.from, target, side)?;
    let still_defends = moved.after.board().attacks_from(moved.to).contains(target);
    let ok = lure.mover == side
        && strike.mover == side
        && is_forcing(lure)
        && gains
        && moved.to != target
        && lure.to != moved.from
        && !still_defends;
    ok.then(|| {
        motif_event(
            Motif::Deflection,
            ply,
            format!(
                "una deviazione: {} difendeva {target}, ma è costretto a spostarsi e {target} resta scoperta",
                piece_phrase(defender, moved.from)
            ),
            vec![moved.from, target],
        )
    })
}

/// Eliminazione del difensore: `side` cattura il pezzo che difendeva la casa `T`,
/// e due mezze mosse dopo cattura in `T` (o dà matto).
fn removing_defender(
    start: &Chess,
    steps: &[LineStep],
    side: Color,
    ply: usize,
) -> Option<IdeaEvent> {
    let (capture, strike) = (steps.get(ply)?, steps.get(ply + 2)?);
    let target = strike.to;
    let gains = strike.capture.is_some() || strike.after.is_checkmate();
    let defender = defender_of(before(start, steps, ply).board(), capture.to, target, side)?;
    let ok = capture.mover == side
        && strike.mover == side
        && capture.capture.is_some()
        && gains
        && capture.to != target;
    ok.then(|| {
        motif_event(
            Motif::RemovingDefender,
            ply,
            format!(
                "elimina il difensore: prende {}, che proteggeva {target}, e poi {target} non è più difesa",
                piece_phrase(defender, capture.to)
            ),
            vec![capture.to, target],
        )
    })
}

/// Mossa intermedia: l'avversario cattura in `S`; `side`, invece di riprendere
/// subito, gioca uno scacco o una cattura altrove, e poi riprende in `S`.
fn intermezzo(steps: &[LineStep], side: Color, ply: usize) -> Option<IdeaEvent> {
    let (taken, middle) = (steps.get(ply)?, steps.get(ply + 1)?);
    let square = taken.to;
    let retakes_later = steps
        .iter()
        .skip(ply + 3)
        .take(3)
        .any(|step| step.mover == side && step.to == square && step.capture.is_some());
    let ok = taken.mover != side
        && taken.capture.is_some()
        && middle.mover == side
        && middle.to != square
        && is_forcing(middle)
        && retakes_later;
    ok.then(|| {
        let how = if middle.after.is_check() {
            "uno scacco"
        } else {
            "una cattura"
        };
        motif_event(
            Motif::Intermezzo,
            ply + 1,
            format!("una mossa intermedia: invece di riprendere subito in {square}, prima {how}, poi la ripresa"),
            vec![square, middle.to],
        )
    })
}

/// I motivi della mossa `step` stessa: scacco doppio, scacco di scoperta, matti
/// tipici, promozione.
fn move_motifs(step: &LineStep, side: Color, ply: usize) -> Vec<IdeaEvent> {
    let mut found = Vec::new();
    let position = &step.after;
    let checkers = position.checkers();
    if checkers.count() >= 2 {
        found.push(motif_event(
            Motif::DoubleCheck,
            ply,
            "uno scacco doppio: due pezzi danno scacco insieme e il re può solo muoversi"
                .to_owned(),
            checkers.into_iter().collect(),
        ));
    } else if checkers.any() && !checkers.contains(step.to) && !step.is_castle {
        found.push(motif_event(
            Motif::DiscoveredAttack,
            ply,
            "uno scacco di scoperta: il pezzo che si sposta apre la linea a un altro, che dà scacco".to_owned(),
            checkers.into_iter().collect(),
        ));
    }
    let board = position.board();
    let enemy_king = board.king_of(side.other());
    if position.is_checkmate()
        && let Some(king) = enemy_king
    {
        let neighbours = shakmaty::attacks::king_attacks(king);
        let boxed_in = (neighbours & !board.by_color(side.other())).is_empty();
        if step.role == Role::Knight && boxed_in {
            found.push(motif_event(
                Motif::SmotheredMate,
                ply,
                "un matto affogato: il re è circondato dai suoi pezzi e il cavallo lo colpisce"
                    .to_owned(),
                vec![king, step.to],
            ));
        } else if matches!(step.role, Role::Rook | Role::Queen)
            && relative_rank(step.to, side) == 7
            && relative_rank(king, side) == 7
        {
            found.push(motif_event(
                Motif::BackRank,
                ply,
                "un matto sull'ultima traversa: il re è chiuso dietro ai suoi pedoni".to_owned(),
                vec![king, step.to],
            ));
        }
    }
    if step.role == Role::Pawn && relative_rank(step.to, side) == 7 {
        found.push(motif_event(
            Motif::Promotion,
            ply,
            format!("il pedone arriva a promozione in {}", step.to),
            vec![step.to],
        ));
    }
    found
}

/// Liberazione di casa: un pezzo di `side` lascia una casa e, la mossa dopo,
/// un altro suo pezzo la usa per una mossa forte.
fn clearance(steps: &[LineStep], side: Color, ply: usize) -> Option<IdeaEvent> {
    let (leaves, uses) = (steps.get(ply)?, steps.get(ply + 2)?);
    let ok = leaves.mover == side
        && uses.mover == side
        && !leaves.is_castle
        && uses.to == leaves.from
        && uses.from != leaves.to
        && (is_forcing(uses) || uses.after.is_checkmate());
    ok.then(|| {
        motif_event(
            Motif::Clearance,
            ply,
            format!(
                "libera la casa {}: il pezzo si sposta e un altro la usa subito",
                leaves.from
            ),
            vec![leaves.from],
        )
    })
}

/// I motivi tattici di `side` nelle prime `within` mezze mosse della variante
/// (ognuno una volta sola, la prima volta che si vede).
///
/// - `previous`: la mossa giocata subito prima della variante, se si conosce
///   (nei puzzle è la mossa dell'avversario): serve a vedere una mossa
///   intermedia quando la cattura da riprendere è proprio quella.
#[must_use]
pub fn motif_events(
    start: &Chess,
    steps: &[LineStep],
    side: Color,
    within: usize,
    previous: Option<&LineStep>,
) -> Vec<IdeaEvent> {
    let mut found: Vec<IdeaEvent> = Vec::new();
    if let Some(previous) = previous {
        let joined: Vec<LineStep> = std::iter::once(previous.clone())
            .chain(steps.iter().cloned())
            .collect();
        found.extend(intermezzo(&joined, side, 0).map(|event| IdeaEvent {
            ply: event.ply.saturating_sub(1),
            ..event
        }));
    }
    for ply in 0..steps.len().min(within) {
        let step = &steps[ply];
        let mut here: Vec<IdeaEvent> = Vec::new();
        if step.mover == side {
            here.extend(move_motifs(step, side, ply));
            // L'eliminazione del difensore spiega meglio di una deviazione la stessa sequenza.
            here.extend(
                removing_defender(start, steps, side, ply)
                    .or_else(|| deflection(start, steps, side, ply)),
            );
            here.extend(attraction(steps, side, ply));
            here.extend(clearance(steps, side, ply));
        } else {
            here.extend(intermezzo(steps, side, ply));
        }
        for event in here {
            if !found.iter().any(|known| known.card_id == event.card_id) {
                found.push(event);
            }
        }
    }
    found.sort_by_key(|event| event.ply);
    found
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color};

    use super::{Motif, fact_motif, motif_events};
    use crate::line_walk::walk;

    fn motifs(fen: &str, moves: &[&str], side: Color) -> Vec<Option<Motif>> {
        let start: Chess = fen
            .parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione");
        let line: Vec<String> = moves.iter().map(|uci| (*uci).to_owned()).collect();
        let steps = walk(&start, &line);
        motif_events(&start, &steps, side, 10, None)
            .iter()
            .map(|event| event.card_id.as_deref().and_then(Motif::from_card_id))
            .collect()
    }

    #[test]
    fn finds_the_back_rank_mate() {
        let found = motifs("6k1/5ppp/8/8/8/8/8/R5K1 w - - 0 1", &["a1a8"], Color::White);
        assert_eq!(found, [Some(Motif::BackRank)]);
    }

    #[test]
    fn finds_the_smothered_mate() {
        // Il re nero in h8, chiuso dalla torre in g8 e dai pedoni: Cf7 matto.
        let found = motifs("6rk/6pp/8/6N1/8/8/8/6K1 w - - 0 1", &["g5f7"], Color::White);
        assert_eq!(found, [Some(Motif::SmotheredMate)]);
    }

    #[test]
    fn finds_the_removal_of_the_defender() {
        // La torre bianca prende il cavallo in c6 che difendeva e5, poi il cavallo prende e5.
        let found = motifs(
            "4k3/8/2n5/4p3/8/5N2/8/2R1K3 w - - 0 1",
            &["c1c6", "e8d7", "f3e5"],
            Color::White,
        );
        assert!(found.contains(&Some(Motif::RemovingDefender)), "{found:?}");
    }

    #[test]
    fn finds_the_double_check() {
        // Il cavallo in e4 va in f6: scacco del cavallo e, scoperta, della torre in e1.
        let found = motifs("4k3/8/8/8/4N3/8/8/4R1K1 w - - 0 1", &["e4f6"], Color::White);
        assert_eq!(found, [Some(Motif::DoubleCheck)]);
    }

    #[test]
    fn reads_the_motif_of_a_detector_fact() {
        assert_eq!(
            fact_motif("Il Bianco può fare una forchetta"),
            Some(Motif::Fork)
        );
        assert_eq!(fact_motif("Inchiodatura: l'alfiere…"), Some(Motif::Pin));
        assert_eq!(fact_motif("Pedone isolato in d4"), None);
    }
}
