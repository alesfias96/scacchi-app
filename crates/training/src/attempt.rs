//! Un puzzle in corso: verifica le mosse di chi risolve e risponde per l'avversario.

use analysis::{FactCategory, analyze};
use app_settings::TrainingMode;
use chess_core::{Game, GameEndReason, Side, describe_line};

use crate::attempt_view::{AttemptView, Feedback, Reveal};
use crate::puzzle::Puzzle;
use crate::rating::updated_rating;
use crate::theme::{theme_hint, theme_names};
use crate::training_error::TrainingError;

/// Il risultato di un puzzle, deciso al primo errore (o aiuto) oppure alla fine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Outcome {
    /// `true` se il puzzle è stato risolto senza errori e senza aiuti.
    pub solved: bool,
    /// Il rating dopo il puzzle (uguale a prima nei ripassi).
    pub rating: i32,
}

/// Un puzzle in corso.
///
/// Come su Lichess, il risultato si decide una volta sola: un errore, un aiuto
/// o la soluzione mostrata contano come puzzle sbagliato, anche se poi si
/// trova la mossa giusta.
#[derive(Debug, Clone)]
pub struct PuzzleAttempt {
    puzzle: Puzzle,
    mode: TrainingMode,
    /// La partita del puzzle: la mossa dell'avversario e le mosse trovate.
    game: Game,
    /// La posizione in `puzzle.moves` della prossima mossa da trovare.
    next: usize,
    /// Il rating prima del puzzle.
    rating_before: i32,
    feedback: Feedback,
    /// Il primo aiuto (il tema a parole), una volta chiesto.
    hint_text: Option<&'static str>,
    hint_square: Option<String>,
    /// Il risultato, una volta deciso.
    outcome: Option<Outcome>,
    /// `true` dopo che [`PuzzleAttempt::take_outcome`] ha già consegnato il risultato.
    outcome_taken: bool,
}

impl PuzzleAttempt {
    /// Comincia un puzzle: gioca la mossa dell'avversario e aspetta chi risolve.
    ///
    /// - Riceve: il puzzle, la modalità e il rating attuale di chi risolve.
    ///
    /// # Errors
    ///
    /// [`TrainingError::InvalidPuzzle`] se la posizione o la prima mossa non sono valide.
    pub fn start(puzzle: Puzzle, mode: TrainingMode, rating: i32) -> Result<Self, TrainingError> {
        let invalid = || TrainingError::InvalidPuzzle(puzzle.id.clone());
        let mut game = Game::from_fen(&puzzle.fen).map_err(|_| invalid())?;
        game.play_uci(&puzzle.moves[0]).map_err(|_| invalid())?;
        Ok(Self {
            puzzle,
            mode,
            game,
            next: 1,
            rating_before: rating,
            feedback: Feedback::Ready,
            hint_text: None,
            hint_square: None,
            outcome: None,
            outcome_taken: false,
        })
    }

    /// Il puzzle in corso.
    #[must_use]
    pub fn puzzle(&self) -> &Puzzle {
        &self.puzzle
    }

