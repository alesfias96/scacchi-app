//! Le parole italiane per descrivere pezzi, colori e case.
//!
//! Tutti i rilevatori scrivono le loro frasi con queste funzioni, così il
//! modo di nominare i pezzi è lo stesso ovunque: "il cavallo bianco in c3",
//! "la donna nera in d8", "l'alfiere bianco in c4".

use shakmaty::{Color, Piece, Role, Square};

/// Il nome del pezzo (senza articolo): "pedone", "cavallo", …
#[must_use]
pub fn role_name(role: Role) -> &'static str {
    match role {
        Role::Pawn => "pedone",
        Role::Knight => "cavallo",
        Role::Bishop => "alfiere",
        Role::Rook => "torre",
        Role::Queen => "donna",
        Role::King => "re",
    }
}

/// `true` per i pezzi di genere femminile (torre, donna).
fn is_feminine(role: Role) -> bool {
    matches!(role, Role::Rook | Role::Queen)
}

/// L'articolo determinativo giusto per il pezzo: "il", "l'", "la".
fn article(role: Role) -> &'static str {
    match role {
        Role::Bishop => "l'",
        Role::Rook | Role::Queen => "la ",
        _ => "il ",
    }
}

/// Il colore come aggettivo, accordato al pezzo: "bianco", "bianca", "nero", "nera".
fn color_adjective(color: Color, role: Role) -> &'static str {
    match (color, is_feminine(role)) {
        (Color::White, false) => "bianco",
        (Color::White, true) => "bianca",
        (Color::Black, false) => "nero",
        (Color::Black, true) => "nera",
    }
}

/// Il nome del giocatore: "il Bianco" o "il Nero".
#[must_use]
pub fn side_name(color: Color) -> &'static str {
    match color {
        Color::White => "il Bianco",
        Color::Black => "il Nero",
    }
}

/// Il nome del giocatore con la maiuscola, per l'inizio di una frase.
#[must_use]
pub fn side_name_capitalized(color: Color) -> &'static str {
    match color {
        Color::White => "Il Bianco",
        Color::Black => "Il Nero",
    }
}

/// Il pezzo con articolo, colore e casa: "il cavallo bianco in c3".
#[must_use]
pub fn piece_phrase(piece: Piece, square: Square) -> String {
    format!(
        "{}{} {} in {square}",
        article(piece.role),
        role_name(piece.role),
        color_adjective(piece.color, piece.role)
    )
}

/// Il pezzo preceduto da "di" (preposizione articolata): "del cavallo bianco
/// in c3", "della torre nera in a8", "dell'alfiere bianco in c4".
#[must_use]
pub fn of_piece_phrase(piece: Piece, square: Square) -> String {
    let preposition = match piece.role {
        Role::Bishop => "dell'",
        Role::Rook | Role::Queen => "della ",
        _ => "del ",
    };
    format!(
        "{preposition}{} {} in {square}",
        role_name(piece.role),
        color_adjective(piece.color, piece.role)
    )
}

/// Come [`piece_phrase`], con la maiuscola iniziale: "Il cavallo bianco in c3".
#[must_use]
pub fn piece_phrase_capitalized(piece: Piece, square: Square) -> String {
    let phrase = piece_phrase(piece, square);
    let mut letters = phrase.chars();
    letters
        .next()
        .map(|first| first.to_uppercase().chain(letters).collect())
        .unwrap_or_default()
}

/// Un elenco italiano: "a", "a e b", "a, b e c".
#[must_use]
pub fn join_list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} e {last}", rest.join(", ")),
    }
}

#[cfg(test)]
mod tests {
    use super::{join_list, of_piece_phrase, piece_phrase, piece_phrase_capitalized};
    use shakmaty::{Color, Piece, Role, Square};

    #[test]
    fn names_pieces_with_the_right_article_and_gender() {
        let knight = Piece {
            color: Color::White,
            role: Role::Knight,
        };
        let queen = Piece {
            color: Color::Black,
            role: Role::Queen,
        };
        let bishop = Piece {
            color: Color::White,
            role: Role::Bishop,
        };

        assert_eq!(piece_phrase(knight, Square::C3), "il cavallo bianco in c3");
        assert_eq!(piece_phrase(queen, Square::D8), "la donna nera in d8");
        assert_eq!(piece_phrase(bishop, Square::C4), "l'alfiere bianco in c4");
        assert_eq!(
            piece_phrase_capitalized(queen, Square::D8),
            "La donna nera in d8"
        );
        assert_eq!(
            of_piece_phrase(bishop, Square::C4),
            "dell'alfiere bianco in c4"
        );
        assert_eq!(of_piece_phrase(queen, Square::D8), "della donna nera in d8");
    }

    #[test]
    fn joins_lists_like_a_sentence() {
        let words = ["a".to_owned(), "b".to_owned(), "c".to_owned()];

        assert_eq!(join_list(&words[..1]), "a");
        assert_eq!(join_list(&words[..2]), "a e b");
        assert_eq!(join_list(&words), "a, b e c");
    }
}
