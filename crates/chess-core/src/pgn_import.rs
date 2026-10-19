//! Lettura di una partita in formato PGN.
//!
//! Legge le intestazioni (in particolare l'eventuale posizione di partenza),
//! la linea principale delle mosse e le varianti `(...)`, anche annidate.
//! Ignora commenti `{...}` e `; ...`, annotazioni `$1`, `!`, `?` e i numeri di mossa.

use shakmaty::san::SanPlus;

use crate::game::{Game, NodeId};
use crate::game_error::GameError;

/// Un pezzo del testo delle mosse.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// Una mossa in notazione SAN.
    Move(String),
    /// `(`: comincia una variante, che sostituisce l'ultima mossa letta.
    Open,
    /// `)`: finisce la variante; si riprende da dove si era.
    Close,
}

/// Legge una partita da un testo PGN, con le sue varianti.
///
/// # Errors
///
/// - [`GameError::InvalidPosition`] se l'intestazione `FEN` non è valida;
/// - [`GameError::InvalidPgn`] se una mossa non è scritta in notazione valida;
/// - [`GameError::IllegalMove`] se una mossa non è legale nella posizione raggiunta.
pub fn from_pgn(text: &str) -> Result<Game, GameError> {
    let mut game = match header_value(text, "FEN") {
        Some(fen) => Game::from_fen(&fen)?,
        None => Game::new(),
    };
    // Per ogni variante aperta: la mossa da cui riprendere quando si chiude.
    let mut resume: Vec<Option<NodeId>> = Vec::new();

    for token in move_tokens(text) {
        match token {
            Token::Open => {
                let last = game.current_node();
                resume.push(last);
                // La variante è un'alternativa all'ultima mossa: si torna a prima di lei.
                let before = last.and_then(|id| game.parent(id));
                game.go_to_node(before)?;
            }
            Token::Close => {
                if let Some(back) = resume.pop() {
                    game.go_to_node(back)?;
                }
            }
            Token::Move(san_text) => {
                let san = san_text
                    .parse::<SanPlus>()
                    .map_err(|_| GameError::InvalidPgn(san_text.clone()))?;
                let chess_move = san
                    .san
                    .to_move(game.current())
                    .map_err(|_| GameError::IllegalMove(san_text.clone()))?;
                game.play_move(chess_move);
            }
        }
    }

    game.go_to_mainline_end();
    Ok(game)
}

/// Il valore di un'intestazione, ad esempio `FEN` in `[FEN "8/8/..."]`.
fn header_value(text: &str, name: &str) -> Option<String> {
    let prefix = format!("[{name} ");
    text.lines()
        .map(str::trim)
        .find(|line| line.starts_with(&prefix))
        .and_then(|line| {
            let start = line.find('"')? + 1;
            let end = line.rfind('"')?;
            line.get(start..end).map(str::to_owned)
        })
}

/// Le mosse e le parentesi delle varianti, già ripulite da numeri, commenti e annotazioni.
fn move_tokens(text: &str) -> Vec<Token> {
    let movetext: String = text
        .lines()
        .filter(|line| !line.trim_start().starts_with('['))
        // Un ';' apre un commento che arriva fino alla fine della riga.
        .map(|line| line.split(';').next().unwrap_or_default())
        .collect::<Vec<_>>()
        .join(" ");

    remove_comments(&movetext)
        .split_whitespace()
        .filter_map(|raw| match raw {
            "(" => Some(Token::Open),
            ")" => Some(Token::Close),
            _ => clean_token(raw).map(Token::Move),
        })
        .collect()
}

/// Toglie i commenti `{...}` e stacca le parentesi delle varianti dalle mosse
/// vicine (così `(1... c5` diventa `( 1... c5`).
fn remove_comments(movetext: &str) -> String {
    let mut result = String::with_capacity(movetext.len());
    let mut in_comment = false;

    for character in movetext.chars() {
        match character {
            '{' if !in_comment => in_comment = true,
            '}' if in_comment => {
                in_comment = false;
                result.push(' ');
            }
            _ if in_comment => {}
            '(' | ')' => {
                result.push(' ');
                result.push(character);
                result.push(' ');
            }
            _ => result.push(character),
        }
    }
    result
}

