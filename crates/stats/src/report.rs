//! Tutte le statistiche insieme, dai dati grezzi.

use crate::game_stats::game_stats;
use crate::judged_move::judge_user_moves;
use crate::level_stats::level_stats;
use crate::opening_stats::opening_stats;
use crate::stats_input::StatsInput;
use crate::stats_view::StatsReport;
use crate::tactics_stats::tactics_stats;

/// Calcola le statistiche della sezione «Progressi».
#[must_use]
pub fn build_report(input: &StatsInput) -> StatsReport {
    let moves = judge_user_moves(input);
    let count = |keep: fn(&crate::GameFacts) -> bool| {
        u32::try_from(input.games.iter().filter(|game| keep(game)).count()).unwrap_or(u32::MAX)
    };
    StatsReport {
        games: count(|_| true),
        analyzed_games: count(|game| game.analyzed),
        unanalyzed_games: count(|game| game.finished && !game.analyzed),
        level: level_stats(input, &moves),
        results: game_stats(input, &moves),
        openings: opening_stats(input, &moves),
        tactics: tactics_stats(input),
    }
}

#[cfg(test)]
mod tests {
    use analysis::GamePhase;
    use app_settings::{ComputerEngine, OpponentKind};
    use chess_core::Side;
    use engine::{Evaluation, LevelInfo};

    use super::build_report;
    use crate::stats_input::{ComputerFacts, GameFacts, MoveFacts, PuzzleFacts, StatsInput};

    /// Una partita contro Stockfish (che ha il Nero), al livello indicato.
    fn computer_game(id: &str, level: u8, winner: Option<Side>, opening: &str) -> GameFacts {
        GameFacts {
            id: id.to_owned(),
            started_at_ms: 1_000,
            opponent: OpponentKind::Computer,
            computer: Some(ComputerFacts {
                side: Side::Black,
                engine: ComputerEngine::Stockfish,
                level: engine::levels()
                    .into_iter()
                    .find(|info| info.level == level)
                    .expect("livello"),
            }),
            finished: true,
            winner,
            opening_name: Some(opening.to_owned()),
            time_initial_ms: Some(300_000),
            analyzed: id == "a",
        }
    }

    /// Una mossa analizzata della partita "a".
    fn analyzed_move(
        side: Side,
        before: i32,
        after: i32,
        phase: GamePhase,
        clock: u32,
    ) -> MoveFacts {
        MoveFacts {
            game_id: "a".to_owned(),
            side,
            played: "e2e4".to_owned(),
            phase: Some(phase),
            evaluation_before: Some(Evaluation::Centipawns(before)),
            evaluation_after: Some(Evaluation::Centipawns(after)),
            best_move: Some("d2d4".to_owned()),
            clock_after_ms: Some(clock),
        }
    }

    fn puzzle(depth: u32, solved: bool, theme: &str, day: i64) -> PuzzleFacts {
        PuzzleFacts {
            played_at_ms: day * 86_400_000,
            is_review: false,
            solved,
            rating_after: 1_500,
            depth,
            themes: vec![theme.to_owned()],
        }
    }

    fn sample() -> StatsInput {
        StatsInput {
            games: vec![
                computer_game(
                    "a",
                    4,
                    Some(Side::White),
                    "Sicilian Defense: Najdorf Variation",
                ),
                computer_game("b", 4, Some(Side::Black), "Sicilian Defense"),
                computer_game("c", 4, None, "French Defense"),
                computer_game("d", 6, Some(Side::White), "French Defense"),
                computer_game("e", 6, Some(Side::White), "Italian Game"),
            ],
            moves: vec![
                analyzed_move(Side::White, 20, 25, GamePhase::Opening, 290_000),
                // Un errore grave con poco tempo, nel finale.
                analyzed_move(Side::White, 50, -400, GamePhase::Endgame, 20_000),
                // Le mosse del computer non contano.
                analyzed_move(Side::Black, 0, 900, GamePhase::Middlegame, 100_000),
            ],
            puzzles: vec![
                puzzle(1, true, "fork", 0),
                puzzle(2, false, "pin", 0),
                puzzle(2, false, "pin", 1),
                puzzle(2, true, "pin", 1),
                puzzle(3, true, "fork", 1),
            ],
            puzzle_rating: 1_520,
            due_reviews: 2,
        }
    }

    #[test]
    fn estimates_the_elo_after_five_games_against_the_computer() {
        let report = build_report(&sample());

        assert_eq!(report.level.elo_games, 5);
        assert!(report.level.estimated_elo.is_some());
        assert_eq!(report.results.against_computer.wins, 3);
        assert_eq!(report.results.against_computer.losses, 1);
        assert_eq!(report.results.against_computer.draws, 1);
        assert_eq!(report.analyzed_games, 1);
    }

    #[test]
    fn games_against_maia_count_with_their_own_level() {
        let mut input = sample();
        let mut maia = computer_game("m", 5, Some(Side::White), "Italian Game");
        maia.computer = Some(ComputerFacts {
            side: Side::Black,
            engine: ComputerEngine::Maia,
            level: LevelInfo {
                level: 5,
                name: "Maia 1500".to_owned(),
                elo: 1500,
            },
        });
        input.games.push(maia);

        let report = build_report(&input);

        assert_eq!(report.level.elo_games, 6);
        let names: Vec<&str> = report
            .results
            .by_level
            .iter()
            .map(|level| level.name.as_str())
            .collect();
        // In ordine di Elo: Circolo (1400), Maia 1500, Esperto (1800).
        assert_eq!(names, ["Circolo", "Maia 1500", "Esperto"]);
    }

    #[test]
    fn counts_only_the_user_moves_by_phase_and_under_time_pressure() {
        let report = build_report(&sample());

        let endgame = report
            .results
            .phases
            .iter()
            .find(|phase| phase.phase == GamePhase::Endgame)
            .expect("fase");
        assert_eq!(endgame.blunders, 1);
        let middlegame = report
            .results
            .phases
            .iter()
            .find(|phase| phase.phase == GamePhase::Middlegame)
            .expect("fase");
        assert_eq!(middlegame.moves, 0);
        let pressure = report
            .results
            .time_pressure
            .expect("partite con l'orologio");
        assert_eq!((pressure.low_time_moves, pressure.low_time_errors), (1, 1));
        assert_eq!(report.level.accuracy_history.len(), 1);
    }

    #[test]
    fn groups_openings_by_family_and_color() {
        let report = build_report(&sample());

        let sicilian = report
            .openings
            .iter()
            .find(|opening| opening.name == "Sicilian Defense")
            .expect("apertura");
        assert_eq!(sicilian.games, 2);
        assert_eq!(sicilian.side, Side::White);
        assert_eq!((sicilian.score.wins, sicilian.score.losses), (1, 1));
    }

    #[test]
    fn measures_calculation_depth_and_weakest_themes() {
        let report = build_report(&sample());

        let two = report
            .tactics
            .by_depth
            .iter()
            .find(|depth| depth.depth == 2)
            .expect("profondità");
        assert_eq!((two.attempts, two.solved), (3, 1));
        assert_eq!(report.tactics.weakest_themes[0].name, "Inchiodatura");
        assert_eq!(report.tactics.depth_history.len(), 2);
        assert_eq!(report.tactics.due_reviews, 2);
    }
}
