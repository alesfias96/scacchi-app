//! Il bilancio della posizione: "chi ha cosa" (gli squilibri).
//!
//! È il metodo dei libri di strategia: prima di scegliere un piano si guarda
//! cosa distingue i due giocatori. Per ciascun lato si misurano:
//! - il **materiale** e la **coppia degli alfieri**;
//! - la **struttura dei pedoni**: pedoni isolati, doppiati, arretrati, le
//!   isole, le **maggioranze** sulle ali, i **pedoni passati**;
//! - lo **spazio**: le case della metà avversaria controllate dai propri pedoni;
//! - l'**attività dei pezzi**: quante mosse ha ciascun pezzo e qual è il
//!   **pezzo peggiore**; gli **alfieri cattivi** (chiusi dai propri pedoni);
//! - le **colonne** aperte e semiaperte occupate dalle torri;
//! - gli **avamposti** liberi e le **case deboli** intorno al re avversario;
//! - la **sicurezza del re** e lo **sviluppo**.
//!
//! Sono solo misure: i piani si scelgono in `strategy_plans`.

use analysis::GamePhase;
use analysis::material::material;
use shakmaty::{Bitboard, Board, Chess, Color, File, Position, Role, Square};

use crate::board_query::{
    has_pawn_on_file, is_open_file, is_outpost, is_passed_pawn, neighbour_files, pieces,
    relative_rank,
};

/// Com'è il centro: chiuso (pedoni centrali bloccati), aperto (quasi senza
/// pedoni centrali) o mobile (tutto il resto).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterKind {
    /// Almeno due pedoni centrali bloccati da un pedone avversario.
    Closed,
    /// Al massimo un pedone sulle colonne d ed e.
    Open,
    /// Né chiuso né aperto: i pedoni centrali possono ancora muoversi o cambiarsi.
    Mobile,
}

/// Il pezzo peggiore di un lato: quello con meno mosse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorstPiece {
    /// Il tipo di pezzo.
    pub role: Role,
    /// Dove sta.
    pub square: Square,
    /// Quante case può raggiungere.
    pub mobility: u32,
}

/// Gli squilibri di un lato.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideImbalances {
    /// Il colore.
    pub color: Color,
    /// Il vantaggio di materiale, in pedoni (negativo se è sotto).
    pub material: i32,
    /// `true` se ha due alfieri e l'avversario no.
    pub bishop_pair: bool,
    /// I pedoni isolati.
    pub isolated: Vec<Square>,
    /// I pedoni arretrati.
    pub backward: Vec<Square>,
    /// Le colonne con pedoni doppiati.
    pub doubled: Vec<File>,
    /// I pedoni passati.
    pub passed: Vec<Square>,
    /// Quante isole di pedoni ha.
    pub islands: u32,
    /// I pedoni sull'ala di donna (colonne a–c) e sull'ala di re (f–h).
    pub queenside_pawns: u32,
    /// Vedi `queenside_pawns`.
    pub kingside_pawns: u32,
    /// Lo spazio: case della metà avversaria attaccate dai propri pedoni.
    pub space: u32,
    /// Il pezzo con meno mosse (cavalli, alfieri, torri, donna), se ne ha pochissime.
    pub worst_piece: Option<WorstPiece>,
    /// Gli alfieri "cattivi": chiusi da molti propri pedoni sulle case del loro colore.
    pub bad_bishops: Vec<Square>,
    /// Le colonne senza propri pedoni (aperte o semiaperte) dove ha una torre o la donna.
    pub files_with_rooks: Vec<File>,
    /// Le colonne aperte (senza pedoni) dove non ha ancora una torre.
    pub open_files_free: Vec<File>,
    /// Gli avamposti liberi per i suoi cavalli (nella metà avversaria, difesi da un pedone).
    pub outposts: Vec<Square>,
    /// Le case deboli intorno al re avversario (che nessun pedone avversario può difendere).
    pub holes_near_enemy_king: Vec<Square>,
    /// I pedoni davanti al proprio re.
    pub king_shield: u32,
    /// `true` se il re ha arroccato (è sulle colonne a–c o f–h della prima traversa).
    pub castled: bool,
    /// `true` se il re è ancora al centro (colonne d–e) con le donne in gioco.
    pub king_in_center: bool,
    /// I pezzi minori ancora sulle case di partenza.
    pub undeveloped: u32,
    /// I cavalli e gli alfieri, per tipo: (cavalli, alfieri).
    pub minors: (u32, u32),
}