/// Ripulisce un "pezzo" di testo: toglie il numero di mossa (`12.` o `12...`),
/// le annotazioni (`!`, `?`) e scarta risultati e codici `$n`.
/// Restituisce `None` se non resta una mossa.
fn clean_token(raw: &str) -> Option<String> {
    let without_number = raw.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.');
    // Se il pezzo era solo un numero, ma il testo originale non conteneva punti,
    // non era un numero di mossa: è un risultato come "1-0" (gestito sotto).
    let token = if raw.contains('.') {
        without_number
    } else {
        raw
    };
    let token = token.trim_end_matches(['!', '?']);

    if token.is_empty()
        || token.starts_with('$')
        || matches!(token, "1-0" | "0-1" | "1/2-1/2" | "*")
    {
        return None;
    }

    // Alcuni programmi scrivono l'arrocco con gli zeri.
    let token = match token.trim_end_matches(['+', '#']) {
        "0-0" => token.replacen("0-0", "O-O", 1),
        "0-0-0" => token.replacen("0-0-0", "O-O-O", 1),
        _ => token.to_owned(),
    };
    Some(token)
}

#[cfg(test)]
mod tests {
    use super::from_pgn;
    use crate::game::Game;
    use crate::game_error::GameError;
    use crate::game_result::GameEndReason;
    use crate::pgn_export::to_pgn;

    fn sans(game: &Game) -> Vec<String> {
        game.moves()
            .iter()
            .map(|record| record.san.clone())
            .collect()
    }

    #[test]
    fn reads_a_simple_game() {
        let game = from_pgn("1. e4 e5 2. Nf3 Nc6 *").expect("PGN valido");

        assert_eq!(sans(&game), ["e4", "e5", "Nf3", "Nc6"]);
    }

    #[test]
    fn reads_variations_and_ignores_comments_and_annotations() {
        let pgn = "[Event \"Prova\"]\n[White \"Bianco\"]\n\n\
                   1. e4 {buona mossa} e5 (1... c5 2. Nf3 (2. c3)) 2. Nf3!? $1 Nc6 ; fine\n*";

        let game = from_pgn(pgn).expect("PGN valido");

        assert_eq!(sans(&game), ["e4", "e5", "Nf3", "Nc6"]);
        // Le varianti: 1... c5 2. Nf3, e dentro di lei 2. c3.
        let tree = game.view().tree;
        assert_eq!(tree.nodes.len(), 7);
        assert_eq!(tree.nodes.iter().filter(|node| !node.mainline).count(), 3);
    }

    #[test]
    fn accepts_castling_written_with_zeros() {
        let game = from_pgn("1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 4. 0-0 *").expect("PGN valido");

        assert_eq!(sans(&game).last().map(String::as_str), Some("O-O"));
    }

    #[test]
    fn starts_from_the_fen_header() {
        let pgn = "[SetUp \"1\"]\n[FEN \"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1\"]\n\n1... e5 *";

        let game = from_pgn(pgn).expect("PGN valido");

        assert_eq!(sans(&game), ["e5"]);
    }

    #[test]
    fn exported_games_are_read_back_identical() {
        let mut game = Game::new();
        for uci in ["f2f3", "e7e5", "g2g4", "d8h4"] {
            game.play_uci(uci).expect("mossa legale");
        }

        let read_back = from_pgn(&to_pgn(&game)).expect("PGN valido");

        assert_eq!(sans(&read_back), sans(&game));
        assert_eq!(
            read_back.final_result().expect("partita finita").reason,
            GameEndReason::Checkmate
        );
    }

    #[test]
    fn variations_survive_export_and_import() {
        let mut game = Game::new();
        for uci in ["e2e4", "e7e5", "g1f3"] {
            game.play_uci(uci).expect("mossa legale");
        }
        game.go_to(1);
        game.play_uci("c7c5").expect("mossa legale");
        game.play_uci("g1f3").expect("mossa legale");

        let pgn = to_pgn(&game);
        assert!(pgn.contains("1. e4 e5 (1... c5 2. Nf3) 2. Nf3 *"));

        let read_back = from_pgn(&pgn).expect("PGN valido");
        assert_eq!(read_back.view().tree.nodes.len(), 5);
        assert_eq!(sans(&read_back), ["e4", "e5", "Nf3"]);
    }

    #[test]
    fn reports_text_that_is_not_a_move() {
        assert_eq!(
            from_pgn("1. e4 banana *").map(|game| game.moves().len()),
            Err(GameError::InvalidPgn("banana".to_owned()))
        );
    }

    #[test]
    fn reports_an_illegal_move() {
        assert_eq!(
            from_pgn("1. e5 *").map(|game| game.moves().len()),
            Err(GameError::IllegalMove("e5".to_owned()))
        );
    }
}
