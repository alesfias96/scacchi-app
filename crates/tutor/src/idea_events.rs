//! Cosa succede lungo una variante, dal punto di vista di chi la gioca.
//!
//! Ogni funzione di questo file cerca un tipo di "evento" strategico o
//! tattico nella variante rigiocata (vedi `line_walk`):
//! - **percorsi dei pezzi**: un pezzo che con una o poche mosse arriva su una
//!   casa importante (un avamposto, una colonna aperta, la settima traversa,
//!   vicino al re avversario);
//! - **rotture di pedone**: una spinta che attacca i pedoni avversari e apre il gioco;
//! - **colonne aperte**: una colonna che si apre e viene presa da una torre o dalla donna;
//! - **materiale**: cosa si vince (o si dà) davvero, pezzo per pezzo;
//! - **tattiche**: i motivi tattici veri (forchette, inchiodature, infilate,
//!   attacchi di scoperta, sovraccarichi, minacce di matto) che compaiono dopo
//!   le mosse di chi gioca. Un pezzo "in presa" a metà di uno scambio non conta;
//! - **strutture**: strutture, temi e finali riconosciuti alla fine ma non all'inizio;
//! - **debolezze**: pedoni avversari che diventano isolati, doppiati o
//!   arretrati, e pedoni passati propri.
//!
//! Ogni evento ha un **peso** (quanto conta per capire l'idea) e la **mezza
//! mossa** della variante in cui si vede.

use std::collections::HashSet;

use analysis::game_phase::game_phase;
use analysis::material::material;
use analysis::wording::{piece_phrase_capitalized, role_name, side_name, side_name_capitalized};
use analysis::{Fact, FactCategory, analyze};
use shakmaty::fen::Fen;
use shakmaty::{Bitboard, Board, Chess, Color, EnPassantMode, File, Piece, Position, Role, Square};

use crate::line_walk::LineStep;
use crate::recognition::recognize;
use crate::topic::Topic;

/// Che tipo di evento è (decide anche il peso, vedi [`IdeaKind::weight`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdeaKind {
    /// La variante finisce con il matto.
    Mate,
    /// Un motivo tattico che porta a guadagnare materiale.
    Combination,
    /// Si guadagna materiale.
    WinsMaterial,
    /// Si dà materiale per qualcos'altro (attacco, iniziativa, posizione).
    Sacrifice,
    /// Un motivo tattico (forchetta, inchiodatura, …).
    Tactic,
    /// Un pedone passato nuovo, o una debolezza nuova dei pedoni avversari.
    Weakness,
    /// Una struttura, un tema o un finale che prima non c'era.
    Structure,
    /// Un pezzo che con una manovra arriva su una casa importante.
    Route,
    /// Una rottura di pedone.
    PawnBreak,
    /// Una colonna che si apre per le torri.
    OpenFile,
    /// Il piano tranquillo: arroccare, sviluppare, portare le torri al centro.
    QuietPlan,
}

impl IdeaKind {
    /// Quanto conta l'evento per spiegare l'idea della variante.
    #[must_use]
    pub fn weight(self) -> u32 {
        match self {
            Self::Mate => 100,
            Self::Combination => 90,
            Self::WinsMaterial => 80,
            Self::Sacrifice => 75,
            Self::Tactic => 65,
            Self::Weakness => 55,
            Self::Structure => 50,
            Self::Route => 45,
            Self::PawnBreak => 40,
            Self::OpenFile => 35,
            Self::QuietPlan => 20,
        }
    }
}

/// Un evento della variante.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdeaEvent {
    /// Il tipo.
    pub kind: IdeaKind,
    /// La mezza mossa della variante dopo cui si vede (0 = la prima mossa).
    pub ply: usize,
    /// La frase.
    pub text: String,
    /// Le case da evidenziare.
    pub squares: Vec<Square>,
    /// Le frecce da disegnare (da, a).
    pub arrows: Vec<(Square, Square)>,
    /// La scheda della base di conoscenza che approfondisce, se c'è.
    pub card_id: Option<String>,
}

impl IdeaEvent {
    /// Un evento con la sola frase.
    fn plain(kind: IdeaKind, ply: usize, text: String) -> Self {
        Self {
            kind,
            ply,
            text,
            squares: Vec::new(),
            arrows: Vec::new(),
            card_id: None,
        }
    }
}