    /// `true` se il puzzle è finito.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.next >= self.end()
    }

    /// Dove finisce la soluzione in `puzzle.moves`: nella modalità "cosa
    /// minaccia?" basta indovinare la prima mossa.
    fn end(&self) -> usize {
        match self.mode {
            TrainingMode::Threat => 2,
            TrainingMode::Depth | TrainingMode::Visualization | TrainingMode::Review => {
                self.puzzle.moves.len()
            }
        }
    }

    /// Prova una mossa di chi risolve.
    ///
    /// La mossa è giusta se è quella della soluzione, oppure se dà scacco
    /// matto (a volte esiste più di un matto). Se è giusta, l'avversario
    /// risponde subito; se è sbagliata, la scacchiera resta com'era e si può
    /// riprovare.
    ///
    /// # Errors
    ///
    /// - [`TrainingError::Finished`] se il puzzle è già finito;
    /// - [`TrainingError::IllegalMove`] se la mossa non è legale.
    pub fn play(&mut self, uci: &str) -> Result<(), TrainingError> {
        if self.is_finished() {
            return Err(TrainingError::Finished);
        }
        let mut tried = self.game.clone();
        tried
            .play_uci(uci)
            .map_err(|_| TrainingError::IllegalMove(uci.to_owned()))?;
        let played = tried.moves().last().map(|record| record.uci.as_str());
        let is_mate = tried
            .result()
            .is_some_and(|result| result.reason == GameEndReason::Checkmate);
        if played != Some(self.puzzle.moves[self.next].as_str()) && !is_mate {
            self.feedback = Feedback::Wrong;
            self.decide(false);
            return Ok(());
        }

        self.game = tried;
        self.hint_square = None;
        // Un matto diverso da quello previsto chiude comunque il puzzle.
        self.next = if is_mate { self.end() } else { self.next + 1 };
        if self.is_finished() {
            self.feedback = Feedback::Solved;
            self.decide(true);
        } else {
            let reply = &self.puzzle.moves[self.next];
            self.game
                .play_uci(reply)
                .map_err(|_| TrainingError::InvalidPuzzle(self.puzzle.id.clone()))?;
            self.next += 1;
            self.feedback = Feedback::Correct;
        }
        Ok(())
    }

    /// Un aiuto, a gradini: il primo dice il tema a parole (es. «Cerca una mossa
    /// che attacchi due pezzi insieme»), il secondo indica il pezzo da muovere.
    /// Se il puzzle non ha un tema con un aiuto, si passa subito al pezzo.
    /// Il puzzle conta come sbagliato.
    ///
    /// # Errors
    ///
    /// [`TrainingError::Finished`] se il puzzle è già finito.
    pub fn hint(&mut self) -> Result<(), TrainingError> {
        if self.is_finished() {
            return Err(TrainingError::Finished);
        }
        if self.hint_text.is_none()
            && let Some(text) = theme_hint(&self.puzzle.themes)
        {
            self.hint_text = Some(text);
        } else {
            let expected = &self.puzzle.moves[self.next];
            self.hint_square = expected.get(0..2).map(str::to_owned);
        }
        self.decide(false);
        Ok(())
    }

    /// Gioca sulla scacchiera il resto della soluzione. Il puzzle conta come sbagliato.
    ///
    /// # Errors
    ///
    /// [`TrainingError::Finished`] se il puzzle è già finito.
    pub fn show_solution(&mut self) -> Result<(), TrainingError> {
        if self.is_finished() {
            return Err(TrainingError::Finished);
        }
        while !self.is_finished() {
            self.game
                .play_uci(&self.puzzle.moves[self.next])
                .map_err(|_| TrainingError::InvalidPuzzle(self.puzzle.id.clone()))?;
            self.next += 1;
        }
        self.hint_square = None;
        self.feedback = Feedback::SolutionShown;
        self.decide(false);
        Ok(())
    }

    /// Fissa il risultato, se non è già deciso.
    fn decide(&mut self, solved: bool) {
        if self.outcome.is_some() {
            return;
        }
        let rating = if self.mode == TrainingMode::Review {
            self.rating_before
        } else {
            updated_rating(self.rating_before, self.puzzle.rating, solved)
        };
        self.outcome = Some(Outcome { solved, rating });
    }

    /// Il risultato, la prima volta che è deciso (per salvarlo una volta sola).
    ///
    /// Restituisce `None` se il risultato non è ancora deciso o è già stato consegnato.
    pub fn take_outcome(&mut self) -> Option<Outcome> {
        if self.outcome_taken {
            return None;
        }
        let outcome = self.outcome?;
        self.outcome_taken = true;
        Some(outcome)
    }

    /// Il colore di chi deve trovare la soluzione.
    fn solver(&self) -> Side {
        // La prima mossa è dell'avversario: chi risolve è l'altro colore.
        self.game
            .moves()
            .first()
            .map_or(Side::White, |record| record.side.opponent())
    }

    /// Cosa si scopre a puzzle finito (`None` solo se i dati del puzzle non sono validi).
    fn reveal(&self) -> Option<Reveal> {
        let solver = self.solver();
        let opening = [self.puzzle.moves[0].clone()];
        let solution = describe_line(
            &self.puzzle.fen,
            &opening,
            &self.puzzle.moves[1..self.end()],
        )
        .ok()?;
        let facts = analyze(&solution.fen)
            .map(|report| {
                report
                    .facts
                    .into_iter()
                    .filter(|fact| {
                        fact.side == solver
                            && matches!(
                                fact.category,
                                FactCategory::Threats | FactCategory::Tactics
                            )
                    })
                    .collect()
            })
            .unwrap_or_default();
        Some(Reveal {
            puzzle_rating: self.puzzle.rating,
            themes: theme_names(&self.puzzle.themes),
            solution,
            facts,
            idea: tutor::solution_idea(&self.puzzle.fen, &self.puzzle.moves[..self.end()]),
        })
    }

    /// La descrizione del puzzle per l'interfaccia.
    #[must_use]
    pub fn view(&self) -> AttemptView {
        let solver = self.solver();
        let finished = self.is_finished();
        let to_count = |index: usize| u32::try_from(index / 2).unwrap_or(u32::MAX);
        AttemptView {
            puzzle_id: self.puzzle.id.clone(),
            mode: self.mode,
            game: self.game.view(),
            solver,
            orientation: if self.mode == TrainingMode::Threat {
                solver.opponent()
            } else {
                solver
            },
            depth: to_count(self.end()),
            found: to_count(self.next),
            feedback: self.feedback,
            hint_text: self.hint_text.map(str::to_owned),
            hint_square: self.hint_square.clone(),
            can_hint: !finished && self.hint_square.is_none(),
            finished,
            clean: finished && self.outcome.is_some_and(|outcome| outcome.solved),
            is_review: self.mode == TrainingMode::Review,
            rating: self
                .outcome
                .map_or(self.rating_before, |outcome| outcome.rating),
            rating_change: self
                .outcome
                .filter(|_| self.mode != TrainingMode::Review)
                .map(|outcome| outcome.rating - self.rating_before),
            reveal: if finished { self.reveal() } else { None },
        }
    }
}

