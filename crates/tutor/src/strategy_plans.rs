//! Le regole dei piani: dagli squilibri ai piani consigliati.
//!
//! Ogni regola guarda il bilancio della posizione (vedi `imbalances`) dal
//! punto di vista di un giocatore e, se vale, propone un piano con un
//! **punteggio** (quanto è urgente o promettente) e il **perché** concreto in
//! questa posizione. Il "come", il "quando usarlo" e il "quando no" stanno
//! nella scheda della base di conoscenza del piano (`knowledge/`).
//! Le frasi non contengono mosse: solo case, che l'interfaccia può evidenziare.

use analysis::GamePhase;
use analysis::wording::piece_phrase;
use shakmaty::{Board, Chess, Color, File, Piece, Position, Role, Square};

use crate::board_query::{is_outpost, pieces, relative_rank, relative_square};
use crate::idea_events::IdeaKind;
use crate::imbalances::{CenterKind, Imbalances, SideImbalances};
use crate::recognition::Recognition;
use crate::structures::{FRENCH_CHAIN, KINGS_INDIAN_CHAIN};
use crate::topic::Topic;
use crate::{middlegame_themes, principles};

/// Un piano proposto da una regola.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanCandidate {
    /// L'argomento della scheda del piano.
    pub topic: Topic,
    /// La chiave della scheda.
    pub key: &'static str,
    /// Per chi è il piano.
    pub side: Color,
    /// Quanto conta (più alto = più importante).
    pub score: u32,
    /// Perché proprio qui.
    pub why: String,
    /// Le case da evidenziare.
    pub squares: Vec<Square>,
}

// Le chiavi dei piani: ognuna ha la sua scheda in `knowledge/`.
/// Attacco di minoranza.
pub const MINORITY_ATTACK: &str = "minority-attack";
/// Maggioranza sull'ala.
pub const PAWN_MAJORITY: &str = "pawn-majority";
/// Rottura centrale.
pub const CENTRAL_BREAK: &str = "central-break";
/// Case deboli intorno al re.
pub const WEAK_SQUARES: &str = "weak-squares";
/// Complesso di case di un colore.
pub const COLOR_COMPLEX: &str = "color-complex";
/// Alfiere cattivo.
pub const BAD_BISHOP: &str = "bad-bishop";
/// Alfiere buono contro cavallo cattivo.
pub const GOOD_BISHOP_BAD_KNIGHT: &str = "good-bishop-bad-knight";
/// Vantaggio di spazio.
pub const SPACE_ADVANTAGE: &str = "space-advantage";
/// Il pezzo peggiore.
pub const WORST_PIECE: &str = "worst-piece";
/// Profilassi.
pub const PROPHYLAXIS: &str = "prophylaxis";
/// Quando cambiare (e quando no).
pub const WHEN_TO_TRADE: &str = "when-to-trade";
/// Cambi contro chi ha più spazio.
pub const TRADES_AGAINST_SPACE: &str = "trades-against-space";
/// Attacco al re con gli arrocchi dallo stesso lato.
pub const SAME_SIDE_ATTACK: &str = "same-side-attack";
/// Il sacrificio greco in h7.
pub const GREEK_GIFT: &str = "greek-gift";
/// Il sacrificio di qualità.
pub const EXCHANGE_SACRIFICE: &str = "exchange-sacrifice";
/// Il sacrificio posizionale di pedone.
pub const PAWN_SACRIFICE: &str = "pawn-sacrifice";
/// Iniziativa e sviluppo.
pub const INITIATIVE: &str = "initiative";
/// Centro chiuso: si gioca sulle ali.
pub const CLOSED_CENTER: &str = "closed-center";
/// Centro aperto: pezzi attivi.
pub const OPEN_GAME: &str = "open-game";
/// Catena di pedoni: attaccare la base.
pub const PAWN_CHAIN_BASE: &str = "pawn-chain-base";
/// Il principio delle due debolezze.
pub const TWO_WEAKNESSES: &str = "two-weaknesses";
/// La trasformazione del vantaggio.
pub const TRANSFORMING_ADVANTAGE: &str = "transforming-advantage";
/// La colonna semiaperta.
pub const HALF_OPEN_FILE: &str = "half-open-file";
/// Il pedone passato lontano.
pub const OUTSIDE_PASSED_PAWN: &str = "outside-passed-pawn";
/// Scegliere il finale giusto.
pub const CHOOSING_ENDGAME: &str = "choosing-endgame";
/// La fortezza.
pub const FORTRESS: &str = "fortress";
/// Zugzwang e opposizione.
pub const ZUGZWANG_OPPOSITION: &str = "zugzwang-opposition";
/// La regola del quadrato.
pub const RULE_OF_SQUARE: &str = "rule-of-square";

