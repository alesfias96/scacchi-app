//! Un puzzle: una posizione e la sua soluzione.

/// Un puzzle del database di Lichess.
///
/// La prima mossa di `moves` è quella dell'**avversario**: il puzzle comincia
/// subito dopo. Poi le mosse si alternano: quelle di chi risolve (da trovare)
/// e le risposte dell'avversario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puzzle {
    /// Il codice del puzzle su Lichess (es. `"003AX"`).
    pub id: String,
    /// La posizione prima della mossa dell'avversario, in notazione FEN.
    pub fen: String,
    /// La mossa dell'avversario e la soluzione, in notazione UCI.
    pub moves: Vec<String>,
    /// La difficoltà del puzzle, in punti (come il rating dei giocatori).
    pub rating: u32,
    /// I temi del puzzle, con i nomi di Lichess (es. `"fork"`, `"mateIn2"`).
    pub themes: Vec<String>,
}

impl Puzzle {
    /// Legge un puzzle da una riga del file: `id,fen,mosse,rating,temi,aperture`.
    ///
    /// Restituisce `None` se la riga non è scritta bene.
    #[must_use]
    pub fn parse(line: &str) -> Option<Self> {
        let mut columns = line.split(',');
        let id = columns.next()?.to_owned();
        let fen = columns.next()?.to_owned();
        let moves: Vec<String> = columns
            .next()?
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        let rating = columns.next()?.parse().ok()?;
        let themes = columns
            .next()?
            .split_whitespace()
            .map(str::to_owned)
            .collect();
        (moves.len() >= 2 && moves.len().is_multiple_of(2)).then_some(Self {
            id,
            fen,
            moves,
            rating,
            themes,
        })
    }

    /// Quante mosse deve trovare chi risolve (la "profondità" del puzzle).
    #[must_use]
    pub fn depth(&self) -> usize {
        self.moves.len() / 2
    }

    /// `true` se il puzzle ha il tema indicato.
    #[must_use]
    pub fn has_theme(&self, theme: &str) -> bool {
        self.themes.iter().any(|own| own == theme)
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;

    const LINE: &str = "00Pw1,5r1k/2p3Rp/3p4/p2Pn3/1p2B3/1P6/PKP2r1P/6R1 b - - 1 28,f2e2 g7h7,1077,endgame mate mateIn1 oneMove operaMate,";

    #[test]
    fn reads_a_line_of_the_file() {
        let puzzle = Puzzle::parse(LINE).expect("riga valida");

        assert_eq!(puzzle.id, "00Pw1");
        assert_eq!(puzzle.moves, ["f2e2", "g7h7"]);
        assert_eq!(puzzle.rating, 1077);
        assert_eq!(puzzle.depth(), 1);
        assert!(puzzle.has_theme("operaMate"));
    }

    #[test]
    fn rejects_a_broken_line() {
        assert_eq!(Puzzle::parse("abc,fen,e2e4,1500,,"), None);
        assert_eq!(Puzzle::parse("solo-un-campo"), None);
    }
}