/// Il bilancio completo della posizione.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Imbalances {
    /// La fase della partita.
    pub phase: GamePhase,
    /// Com'è il centro.
    pub center: CenterKind,
    /// Gli squilibri del Bianco.
    pub white: SideImbalances,
    /// Gli squilibri del Nero.
    pub black: SideImbalances,
}

impl Imbalances {
    /// Gli squilibri di un lato.
    #[must_use]
    pub fn of(&self, color: Color) -> &SideImbalances {
        match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }
}

/// Le colonne dell'ala di donna e dell'ala di re.
const QUEENSIDE: [File; 3] = [File::A, File::B, File::C];
const KINGSIDE: [File; 3] = [File::F, File::G, File::H];
/// Sotto questo numero di mosse un pezzo è "il peggiore".
const WORST_MOBILITY: u32 = 2;
/// Da quanti propri pedoni sul suo colore (al centro) un alfiere è cattivo.
const BAD_BISHOP_PAWNS: usize = 3;

/// Il numero di case di una scacchiera di bit, come `u32`.
fn count(squares: Bitboard) -> u32 {
    u32::try_from(squares.count()).unwrap_or(u32::MAX)
}

/// `true` se il pedone di `color` in `square` è arretrato: nessun proprio
/// pedone sulle colonne vicine può più difenderlo e la casa davanti è
/// controllata da un pedone avversario.
fn is_backward(board: &Board, color: Color, square: Square) -> bool {
    let rank = relative_rank(square, color);
    let own = pieces(board, color, Role::Pawn);
    let supported = own.into_iter().any(|pawn| {
        neighbour_files(square.file()).contains(&pawn.file()) && relative_rank(pawn, color) <= rank
    });
    let Some(front) = square.offset(if color == Color::White { 8 } else { -8 }) else {
        return false;
    };
    let enemy = pieces(board, color.other(), Role::Pawn);
    let controlled = !(shakmaty::attacks::pawn_attacks(color, front) & enemy).is_empty();
    !supported && controlled
}

/// Le isole di pedoni: gruppi di colonne vicine con pedoni.
fn islands(board: &Board, color: Color) -> u32 {
    let mut islands = 0;
    let mut inside = false;
    for file in File::ALL {
        let has = has_pawn_on_file(board, color, file);
        if has && !inside {
            islands += 1;
        }
        inside = has;
    }
    islands
}

/// I pedoni di `color` sulle colonne indicate.
fn pawns_on(board: &Board, color: Color, files: &[File]) -> u32 {
    files
        .iter()
        .map(|file| count(pieces(board, color, Role::Pawn) & Bitboard::from_file(*file)))
        .sum()
}

/// Le case della metà avversaria attaccate dai pedoni di `color`.
fn space(board: &Board, color: Color) -> u32 {
    let mut attacked = Bitboard::EMPTY;
    for pawn in pieces(board, color, Role::Pawn) {
        attacked |= shakmaty::attacks::pawn_attacks(color, pawn);
    }
    count(
        attacked
            .into_iter()
            .filter(|square| relative_rank(*square, color) >= 5)
            .collect(),
    )
}

/// Quante case può raggiungere il pezzo in `square` (non occupate da propri pezzi).
fn mobility(board: &Board, square: Square, color: Color) -> u32 {
    count(board.attacks_from(square) & !board.by_color(color))
}

/// Il pezzo con meno mosse, se ne ha al massimo [`WORST_MOBILITY`].
fn worst_piece(board: &Board, color: Color) -> Option<WorstPiece> {
    [Role::Knight, Role::Bishop, Role::Rook, Role::Queen]
        .into_iter()
        .flat_map(|role| {
            pieces(board, color, role)
                .into_iter()
                .map(move |square| (role, square))
        })
        .map(|(role, square)| WorstPiece {
            role,
            square,
            mobility: mobility(board, square, color),
        })
        .filter(|piece| piece.mobility <= WORST_MOBILITY)
        .min_by_key(|piece| piece.mobility)
}