/// Le chiavi dei piani nuovi (le altre vengono dai riconoscitori, vedi
/// `recognition::KNOWN_KEYS`): ognuna deve avere la sua scheda.
pub const STRATEGY_KEYS: &[(Topic, &str)] = &[
    (Topic::Middlegame, MINORITY_ATTACK),
    (Topic::Middlegame, PAWN_MAJORITY),
    (Topic::Middlegame, CENTRAL_BREAK),
    (Topic::Middlegame, WEAK_SQUARES),
    (Topic::Middlegame, COLOR_COMPLEX),
    (Topic::Middlegame, BAD_BISHOP),
    (Topic::Middlegame, GOOD_BISHOP_BAD_KNIGHT),
    (Topic::Middlegame, SPACE_ADVANTAGE),
    (Topic::Middlegame, WORST_PIECE),
    (Topic::Middlegame, PROPHYLAXIS),
    (Topic::Middlegame, WHEN_TO_TRADE),
    (Topic::Middlegame, TRADES_AGAINST_SPACE),
    (Topic::Middlegame, SAME_SIDE_ATTACK),
    (Topic::Middlegame, GREEK_GIFT),
    (Topic::Middlegame, EXCHANGE_SACRIFICE),
    (Topic::Middlegame, PAWN_SACRIFICE),
    (Topic::Middlegame, INITIATIVE),
    (Topic::Middlegame, CLOSED_CENTER),
    (Topic::Middlegame, OPEN_GAME),
    (Topic::Middlegame, PAWN_CHAIN_BASE),
    (Topic::Middlegame, TWO_WEAKNESSES),
    (Topic::Middlegame, TRANSFORMING_ADVANTAGE),
    (Topic::Middlegame, HALF_OPEN_FILE),
    (Topic::Endgame, OUTSIDE_PASSED_PAWN),
    (Topic::Endgame, CHOOSING_ENDGAME),
    (Topic::Endgame, FORTRESS),
    (Topic::Endgame, ZUGZWANG_OPPOSITION),
    (Topic::Endgame, RULE_OF_SQUARE),
];

/// Gli eventi di una variante di Stockfish (vedi `idea_events`) che
/// confermano un piano: se la variante migliore li contiene, il tutor dice
/// «Stockfish conferma».
#[must_use]
pub fn confirming_kinds(key: &str) -> &'static [IdeaKind] {
    use IdeaKind::{
        Combination, Mate, OpenFile, PawnBreak, QuietPlan, Route, Sacrifice, Structure, Tactic,
        Weakness, WinsMaterial,
    };
    match key {
        MINORITY_ATTACK | PAWN_MAJORITY | CENTRAL_BREAK | CLOSED_CENTER | PAWN_CHAIN_BASE => {
            &[PawnBreak, Weakness]
        }
        OUTSIDE_PASSED_PAWN | RULE_OF_SQUARE | ZUGZWANG_OPPOSITION | principles::PASSED_PAWN => {
            &[Weakness, WinsMaterial]
        }
        principles::OPEN_FILE | HALF_OPEN_FILE => &[OpenFile, Route],
        SAME_SIDE_ATTACK
        | GREEK_GIFT
        | INITIATIVE
        | PAWN_SACRIFICE
        | middlegame_themes::OPPOSITE_CASTLING => &[Mate, Combination, Sacrifice, Tactic, Route],
        EXCHANGE_SACRIFICE => &[Sacrifice],
        TWO_WEAKNESSES | BAD_BISHOP => &[Weakness, Structure, WinsMaterial],
        TRANSFORMING_ADVANTAGE
        | CHOOSING_ENDGAME
        | FORTRESS
        | WHEN_TO_TRADE
        | TRADES_AGAINST_SPACE
        | OPEN_GAME
        | PROPHYLAXIS
        | principles::BISHOP_PAIR => &[Structure, QuietPlan, WinsMaterial],
        _ => &[Route],
    }
}

