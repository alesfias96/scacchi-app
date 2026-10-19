//! La sicurezza dei re: scacco, arrocco, scudo di pedoni, colonne aperte, pezzi che puntano il re.

use shakmaty::attacks::king_attacks;
use shakmaty::{Bitboard, Board, CastlingSide, Chess, Color, File, Position, Rank, Role, Square};

use crate::fact::{Fact, FactCategory};
use crate::game_phase::{GamePhase, game_phase};
use crate::wording::{join_list, piece_phrase, side_name_capitalized};

/// Lo scudo è debole con al più questo numero di colonne protette da un pedone.
const WEAK_SHIELD_FILES: usize = 1;
/// Da quanti pezzi nemici che puntano la zona del re in su c'è "pressione".
const PRESSURE_ATTACKERS: usize = 3;

/// "re bianco" / "re nero".
fn king_name(color: Color) -> &'static str {
    match color {
        Color::White => "re bianco",
        Color::Black => "re nero",
    }
}

/// "bianchi" / "neri".
fn plural_color(color: Color) -> &'static str {
    match color {
        Color::White => "bianchi",
        Color::Black => "neri",
    }
}

/// Le colonne del re e quelle vicine.
fn files_around(king: Square) -> Vec<File> {
    [
        king.file().offset(-1),
        Some(king.file()),
        king.file().offset(1),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// La traversa di partenza del colore e le due davanti (dove stanno i pedoni dello scudo).
fn home_ranks(color: Color) -> (Rank, Bitboard) {
    match color {
        Color::White => (
            Rank::First,
            Bitboard::from_rank(Rank::Second) | Bitboard::from_rank(Rank::Third),
        ),
        Color::Black => (
            Rank::Eighth,
            Bitboard::from_rank(Rank::Seventh) | Bitboard::from_rank(Rank::Sixth),
        ),
    }
}

/// I fatti sul re arroccato (o comunque portato su un'ala): scudo e colonne aperte.
fn shelter_facts(board: &Board, color: Color, king: Square) -> Vec<Fact> {
    let (_, shield_ranks) = home_ranks(color);
    let own_pawns = board.pawns() & board.by_color(color);
    let mut facts = Vec::new();
    let shielded = files_around(king)
        .into_iter()
        .filter(|file| !(Bitboard::from_file(*file) & shield_ranks & own_pawns).is_empty())
        .count();
    if shielded <= WEAK_SHIELD_FILES {
        facts.push(
            Fact::new(
                FactCategory::Kings,
                color,
                format!(
                    "Lo scudo di pedoni davanti al {} è debole",
                    king_name(color)
                ),
            )
            .with_squares([king]),
        );
    }
    for file in files_around(king) {
        if (Bitboard::from_file(file) & own_pawns).is_empty() {
            facts.push(
                Fact::new(
                    FactCategory::Kings,
                    color.other(),
                    format!(
                        "La colonna {} vicino al {} non ha pedoni {}: è una via d'attacco",
                        file.char(),
                        king_name(color),
                        plural_color(color)
                    ),
                )
                .with_squares([Square::from_coords(file, king.rank())]),
            );
        }
    }
    facts
}

/// Il fatto "pressione sul re": almeno tre pezzi nemici puntano le case vicine al re.
fn pressure_fact(board: &Board, color: Color, king: Square) -> Option<Fact> {
    let enemy = color.other();
    let zone = king_attacks(king).with(king);
    let attackers: Vec<Square> = board
        .by_color(enemy)
        .into_iter()
        .filter(|square| {
            !matches!(board.role_at(*square), Some(Role::Pawn | Role::King))
                && !(board.attacks_from(*square) & zone).is_empty()
        })
        .collect();
    if attackers.len() < PRESSURE_ATTACKERS {
        return None;
    }
    let names: Vec<String> = attackers
        .iter()
        .filter_map(|square| {
            board
                .piece_at(*square)
                .map(|piece| piece_phrase(piece, *square))
        })
        .collect();
    let mut fact = Fact::new(
        FactCategory::Kings,
        enemy,
        format!(
            "{} preme sul {}: {} puntano le case vicine al re",
            side_name_capitalized(enemy),
            king_name(color),
            join_list(&names)
        ),
    )
    .with_squares([king]);
    for square in attackers {
        fact = fact.with_arrow(square, king);
    }
    Some(fact)
}

/// I fatti sulla sicurezza di entrambi i re.
///
/// Nel finale il re deve essere attivo: scudo, arrocco e pressione non si
/// segnalano più (resta solo lo scacco).
#[must_use]
pub fn king_facts(position: &Chess) -> Vec<Fact> {
    let board = position.board();
    let phase = game_phase(position);
    let mut facts = Vec::new();
    for color in [Color::White, Color::Black] {
        let Some(king) = board.king_of(color) else {
            continue;
        };
        if position.turn() == color && position.is_check() {
            facts.push(
                Fact::new(
                    FactCategory::Kings,
                    color.other(),
                    format!("Il {} è sotto scacco", king_name(color)),
                )
                .with_squares([king]),
            );
        }
        if phase == GamePhase::Endgame {
            continue;
        }
        let (home, _) = home_ranks(color);
        let on_home_rank = king.rank() == home;
        let can_castle = position.castles().has(color, CastlingSide::KingSide)
            || position.castles().has(color, CastlingSide::QueenSide);
        if on_home_rank && king.file() == File::E && !can_castle && phase == GamePhase::Middlegame {
            facts.push(
                Fact::new(
                    FactCategory::Kings,
                    color,
                    format!(
                        "Il {} è rimasto al centro e non può più arroccare",
                        king_name(color)
                    ),
                )
                .with_squares([king]),
            );
        }
        let on_a_wing = matches!(
            king.file(),
            File::A | File::B | File::C | File::F | File::G | File::H
        );
        if on_home_rank && on_a_wing {
            facts.extend(shelter_facts(board, color, king));
        }
        facts.extend(pressure_fact(board, color, king));
    }
    facts
}

#[cfg(test)]
mod tests {
    use super::king_facts;
    use crate::test_position::{position, texts};
    use chess_core::Side;

    #[test]
    fn reports_check() {
        let game = position("4k3/8/8/8/8/8/4q3/4K3 w - - 0 1");

        assert!(texts(&king_facts(&game)).contains(&"Il re bianco è sotto scacco".to_owned()));
    }

    #[test]
    fn sees_a_weak_shield_and_an_open_file() {
        // Re bianco in g1 con un solo pedone (h2) davanti: le colonne f e g sono senza pedoni.
        let game =
            position("r1bq1rk1/pppp1ppp/2n2n2/2b1p3/2B1P3/2NP1N2/PPP2Q1P/R1B2RK1 w - - 0 20");

        let found = texts(&king_facts(&game));

        assert!(found.contains(&"Lo scudo di pedoni davanti al re bianco è debole".to_owned()));
        assert!(
            found
                .iter()
                .any(|text| text.starts_with("La colonna g vicino al re bianco"))
        );
    }

    #[test]
    fn sees_pieces_aiming_at_the_king() {
        // Torre f8, alfiere c5, cavallo g4 e donna h4 neri puntano le case vicine al re in g1.
        let game = position("5rk1/8/8/2b5/6nq/8/5PPP/R2Q2K1 w - - 0 20");

        let facts = king_facts(&game);

        let pressure = facts
            .iter()
            .find(|fact| fact.text.starts_with("Il Nero preme sul re bianco"))
            .expect("pressione trovata");
        assert_eq!(pressure.side, Side::Black);
        assert_eq!(pressure.arrows.len(), 4);
    }
}
