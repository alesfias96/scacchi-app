//! L'attività dei pezzi: mobilità, cavalli sul bordo, alfieri "cattivi", torri sulle colonne aperte.

use shakmaty::{Bitboard, Board, Chess, Color, File, Piece, Position, Rank, Role, Square};

use crate::fact::{Fact, FactCategory};
use crate::game_phase::{GamePhase, game_phase};
use crate::wording::{piece_phrase_capitalized, side_name, side_name_capitalized};

/// Da quanti pedoni sulle case del suo colore in su un alfiere è "cattivo".
const BAD_BISHOP_PAWNS: usize = 4;
/// La differenza di mobilità che vale la pena segnalare.
const NOTABLE_MOBILITY_GAP: usize = 10;

/// Quante mosse legali avrebbe ciascun giocatore. Per chi non è di turno si
/// "passa il turno"; se non si può (chi muove è sotto scacco) si restituisce `None`.
fn mobility(position: &Chess) -> Option<(usize, usize)> {
    let mover = position.legal_moves().len();
    let other = position.clone().swap_turn().ok()?.legal_moves().len();
    Some(match position.turn() {
        Color::White => (mover, other),
        Color::Black => (other, mover),
    })
}

/// La settima traversa vista da un colore (la seconda per il Nero).
fn seventh_rank(color: Color) -> Rank {
    match color {
        Color::White => Rank::Seventh,
        Color::Black => Rank::Second,
    }
}

/// Un fatto su un solo pezzo.
fn about(piece: Piece, square: Square, text: &str) -> Fact {
    Fact::new(
        FactCategory::Activity,
        piece.color,
        format!("{} {text}", piece_phrase_capitalized(piece, square)),
    )
    .with_squares([square])
}

/// I fatti sui singoli pezzi: cavalli sul bordo, alfieri cattivi, torri attive.
///
/// L'alfiere "cattivo" non si segnala in apertura: lì gli alfieri sono ancora
/// sulle case di partenza e i pedoni non hanno ancora trovato il loro posto.
fn piece_facts(board: &Board, phase: GamePhase) -> Vec<Fact> {
    let mut facts = Vec::new();
    for square in board.occupied() {
        let Some(piece) = board.piece_at(square) else {
            continue;
        };
        let own_pawns = board.pawns() & board.by_color(piece.color);
        match piece.role {
            Role::Knight if matches!(square.file(), File::A | File::H) => {
                facts.push(about(
                    piece,
                    square,
                    "è sul bordo: da lì controlla poche case",
                ));
            }
            Role::Bishop if phase != GamePhase::Opening => {
                let same_color = if square.is_light() {
                    Bitboard::LIGHT_SQUARES
                } else {
                    Bitboard::DARK_SQUARES
                };
                let blocking = (own_pawns & same_color).count();
                if blocking >= BAD_BISHOP_PAWNS && blocking * 2 > own_pawns.count() {
                    facts.push(about(
                        piece,
                        square,
                        &format!("è \"cattivo\": {blocking} dei suoi pedoni stanno su case del suo colore e lo chiudono"),
                    ));
                }
            }
            Role::Rook => {
                let file = Bitboard::from_file(square.file());
                if (file & board.pawns()).is_empty() {
                    facts.push(about(
                        piece,
                        square,
                        "è su una colonna aperta (senza pedoni)",
                    ));
                } else if (file & own_pawns).is_empty() {
                    facts.push(about(
                        piece,
                        square,
                        "è su una colonna semiaperta (senza pedoni amici)",
                    ));
                }
                if square.rank() == seventh_rank(piece.color) {
                    facts.push(about(
                        piece,
                        square,
                        "è in settima traversa, dove attacca i pedoni rimasti indietro",
                    ));
                }
            }
            _ => {}
        }
    }
    facts
}

/// I fatti sull'attività dei pezzi di entrambi i giocatori.
#[must_use]
pub fn activity_facts(position: &Chess) -> Vec<Fact> {
    let mut facts = piece_facts(position.board(), game_phase(position));
    if let Some((white, black)) = mobility(position)
        && white.abs_diff(black) >= NOTABLE_MOBILITY_GAP
    {
        let (leader, more, less) = if white > black {
            (Color::White, white, black)
        } else {
            (Color::Black, black, white)
        };
        facts.push(Fact::new(
            FactCategory::Activity,
            leader,
            format!(
                "{} ha i pezzi più liberi: {more} mosse possibili contro {less} per {}",
                side_name_capitalized(leader),
                side_name(leader.other())
            ),
        ));
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::activity_facts;
    use crate::test_position::{position, texts};

    #[test]
    fn sees_a_knight_on_the_rim_and_a_rook_on_an_open_file() {
        let game = position("4k3/pp6/8/8/8/7N/PP6/3RK3 w - - 0 1");

        let found = texts(&activity_facts(&game));

        assert!(found.contains(
            &"Il cavallo bianco in h3 è sul bordo: da lì controlla poche case".to_owned()
        ));
        assert!(
            found.contains(
                &"La torre bianca in d1 è su una colonna aperta (senza pedoni)".to_owned()
            )
        );
    }

    #[test]
    fn sees_a_bad_bishop() {
        // Alfiere bianco in c1, su casa scura; i pedoni b2, d2, e3, f4 e g3 sono tutti su case scure.
        let game = position("4k3/8/8/8/5P2/4P1P1/1P1P4/2B1K3 w - - 0 30");

        let found = texts(&activity_facts(&game));

        assert!(
            found
                .iter()
                .any(|text| text.starts_with("L'alfiere bianco in c1 è \"cattivo\""))
        );
    }

    #[test]
    fn compares_mobility_when_the_gap_is_large() {
        // Il Bianco ha donna e torre, il Nero solo il re.
        let game = position("7k/8/8/8/8/8/Q7/4R1K1 w - - 0 1");

        let found = texts(&activity_facts(&game));

        assert!(
            found
                .iter()
                .any(|text| text.starts_with("Il Bianco ha i pezzi più liberi"))
        );
    }
}