/// Un piano proposto.
fn plan(topic: Topic, key: &'static str, side: Color, score: u32, why: String) -> PlanCandidate {
    PlanCandidate {
        topic,
        key,
        side,
        score,
        why,
        squares: Vec::new(),
    }
}

/// Lo stesso piano, con le case da evidenziare.
fn with_squares(mut candidate: PlanCandidate, squares: &[Square]) -> PlanCandidate {
    candidate.squares = squares.to_vec();
    candidate
}

/// Le case scritte in elenco: "d4", "d4 e e5", "c6, d6 e e6".
fn squares_text(squares: &[Square]) -> String {
    let names: Vec<String> = squares.iter().map(ToString::to_string).collect();
    analysis::wording::join_list(&names)
}

/// `true` se sulla scacchiera ci sono solo re e pedoni.
fn is_pawn_endgame(board: &Board) -> bool {
    (board.occupied() & !board.pawns() & !board.kings()).is_empty()
}

/// I pezzi (non re, non pedoni) di entrambi.
fn piece_count(board: &Board) -> usize {
    (board.occupied() & !board.pawns() & !board.kings()).count()
}

/// I piani dei finali.
fn endgame_plans(board: &Board, us: &SideImbalances, color: Color) -> Vec<PlanCandidate> {
    let mut found = Vec::new();
    if is_pawn_endgame(board) {
        if let Some(pawn) = us.passed.first() {
            found.push(with_squares(
                plan(
                    Topic::Endgame,
                    RULE_OF_SQUARE,
                    color,
                    70,
                    format!(
                        "nel finale di pedoni il pedone passato in {pawn} decide tutto: conta se il re avversario riesce a entrare nel suo quadrato"
                    ),
                ),
                &[*pawn],
            ));
        }
        found.push(plan(
            Topic::Endgame,
            ZUGZWANG_OPPOSITION,
            color,
            60,
            "è un finale di soli pedoni: i re si fronteggiano e chi ha l'opposizione costringe l'altro a cedere il passo".to_owned(),
        ));
    }
    if let Some(pawn) = us
        .passed
        .iter()
        .find(|pawn| matches!(pawn.file(), File::A | File::B | File::G | File::H))
    {
        found.push(with_squares(
            plan(
                Topic::Endgame,
                OUTSIDE_PASSED_PAWN,
                color,
                65,
                format!(
                    "il pedone passato in {pawn} è lontano dagli altri: attira lì il re avversario mentre il tuo re va a prendere i pedoni dall'altra parte"
                ),
            ),
            &[*pawn],
        ));
    }
    if us.material <= -2 {
        found.push(plan(
            Topic::Endgame,
            FORTRESS,
            color,
            55,
            format!(
                "sei sotto di {} punti di materiale: cerca una fortezza, una posizione che l'avversario non riesce a sfondare",
                -us.material
            ),
        ));
    }
    found.push(plan(
        Topic::Principle,
        principles::KING_ACTIVITY,
        color,
        45,
        "nel finale il re diventa un pezzo forte: portalo verso il centro e verso i pedoni"
            .to_owned(),
    ));
    found
}