/// Il massimo di mosse di un percorso: oltre, il pezzo "vaga" e non è una manovra.
const MAX_ROUTE_MOVES: usize = 4;
/// Le parole con cui i rilevatori descrivono una debolezza dei pedoni.
const WEAK_PAWN_WORDS: [&str; 3] = ["isolato", "doppiat", "arretrato"];
/// La parola con cui i rilevatori descrivono un pedone passato.
const PASSED_PAWN_WORD: &str = "passato";
/// Le parole con cui i rilevatori descrivono una minaccia di matto.
const MATE_WORD: &str = "matto";

/// La traversa vista da `color` (0 = la sua prima traversa, 7 = l'ultima).
fn relative_rank(square: Square, color: Color) -> u32 {
    let rank = u32::from(square.rank());
    if color == Color::White {
        rank
    } else {
        7 - rank
    }
}

/// I pedoni avversari che potrebbero, avanzando, attaccare `square`.
fn enemy_pawns_that_can_chase(board: &Board, square: Square, color: Color) -> Bitboard {
    let enemy = board.by_color(color.other()) & board.by_role(Role::Pawn);
    let file = u32::from(square.file());
    let rank = relative_rank(square, color);
    enemy
        .into_iter()
        .filter(|pawn| {
            u32::from(pawn.file()).abs_diff(file) == 1 && relative_rank(*pawn, color) > rank
        })
        .collect()
}

/// `true` se `square` è un avamposto per `color`: nella metà avversaria,
/// difeso da un proprio pedone e senza pedoni avversari che possano cacciarlo.
fn is_outpost(board: &Board, square: Square, color: Color) -> bool {
    if !(3..=5).contains(&relative_rank(square, color)) {
        return false;
    }
    let own_pawns = board.by_color(color) & board.by_role(Role::Pawn);
    let defended = !(shakmaty::attacks::pawn_attacks(color.other(), square) & own_pawns).is_empty();
    defended && enemy_pawns_that_can_chase(board, square, color).is_empty()
}

/// `true` se sulla colonna non ci sono pedoni di `color`.
fn file_has_no_pawns_of(board: &Board, file: File, color: Color) -> bool {
    (board.by_color(color) & board.by_role(Role::Pawn) & Bitboard::from_file(file)).is_empty()
}

/// Perché la casa d'arrivo di un pezzo è importante, se lo è. `moves` è
/// quante mosse ha fatto il pezzo: avvicinarsi al re conta solo con una manovra.
fn why_square_matters(
    board: &Board,
    role: Role,
    square: Square,
    color: Color,
    moves: usize,
) -> Option<String> {
    match role {
        Role::Knight | Role::Bishop if is_outpost(board, square, color) => Some(format!(
            "in {square} è su un avamposto, difeso da un pedone, e nessun pedone avversario può cacciarlo"
        )),
        Role::Rook | Role::Queen if relative_rank(square, color) == 6 => {
            Some("arriva sulla settima traversa, dove attacca i pedoni e chiude il re".to_owned())
        }
        Role::Rook if file_has_no_pawns_of(board, square.file(), color) => Some(format!(
            "si piazza sulla colonna {} senza pedoni propri, da cui entra nella posizione avversaria",
            square.file().char()
        )),
        Role::Knight | Role::Bishop | Role::Queen if moves >= 2 => board
            .king_of(color.other())
            .filter(|king| king.distance(square) <= 2)
            .map(|_| "si avvicina al re avversario per l'attacco".to_owned()),
        _ => None,
    }
}