#[cfg(test)]
mod tests {
    use app_settings::TrainingMode;

    use super::PuzzleAttempt;
    use crate::attempt_view::Feedback;
    use crate::puzzle::Puzzle;

    /// Un puzzle inventato: il nero gioca h7h6, il bianco dà matto con la
    /// donna (Dxf7#), una mossa sola da trovare.
    fn mate_in_one() -> Puzzle {
        Puzzle {
            id: "TEST1".to_owned(),
            fen: "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR b KQkq - 3 3".to_owned(),
            moves: vec!["h7h6".to_owned(), "f3f7".to_owned()],
            rating: 1500,
            themes: vec!["mateIn1".to_owned(), "short".to_owned()],
        }
    }

    /// Un puzzle inventato con due mosse da trovare (Cc3, poi Cd5).
    fn fork_in_two() -> Puzzle {
        Puzzle {
            id: "TEST2".to_owned(),
            fen: "4k3/8/8/3q4/8/8/8/1N2K3 b - - 0 1".to_owned(),
            moves: vec![
                "d5d4".to_owned(),
                "b1c3".to_owned(),
                "e8d8".to_owned(),
                "c3d5".to_owned(),
            ],
            rating: 1500,
            themes: vec!["fork".to_owned()],
        }
    }

    #[test]
    fn starts_after_the_opponent_move() {
        let attempt =
            PuzzleAttempt::start(mate_in_one(), TrainingMode::Depth, 1500).expect("valido");
        let view = attempt.view();

        assert_eq!(view.game.moves.len(), 1);
        assert_eq!(view.solver, chess_core::Side::White);
        assert_eq!(view.depth, 1);
        assert_eq!(view.feedback, Feedback::Ready);
        assert!(view.reveal.is_none());
    }