/// I piani legati ai pedoni: maggioranze, minoranze, pedoni passati, catene, rotture.
fn pawn_plans(
    position: &Chess,
    us: &SideImbalances,
    them: &SideImbalances,
    imbalances: &Imbalances,
    recognized: &[Recognition],
) -> Vec<PlanCandidate> {
    let color = us.color;
    let endgame = imbalances.phase == GamePhase::Endgame;
    let mut found = Vec::new();
    if let Some(pawn) = us
        .passed
        .first()
        .filter(|_| !is_pawn_endgame(position.board()))
    {
        found.push(with_squares(
            plan(
                Topic::Principle,
                principles::PASSED_PAWN,
                color,
                if endgame { 65 } else { 50 },
                format!("hai un pedone passato in {pawn}: va spinto e sostenuto dai pezzi, perché nessun pedone può fermarlo"),
            ),
            &[*pawn],
        ));
    }
    // L'attacco di minoranza si fa con i re lontani dall'ala di donna:
    // con un re arroccato lungo, spingere quei pedoni è un attacco al re.
    let queenside_king = |side: Color| {
        position
            .board()
            .king_of(side)
            .is_some_and(|king| matches!(king.file(), File::A | File::B | File::C))
    };
    if !endgame
        && us.queenside_pawns >= 2
        && us.queenside_pawns < them.queenside_pawns
        && imbalances.center != CenterKind::Open
        && !queenside_king(color)
        && !queenside_king(color.other())
    {
        found.push(plan(
            Topic::Middlegame,
            MINORITY_ATTACK,
            color,
            60,
            format!(
                "hai {} pedoni sull'ala di donna contro {}: spingendoli contro quelli avversari gli lasci un pedone debole (isolato o arretrato)",
                us.queenside_pawns, them.queenside_pawns
            ),
        ));
    }
    for (wing, own, other) in [
        ("di donna", us.queenside_pawns, them.queenside_pawns),
        ("di re", us.kingside_pawns, them.kingside_pawns),
    ] {
        if own > other && own >= 2 {
            found.push(plan(
                Topic::Middlegame,
                PAWN_MAJORITY,
                color,
                if endgame { 60 } else { 45 },
                format!("hai la maggioranza sull'ala {wing} ({own} pedoni contro {other}): avanzando con questi pedoni crei un pedone passato"),
            ));
        }
    }
    if imbalances.center == CenterKind::Closed && !endgame {
        found.push(plan(
            Topic::Middlegame,
            CLOSED_CENTER,
            color,
            55,
            "il centro è chiuso: lì non si passa, quindi si gioca sulle ali, spingendo i pedoni dalla parte in cui punta la tua catena".to_owned(),
        ));
    }
    if recognized
        .iter()
        .any(|found| found.key == FRENCH_CHAIN || found.key == KINGS_INDIAN_CHAIN)
    {
        found.push(plan(
            Topic::Middlegame,
            PAWN_CHAIN_BASE,
            color,
            55,
            "c'è una catena di pedoni: si attacca alla base, il pedone più arretrato della catena avversaria, che non è difeso da altri pedoni".to_owned(),
        ));
    }
    if let Some((from, to)) = central_break(position.board(), color)
        && imbalances.center != CenterKind::Open
        && !endgame
    {
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                CENTRAL_BREAK,
                color,
                50,
                format!("la spinta del pedone da {from} a {to} attacca i pedoni avversari al centro e apre il gioco"),
            ),
            &[from, to],
        ));
    }
    found
}

/// Una spinta di pedone di `color` (al centro, colonne c–f) verso una casa
/// libera da cui attaccherebbe un pedone avversario.
fn central_break(board: &Board, color: Color) -> Option<(Square, Square)> {
    let enemy = pieces(board, color.other(), Role::Pawn);
    pieces(board, color, Role::Pawn)
        .into_iter()
        .filter(|pawn| matches!(pawn.file(), File::C | File::D | File::E | File::F))
        .find_map(|pawn| {
            let to = pawn.offset(if color == Color::White { 8 } else { -8 })?;
            let free = board.piece_at(to).is_none();
            let attacks = (shakmaty::attacks::pawn_attacks(color, to) & enemy).any();
            (free && attacks).then_some((pawn, to))
        })
}

