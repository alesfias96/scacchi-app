//! Lo scambio statico: chi guadagna se si comincia a catturare su una casa.

use shakmaty::{Bitboard, Board, Color, Role, Square};

use crate::piece_value::piece_value;

/// Il pezzo di `color` che può catturare su `target` con meno valore, tra
/// quelli ancora presenti in `occupied`: casa e tipo di pezzo.
pub(crate) fn least_valuable_attacker(
    board: &Board,
    target: Square,
    color: Color,
    occupied: Bitboard,
) -> Option<(Square, Role)> {
    (board.attacks_to(target, color, occupied) & occupied)
        .into_iter()
        .filter_map(|square| board.role_at(square).map(|role| (square, role)))
        .min_by_key(|(_, role)| piece_value(*role))
}

/// Quanto guadagna `attacker` (in pedoni) cominciando a catturare il pezzo su
/// `target`, se entrambi i giocatori continuano a catturare finché conviene.
///
/// È lo "scambio statico" che usano i giocatori: si cattura sempre con il
/// pezzo che vale meno, e ognuno può smettere quando non gli conviene più.
/// Restituisce 0 se la casa è vuota, se non si può catturare o se catturare
/// non conviene.
///
/// Semplificazione: non considera i pezzi inchiodati (un pezzo inchiodato al
/// re in realtà non potrebbe catturare). Per i fatti mostrati è abbastanza
/// preciso; il giudizio esatto resta a Stockfish.
#[must_use]
pub fn exchange_gain(board: &Board, target: Square, attacker: Color) -> i32 {
    let Some(victim) = board.role_at(target) else {
        return 0;
    };
    let mut occupied = board.occupied();
    // gains[i] = quanto guadagna chi fa la cattura numero i, se poi lo scambio
    // si ferma lì (la cattura i prende il pezzo che ha catturato al passo i - 1).
    let mut gains = vec![piece_value(victim)];
    let mut color = attacker;
    let Some((mut from, mut role)) = least_valuable_attacker(board, target, color, occupied) else {
        return 0;
    };
    loop {
        occupied.discard(from);
        let next_color = color.other();
        let next = least_valuable_attacker(board, target, next_color, occupied);
        // Il re non può catturare su una casa ancora difesa.
        if role == Role::King && next.is_some() {
            gains.pop();
            break;
        }
        let Some((next_from, next_role)) = next else {
            break;
        };
        let last = *gains.last().unwrap_or(&0);
        gains.push(piece_value(role) - last);
        color = next_color;
        from = next_from;
        role = next_role;
    }
    // Da dietro in avanti: chi dovrebbe ricatturare lo fa solo se gli conviene.
    // Se la cattura successiva gli rende più che fermarsi, il guadagno di chi
    // ha catturato prima si riduce di conseguenza.
    for step in (1..gains.len()).rev() {
        gains[step - 1] = -(-gains[step - 1]).max(gains[step]);
    }
    // Chi comincia può anche non catturare affatto: il guadagno non è mai negativo.
    gains.first().copied().unwrap_or(0).max(0)
}

#[cfg(test)]
mod tests {
    use super::exchange_gain;
    use shakmaty::fen::Fen;
    use shakmaty::{Board, CastlingMode, Chess, Color, Position, Square};

    fn board(fen: &str) -> Board {
        let position: Chess = fen
            .parse::<Fen>()
            .expect("FEN valida")
            .into_position(CastlingMode::Standard)
            .expect("posizione legale");
        position.board().clone()
    }

    #[test]
    fn an_undefended_piece_is_won_entirely() {
        // Il cavallo nero in e5 è attaccato dal pedone d4 e non è difeso.
        let board = board("4k3/8/8/4n3/3P4/8/8/4K3 w - - 0 1");

        assert_eq!(exchange_gain(&board, Square::E5, Color::White), 3);
    }

    #[test]
    fn a_defended_pawn_attacked_by_a_queen_is_not_worth_taking() {
        // Pedone e5 difeso da d6: la donna in e2 lo prenderebbe e verrebbe catturata.
        let board = board("4k3/8/3p4/4p3/8/8/4Q3/4K3 w - - 0 1");

        assert_eq!(exchange_gain(&board, Square::E5, Color::White), 0);
    }

    #[test]
    fn counts_the_whole_sequence_of_captures() {
        // Torre nera in d5 difesa dal pedone e6; il Bianco attacca con il pedone c4
        // e la torre d1: cxd5 exd5 Txd5. Vince torre (5) e pedone (1), perde un pedone (1).
        let board = board("4k3/8/4p3/3r4/2P5/8/8/3RK3 w - - 0 1");

        assert_eq!(exchange_gain(&board, Square::D5, Color::White), 5);
    }

    #[test]
    fn the_king_does_not_capture_a_defended_piece() {
        // Pedone nero in e2 difeso dalla torre in e8: il re bianco non può prenderlo.
        let board = board("4r1k1/8/8/8/8/8/4p3/4K3 w - - 0 1");

        assert_eq!(exchange_gain(&board, Square::E2, Color::White), 0);
    }
}
