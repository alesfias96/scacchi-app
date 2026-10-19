//! La "fotografia" della partita, da salvare nell'archivio e da cui ripartire.

use std::time::Instant;

use app_settings::{ComputerEngine, OpponentKind, TimeControl};
use chess_core::{Game, GameEndReason, GameResult, MoveRecord, Side, from_pgn};
use openings::Opening;

use crate::clock::{Clock, to_millis};
use crate::game_session::{Computer, GameSession, next_token};
use crate::move_timer::{MoveTimer, MoveTiming};
use crate::session_error::SessionError;

/// Una mossa salvata: la mossa e i suoi tempi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveSnapshot {
    /// La mossa, con notazione e numero.
    pub record: MoveRecord,
    /// Quanto ha pensato chi l'ha giocata e quanto tempo gli restava.
    pub timing: MoveTiming,
}

/// Tutto ciò che serve per salvare una partita e riprenderla più tardi.
///
/// Sono dati semplici, senza comportamento: la crate `storage` li scrive nel
/// database e li rilegge; [`GameSession::restore`] ne ricava di nuovo la partita.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSnapshot {
    /// Il nome unico della partita.
    pub id: String,
    /// Quando è cominciata, in millesimi di secondo dal 1970.
    pub started_at_ms: i64,
    /// Contro chi si gioca.
    pub opponent: OpponentKind,
    /// Il colore del computer, se si gioca contro il computer.
    pub computer_side: Option<Side>,
    /// Il motore del computer (Stockfish anche a due giocatori, dove non conta).
    pub engine: ComputerEngine,
    /// Il livello del computer, se si gioca contro il computer.
    pub level: Option<u8>,
    /// Il tempo di riflessione, se la partita ha l'orologio.
    pub time_control: Option<TimeControl>,
    /// Il tempo rimasto al Bianco e al Nero, in millesimi di secondo, se c'è l'orologio.
    pub clock_ms: Option<(u32, u32)>,
    /// La posizione di partenza, in notazione FEN.
    pub starting_fen: String,
    /// Le mosse giocate, con i loro tempi.
    pub moves: Vec<MoveSnapshot>,
    /// Il risultato, se la partita è finita.
    pub result: Option<GameResult>,
    /// L'apertura riconosciuta.
    pub opening: Option<Opening>,
    /// La partita in formato PGN (comoda per esportarla senza ricostruirla).
    pub pgn: String,
}

impl GameSession {
    /// Fotografa la partita nell'istante `now`, per salvarla.
    #[must_use]
    pub fn snapshot(&self, now: Instant) -> SessionSnapshot {
        // Si salvano le mosse della linea principale; le varianti stanno nel PGN.
        let moves = self
            .game
            .mainline()
            .into_iter()
            .enumerate()
            .map(|(index, record)| MoveSnapshot {
                record,
                timing: self.timer.timing(index),
            })
            .collect();
        SessionSnapshot {
            id: self.id.clone(),
            started_at_ms: self.started_at_ms,
            opponent: if self.computer.is_some() {
                OpponentKind::Computer
            } else {
                OpponentKind::TwoPlayers
            },
            computer_side: self.computer.map(|computer| computer.side),
            engine: self
                .computer
                .map_or(ComputerEngine::default(), |computer| computer.engine),
            level: self.computer.map(|computer| computer.level),
            time_control: self.clock.as_ref().map(Clock::control),
            clock_ms: self.clock.as_ref().map(|clock| {
                (
                    to_millis(clock.remaining(Side::White, now)),
                    to_millis(clock.remaining(Side::Black, now)),
                )
            }),
            starting_fen: self.game.starting_fen(),
            moves,
            result: self.game.final_result(),
            opening: self.opening(),
            pgn: self.export_pgn(),
        }
    }

    /// Ricostruisce una partita salvata.
    ///
    /// Le mosse vengono rigiocate una per una (così le regole le controllano
    /// di nuovo); l'abbandono e il tempo scaduto vengono riapplicati. Se la
    /// partita aveva l'orologio, l'orologio resta fermo fino alla prossima mossa:
    /// il tempo passato ad app chiusa non conta.
    ///
    /// # Errors
    ///
    /// [`SessionError::Game`] se la posizione di partenza o una mossa non sono valide
    /// (il salvataggio è rovinato).
    pub fn restore(snapshot: &SessionSnapshot, now: Instant) -> Result<Self, SessionError> {
        let mut game = with_variations(snapshot).map_or_else(
            || -> Result<Game, SessionError> {
                let mut game = Game::from_fen(&snapshot.starting_fen)?;
                for saved in &snapshot.moves {
                    game.play_uci(&saved.record.uci)?;
                }
                Ok(game)
            },
            Ok,
        )?;
        if let Some(result) = &snapshot.result {
            reapply_adjudication(&mut game, result)?;
        }
        let computer = snapshot
            .computer_side
            .zip(snapshot.level)
            .map(|(side, level)| Computer {
                side,
                engine: snapshot.engine,
                level,
            });
        let clock = snapshot.time_control.map(|control| {
            let initial = control.initial_seconds.saturating_mul(1000);
            let (white_ms, black_ms) = snapshot.clock_ms.unwrap_or((initial, initial));
            Clock::restore(control, white_ms, black_ms, snapshot.result.is_some())
        });
        let timings = snapshot.moves.iter().map(|saved| saved.timing).collect();
        Ok(Self {
            id: snapshot.id.clone(),
            started_at_ms: snapshot.started_at_ms,
            game,
            computer,
            clock,
            timer: MoveTimer::with_timings(now, timings),
            token: next_token(),
        })
    }
}