/// I piani dei pezzi: colonne, avamposti, alfieri, pezzo peggiore, spazio.
fn piece_plans(
    us: &SideImbalances,
    them: &SideImbalances,
    imbalances: &Imbalances,
) -> Vec<PlanCandidate> {
    let color = us.color;
    let endgame = imbalances.phase == GamePhase::Endgame;
    let opening = imbalances.phase == GamePhase::Opening;
    let mut found = Vec::new();
    if let Some(file) = us.open_files_free.first() {
        found.push(plan(
            Topic::Principle,
            principles::OPEN_FILE,
            color,
            45,
            format!(
                "la colonna {} è aperta: occupala con una torre prima dell'avversario",
                file.char()
            ),
        ));
    }
    let weak_targets: Vec<Square> = them
        .isolated
        .iter()
        .chain(&them.backward)
        .copied()
        .collect();
    if let Some((file, target)) = us.files_with_rooks.iter().find_map(|file| {
        weak_targets
            .iter()
            .find(|square| square.file() == *file)
            .map(|square| (*file, *square))
    }) {
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                HALF_OPEN_FILE,
                color,
                50,
                format!(
                    "la tua torre sulla colonna {} preme sul pedone debole in {target}",
                    file.char()
                ),
            ),
            &[target],
        ));
    }
    if let Some(square) = us.outposts.first().filter(|_| us.minors.0 > 0 && !endgame) {
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                middlegame_themes::OUTPOST,
                color,
                50,
                format!("la casa {square} è un avamposto: un cavallo lì non può essere cacciato da nessun pedone"),
            ),
            &[*square],
        ));
    }
    if let Some(worst) = us.worst_piece.filter(|_| !opening) {
        let piece = Piece {
            color,
            role: worst.role,
        };
        let moves = if worst.mobility == 1 {
            "mossa"
        } else {
            "mosse"
        };
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                WORST_PIECE,
                color,
                50,
                format!(
                    "il tuo pezzo peggiore è {}: ha solo {} {moves}. Trovagli una casa migliore: è il modo più semplice di migliorare la posizione",
                    piece_phrase(piece, worst.square),
                    worst.mobility
                ),
            ),
            &[worst.square],
        ));
    }
    found
}

/// I piani degli alfieri e dello spazio: alfiere cattivo, alfiere contro cavallo,
/// coppia degli alfieri, spazio, centro aperto.
fn bishop_and_space_plans(
    us: &SideImbalances,
    them: &SideImbalances,
    imbalances: &Imbalances,
) -> Vec<PlanCandidate> {
    let color = us.color;
    let endgame = imbalances.phase == GamePhase::Endgame;
    let opening = imbalances.phase == GamePhase::Opening;
    let mut found = Vec::new();
    if let Some(bishop) = them.bad_bishops.first() {
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                BAD_BISHOP,
                color,
                45,
                format!("l'alfiere avversario in {bishop} è cattivo, chiuso dai suoi pedoni: lascia quei pedoni dove sono e cambia gli altri pezzi"),
            ),
            &[*bishop],
        ));
    }
    if let Some(bishop) = us.bad_bishops.first() {
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                BAD_BISHOP,
                color,
                40,
                format!("il tuo alfiere in {bishop} è cattivo: cambialo, oppure porta i pedoni sulle case dell'altro colore"),
            ),
            &[*bishop],
        ));
    }
    if us.minors.1 >= 1 && them.minors.1 == 0 && them.minors.0 >= 1 && !opening {
        found.push(plan(
            Topic::Middlegame,
            GOOD_BISHOP_BAD_KNIGHT,
            color,
            40,
            "hai un alfiere contro un cavallo: con pedoni su tutte e due le ali l'alfiere, che arriva lontano, è più forte".to_owned(),
        ));
    }
    if us.bishop_pair && imbalances.center != CenterKind::Closed {
        found.push(plan(
            Topic::Principle,
            principles::BISHOP_PAIR,
            color,
            40,
            "hai la coppia degli alfieri: apri la posizione, più linee ci sono e più valgono"
                .to_owned(),
        ));
    }
    if !endgame && us.space >= them.space + 3 {
        found.push(plan(
            Topic::Middlegame,
            SPACE_ADVANTAGE,
            color,
            45,
            format!(
                "hai più spazio ({} case della metà avversaria contro {}): evita i cambi, l'avversario fatica a muovere i suoi pezzi",
                us.space, them.space
            ),
        ));
    }
    if !endgame && them.space >= us.space + 3 {
        found.push(plan(
            Topic::Middlegame,
            TRADES_AGAINST_SPACE,
            color,
            45,
            "l'avversario ha più spazio: cambia qualche pezzo e cerca una rottura di pedone per respirare".to_owned(),
        ));
    }
    if imbalances.center == CenterKind::Open && !endgame {
        found.push(plan(
            Topic::Middlegame,
            OPEN_GAME,
            color,
            35,
            "il centro è aperto: contano i pezzi attivi e le linee aperte, la coppia degli alfieri e le torri centrali valgono di più".to_owned(),
        ));
    }
    found
}