/// Gli alfieri cattivi: molti propri pedoni centrali sulle case del loro
/// colore, e poche mosse.
fn bad_bishops(board: &Board, color: Color) -> Vec<Square> {
    let own_pawns = pieces(board, color, Role::Pawn);
    pieces(board, color, Role::Bishop)
        .into_iter()
        .filter(|bishop| {
            let same_color = own_pawns
                .into_iter()
                .filter(|pawn| {
                    pawn.is_light() == bishop.is_light()
                        && matches!(pawn.file(), File::C | File::D | File::E | File::F)
                })
                .count();
            same_color >= BAD_BISHOP_PAWNS && mobility(board, *bishop, color) <= 4
        })
        .collect()
}

/// Le colonne del re e quelle vicine.
fn king_files(king: Square) -> Vec<File> {
    [
        king.file().offset(-1),
        Some(king.file()),
        king.file().offset(1),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// I pedoni davanti al re: sulle sue colonne, nella seconda e terza traversa.
fn king_shield(board: &Board, color: Color) -> u32 {
    let Some(king) = board.king_of(color) else {
        return 0;
    };
    let files = king_files(king);
    count(
        pieces(board, color, Role::Pawn)
            .into_iter()
            .filter(|pawn| {
                files.contains(&pawn.file()) && (2..=3).contains(&relative_rank(*pawn, color))
            })
            .collect(),
    )
}

/// Le case deboli intorno al re di `defender`: davanti al re (seconda e terza
/// traversa del difensore), che nessun suo pedone può più controllare.
fn holes_near_king(board: &Board, defender: Color) -> Vec<Square> {
    let Some(king) = board.king_of(defender) else {
        return Vec::new();
    };
    let own_pawns = pieces(board, defender, Role::Pawn);
    king_files(king)
        .into_iter()
        .flat_map(|file| {
            [2, 3]
                .into_iter()
                .map(move |rank| crate::board_query::relative_square(file, rank, defender))
        })
        .filter(|square| {
            // Nessun pedone del difensore, ora o avanzando, può attaccare la casa.
            let can_guard = own_pawns.into_iter().any(|pawn| {
                neighbour_files(square.file()).contains(&pawn.file())
                    && relative_rank(pawn, defender) < relative_rank(*square, defender)
            });
            !can_guard && !own_pawns.contains(*square)
        })
        .collect()
}

/// Gli squilibri di un lato.
fn side(board: &Board, color: Color, has_queens: bool) -> SideImbalances {
    let own_pawns = pieces(board, color, Role::Pawn);
    let isolated = own_pawns
        .into_iter()
        .filter(|pawn| {
            neighbour_files(pawn.file())
                .into_iter()
                .all(|file| !has_pawn_on_file(board, color, file))
        })
        .collect();
    let doubled = File::ALL
        .into_iter()
        .filter(|file| count(own_pawns & Bitboard::from_file(*file)) >= 2)
        .collect();
    let heavy = pieces(board, color, Role::Rook) | pieces(board, color, Role::Queen);
    let rooks = pieces(board, color, Role::Rook);
    let king = board.king_of(color);
    let castled = king.is_some_and(|king| {
        relative_rank(king, color) == 1 && !matches!(king.file(), File::D | File::E)
    });
    let home_minors = [File::B, File::C, File::F, File::G]
        .into_iter()
        .map(|file| crate::board_query::relative_square(file, 1, color))
        .filter(|square| {
            board.piece_at(*square).is_some_and(|piece| {
                piece.color == color && matches!(piece.role, Role::Knight | Role::Bishop)
            })
        })
        .count();
    let balance = material(board).advantage;
    SideImbalances {
        color,
        material: if color == Color::White {
            balance
        } else {
            -balance
        },
        bishop_pair: count(pieces(board, color, Role::Bishop)) >= 2
            && count(pieces(board, color.other(), Role::Bishop)) < 2,
        isolated,
        backward: own_pawns
            .into_iter()
            .filter(|pawn| is_backward(board, color, *pawn))
            .collect(),
        doubled,
        passed: own_pawns
            .into_iter()
            .filter(|pawn| is_passed_pawn(board, color, *pawn))
            .collect(),
        islands: islands(board, color),
        queenside_pawns: pawns_on(board, color, &QUEENSIDE),
        kingside_pawns: pawns_on(board, color, &KINGSIDE),
        space: space(board, color),
        worst_piece: worst_piece(board, color),
        bad_bishops: bad_bishops(board, color),
        files_with_rooks: File::ALL
            .into_iter()
            .filter(|file| {
                !has_pawn_on_file(board, color, *file) && (heavy & Bitboard::from_file(*file)).any()
            })
            .collect(),
        open_files_free: if rooks.is_empty() {
            Vec::new()
        } else {
            File::ALL
                .into_iter()
                .filter(|file| {
                    is_open_file(board, *file) && (rooks & Bitboard::from_file(*file)).is_empty()
                })
                .collect()
        },
        outposts: Bitboard::FULL
            .into_iter()
            .filter(|square| board.piece_at(*square).is_none() && is_outpost(board, *square, color))
            .filter(|square| matches!(square.file(), File::C | File::D | File::E | File::F))
            .collect(),
        // Le case deboli vicino al re contano solo se c'è un attacco possibile:
        // con le donne in gioco e il re avversario già arroccato (non al centro).
        holes_near_enemy_king: if has_queens
            && board
                .king_of(color.other())
                .is_some_and(|king| !matches!(king.file(), File::D | File::E))
        {
            holes_near_king(board, color.other())
        } else {
            Vec::new()
        },
        king_shield: king_shield(board, color),
        castled,
        king_in_center: has_queens
            && king.is_some_and(|king| matches!(king.file(), File::D | File::E)),
        undeveloped: u32::try_from(home_minors).unwrap_or(0),
        minors: (
            count(pieces(board, color, Role::Knight)),
            count(pieces(board, color, Role::Bishop)),
        ),
    }
}

/// Com'è il centro.
fn center_kind(board: &Board) -> CenterKind {
    let central = (Bitboard::from_file(File::D) | Bitboard::from_file(File::E)) & board.pawns();
    if count(central) <= 1 {
        return CenterKind::Open;
    }
    let blocked = (pieces(board, Color::White, Role::Pawn) & central)
        .into_iter()
        .filter(|pawn| {
            pawn.offset(8)
                .is_some_and(|front| pieces(board, Color::Black, Role::Pawn).contains(front))
        })
        .count();
    if blocked >= 2 {
        CenterKind::Closed
    } else {
        CenterKind::Mobile
    }
}

/// Il bilancio della posizione.
#[must_use]
pub fn imbalances(position: &Chess, phase: GamePhase) -> Imbalances {
    let board = position.board();
    let has_queens = board.queens().any();
    Imbalances {
        phase,
        center: center_kind(board),
        white: side(board, Color::White, has_queens),
        black: side(board, Color::Black, has_queens),
    }
}

#[cfg(test)]
mod tests {
    use analysis::GamePhase;
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color, File, Square};

    use super::{CenterKind, imbalances};

    fn position(fen: &str) -> Chess {
        fen.parse::<Fen>()
            .expect("FEN")
            .into_position(CastlingMode::Standard)
            .expect("posizione")
    }

    #[test]
    fn reads_structure_space_and_majorities() {
        // Bianco: pedoni a2 b2 d4 (isolato) e f2 g2 h2; Nero: a7 b7 c7, e6, f7 g7 h7.
        let found = imbalances(
            &position("4k3/ppp2ppp/4p3/8/3P4/8/PP3PPP/4K3 w - - 0 1"),
            GamePhase::Endgame,
        );

        assert_eq!(found.white.isolated, [Square::D4]);
        assert_eq!(found.black.queenside_pawns, 3);
        assert_eq!(found.white.queenside_pawns, 2);
        assert_eq!(found.white.islands, 3);
        assert_eq!(found.of(Color::Black).islands, 2);
        assert_eq!(found.center, CenterKind::Mobile);
    }

    #[test]
    fn finds_open_files_outposts_and_the_worst_piece() {
        // La torre bianca in a1 è chiusa dal pedone a2 e dal cavallo b1; colonna c aperta.
        let found = imbalances(
            &position("r3k3/pp3ppp/3p4/4p3/4P3/2N5/PP3PPP/RN2K2R w - - 0 1"),
            GamePhase::Middlegame,
        );

        assert!(found.white.open_files_free.contains(&File::C));
        assert!(found.white.outposts.contains(&Square::D5));
        assert!(found.white.worst_piece.is_some());
    }

    #[test]
    fn a_closed_center_is_recognized() {
        let found = imbalances(
            &position("4k3/pp4pp/4p3/2ppP3/3P4/2P5/PP4PP/4K3 w - - 0 1"),
            GamePhase::Middlegame,
        );

        assert_eq!(found.center, CenterKind::Closed);
    }
}