/// La partita letta dal PGN salvato, con le sue varianti, se la sua linea
/// principale è proprio quella delle mosse salvate (altrimenti `None`: si
/// rigiocano le mosse, senza varianti).
fn with_variations(snapshot: &SessionSnapshot) -> Option<Game> {
    let game = from_pgn(&snapshot.pgn).ok()?;
    let mainline: Vec<String> = game
        .mainline()
        .into_iter()
        .map(|record| record.uci)
        .collect();
    let saved: Vec<&str> = snapshot
        .moves
        .iter()
        .map(|saved| saved.record.uci.as_str())
        .collect();
    (game.starting_fen() == snapshot.starting_fen && mainline == saved).then_some(game)
}

/// Riapplica un risultato deciso fuori dalla scacchiera (abbandono, tempo scaduto).
/// Gli altri risultati (matto, patte) le regole li ritrovano da sole rigiocando le mosse.
fn reapply_adjudication(game: &mut Game, result: &GameResult) -> Result<(), SessionError> {
    let loser = result.winner.map(Side::opponent);
    match (result.reason, loser) {
        (GameEndReason::Resignation, Some(loser)) => game.resign(loser)?,
        (GameEndReason::Timeout, Some(loser)) => game.time_forfeit(loser)?,
        // Patta per tempo: il tempo l'ha finito chi doveva muovere.
        (GameEndReason::TimeoutVsInsufficientMaterial, _) => {
            let flagged = game.last_turn();
            game.time_forfeit(flagged)?;
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use app_settings::{ColorChoice, ComputerEngine, GameSetup, OpponentKind, TimeControl};
    use chess_core::{GameEndReason, Side};

    use crate::game_session::GameSession;

    fn computer_blitz() -> GameSetup {
        GameSetup {
            opponent: OpponentKind::Computer,
            color: ColorChoice::White,
            level: 6,
            time_control: Some(TimeControl {
                initial_seconds: 180,
                increment_seconds: 2,
            }),
            ..GameSetup::default()
        }
    }

    #[test]
    fn a_game_against_maia_is_restored_against_maia() {
        let now = Instant::now();
        let setup = GameSetup {
            engine: ComputerEngine::Maia,
            maia_level: 7,
            ..computer_blitz()
        };
        let mut session = GameSession::start(&setup, Side::White, now);
        session.play("e2e4", now).expect("mossa legale");

        let snapshot = session.snapshot(now);
        let restored = GameSession::restore(&snapshot, now).expect("partita valida");

        assert_eq!(snapshot.engine, ComputerEngine::Maia);
        assert_eq!(snapshot.level, Some(7));
        assert_eq!(
            restored.maia_request(now).map(|request| request.level),
            Some(7)
        );
    }

    #[test]
    fn a_restored_game_is_the_same_game() {
        let start = Instant::now();
        let mut session = GameSession::start(&computer_blitz(), Side::White, start);
        session
            .play("e2e4", start + Duration::from_secs(4))
            .expect("mossa legale");
        let request = session
            .engine_request(start + Duration::from_secs(4))
            .expect("tocca al computer");
        session
            .play_engine(request.token, "c7c5", start + Duration::from_secs(9))
            .expect("mossa del computer");

        let snapshot = session.snapshot(start + Duration::from_secs(9));
        let restored = GameSession::restore(&snapshot, start + Duration::from_secs(60))
            .expect("partita valida");

        assert_eq!(restored.id(), session.id());
        assert_eq!(restored.snapshot(start + Duration::from_secs(60)), snapshot);
        assert_eq!(snapshot.moves[0].timing.time_spent_ms, Some(4_000));
        assert_eq!(snapshot.moves[1].timing.time_spent_ms, Some(5_000));
        assert_eq!(snapshot.opening.expect("apertura").eco, "B20");
        // L'orologio è fermo: il tempo ad app chiusa non è stato contato.
        let view = restored.view(start + Duration::from_secs(60));
        assert_eq!(view.clock.expect("orologio").running, None);
        assert!(view.can_move);
    }

    #[test]
    fn an_abandoned_game_stays_abandoned() {
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        session.resign(now).expect("la partita non è finita");

        let restored = GameSession::restore(&session.snapshot(now), now).expect("partita valida");

        let result = restored.view(now).game.result.expect("partita finita");
        assert_eq!(result.reason, GameEndReason::Resignation);
        assert_eq!(result.winner, Some(Side::White));
    }

    #[test]
    fn a_broken_save_is_reported() {
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        let mut snapshot = session.snapshot(now);
        snapshot.moves[0].record.uci = "e2e5".to_owned();

        assert!(GameSession::restore(&snapshot, now).is_err());
    }

    #[test]
    fn variations_come_back_with_the_saved_game() {
        let now = Instant::now();
        let mut session = GameSession::default();
        for uci in ["e2e4", "e7e5"] {
            session.play(uci, now).expect("mossa legale");
        }
        session.go_to(1);
        session.play("c7c5", now).expect("variante");

        let restored = GameSession::restore(&session.snapshot(now), now).expect("ripristino");

        let view = restored.view(now);
        assert_eq!(view.game.tree.nodes.len(), 3);
        assert_eq!(view.game.moves.len(), 2);
        assert_eq!(view.game.moves[1].san, "e5");
    }
}
