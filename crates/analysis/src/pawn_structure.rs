//! La struttura dei pedoni: isolati, doppiati, passati, arretrati, catene e isole.

use shakmaty::attacks::pawn_attacks;
use shakmaty::{Bitboard, Board, Color, File, Piece, Rank, Role, Square};

use crate::fact::{Fact, FactCategory};
use crate::wording::{join_list, piece_phrase_capitalized, side_name_capitalized};

/// Quanti pezzi ci vogliono in una catena perché valga la pena segnalarla.
const MIN_CHAIN_LENGTH: usize = 3;
/// Da quante isole di pedoni in su la struttura è segnalata come frammentata.
const MANY_ISLANDS: usize = 3;

/// Le case di una colonna.
fn file_squares(file: File) -> Bitboard {
    Bitboard::from_file(file)
}

/// Le colonne vicine (a sinistra e a destra), quelle che esistono.
fn adjacent_files(file: File) -> Bitboard {
    [file.offset(-1), file.offset(1)]
        .into_iter()
        .flatten()
        .map(file_squares)
        .fold(Bitboard::EMPTY, |all, next| all | next)
}

/// La traversa "vista da" un colore: per il Nero la prima è l'ottava.
fn relative_rank(color: Color, rank: Rank) -> u32 {
    match color {
        Color::White => rank.to_u32(),
        Color::Black => rank.flip_vertical().to_u32(),
    }
}

/// Le case davanti a `square` (verso la promozione di `color`), sulle colonne indicate.
fn ahead(color: Color, square: Square, files: Bitboard) -> Bitboard {
    let mine = relative_rank(color, square.rank());
    files
        .into_iter()
        .filter(|other| relative_rank(color, other.rank()) > mine)
        .collect()
}

/// Un pedone di `color` in `square`.
fn pawn(color: Color) -> Piece {
    Piece {
        color,
        role: Role::Pawn,
    }
}

/// I fatti sulla struttura pedonale di entrambi i giocatori.
#[must_use]
pub fn pawn_facts(board: &Board) -> Vec<Fact> {
    let mut facts = Vec::new();
    for color in [Color::White, Color::Black] {
        let mine = board.pawns() & board.by_color(color);
        let theirs = board.pawns() & board.by_color(color.other());
        facts.extend(doubled(color, mine));
        for square in mine {
            let file = square.file();
            let neighbours = adjacent_files(file);
            let phrase = piece_phrase_capitalized(pawn(color), square);
            if (mine & neighbours).is_empty() {
                facts.push(single(color, square, format!("{phrase} è isolato: nessun pedone amico sulle colonne vicine può difenderlo")));
            } else if is_backward(board, color, square, mine) {
                facts.push(single(color, square, format!("{phrase} è arretrato: è rimasto indietro e la casa davanti è controllata da un pedone nemico")));
            }
            if (ahead(color, square, file_squares(file) | neighbours) & theirs).is_empty() {
                facts.push(single(
                    color,
                    square,
                    format!("{phrase} è passato: nessun pedone nemico può fermarlo"),
                ));
            }
        }
        facts.extend(chains(color, mine));
        facts.extend(islands(color, mine));
    }
    facts
}

/// Un fatto su un solo pedone.
fn single(color: Color, square: Square, text: String) -> Fact {
    Fact::new(FactCategory::Pawns, color, text).with_squares([square])
}

/// I pedoni doppiati: due o più sulla stessa colonna.
fn doubled(color: Color, mine: Bitboard) -> Vec<Fact> {
    File::ALL
        .into_iter()
        .filter_map(|file| {
            let on_file = mine & file_squares(file);
            (on_file.count() >= 2).then(|| {
                Fact::new(
                    FactCategory::Pawns,
                    color,
                    format!(
                        "Pedoni {} doppiati sulla colonna {}: si ostacolano a vicenda",
                        if color == Color::White {
                            "bianchi"
                        } else {
                            "neri"
                        },
                        file.char()
                    ),
                )
                .with_squares(on_file)
            })
        })
        .collect()
}

/// `true` se il pedone è arretrato: nessun pedone amico sulle colonne vicine
/// alla sua altezza o più indietro (nessuno può difenderlo avanzando), e la
/// casa davanti a lui è controllata da un pedone nemico.
fn is_backward(board: &Board, color: Color, square: Square, mine: Bitboard) -> bool {
    let rank = relative_rank(color, square.rank());
    let supporters = (mine & adjacent_files(square.file()))
        .into_iter()
        .any(|other| relative_rank(color, other.rank()) <= rank);
    let step = if color == Color::White { 8 } else { -8 };
    let Some(front) = square.offset(step) else {
        return false;
    };
    let enemy_pawns = board.pawns() & board.by_color(color.other());
    let controlled = !(pawn_attacks(color, front) & enemy_pawns).is_empty();
    !supporters && controlled
}

