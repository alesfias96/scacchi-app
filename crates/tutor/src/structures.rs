//! Le strutture pedonali tipiche: la disposizione dei pedoni decide i piani.
//!
//! Ogni regola guarda la scacchiera dal lato di chi "ha" la struttura (vedi
//! `board_query`), e si prova per entrambi i colori.

use shakmaty::{Board, Color, File, Square};

use analysis::wording::side_name_capitalized;

use crate::board_query::{has_pawn_on_file, is_open_file, pawn_at, relative_square};
use crate::recognition::Recognition;
use crate::topic::Topic;

/// Il pedone isolato di donna (l'"isolano").
pub const ISOLATED_QUEEN_PAWN: &str = "isolated-queen-pawn";
/// I pedoni sospesi in c e d.
pub const HANGING_PAWNS: &str = "hanging-pawns";
/// La struttura Carlsbad (Gambetto di Donna, variante di cambio).
pub const CARLSBAD: &str = "carlsbad";
/// La morsa Maróczy (pedoni in c4 ed e4 contro la Siciliana).
pub const MAROCZY_BIND: &str = "maroczy-bind";
/// Lo Stonewall (pedoni in d4, e3, f4).
pub const STONEWALL: &str = "stonewall";
/// La catena della Francese (e5–d4 contro e6–d5).
pub const FRENCH_CHAIN: &str = "french-chain";
/// La catena della Est-Indiana (d5–e4 contro d6–e5).
pub const KINGS_INDIAN_CHAIN: &str = "kings-indian-chain";
/// La struttura Benoni (d5 contro c5–d6).
pub const BENONI: &str = "benoni";
/// Il centro aperto (nessun pedone sulle colonne d ed e).
pub const OPEN_CENTER: &str = "open-center";

/// Una struttura riconosciuta per `owner`.
fn found(
    key: &'static str,
    owner: Option<Color>,
    text: String,
    squares: Vec<Square>,
) -> Recognition {
    Recognition {
        topic: Topic::Structure,
        key,
        owner,
        text,
        squares,
    }
}

/// Le case di `color` indicate (colonna, traversa vista da `color`).
fn squares_of(color: Color, cells: &[(File, u32)]) -> Vec<Square> {
    cells
        .iter()
        .map(|(file, rank)| relative_square(*file, *rank, color))
        .collect()
}

/// Le strutture con pedoni isolati o sospesi di cui `owner` è "proprietario".
fn isolated_structures(board: &Board, owner: Color) -> Vec<Recognition> {
    let side = side_name_capitalized(owner);
    let mut result = Vec::new();
    let pawn = |color: Color, file: File, rank: u32| pawn_at(board, color, file, rank);
    let on_file = |color: Color, file: File| has_pawn_on_file(board, color, file);

    // L'isolano: pedone in d4 (o già in d5), senza pedoni amici in c ed e.
    if let Some(rank) = [4, 5].into_iter().find(|rank| pawn(owner, File::D, *rank))
        && !on_file(owner, File::C)
        && !on_file(owner, File::E)
    {
        let square = relative_square(File::D, rank, owner);
        result.push(found(
            ISOLATED_QUEEN_PAWN,
            Some(owner),
            format!("{side} ha un pedone isolato di donna in {square}"),
            vec![square],
        ));
    }

    // I pedoni sospesi: c e d affiancati, senza pedoni amici in b ed e.
    if let Some(rank) = [3, 4]
        .into_iter()
        .find(|rank| pawn(owner, File::C, *rank) && pawn(owner, File::D, *rank))
        && !on_file(owner, File::B)
        && !on_file(owner, File::E)
    {
        let squares = squares_of(owner, &[(File::C, rank), (File::D, rank)]);
        result.push(found(
            HANGING_PAWNS,
            Some(owner),
            format!(
                "{side} ha i pedoni sospesi in {} e {}",
                squares[0], squares[1]
            ),
            squares,
        ));
    }

    result
}