    #[test]
    fn the_right_move_solves_the_puzzle_and_raises_the_rating() {
        let mut attempt =
            PuzzleAttempt::start(mate_in_one(), TrainingMode::Depth, 1500).expect("valido");

        attempt.play("f3f7").expect("mossa legale");
        let view = attempt.view();

        assert!(view.finished && view.clean);
        assert_eq!(view.rating_change, Some(16));
        let reveal = view.reveal.expect("rivelato a fine puzzle");
        assert_eq!(reveal.solution.san, ["Qxf7#"]);
        assert_eq!(reveal.themes, ["Matto in 1"]);
        assert_eq!(
            attempt.take_outcome().map(|outcome| outcome.solved),
            Some(true)
        );
        assert_eq!(attempt.take_outcome(), None);
    }

    #[test]
    fn a_wrong_move_can_be_retried_but_the_puzzle_counts_as_failed() {
        let mut attempt =
            PuzzleAttempt::start(mate_in_one(), TrainingMode::Depth, 1500).expect("valido");

        attempt.play("f3f6").expect("mossa legale");
        assert_eq!(attempt.view().feedback, Feedback::Wrong);
        assert_eq!(attempt.view().game.moves.len(), 1);

        attempt.play("f3f7").expect("mossa legale");
        let view = attempt.view();
        assert!(view.finished && !view.clean);
        assert_eq!(view.rating_change, Some(-16));
    }

    #[test]
    fn the_opponent_answers_after_a_right_move() {
        let mut attempt =
            PuzzleAttempt::start(fork_in_two(), TrainingMode::Depth, 1500).expect("valido");

        attempt.play("b1c3").expect("mossa legale");
        let view = attempt.view();

        assert_eq!(view.feedback, Feedback::Correct);
        assert_eq!(view.game.moves.len(), 3);
        assert_eq!(view.found, 1);
        assert!(!view.finished);
    }

    #[test]
    fn a_hint_shows_the_piece_and_the_solution_finishes_the_puzzle() {
        let mut attempt =
            PuzzleAttempt::start(fork_in_two(), TrainingMode::Depth, 1500).expect("valido");

        attempt.hint().expect("in corso");
        let first = attempt.view();
        assert_eq!(
            first.hint_text.as_deref(),
            Some("Cerca una mossa che attacchi due pezzi insieme.")
        );
        assert_eq!(first.hint_square, None);
        assert!(first.can_hint && !first.clean);

        attempt.hint().expect("in corso");
        assert_eq!(attempt.view().hint_square.as_deref(), Some("b1"));
        assert!(!attempt.view().can_hint);

        attempt.show_solution().expect("in corso");
        let view = attempt.view();
        assert_eq!(view.feedback, Feedback::SolutionShown);
        assert_eq!(view.game.moves.len(), 4);
        assert!(view.finished && !view.clean);
        assert!(attempt.play("e1e2").is_err());
    }

    #[test]
    fn threat_mode_asks_only_the_first_move_from_the_defender_side() {
        let mut attempt =
            PuzzleAttempt::start(fork_in_two(), TrainingMode::Threat, 1500).expect("valido");
        assert_eq!(attempt.view().orientation, chess_core::Side::Black);

        attempt.play("b1c3").expect("mossa legale");

        assert!(attempt.view().finished);
        assert_eq!(attempt.view().depth, 1);
    }

    #[test]
    fn a_review_does_not_change_the_rating() {
        let mut attempt =
            PuzzleAttempt::start(mate_in_one(), TrainingMode::Review, 1500).expect("valido");

        attempt.play("f3f7").expect("mossa legale");

        assert_eq!(attempt.view().rating, 1500);
        assert_eq!(attempt.view().rating_change, None);
    }
}