/// Le manovre: pezzi di `side` che, con al massimo [`MAX_ROUTE_MOVES`] mosse e
/// senza tornare indietro, arrivano su una casa importante.
#[must_use]
pub fn route_events(steps: &[LineStep], side: Color) -> Vec<IdeaEvent> {
    // Ogni percorso: il pezzo, le case toccate, l'ultima mezza mossa, se è ancora valido.
    let mut routes: Vec<(Role, Vec<Square>, usize, bool)> = Vec::new();
    for (ply, step) in steps.iter().enumerate() {
        if step.mover != side {
            // Un pezzo catturato non ha più un percorso.
            routes.retain(|(_, squares, _, _)| squares.last() != Some(&step.to));
            continue;
        }
        if step.is_castle || step.role == Role::Pawn || step.role == Role::King {
            continue;
        }
        match routes
            .iter_mut()
            .find(|(_, squares, _, _)| squares.last() == Some(&step.from))
        {
            Some(route) => {
                // Tornare su una casa già toccata, o girare a lungo, non è una manovra.
                if route.1.contains(&step.to) || route.1.len() > MAX_ROUTE_MOVES {
                    route.3 = false;
                }
                route.1.push(step.to);
                route.2 = ply;
            }
            None => routes.push((step.role, vec![step.from, step.to], ply, true)),
        }
    }
    routes
        .into_iter()
        .filter(|(_, _, _, valid)| *valid)
        .filter_map(|(role, squares, ply, _)| {
            let last = *squares.last()?;
            let moves = squares.len() - 1;
            let reason = why_square_matters(steps[ply].after.board(), role, last, side, moves)?;
            let path = squares
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("–");
            Some(IdeaEvent {
                kind: IdeaKind::Route,
                ply,
                text: format!(
                    "{} va {path}: {reason}",
                    piece_phrase_capitalized(Piece { color: side, role }, squares[0])
                ),
                arrows: squares.windows(2).map(|pair| (pair[0], pair[1])).collect(),
                squares: vec![last],
                card_id: is_outpost(steps[ply].after.board(), last, side)
                    .then(|| "middlegame/outpost".to_owned()),
            })
        })
        .collect()
}

/// Le rotture: spinte di pedone di `side` che toccano (attaccano o catturano)
/// un pedone avversario.
#[must_use]
pub fn pawn_break_events(steps: &[LineStep], side: Color) -> Vec<IdeaEvent> {
    steps
        .iter()
        .enumerate()
        .filter(|(_, step)| step.mover == side && step.role == Role::Pawn)
        .filter_map(|(ply, step)| {
            let board = step.after.board();
            let enemy_pawns = board.by_color(side.other()) & board.by_role(Role::Pawn);
            let attacks = shakmaty::attacks::pawn_attacks(side, step.to) & enemy_pawns;
            let touches = step.capture == Some(Role::Pawn) || !attacks.is_empty();
            let text = if step.capture.is_some() {
                "i pedoni si cambiano e il gioco si apre".to_owned()
            } else {
                "la spinta di pedone attacca i pedoni avversari e prepara l'apertura del gioco"
                    .to_owned()
            };
            touches.then(|| IdeaEvent {
                kind: IdeaKind::PawnBreak,
                ply,
                text,
                squares: attacks.into_iter().collect(),
                arrows: vec![(step.from, step.to)],
                card_id: None,
            })
        })
        .take(2)
        .collect()
}

/// Le colonne che alla fine della variante sono senza pedoni di `side` e
/// occupate da una sua torre o dalla donna, ma che all'inizio avevano un suo pedone.
#[must_use]
pub fn open_file_events(start: &Chess, steps: &[LineStep], side: Color) -> Vec<IdeaEvent> {
    let Some(last) = steps.last() else {
        return Vec::new();
    };
    let board = last.after.board();
    let heavy = board.by_color(side) & (board.by_role(Role::Rook) | board.by_role(Role::Queen));
    File::ALL
        .into_iter()
        .filter(|file| {
            !file_has_no_pawns_of(start.board(), *file, side)
                && file_has_no_pawns_of(board, *file, side)
                && !(heavy & Bitboard::from_file(*file)).is_empty()
        })
        .map(|file| IdeaEvent {
            kind: IdeaKind::OpenFile,
            ply: steps.len() - 1,
            text: format!(
                "si apre la colonna {} e le torri la occupano: da lì entrano nella posizione avversaria",
                file.char()
            ),
            squares: (heavy & Bitboard::from_file(file)).into_iter().collect(),
            arrows: Vec::new(),
            card_id: Some("principles/open-file".to_owned()),
        })
        .collect()
}

/// I pezzi di `color` sulla scacchiera, per tipo (pedoni, cavalli, alfieri, torri, donne).
fn pieces(board: &Board, color: Color) -> [i32; 5] {
    let count =
        |role| i32::try_from((board.by_color(color) & board.by_role(role)).count()).unwrap_or(0);
    [
        count(Role::Pawn),
        count(Role::Knight),
        count(Role::Bishop),
        count(Role::Rook),
        count(Role::Queen),
    ]
}