/// `true` se il re è sull'ala di re (colonne f–h).
fn on_kingside(square: Square) -> bool {
    matches!(square.file(), File::F | File::G | File::H)
}

/// Le condizioni del sacrificio greco per `color`: re avversario arroccato
/// corto, un proprio alfiere che vede h7 (h2 per il Nero), un cavallo che può
/// arrivare in g5 (g4), la donna in gioco e nessun cavallo avversario in f6 (f3).
fn greek_gift_ready(board: &Board, color: Color) -> bool {
    let them = color.other();
    let h7 = relative_square(File::H, 7, color);
    let g5 = relative_square(File::G, 5, color);
    let f6 = relative_square(File::F, 6, color);
    let king_home = relative_square(File::G, 8, color);
    let enemy_knight_f6 = pieces(board, them, Role::Knight).contains(f6);
    let bishop_sees = pieces(board, color, Role::Bishop)
        .into_iter()
        .any(|bishop| board.attacks_from(bishop).contains(h7));
    let knight_reaches = pieces(board, color, Role::Knight)
        .into_iter()
        .any(|knight| shakmaty::attacks::knight_attacks(knight).contains(g5));
    board.king_of(them) == Some(king_home)
        && pieces(board, them, Role::Pawn).contains(h7)
        && pieces(board, color, Role::Queen).any()
        && bishop_sees
        && knight_reaches
        && !enemy_knight_f6
}

/// I piani d'attacco al re: arrocchi, sacrificio greco, case deboli, complesso di colore.
fn king_attack_plans(
    board: &Board,
    us: &SideImbalances,
    them: &SideImbalances,
) -> Vec<PlanCandidate> {
    let color = us.color;
    let mut found = Vec::new();
    let (our_king, their_king) = (board.king_of(color), board.king_of(color.other()));
    let has_queens = board.queens().any();
    if let (Some(ours), Some(theirs)) = (our_king, their_king)
        && us.castled
        && them.castled
        && has_queens
    {
        if on_kingside(ours) == on_kingside(theirs) {
            if them.king_shield <= 2 && us.space >= them.space {
                found.push(plan(
                    Topic::Middlegame,
                    SAME_SIDE_ATTACK,
                    color,
                    50,
                    "i re sono arroccati dallo stesso lato e quello avversario è poco protetto: attaccalo con i pezzi, i tuoi pedoni avanzano con prudenza".to_owned(),
                ));
            }
        } else {
            found.push(plan(
                Topic::Middlegame,
                middlegame_themes::OPPOSITE_CASTLING,
                color,
                60,
                "i re sono su lati opposti: lancia i pedoni contro il re avversario, vince chi apre per primo le linee".to_owned(),
            ));
        }
    }
    if greek_gift_ready(board, color) {
        found.push(plan(
            Topic::Middlegame,
            GREEK_GIFT,
            color,
            70,
            "ci sono le condizioni del sacrificio greco: alfiere contro h7, cavallo pronto ad andare in g5 e la donna che può arrivare in h5. Calcola bene prima di sacrificare".to_owned(),
        ));
    }
    if us.holes_near_enemy_king.len() >= 2 {
        let holes: Vec<Square> = us.holes_near_enemy_king.iter().copied().take(3).collect();
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                WEAK_SQUARES,
                color,
                55,
                format!(
                    "intorno al re avversario ci sono case deboli ({}): nessun pedone può più difenderle, portaci i tuoi pezzi",
                    squares_text(&holes)
                ),
            ),
            &holes,
        ));
        // Se l'avversario non ha più l'alfiere di quel colore, le case sono ancora più deboli.
        let light = holes.iter().filter(|square| square.is_light()).count() >= 2;
        let dark = holes.iter().filter(|square| !square.is_light()).count() >= 2;
        let their_bishops: Vec<Square> = pieces(board, color.other(), Role::Bishop)
            .into_iter()
            .collect();
        let missing = |want_light: bool| {
            !their_bishops
                .iter()
                .any(|bishop| bishop.is_light() == want_light)
        };
        if (light && missing(true)) || (dark && missing(false)) {
            let shade = if light && missing(true) {
                "chiare"
            } else {
                "scure"
            };
            found.push(plan(
                Topic::Middlegame,
                COLOR_COMPLEX,
                color,
                50,
                format!("l'avversario non ha più l'alfiere delle case {shade}: le case {shade} intorno al suo re non le difende nessuno"),
            ));
        }
    }
    found
}