/// Le strutture con catene e blocchi di pedoni di cui `owner` è "proprietario".
fn blocked_structures(board: &Board, owner: Color) -> Vec<Recognition> {
    let enemy = owner.other();
    let side = side_name_capitalized(owner);
    let mut result = Vec::new();
    let pawn = |color: Color, file: File, rank: u32| pawn_at(board, color, file, rank);
    let on_file = |color: Color, file: File| has_pawn_on_file(board, color, file);

    // Carlsbad: d4 ed e3 contro c6 e d5; nessun pedone in c per chi attacca,
    // nessun pedone in e per l'altro.
    if pawn(owner, File::D, 4)
        && !on_file(owner, File::C)
        && (pawn(owner, File::E, 2) || pawn(owner, File::E, 3))
        && pawn(enemy, File::D, 4)
        && (pawn(enemy, File::C, 2) || pawn(enemy, File::C, 3))
        && !on_file(enemy, File::E)
    {
        result.push(found(
            CARLSBAD,
            Some(owner),
            format!(
                "Struttura Carlsbad: {side} può preparare l'attacco di minoranza sull'ala di donna"
            ),
            squares_of(owner, &[(File::D, 4)]),
        ));
    }

    // Maróczy: c4 ed e4 senza pedone in d, contro un pedone in d6 senza pedone in c.
    if pawn(owner, File::C, 4)
        && pawn(owner, File::E, 4)
        && !on_file(owner, File::D)
        && on_file(enemy, File::D)
        && !on_file(enemy, File::C)
    {
        result.push(found(
            MAROCZY_BIND,
            Some(owner),
            format!("{side} stringe la morsa Maróczy con i pedoni in c e in e"),
            squares_of(owner, &[(File::C, 4), (File::E, 4)]),
        ));
    }

    // Stonewall: d4, e3, f4.
    if pawn(owner, File::D, 4) && pawn(owner, File::E, 3) && pawn(owner, File::F, 4) {
        result.push(found(
            STONEWALL,
            Some(owner),
            format!("{side} ha la struttura Stonewall (il \"muro di pietra\")"),
            squares_of(owner, &[(File::D, 4), (File::E, 3), (File::F, 4)]),
        ));
    }

    // Catena della Francese: e5 e d4 contro e6 e d5.
    if pawn(owner, File::E, 5)
        && pawn(owner, File::D, 4)
        && pawn(enemy, File::E, 3)
        && pawn(enemy, File::D, 4)
    {
        result.push(found(
            FRENCH_CHAIN,
            Some(owner),
            format!(
                "Catena di pedoni come nella Difesa Francese: {side} ha più spazio sull'ala di re"
            ),
            squares_of(owner, &[(File::D, 4), (File::E, 5)]),
        ));
    }

    // Catena della Est-Indiana: d5 ed e4 contro d6 ed e5.
    if pawn(owner, File::D, 5)
        && pawn(owner, File::E, 4)
        && pawn(enemy, File::D, 3)
        && pawn(enemy, File::E, 4)
    {
        result.push(found(
            KINGS_INDIAN_CHAIN,
            Some(owner),
            format!("Centro chiuso come nella Est-Indiana: {side} gioca sull'ala di donna, l'avversario sull'ala di re"),
            squares_of(owner, &[(File::D, 5), (File::E, 4)]),
        ));
    }

    // Benoni: d5 senza pedone in c, contro c5 e d6 senza pedone in e.
    if pawn(owner, File::D, 5)
        && !on_file(owner, File::C)
        && pawn(enemy, File::C, 4)
        && pawn(enemy, File::D, 3)
        && !on_file(enemy, File::E)
    {
        result.push(found(
            BENONI,
            Some(owner),
            format!("Struttura Benoni: {side} ha la maggioranza al centro, l'avversario sull'ala di donna"),
            squares_of(owner, &[(File::D, 5)]),
        ));
    }

    result
}

/// Le strutture pedonali riconosciute nella posizione (per entrambi i colori).
#[must_use]
pub fn recognize_structures(board: &Board) -> Vec<Recognition> {
    let mut result = Vec::new();
    for color in [Color::White, Color::Black] {
        result.extend(isolated_structures(board, color));
        result.extend(blocked_structures(board, color));
    }
    // Il centro aperto vale per tutti e due; conta solo con pedoni sulla scacchiera.
    if is_open_file(board, File::D) && is_open_file(board, File::E) && board.pawns().count() >= 6 {
        result.push(found(
            OPEN_CENTER,
            None,
            "Il centro è aperto: non ci sono pedoni sulle colonne d ed e".to_owned(),
            Vec::new(),
        ));
    }
    result
}

#[cfg(test)]
mod tests {
    use shakmaty::fen::Fen;
    use shakmaty::{CastlingMode, Chess, Color, Position};

    use super::{
        CARLSBAD, ISOLATED_QUEEN_PAWN, KINGS_INDIAN_CHAIN, OPEN_CENTER, recognize_structures,
    };

    fn keys(fen: &str) -> Vec<(&'static str, Option<Color>)> {
        let position: Chess = fen
            .parse::<Fen>()
            .expect("FEN valida")
            .into_position(CastlingMode::Standard)
            .expect("posizione legale");
        recognize_structures(position.board())
            .into_iter()
            .map(|found| (found.key, found.owner))
            .collect()
    }

    #[test]
    fn finds_the_isolated_queen_pawn() {
        let found = keys("r1bq1rk1/pp2bppp/2n1pn2/8/3P4/2NB1N2/PP3PPP/R1BQ1RK1 w - - 0 9");

        assert!(found.contains(&(ISOLATED_QUEEN_PAWN, Some(Color::White))));
    }

    #[test]
    fn finds_the_carlsbad_structure() {
        let found = keys("r1bq1rk1/pp1nbppp/2p2n2/3p2B1/3P4/2NBP3/PPQ2PPP/R3K1NR w KQ - 0 9");

        assert!(found.contains(&(CARLSBAD, Some(Color::White))));
    }

    #[test]
    fn finds_the_kings_indian_chain() {
        let found = keys("r1bq1rk1/pppnn1bp/3p2p1/3Ppp2/2P1P3/2N2N2/PP2BPPP/R1BQ1RK1 w - - 0 10");

        assert!(found.contains(&(KINGS_INDIAN_CHAIN, Some(Color::White))));
    }

    #[test]
    fn finds_an_open_center() {
        let found = keys("r1bq1rk1/ppp2ppp/2n2n2/2b5/2B5/2N2N2/PPP2PPP/R1BQ1RK1 w - - 0 8");

        assert!(found.contains(&(OPEN_CENTER, None)));
    }

    #[test]
    fn the_starting_position_has_no_special_structure() {
        assert_eq!(
            keys("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            []
        );
    }
}
