//! I dati grezzi da cui si calcolano le statistiche.

use analysis::GamePhase;
use app_settings::OpponentKind;
use chess_core::Side;
use engine::Evaluation;

/// Una partita.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameFacts {
    /// Il nome unico della partita.
    pub id: String,
    /// Quando è cominciata (millesimi di secondo dal 1970).
    pub started_at_ms: i64,
    /// Contro chi.
    pub opponent: OpponentKind,
    /// Il colore e il livello del computer, se c'era.
    pub computer: Option<(Side, u8)>,
    /// `true` se la partita è finita.
    pub finished: bool,
    /// Chi ha vinto (`None` = patta o non finita).
    pub winner: Option<Side>,
    /// Il nome dell'apertura (di Lichess, in inglese).
    pub opening_name: Option<String>,
    /// Il tempo iniziale per giocatore, se c'era l'orologio (millesimi di secondo).
    pub time_initial_ms: Option<u32>,
    /// `true` se la partita è stata analizzata.
    pub analyzed: bool,
}

impl GameFacts {
    /// I colori giocati dalla persona: contro il computer uno solo, a due
    /// giocatori entrambi (si gioca da soli o con un amico, ma è sempre l'utente).
    #[must_use]
    pub fn user_sides(&self) -> Vec<Side> {
        match self.computer {
            Some((computer, _)) => vec![computer.opponent()],
            None => vec![Side::White, Side::Black],
        }
    }
}

/// Una mossa di una partita analizzata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveFacts {
    /// La partita.
    pub game_id: String,
    /// Chi l'ha giocata.
    pub side: Side,
    /// La mossa giocata (UCI).
    pub played: String,
    /// La fase di gioco prima della mossa.
    pub phase: Option<GamePhase>,
    /// La valutazione prima della mossa (per il Bianco).
    pub evaluation_before: Option<Evaluation>,
    /// La valutazione dopo la mossa (per il Bianco).
    pub evaluation_after: Option<Evaluation>,
    /// La mossa migliore (UCI).
    pub best_move: Option<String>,
    /// Il tempo che restava a chi ha mosso, dopo la mossa (millesimi di secondo).
    pub clock_after_ms: Option<u32>,
}

/// Un tentativo di puzzle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuzzleFacts {
    /// Quando è stato giocato.
    pub played_at_ms: i64,
    /// `true` se era un ripasso (non cambia il rating).
    pub is_review: bool,
    /// `true` se risolto senza errori né aiuti.
    pub solved: bool,
    /// Il rating dopo il tentativo.
    pub rating_after: i32,
    /// Quante mosse bisognava trovare.
    pub depth: u32,
    /// I temi del puzzle (nomi di Lichess, es. `"fork"`).
    pub themes: Vec<String>,
}

/// Tutti i dati delle statistiche.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StatsInput {
    /// Le partite del periodo, dalla più vecchia.
    pub games: Vec<GameFacts>,
    /// Le mosse delle partite analizzate del periodo.
    pub moves: Vec<MoveFacts>,
    /// I tentativi dei puzzle del periodo, in ordine di tempo.
    pub puzzles: Vec<PuzzleFacts>,
    /// Il rating dei puzzle attuale.
    pub puzzle_rating: i32,
    /// Quanti puzzle sono da ripassare oggi.
    pub due_reviews: u32,
}