/// Il vantaggio di materiale di `side`, in pedoni.
fn advantage(board: &Board, side: Color) -> i32 {
    let balance = material(board).advantage;
    if side == Color::White {
        balance
    } else {
        -balance
    }
}

/// Il materiale "assestato" alla fine. Se l'ultima mossa è una cattura di
/// `side` e il suo pezzo può essere ripreso, lo scambio non è finito: si guarda
/// la posizione prima. Se invece l'ultima cattura è dell'avversario (una
/// ripresa), lo scambio è chiuso e la posizione finale vale.
fn settled_board(start: &Chess, steps: &[LineStep], side: Color) -> Board {
    let Some((last, rest)) = steps.split_last() else {
        return start.board().clone();
    };
    let board = last.after.board();
    let unfinished = last.mover == side
        && last.capture.is_some()
        && !board
            .attacks_to(last.to, side.other(), board.occupied())
            .is_empty();
    if unfinished {
        rest.last()
            .map_or_else(|| start.board().clone(), |step| step.after.board().clone())
    } else {
        board.clone()
    }
}

/// I pezzi persi da ciascuno, tolti quelli scambiati alla pari (un cavallo per
/// un alfiere vale come uno scambio di pezzi minori).
fn net_losses(start: &Board, end: &Board, side: Color) -> ([i32; 5], [i32; 5]) {
    let lost = |color| {
        let before = pieces(start, color);
        let after = pieces(end, color);
        let mut lost = [0; 5];
        for index in 0..5 {
            lost[index] = (before[index] - after[index]).max(0);
        }
        lost
    };
    let (mut own, mut theirs) = (lost(side), lost(side.other()));
    for index in 0..5 {
        let traded = own[index].min(theirs[index]);
        own[index] -= traded;
        theirs[index] -= traded;
    }
    // Pezzi minori: un cavallo per un alfiere è uno scambio alla pari.
    let own_minor = own[1] + own[2];
    let their_minor = theirs[1] + theirs[2];
    let traded = own_minor.min(their_minor);
    for (list, mut remaining) in [(&mut own, traded), (&mut theirs, traded)] {
        for index in [1, 2] {
            let take = list[index].min(remaining);
            list[index] -= take;
            remaining -= take;
        }
    }
    (own, theirs)
}

/// I pezzi di un elenco a parole: "un cavallo e due pedoni".
fn describe_pieces(counts: [i32; 5]) -> String {
    let names = [
        Role::Pawn,
        Role::Knight,
        Role::Bishop,
        Role::Rook,
        Role::Queen,
    ];
    let mut parts: Vec<String> = Vec::new();
    // Dal pezzo più importante al pedone.
    for index in (0..5).rev() {
        let count = counts[index];
        if count <= 0 {
            continue;
        }
        let role = names[index];
        let name = role_name(role);
        parts.push(match (count, role) {
            (1, Role::Rook | Role::Queen) => format!("una {name}"),
            (1, _) => format!("un {name}"),
            (n, Role::Pawn) => format!("{n} pedoni"),
            (n, Role::Knight) => format!("{n} cavalli"),
            (n, Role::Bishop) => format!("{n} alfieri"),
            (n, Role::Rook) => format!("{n} torri"),
            (n, _) => format!("{n} donne"),
        });
    }
    analysis::wording::join_list(&parts)
}