/// I piani d'attacco nel mediogioco: al re, con l'iniziativa, con i sacrifici.
fn attack_plans(
    board: &Board,
    us: &SideImbalances,
    them: &SideImbalances,
    imbalances: &Imbalances,
) -> Vec<PlanCandidate> {
    let color = us.color;
    if imbalances.phase == GamePhase::Endgame {
        return Vec::new();
    }
    let mut found = king_attack_plans(board, us, them);
    let lead = them.undeveloped.saturating_sub(us.undeveloped);
    if lead >= 2 && them.king_in_center {
        found.push(plan(
            Topic::Middlegame,
            INITIATIVE,
            color,
            60,
            format!(
                "sei più sviluppato (l'avversario ha ancora {} pezzi a casa, tu {}) e il suo re è al centro: apri il gioco subito, prima che recuperi",
                them.undeveloped, us.undeveloped
            ),
        ));
        if lead >= 3 {
            found.push(plan(
                Topic::Middlegame,
                PAWN_SACRIFICE,
                color,
                55,
                "per aprire le linee verso un re rimasto al centro può valere anche un pedone: qui lo sviluppo conta più del materiale".to_owned(),
            ));
        }
    }
    let strong_enemy_knight =
        pieces(board, color.other(), Role::Knight)
            .into_iter()
            .find(|knight| {
                relative_rank(*knight, color) <= 4 && is_outpost(board, *knight, color.other())
            });
    if let Some(knight) = strong_enemy_knight.filter(|_| pieces(board, color, Role::Rook).any()) {
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                EXCHANGE_SACRIFICE,
                color,
                35,
                format!("il cavallo avversario in {knight} è fortissimo e nessun pedone può cacciarlo: a volte si dà la qualità (una torre per lui) per eliminarlo"),
            ),
            &[knight],
        ));
    }
    found
}

/// I piani legati al materiale e alle debolezze: cambi, due debolezze, trasformazioni.
fn balance_plans(
    board: &Board,
    us: &SideImbalances,
    them: &SideImbalances,
    imbalances: &Imbalances,
) -> Vec<PlanCandidate> {
    let color = us.color;
    let opening = imbalances.phase == GamePhase::Opening;
    let endgame = imbalances.phase == GamePhase::Endgame;
    let mut found = Vec::new();
    let weak: Vec<Square> = them
        .isolated
        .iter()
        .chain(&them.backward)
        .copied()
        .collect();
    if let Some(target) = weak.first().filter(|_| !opening) {
        found.push(with_squares(
            plan(
                Topic::Middlegame,
                TWO_WEAKNESSES,
                color,
                50,
                format!("l'avversario ha un pedone debole in {target}: attaccalo, poi creane una seconda sull'altra ala. Una debolezza si difende, due no"),
            ),
            &[*target],
        ));
    }
    if us.material >= 1 && !endgame {
        found.push(plan(
            Topic::Middlegame,
            TRANSFORMING_ADVANTAGE,
            color,
            45,
            format!(
                "sei avanti di {} {}: trasforma il vantaggio, semplifica e porta la partita in un finale che si vince",
                us.material,
                if us.material == 1 { "punto" } else { "punti" }
            ),
        ));
    }
    if us.material >= 2 && piece_count(board) <= 6 {
        found.push(plan(
            Topic::Endgame,
            CHOOSING_ENDGAME,
            color,
            50,
            "sei avanti e i pezzi sono pochi: scegli bene cosa cambiare, alcuni finali si vincono facilmente, altri finiscono patti".to_owned(),
        ));
    }
    if us.material <= -2 && !endgame {
        found.push(plan(
            Topic::Middlegame,
            WHEN_TO_TRADE,
            color,
            45,
            "sei sotto di materiale: evita i cambi di pezzi, ogni cambio aiuta l'avversario; cerca complicazioni e iniziativa".to_owned(),
        ));
    }
    found
}

