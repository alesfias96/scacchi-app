//! Le tattiche "di linea": inchiodature, infilate e attacchi di scoperta.
//!
//! Alfieri, torri e donne agiscono lungo linee (diagonali, colonne, traverse).
//! Quando sulla linea ci sono due pezzi uno dietro l'altro nascono tre tattiche:
//! - **inchiodatura**: il pezzo davanti è nemico e dietro c'è un nemico più
//!   prezioso (o il re): se il pezzo davanti si sposta, si perde quello dietro;
//! - **infilata**: il pezzo davanti è nemico e più prezioso di quello dietro:
//!   deve spostarsi e lascia catturare quello dietro;
//! - **attacco di scoperta**: il pezzo davanti è nostro; spostandolo, il nostro
//!   pezzo a lunga gittata attacca il nemico dietro (se è il re: scacco di scoperta).

use shakmaty::attacks::{aligned, attacks, between};
use shakmaty::{Board, Piece, Role, Square};

use crate::exchange::exchange_gain;
use crate::fact::{Fact, FactCategory};
use crate::piece_value::piece_value;
use crate::wording::{piece_phrase, role_name};

/// Due pezzi uno dietro l'altro sulla linea di un pezzo a lunga gittata.
struct LineUp {
    /// Il pezzo a lunga gittata (alfiere, torre o donna) e la sua casa.
    slider: (Piece, Square),
    /// Il primo pezzo sulla linea.
    front: (Piece, Square),
    /// Il pezzo subito dietro.
    back: (Piece, Square),
}

/// Tutti gli allineamenti "pezzo a lunga gittata → pezzo → pezzo" della scacchiera.
fn line_ups(board: &Board) -> Vec<LineUp> {
    let sliders =
        board.by_role(Role::Bishop) | board.by_role(Role::Rook) | board.by_role(Role::Queen);
    let mut found = Vec::new();
    for slider_square in sliders {
        let Some(slider) = board.piece_at(slider_square) else {
            continue;
        };
        let direct = board.attacks_from(slider_square);
        for front_square in direct & board.occupied() {
            let Some(front) = board.piece_at(front_square) else {
                continue;
            };
            // Togliendo il pezzo davanti, che cosa "vede" in più il pezzo a lunga gittata?
            let through = attacks(
                slider_square,
                slider,
                board.occupied().without(front_square),
            );
            for back_square in (through & !direct) & board.occupied() {
                let behind_front = aligned(slider_square, front_square, back_square)
                    && between(slider_square, back_square).contains(front_square);
                if let (true, Some(back)) = (behind_front, board.piece_at(back_square)) {
                    found.push(LineUp {
                        slider: (slider, slider_square),
                        front: (front, front_square),
                        back: (back, back_square),
                    });
                }
            }
        }
    }
    found
}

/// Descrive un allineamento come inchiodatura, infilata o scoperta (se lo è).
fn describe(board: &Board, line_up: &LineUp) -> Option<Fact> {
    let (slider, slider_square) = line_up.slider;
    let (front, front_square) = line_up.front;
    let (back, back_square) = line_up.back;
    let attacker = slider.color;
    let enemy = attacker.other();
    if back.color != enemy {
        return None;
    }
    let text = if front.color == enemy {
        if back.role == Role::King {
            format!(
                "Inchiodatura: {} inchioda {} al re, che non può muoversi da lì",
                piece_phrase(slider, slider_square),
                piece_phrase(front, front_square)
            )
        } else if front.role == Role::King {
            format!(
                "Infilata: {} dà scacco e, quando il re si sposta, cattura {}",
                piece_phrase(slider, slider_square),
                piece_phrase(back, back_square)
            )
        } else if piece_value(back.role) > piece_value(front.role) && winnable(board, line_up) {
            format!(
                "Inchiodatura: {} inchioda {} {} in {back_square}",
                piece_phrase(slider, slider_square),
                piece_phrase(front, front_square),
                article_to(back.role)
            )
        } else if piece_value(front.role) > piece_value(back.role) && winnable(board, line_up) {
            format!(
                "Infilata: {} attacca {}; se si sposta, resta da catturare {}",
                piece_phrase(slider, slider_square),
                piece_phrase(front, front_square),
                piece_phrase(back, back_square)
            )
        } else {
            return None;
        }
    } else {
        // Attacco di scoperta: il pezzo davanti è nostro (e non è il re).
        let worth = back.role == Role::King
            || piece_value(back.role) > piece_value(slider.role)
            || exchange_gain(board, back_square, attacker) > 0;
        if front.role == Role::King || !worth {
            return None;
        }
        let kind = if back.role == Role::King {
            "Scacco di scoperta possibile"
        } else {
            "Attacco di scoperta possibile"
        };
        format!(
            "{kind}: spostando {}, {} attaccherebbe {}",
            piece_phrase(front, front_square),
            piece_phrase(slider, slider_square),
            piece_phrase(back, back_square)
        )
    };
    Some(
        Fact::new(FactCategory::Tactics, attacker, text)
            .with_squares([front_square, back_square])
            .with_arrow(slider_square, back_square),
    )
}