/// Il bilancio del materiale per `side` alla fine della variante.
///
/// `good_for_side` dice se, secondo il motore, la variante è buona per lui:
/// materiale in meno con una variante buona è un **sacrificio**.
#[must_use]
pub fn material_event(
    start: &Chess,
    steps: &[LineStep],
    side: Color,
    good_for_side: bool,
) -> Option<IdeaEvent> {
    let end = settled_board(start, steps, side);
    let gained = advantage(&end, side) - advantage(start.board(), side);
    let (own, theirs) = net_losses(start.board(), &end, side);
    let ply = steps
        .iter()
        .rposition(|step| step.capture.is_some())
        .unwrap_or(0);
    // La qualità: una torre presa in cambio di un pezzo minore.
    let exchange = theirs[3] == 1 && own[1] + own[2] == 1 && own[3] == 0;
    if gained >= 1 {
        let won = if exchange {
            "la qualità (una torre per un pezzo minore)".to_owned()
        } else if own.iter().any(|count| *count > 0) {
            format!(
                "{} in cambio di {}",
                describe_pieces(theirs),
                describe_pieces(own)
            )
        } else {
            describe_pieces(theirs)
        };
        Some(IdeaEvent {
            card_id: Some("principles/trade-when-ahead".to_owned()),
            ..IdeaEvent::plain(
                IdeaKind::WinsMaterial,
                ply,
                format!("alla fine {} vince {won}", side_name(side)),
            )
        })
    } else if gained <= -1 && good_for_side {
        Some(IdeaEvent::plain(
            IdeaKind::Sacrifice,
            ply,
            format!(
                "{} sacrifica {}, ma in cambio ottiene qualcosa che vale di più: attacco, iniziativa o una posizione migliore",
                side_name_capitalized(side),
                describe_pieces(own)
            ),
        ))
    } else {
        None
    }
}

/// La FEN di una posizione.
fn fen_of(position: &Chess) -> String {
    Fen::from_position(position, EnPassantMode::Legal).to_string()
}

/// Il colore come lo usano i fatti dei rilevatori.
fn side_of(color: Color) -> chess_core::Side {
    if color == Color::White {
        chess_core::Side::White
    } else {
        chess_core::Side::Black
    }
}

/// La frase con la prima lettera minuscola (per metterla dopo "Dopo Cf3: …").
fn lowercase_first(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_lowercase().collect::<String>() + chars.as_str()
    })
}

/// `true` per i motivi tattici veri: le tattiche dei rilevatori che colpiscono
/// almeno un pezzo avversario vero (cavallo, alfiere, torre o donna; il re da
/// solo non basta) e le minacce di matto. Non contano i pezzi semplicemente
/// "in presa", né un'inchiodatura su un pedone.
fn is_real_tactic(fact: &Fact, board: &Board, side: Color) -> bool {
    match fact.category {
        FactCategory::Tactics => fact
            .squares
            .iter()
            .filter_map(|name| name.parse::<Square>().ok())
            .filter_map(|square| board.piece_at(square))
            .any(|piece| {
                piece.color == side.other() && !matches!(piece.role, Role::Pawn | Role::King)
            }),
        FactCategory::Threats => fact.text.contains(MATE_WORD),
        _ => false,
    }
}

/// Un fatto come evento.
fn fact_event(kind: IdeaKind, ply: usize, text: String, fact: &Fact) -> IdeaEvent {
    IdeaEvent {
        kind,
        ply,
        text,
        squares: fact
            .squares
            .iter()
            .filter_map(|name| name.parse().ok())
            .collect(),
        arrows: fact
            .arrows
            .iter()
            .filter_map(|arrow| Some((arrow.from.parse().ok()?, arrow.to.parse().ok()?)))
            .collect(),
        card_id: None,
    }
}

/// I motivi tattici che compaiono dopo le mosse di `side` nelle prime
/// `within` mezze mosse (ognuno una volta sola, la prima che si vede).
#[must_use]
pub fn tactic_events(
    start: &Chess,
    steps: &[LineStep],
    side: Color,
    within: usize,
) -> Vec<IdeaEvent> {
    let side_view = side_of(side);
    let mut known: HashSet<String> = analyze(&fen_of(start))
        .map(|report| report.facts.into_iter().map(|fact| fact.text).collect())
        .unwrap_or_default();
    let mut events = Vec::new();
    for (ply, step) in steps.iter().enumerate().take(within) {
        if step.mover != side {
            continue;
        }
        let Some(report) = analyze(&fen_of(&step.after)) else {
            continue;
        };
        for fact in report.facts {
            let real = is_real_tactic(&fact, step.after.board(), side);
            if real && fact.side == side_view && known.insert(fact.text.clone()) {
                let text = lowercase_first(&fact.text);
                events.push(fact_event(IdeaKind::Tactic, ply, text, &fact));
            }
        }
    }
    events
}