/// Tutti i piani possibili per `color`, dal più importante (un piano per chiave).
#[must_use]
pub fn plan_candidates(
    position: &Chess,
    imbalances: &Imbalances,
    recognized: &[Recognition],
    color: Color,
) -> Vec<PlanCandidate> {
    let board = position.board();
    let us = imbalances.of(color);
    let them = imbalances.of(color.other());
    let mut found = Vec::new();
    if imbalances.phase == GamePhase::Endgame {
        found.extend(endgame_plans(board, us, color));
    }
    found.extend(pawn_plans(position, us, them, imbalances, recognized));
    found.extend(piece_plans(us, them, imbalances));
    found.extend(bishop_and_space_plans(us, them, imbalances));
    found.extend(attack_plans(board, us, them, imbalances));
    found.extend(balance_plans(board, us, them, imbalances));
    found.sort_by_key(|candidate| std::cmp::Reverse(candidate.score));
    let mut seen: Vec<&'static str> = Vec::new();
    found.retain(|candidate| {
        let fresh = !seen.contains(&candidate.key);
        seen.push(candidate.key);
        fresh
    });
    found
}

/// La frase con la prima lettera minuscola (per un titolo dentro una frase).
pub(crate) fn lowercase_first(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_lowercase().collect::<String>() + chars.as_str()
    })
}

/// La profilassi: se l'avversario ha un piano forte, il primo piano è impedirlo.
#[must_use]
pub fn prophylaxis(opponent: &PlanCandidate, title: &str, color: Color) -> Option<PlanCandidate> {
    (opponent.score >= 55).then(|| {
        with_squares(
            plan(
                Topic::Middlegame,
                PROPHYLAXIS,
                color,
                opponent.score - 5,
                format!("l'avversario ha un piano pericoloso ({}): prima di fare il tuo, cerca la mossa che glielo impedisce", lowercase_first(title)),
            ),
            &opponent.squares,
        )
    })
}

#[cfg(test)]
mod tests {
    use analysis::game_phase::game_phase;
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color};

    use super::{MINORITY_ATTACK, PAWN_MAJORITY, RULE_OF_SQUARE, plan_candidates};
    use crate::imbalances::imbalances;
    use crate::recognition::recognize;

    fn keys(fen: &str, color: Color) -> Vec<&'static str> {
        let position: Chess = fen
            .parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione");
        let phase = game_phase(&position);
        let found = imbalances(&position, phase);
        plan_candidates(&position, &found, &recognize(&position, phase), color)
            .into_iter()
            .map(|candidate| candidate.key)
            .collect()
    }

    #[test]
    fn a_pawn_endgame_with_a_passed_pawn_counts_the_square() {
        assert_eq!(
            keys("8/8/8/1P6/8/8/5k2/K7 w - - 0 1", Color::White).first(),
            Some(&RULE_OF_SQUARE)
        );
    }

    #[test]
    fn the_carlsbad_minority_attack_is_proposed_to_white() {
        // Carlsbad: Bianco a2 b2 d4 e3 f2 g2 h2, Nero a7 b7 c6 d5 f7 g7 h7.
        let found = keys(
            "r1bq1rk1/pp2bppp/2p2n2/3p4/3P4/2NBPN2/PP3PPP/R2QK2R w KQ - 0 10",
            Color::White,
        );
        assert!(found.contains(&MINORITY_ATTACK), "{found:?}");
        // Il Nero ha la maggioranza sull'ala di donna.
        assert!(
            keys(
                "r1bq1rk1/pp2bppp/2p2n2/3p4/3P4/2NBPN2/PP3PPP/R2QK2R w KQ - 0 10",
                Color::Black
            )
            .contains(&PAWN_MAJORITY)
        );
    }
}