/// `true` se il pezzo dietro si può davvero vincere quando quello davanti si
/// sposta: è indifeso (nessuno lo protegge, a parte il pezzo davanti), oppure
/// vale più del pezzo a lunga gittata che lo catturerebbe.
///
/// Senza questo controllo si segnalerebbero "inchiodature" che non portano a
/// nulla, come una donna che punta un alfiere ben difeso.
fn winnable(board: &Board, line_up: &LineUp) -> bool {
    let (slider, _) = line_up.slider;
    let (_, front_square) = line_up.front;
    let (back, back_square) = line_up.back;
    let defenders = board
        .attacks_to(back_square, back.color, board.occupied())
        .without(front_square);
    defenders.is_empty() || piece_value(back.role) > piece_value(slider.role)
}

/// "alla donna", "al cavallo", "all'alfiere", "alla torre".
fn article_to(role: Role) -> String {
    let name = role_name(role);
    match role {
        Role::Bishop => format!("all'{name}"),
        Role::Rook | Role::Queen => format!("alla {name}"),
        _ => format!("al {name}"),
    }
}

/// Le inchiodature, le infilate e gli attacchi di scoperta della posizione.
#[must_use]
pub fn line_facts(board: &Board) -> Vec<Fact> {
    line_ups(board)
        .iter()
        .filter_map(|line_up| describe(board, line_up))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::line_facts;
    use crate::test_position::{position, texts};
    use shakmaty::Position;

    #[test]
    fn sees_a_pin_to_the_king() {
        // L'alfiere bianco in b5 inchioda il cavallo nero in c6 al re in e8.
        let game = position("4k3/8/2n5/1B6/8/8/8/4K3 w - - 0 1");

        assert_eq!(
            texts(&line_facts(game.board())),
            [
                "Inchiodatura: l'alfiere bianco in b5 inchioda il cavallo nero in c6 al re, che non può muoversi da lì"
            ]
        );
    }

    #[test]
    fn sees_a_pin_to_a_more_valuable_piece() {
        // La torre bianca in e1 inchioda l'alfiere nero in e5 alla donna in e8.
        let game = position("4q1k1/8/8/4b3/8/8/8/4R1K1 w - - 0 1");

        assert_eq!(
            texts(&line_facts(game.board())),
            ["Inchiodatura: la torre bianca in e1 inchioda l'alfiere nero in e5 alla donna in e8"]
        );
    }

    #[test]
    fn sees_a_skewer_through_the_king() {
        // La torre bianca in a1 dà scacco al re in a5; dietro c'è la donna in a8.
        let game = position("q7/8/8/k7/8/8/8/R5K1 b - - 0 1");

        let found = texts(&line_facts(game.board()));

        assert!(
            found
                .iter()
                .any(|text| text.starts_with("Infilata: la torre bianca in a1 dà scacco"))
        );
    }

    #[test]
    fn sees_a_discovered_check() {
        // Il cavallo bianco in e4 copre la torre in e1 dal re nero in e8.
        let game = position("4k3/8/8/8/4N3/8/8/4R1K1 w - - 0 1");

        assert_eq!(
            texts(&line_facts(game.board())),
            [
                "Scacco di scoperta possibile: spostando il cavallo bianco in e4, la torre bianca in e1 attaccherebbe il re nero in e8"
            ]
        );
    }

    #[test]
    fn ignores_a_pin_that_wins_nothing() {
        // La donna in f3 e l'alfiere in c4 puntano f7, ma dietro ci sono pezzi
        // difesi che valgono meno di loro: non sono vere inchiodature.
        let game = position("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 2 4");

        assert_eq!(line_facts(game.board()), Vec::new());
    }
}