/// Strutture, temi e finali riconosciuti alla fine della variante ma non
/// all'inizio (i principi generici, come "sviluppa i pezzi", non sono un'idea).
#[must_use]
pub fn structure_events(start: &Chess, steps: &[LineStep]) -> Vec<IdeaEvent> {
    let Some(last) = steps.last() else {
        return Vec::new();
    };
    let before: HashSet<&str> = recognize(start, game_phase(start))
        .iter()
        .map(|found| found.key)
        .collect();
    recognize(&last.after, game_phase(&last.after))
        .into_iter()
        .filter(|found| found.topic != Topic::Principle && !before.contains(found.key))
        .take(1)
        .map(|found| IdeaEvent {
            kind: IdeaKind::Structure,
            ply: steps.len() - 1,
            text: format!(
                "si arriva a questa situazione: {}",
                lowercase_first(&found.text)
            ),
            squares: found.squares.clone(),
            arrows: Vec::new(),
            card_id: Some(format!("{}/{}", found.topic.folder(), found.key)),
        })
        .collect()
}

/// Le debolezze nuove dei pedoni avversari e i pedoni passati nuovi di `side`,
/// alla fine della variante.
#[must_use]
pub fn weakness_events(start: &Chess, steps: &[LineStep], side: Color) -> Vec<IdeaEvent> {
    let Some(last) = steps.last() else {
        return Vec::new();
    };
    let known: HashSet<String> = analyze(&fen_of(start))
        .map(|report| report.facts.into_iter().map(|fact| fact.text).collect())
        .unwrap_or_default();
    let (own, opponent) = (side_of(side), side_of(side.other()));
    analyze(&fen_of(&last.after))
        .map(|report| report.facts)
        .unwrap_or_default()
        .into_iter()
        .filter(|fact| fact.category == FactCategory::Pawns && !known.contains(&fact.text))
        .filter(|fact| {
            let weak = WEAK_PAWN_WORDS.iter().any(|word| fact.text.contains(word));
            let passed = fact.text.contains(PASSED_PAWN_WORD);
            (weak && fact.side == opponent) || (passed && fact.side == own)
        })
        .take(2)
        .map(|fact| {
            let text = format!("alla fine {}", lowercase_first(&fact.text));
            fact_event(IdeaKind::Weakness, steps.len() - 1, text, &fact)
        })
        .collect()
}

/// Entro quante mezze mosse si guarda il piano tranquillo.
const QUIET_PLIES: usize = 12;

/// Il piano "tranquillo" di `side` nelle prime mosse della variante, quando
/// non succede nulla di tattico: arroccare, sviluppare i pezzi minori, portare
/// le torri sulle colonne centrali, spingere i pedoni centrali. Le mosse non
/// si scrivono nella frase: l'interfaccia le mostra nella sua notazione.
#[must_use]
pub fn quiet_plan_event(steps: &[LineStep], side: Color) -> Option<IdeaEvent> {
    let (mut castled, mut developed, mut rooks, mut pushes) = (false, false, false, false);
    let mut last_ply = 0;
    for (ply, step) in steps.iter().enumerate().take(QUIET_PLIES) {
        if step.mover != side {
            continue;
        }
        let from_home = relative_rank(step.from, side) == 0;
        let central = matches!(step.to.file(), File::C | File::D | File::E | File::F);
        if step.is_castle {
            castled = true;
        } else if matches!(step.role, Role::Knight | Role::Bishop) && from_home {
            developed = true;
        } else if step.role == Role::Rook && central {
            rooks = true;
        } else if step.role == Role::Pawn && central && step.capture.is_none() {
            pushes = true;
        } else {
            continue;
        }
        last_ply = ply;
    }
    let parts: Vec<String> = [
        (castled, "mettere il re al sicuro con l'arrocco"),
        (developed, "sviluppare i pezzi che sono ancora a casa"),
        (rooks, "portare le torri sulle colonne centrali"),
        (pushes, "guadagnare spazio al centro con i pedoni"),
    ]
    .into_iter()
    .filter(|(done, _)| *done)
    .map(|(_, text)| text.to_owned())
    .collect();
    (!parts.is_empty()).then(|| {
        IdeaEvent::plain(
            IdeaKind::QuietPlan,
            last_ply,
            format!(
                "un piano tranquillo: {}, senza concedere nulla all'avversario",
                analysis::wording::join_list(&parts)
            ),
        )
    })
}