/// Le catene: pedoni che si difendono in diagonale, uno dietro l'altro.
fn chains(color: Color, mine: Bitboard) -> Vec<Fact> {
    let mut seen = Bitboard::EMPTY;
    let mut facts = Vec::new();
    for start in mine {
        if seen.contains(start) {
            continue;
        }
        // Raccoglie tutti i pedoni collegati da difese diagonali.
        let mut chain = Vec::new();
        let mut todo = vec![start];
        while let Some(square) = todo.pop() {
            if seen.contains(square) {
                continue;
            }
            seen.add(square);
            chain.push(square);
            let linked = (pawn_attacks(color, square) | pawn_attacks(color.other(), square)) & mine;
            todo.extend(linked);
        }
        if chain.len() >= MIN_CHAIN_LENGTH {
            chain.sort_by_key(|square| relative_rank(color, square.rank()));
            let names: Vec<String> = chain.iter().map(ToString::to_string).collect();
            let base = chain[0];
            facts.push(
                Fact::new(
                    FactCategory::Pawns,
                    color,
                    format!(
                        "Catena di pedoni {}: {} (la base, il punto debole, è in {base})",
                        if color == Color::White {
                            "bianchi"
                        } else {
                            "neri"
                        },
                        join_list(&names)
                    ),
                )
                .with_squares(chain),
            );
        }
    }
    facts
}

/// Le isole: gruppi di pedoni su colonne vicine, separati da colonne senza pedoni.
fn islands(color: Color, mine: Bitboard) -> Option<Fact> {
    let mut count = 0;
    let mut previous_had_pawn = false;
    for file in File::ALL {
        let has_pawn = !(mine & file_squares(file)).is_empty();
        if has_pawn && !previous_had_pawn {
            count += 1;
        }
        previous_had_pawn = has_pawn;
    }
    (count >= MANY_ISLANDS).then(|| {
        Fact::new(
            FactCategory::Pawns,
            color,
            format!(
                "{} ha {count} isole di pedoni: più isole, più punti deboli da difendere",
                side_name_capitalized(color)
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::pawn_facts;
    use crate::test_position::{position, texts};
    use shakmaty::Position;

    #[test]
    fn recognises_isolated_and_passed_pawns() {
        // Bianco: pedone isolato in d4. Nero: pedone passato in a5.
        let game = position("4k3/8/8/p7/3P4/8/8/4K3 w - - 0 1");

        let found = texts(&pawn_facts(game.board()));

        assert!(found.contains(&"Il pedone bianco in d4 è isolato: nessun pedone amico sulle colonne vicine può difenderlo".to_owned()));
        assert!(found.contains(
            &"Il pedone nero in a5 è passato: nessun pedone nemico può fermarlo".to_owned()
        ));
    }

    #[test]
    fn recognises_doubled_pawns_and_islands() {
        // Bianco: pedoni doppiati in c2 e c3, e quattro isole (colonne a, c, e, g).
        let game = position("4k3/8/8/8/8/2P5/P1P1P1P1/4K3 w - - 0 1");

        let found = texts(&pawn_facts(game.board()));

        assert!(found.contains(
            &"Pedoni bianchi doppiati sulla colonna c: si ostacolano a vicenda".to_owned()
        ));
        assert!(found.contains(
            &"Il Bianco ha 4 isole di pedoni: più isole, più punti deboli da difendere".to_owned()
        ));
    }

    #[test]
    fn recognises_a_backward_pawn() {
        // Il pedone nero in d6 è rimasto indietro rispetto a c5 ed e5, e la
        // casa d5 davanti a lui è controllata dal pedone bianco in e4.
        let game = position("4k3/8/3p4/2p1p3/4P3/8/8/4K3 w - - 0 1");

        let found = texts(&pawn_facts(game.board()));

        assert!(
            found
                .iter()
                .any(|text| text.starts_with("Il pedone nero in d6 è arretrato"))
        );
    }

    #[test]
    fn recognises_a_chain() {
        // Catena bianca c3-d4-e5: ogni pedone difende quello davanti.
        let game = position("4k3/8/8/4P3/3P4/2P5/8/4K3 w - - 0 1");

        let found = texts(&pawn_facts(game.board()));

        assert!(found.contains(
            &"Catena di pedoni bianchi: c3, d4 e e5 (la base, il punto debole, è in c3)".to_owned()
        ));
    }
}
