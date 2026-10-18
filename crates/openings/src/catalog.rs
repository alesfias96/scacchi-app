//! L'elenco delle aperture, pronto per la ricerca.

use std::collections::HashMap;
use std::sync::LazyLock;

use shakmaty::fen::Epd;
use shakmaty::san::San;
use shakmaty::{Chess, EnPassantMode, Position};

use crate::opening::Opening;

/// Il file delle aperture di Lichess, incluso nell'app durante la compilazione.
/// Una riga di intestazione, poi `eco<TAB>nome<TAB>mosse` per ogni apertura.
const SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../data/openings/lichess-openings.tsv"
));

/// Le aperture, indicizzate per posizione.
///
/// La chiave è la posizione raggiunta dalle mosse dell'apertura, scritta in
/// formato EPD (una FEN senza i contatori delle mosse): così un'apertura viene
/// riconosciuta anche se ci si arriva con un ordine di mosse diverso.
/// L'elenco viene preparato la prima volta che serve e poi riusato.
static CATALOG: LazyLock<HashMap<String, Opening>> = LazyLock::new(|| build(SOURCE));

/// L'apertura che corrisponde a una posizione, se c'è nell'elenco.
pub(crate) fn lookup(position: &Chess) -> Option<&'static Opening> {
    CATALOG.get(&position_key(position))
}

/// La chiave di ricerca di una posizione (formato EPD).
pub(crate) fn position_key(position: &Chess) -> String {
    Epd::from_position(position, EnPassantMode::Legal).to_string()
}

/// Prepara l'elenco a partire dal testo del file.
///
/// Le righe scritte male (non dovrebbero esserci) vengono saltate.
fn build(source: &str) -> HashMap<String, Opening> {
    source
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut columns = line.split('\t');
            let (eco, name, moves) = (columns.next()?, columns.next()?, columns.next()?);
            let position = play_line(moves)?;
            Some((
                position_key(&position),
                Opening {
                    eco: eco.to_owned(),
                    name: name.to_owned(),
                },
            ))
        })
        .collect()
}

/// Gioca le mosse di un'apertura (es. `"1. e4 c5 2. Nf3"`) dalla posizione iniziale.
fn play_line(moves: &str) -> Option<Chess> {
    let mut position = Chess::default();
    // I numeri delle mosse ("1.", "2.") non sono mosse: si saltano.
    for word in moves.split_whitespace().filter(|word| !word.ends_with('.')) {
        let chess_move = word.parse::<San>().ok()?.to_move(&position).ok()?;
        position.play_unchecked(chess_move);
    }
    Some(position)
}

#[cfg(test)]
mod tests {
    use super::{CATALOG, build};

    #[test]
    fn reads_the_whole_lichess_list() {
        // L'elenco di Lichess ha alcune migliaia di aperture; poche condividono la
        // stessa posizione finale, quindi le chiavi sono comunque migliaia.
        assert!(CATALOG.len() > 3000);
    }

    #[test]
    fn skips_the_header_and_broken_lines() {
        let catalog = build("eco\tname\tpgn\nB00\tKing's Pawn Game\t1. e4\nX99\tRotta\t1. e5\n");

        assert_eq!(catalog.len(), 1);
    }
}
