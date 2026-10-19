//! La partita in corso: avversario, orologio, chi può muovere.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use app_settings::{ColorChoice, ComputerEngine, GameSetup, OpponentKind};
use chess_core::{Game, NodeId, Side, to_pgn};
use engine::{SearchLimits, SearchPosition, strength};
use openings::Opening;

use crate::clock::{Clock, to_millis};
use crate::computer_level::{clamp_level, computer_level};
use crate::computer_view::ComputerView;
use crate::engine_request::EngineRequest;
use crate::maia_request::MaiaRequest;
use crate::move_timer::MoveTimer;
use crate::session_error::SessionError;
use crate::session_id::{new_session_id, now_unix_ms};
use crate::session_view::SessionView;

/// Senza orologio, il computer non risponde prima di così: una risposta
/// istantanea, a chi gioca, sembra innaturale.
const MIN_REPLY_UNTIMED: Duration = Duration::from_millis(600);
/// Con l'orologio la pausa è più breve, e al massimo un sessantesimo del
/// tempo rimasto al computer, così nelle partite veloci non gli costa troppo.
const MIN_REPLY_TIMED: Duration = Duration::from_millis(400);
/// La frazione del tempo rimasto che la pausa non deve superare.
const MIN_REPLY_TIME_FRACTION: u32 = 60;

/// Il contatore dei contrassegni: unico per tutta l'app, così un contrassegno
/// di una partita vecchia non può mai coincidere con uno di quella nuova.
static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);

/// Un nuovo contrassegno, diverso da tutti i precedenti.
pub(crate) fn next_token() -> u64 {
    NEXT_TOKEN.fetch_add(1, Ordering::Relaxed)
}

/// Il computer avversario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Computer {
    /// Il suo colore.
    pub(crate) side: Side,
    /// Il motore che sceglie le sue mosse.
    pub(crate) engine: ComputerEngine,
    /// Il suo livello (da 1 a 10 per Stockfish, da 1 a 9 per Maia).
    pub(crate) level: u8,
}

/// La partita in corso, con le regole del modo di gioco scelto.
///
/// - **Due giocatori senza orologio**: si muove liberamente, anche da una
///   posizione passata (il seguito viene sostituito), e si ritirano le mosse.
/// - **Contro il computer** o **con l'orologio**: si gioca solo dall'ultima
///   posizione; contro il computer si muovono solo i propri pezzi.
/// - **Con l'orologio** le mosse non si ritirano.
///
/// Il tempo non viene mai letto da qui dentro: ogni funzione che ne ha
/// bisogno riceve l'istante `now` (solo la data di inizio, da salvare
/// nell'archivio, viene letta dall'orologio del sistema).
///
/// I campi sono visibili al resto della crate perché la "fotografia" da
/// salvare nell'archivio (`session_snapshot.rs`) li legge e li ricostruisce.
#[derive(Debug, Clone)]
pub struct GameSession {
    /// Il nome unico della partita, con cui viene salvata nell'archivio.
    pub(crate) id: String,
    /// Quando è cominciata, in millesimi di secondo dal 1970.
    pub(crate) started_at_ms: i64,
    /// La partita.
    pub(crate) game: Game,
    /// Il computer avversario, se c'è.
    pub(crate) computer: Option<Computer>,
    /// L'orologio, se c'è.
    pub(crate) clock: Option<Clock>,
    /// Il tempo impiegato per ogni mossa.
    pub(crate) timer: MoveTimer,
    /// Il contrassegno dello stato attuale (vedi [`EngineRequest`]).
    pub(crate) token: u64,
}

impl Default for GameSession {
    /// Una partita a due giocatori, senza orologio, dalla posizione iniziale.
    fn default() -> Self {
        Self::from_game(Game::new(), Instant::now())
    }
}

impl GameSession {
    /// Una partita a due giocatori senza orologio, a partire da `game`
    /// (usata ad esempio per una partita incollata). Il cronometro delle
    /// mosse parte da `now`.
    #[must_use]
    pub fn from_game(game: Game, now: Instant) -> Self {
        Self {
            id: new_session_id(),
            started_at_ms: now_unix_ms(),
            game,
            computer: None,
            clock: None,
            timer: MoveTimer::new(now),
            token: next_token(),
        }
    }

    /// Il nome unico della partita.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// La "versione" della partita: cambia a ogni mossa, mossa ritirata,
    /// abbandono o tempo scaduto, ma non quando si scorrono le mosse. Serve a
    /// salvare la partita solo quando è cambiata davvero.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.token
    }

    /// `true` se nella partita è stata giocata almeno una mossa (solo allora
    /// vale la pena salvarla nell'archivio).
    #[must_use]
    pub fn has_moves(&self) -> bool {
        !self.game.mainline().is_empty()
    }

    /// Comincia una partita nuova, come scelto nella finestra "Nuova partita".
    ///
    /// - Riceve: la configurazione, il risultato del "lancio della moneta"
    ///   (usato solo se il colore è da estrarre a sorte, vedi [`crate::coin_flip`])
    ///   e l'istante in cui comincia.
    /// - Restituisce: la partita, pronta a cominciare.
    #[must_use]
    pub fn start(setup: &GameSetup, coin: Side, now: Instant) -> Self {
        let mut session = Self::from_game(Game::new(), now);
        if setup.opponent == OpponentKind::Computer {
            let human = match setup.color {
                ColorChoice::White => Side::White,
                ColorChoice::Black => Side::Black,
                ColorChoice::Random => coin,
            };
            let level = match setup.engine {
                ComputerEngine::Stockfish => setup.level,
                ComputerEngine::Maia => setup.maia_level,
            };
            session.computer = Some(Computer {
                side: human.opponent(),
                engine: setup.engine,
                level: clamp_level(setup.engine, level),
            });
        }
        session.clock = setup.time_control.map(Clock::new);
        session
    }

    /// La partita come la mostra l'interfaccia, nell'istante `now`.
    #[must_use]
    pub fn view(&self, now: Instant) -> SessionView {
        SessionView {
            id: self.id.clone(),
            game: self.game.view(),
            opening: self.opening(),
            opponent: if self.computer.is_some() {
                OpponentKind::Computer
            } else {
                OpponentKind::TwoPlayers
            },
            computer: self.computer.map(|computer| ComputerView {
                side: computer.side,
                engine: computer.engine,
                level: computer_level(computer.engine, computer.level),
            }),
            time_control: self.clock.as_ref().map(Clock::control),
            clock: self.clock.as_ref().map(|clock| clock.view(now)),
            can_move: self.check_can_move().is_ok(),
            can_undo: self.can_undo(),
            can_resign: self.game.final_result().is_none(),
            can_edit_variations: self.can_edit_variations(),
            computer_thinking: self.computer_to_move(),
        }
    }

    /// La persona gioca una mossa (in notazione UCI) nella posizione mostrata.
    ///
    /// # Errors
    ///
    /// - [`SessionError::NotYourTurn`] se tocca al computer;
    /// - [`SessionError::NotAtEnd`] se, contro il computer o con l'orologio,
    ///   è mostrata una posizione passata;
    /// - [`SessionError::Game`] se la mossa non è legale o la partita è finita
    ///   (anche per il tempo appena scaduto).
    pub fn play(&mut self, uci: &str, now: Instant) -> Result<(), SessionError> {
        self.check_time(now);
        self.check_can_move()?;
        // Solo una mossa in fondo alla linea principale fa andare avanti la
        // partita (orologio, tempi); le altre diventano varianti.
        let extends_mainline = self.game.is_at_mainline_end();
        let mover = self.game.turn();
        self.game.play_uci(uci)?;
        if extends_mainline {
            self.after_move(mover, now);
        } else {
            self.token = next_token();
        }
        Ok(())
    }

    /// Mostra la posizione dopo una mossa dell'albero (`None` = partenza), anche in una variante.
    ///
    /// # Errors
    ///
    /// [`SessionError::Game`] se la mossa non è (più) nella partita.
    pub fn go_to_node(&mut self, node: Option<NodeId>) -> Result<(), SessionError> {
        Ok(self.game.go_to_node(node)?)
    }

    /// Fa diventare linea principale la variante che passa dalla mossa indicata.
    ///
    /// # Errors
    ///
    /// - [`SessionError::VariationsLocked`] se la partita non lo permette (finita,
    ///   contro il computer o con l'orologio: la linea principale è la partita giocata);
    /// - [`SessionError::Game`] se la mossa non è (più) nella partita.
    pub fn promote_variation(&mut self, node: NodeId, now: Instant) -> Result<(), SessionError> {
        if !self.can_edit_variations() {
            return Err(SessionError::VariationsLocked);
        }
        let before = self.mainline_uci();
        self.game.promote(node)?;
        // I tempi valgono solo per le mosse che restano uguali all'inizio della linea principale.
        let kept = before
            .iter()
            .zip(self.mainline_uci())
            .take_while(|(old, new)| **old == *new)
            .count();
        self.timer.truncate(kept, now);
        self.token = next_token();
        Ok(())
    }

    /// Elimina una variante (la mossa indicata e il suo seguito).
    ///
    /// # Errors
    ///
    /// [`SessionError::Game`] se la mossa non è (più) nella partita o è della linea principale.
    pub fn delete_variation(&mut self, node: NodeId) -> Result<(), SessionError> {
        self.game.delete_variation(node)?;
        self.token = next_token();
        Ok(())
    }

    /// Il tempo minimo prima della risposta del computer: senza orologio una
    /// pausa fissa, con l'orologio una più breve (al massimo un sessantesimo
    /// del tempo che gli resta).
    fn min_reply(&self, computer: Computer, now: Instant) -> Duration {
        self.clock.as_ref().map_or(MIN_REPLY_UNTIMED, |clock| {
            MIN_REPLY_TIMED.min(clock.remaining(computer.side, now) / MIN_REPLY_TIME_FRACTION)
        })
    }

    /// Il computer, se tocca a lui muovere con quel motore.
    fn computer_turn(&self, wanted: ComputerEngine) -> Option<Computer> {
        self.computer
            .filter(|computer| computer.engine == wanted && self.computer_to_move())
    }

    /// La partita dalla posizione di partenza, come la vogliono i motori.
    fn search_position(&self) -> SearchPosition {
        SearchPosition {
            start_fen: self.game.starting_fen(),
            moves: self.mainline_uci(),
        }
    }

    /// La richiesta da mandare a Maia, se il computer è Maia e tocca a lui.
    #[must_use]
    pub fn maia_request(&self, now: Instant) -> Option<MaiaRequest> {
        let computer = self.computer_turn(ComputerEngine::Maia)?;
        Some(MaiaRequest {
            token: self.token,
            position: self.search_position(),
            level: computer.level,
            min_reply: self.min_reply(computer, now),
        })
    }

    /// La richiesta da mandare a Stockfish, se il computer è Stockfish e tocca a lui.
    ///
    /// Restituisce `None` se non c'è il computer, se è Maia, se non è il suo
    /// turno o se la partita è finita.
    #[must_use]
    pub fn engine_request(&self, now: Instant) -> Option<EngineRequest> {
        let computer = self.computer_turn(ComputerEngine::Stockfish)?;
        let level = strength(computer.level);
        let limits = match &self.clock {
            Some(clock) => {
                let ms = |side| Some(u64::from(to_millis(clock.remaining(side, now))));
                let increment = Some(u64::from(clock.control().increment_seconds) * 1000);
                SearchLimits {
                    white_time_ms: ms(Side::White),
                    black_time_ms: ms(Side::Black),
                    white_increment_ms: increment,
                    black_increment_ms: increment,
                    depth: level.depth,
                    ..SearchLimits::default()
                }
            }
            None => SearchLimits {
                move_time_ms: Some(level.move_time_ms),
                depth: level.depth,
                ..SearchLimits::default()
            },
        };
        Some(EngineRequest {
            token: self.token,
            position: self.search_position(),
            limits,
            options: level.options,
            min_reply: self.min_reply(computer, now),
        })
    }

    /// Gioca la mossa trovata dal computer, alla fine della partita.
    ///
    /// # Errors
    ///
    /// - [`SessionError::StaleEngineMove`] se nel frattempo la partita è cambiata;
    /// - [`SessionError::Game`] se la mossa non è legale o la partita è finita.
    pub fn play_engine(&mut self, token: u64, uci: &str, now: Instant) -> Result<(), SessionError> {
        if token != self.token || !self.computer_to_move() {
            return Err(SessionError::StaleEngineMove);
        }
        self.check_time(now);
        // L'utente potrebbe star guardando una posizione passata: la mossa del
        // computer va comunque in fondo alla partita.
        self.game.go_to_mainline_end();
        let mover = self.game.turn();
        self.game.play_uci(uci)?;
        self.after_move(mover, now);
        Ok(())
    }

    /// Ritira l'ultima mossa. Contro il computer ritira anche la sua risposta,
    /// così torna il turno della persona.
    ///
    /// # Errors
    ///
    /// [`SessionError::UndoNotAllowed`] se la partita ha l'orologio.
    pub fn undo(&mut self, now: Instant) -> Result<(), SessionError> {
        if self.clock.is_some() {
            return Err(SessionError::UndoNotAllowed);
        }
        match self.computer {
            Some(computer) => {
                if !self.has_human_move(computer) {
                    return Ok(());
                }
                // Toglie mosse finché non ha tolto l'ultima della persona.
                while let Some(last) = self.game.mainline().last().map(|record| record.side) {
                    self.game.undo();
                    if last != computer.side {
                        break;
                    }
                }
            }
            None => self.game.undo(),
        }
        self.timer.truncate(self.game.mainline().len(), now);
        self.token = next_token();
        Ok(())
    }

    /// Mostra la posizione dopo `ply` mosse (0 = partenza).
    pub fn go_to(&mut self, ply: usize) {
        self.game.go_to(ply);
    }

    /// Abbandona: contro il computer abbandona la persona; a due giocatori
    /// abbandona chi deve muovere.
    ///
    /// # Errors
    ///
    /// [`SessionError::Game`] se la partita è già finita.
    pub fn resign(&mut self, now: Instant) -> Result<(), SessionError> {
        let loser = self.computer.map_or_else(
            || self.game.last_turn(),
            |computer| computer.side.opponent(),
        );
        self.game.resign(loser)?;
        if let Some(clock) = &mut self.clock {
            clock.stop(now);
        }
        self.token = next_token();
        Ok(())
    }

    /// Controlla l'orologio: se a chi deve muovere è finito il tempo, la
    /// partita finisce.
    ///
    /// Restituisce `true` se la partita è appena finita per il tempo.
    pub fn check_time(&mut self, now: Instant) -> bool {
        let Some(clock) = &mut self.clock else {
            return false;
        };
        if self.game.final_result().is_some() {
            return false;
        }
        let Some(loser) = clock.flagged(now) else {
            return false;
        };
        clock.stop(now);
        let _ = self.game.time_forfeit(loser);
        self.token = next_token();
        true
    }

    /// Fra quanto finirà il tempo di chi sta pensando, se l'orologio è in moto.
    ///
    /// Serve al ponte per controllare l'orologio esattamente in quell'istante.
    #[must_use]
    pub fn time_until_flag(&self, now: Instant) -> Option<Duration> {
        if self.game.final_result().is_some() {
            return None;
        }
        let clock = self.clock.as_ref()?;
        clock.running_side().map(|side| clock.remaining(side, now))
    }

    /// La posizione mostrata, da far analizzare al motore; `None` se in quella
    /// posizione la partita è finita (non c'è nulla da analizzare).
    #[must_use]
    pub fn analysis_position(&self) -> Option<SearchPosition> {
        if self.game.result().is_some() {
            return None;
        }
        Some(SearchPosition {
            start_fen: self.game.starting_fen(),
            moves: self.uci_moves(self.game.cursor()),
        })
    }

    /// La posizione mostrata, in notazione FEN.
    #[must_use]
    pub fn shown_fen(&self) -> String {
        self.game.fen()
    }

    /// La partita in formato PGN.
    #[must_use]
    pub fn export_pgn(&self) -> String {
        to_pgn(&self.game)
    }

    /// L'apertura giocata (la più precisa riconosciuta), se c'è.
    #[must_use]
    pub fn opening(&self) -> Option<Opening> {
        openings::identify(&self.game.starting_fen(), &self.mainline_uci())
    }

    /// Dopo una mossa: aggiorna l'orologio (e lo ferma se la partita è finita)
    /// e cambia il contrassegno.
    fn after_move(&mut self, mover: Side, now: Instant) {
        let mut clock_after_ms = None;
        if let Some(clock) = &mut self.clock {
            clock.press(mover, now);
            if self.game.final_result().is_some() {
                clock.stop(now);
            }
            clock_after_ms = Some(to_millis(clock.remaining(mover, now)));
        }
        self.timer
            .record(self.game.mainline().len(), now, clock_after_ms);
        self.token = next_token();
    }

    /// Controlla se la persona può muovere adesso, e se no perché.
    fn check_can_move(&self) -> Result<(), SessionError> {
        if self.game.result().is_some() {
            return Err(chess_core::GameError::GameOver.into());
        }
        // A partita finita si può giocare dappertutto: le mosse diventano varianti.
        if self.game.final_result().is_some() {
            return Ok(());
        }
        let strict = self.computer.is_some() || self.clock.is_some();
        if strict && !self.game.is_at_mainline_end() {
            return Err(SessionError::NotAtEnd);
        }
        match self.computer {
            Some(computer) if self.game.turn() == computer.side => Err(SessionError::NotYourTurn),
            _ => Ok(()),
        }
    }

    /// `true` se tocca al computer nell'ultima posizione e la partita non è finita.
    fn computer_to_move(&self) -> bool {
        self.computer.is_some_and(|computer| {
            self.game.final_result().is_none() && self.game.last_turn() == computer.side
        })
    }

    /// `true` se si può ritirare la mossa.
    fn can_undo(&self) -> bool {
        if self.clock.is_some() {
            return false;
        }
        match self.computer {
            Some(computer) => self.has_human_move(computer),
            None => !self.game.mainline().is_empty(),
        }
    }

    /// `true` se le varianti si possono promuovere: solo nelle partite a due
    /// giocatori senza orologio e non finite (altrimenti la linea principale è
    /// la partita giocata davvero, e non si cambia).
    fn can_edit_variations(&self) -> bool {
        self.computer.is_none() && self.clock.is_none() && self.game.final_result().is_none()
    }

    /// `true` se la persona ha già giocato almeno una mossa contro il computer.
    fn has_human_move(&self, computer: Computer) -> bool {
        self.game
            .mainline()
            .iter()
            .any(|record| record.side != computer.side)
    }

    /// Le mosse della linea principale, in notazione UCI.
    pub(crate) fn mainline_uci(&self) -> Vec<String> {
        self.game
            .mainline()
            .into_iter()
            .map(|record| record.uci)
            .collect()
    }

    /// Le prime `count` mosse della linea guardata, in notazione UCI.
    pub(crate) fn uci_moves(&self, count: usize) -> Vec<String> {
        self.game.moves()[..count]
            .iter()
            .map(|record| record.uci.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use app_settings::{ColorChoice, ComputerEngine, GameSetup, OpponentKind, TimeControl};
    use chess_core::{GameEndReason, Side};

    use super::GameSession;
    use crate::session_error::SessionError;

    fn against_computer(color: ColorChoice, time_control: Option<TimeControl>) -> GameSetup {
        GameSetup {
            opponent: OpponentKind::Computer,
            color,
            engine: ComputerEngine::Stockfish,
            level: 4,
            maia_level: 5,
            time_control,
        }
    }

    #[test]
    fn maia_gets_its_own_request_with_its_own_level() {
        let now = Instant::now();
        let setup = GameSetup {
            engine: ComputerEngine::Maia,
            maia_level: 3,
            ..against_computer(ColorChoice::White, None)
        };
        let mut session = GameSession::start(&setup, Side::White, now);
        session.play("e2e4", now).expect("mossa legale");

        assert_eq!(session.engine_request(now), None, "Stockfish non c'entra");
        let request = session.maia_request(now).expect("tocca a Maia");
        assert_eq!(request.level, 3);
        assert_eq!(request.position.moves, ["e2e4"]);
        let computer = session.view(now).computer.expect("computer");
        assert_eq!(computer.engine, ComputerEngine::Maia);
        assert_eq!(computer.level.name, "Maia 1300");

        session
            .play_engine(request.token, "e7e5", now)
            .expect("mossa di Maia");
        assert_eq!(session.maia_request(now), None, "ora tocca alla persona");
    }

    const BLITZ: TimeControl = TimeControl {
        initial_seconds: 180,
        increment_seconds: 2,
    };

    #[test]
    fn the_default_is_a_free_two_player_game() {
        let mut session = GameSession::default();
        let now = Instant::now();
        session.play("e2e4", now).expect("mossa legale");
        session.go_to(0);

        // A due giocatori senza orologio si può rigiocare da una posizione passata.
        session
            .play("d2d4", now)
            .expect("mossa da una posizione passata");
        let view = session.view(now);
        assert_eq!(view.opponent, OpponentKind::TwoPlayers);
        assert_eq!(view.game.moves.len(), 1);
        assert!(view.can_undo);
        assert_eq!(session.engine_request(now), None);
    }

    #[test]
    fn the_computer_moves_when_it_is_its_turn() {
        let now = Instant::now();
        let mut session = GameSession::start(
            &against_computer(ColorChoice::White, None),
            Side::White,
            now,
        );
        assert_eq!(session.engine_request(now), None);

        session.play("e2e4", now).expect("mossa legale");

        let request = session.engine_request(now).expect("tocca al computer");
        assert_eq!(request.position.moves, ["e2e4"]);
        assert_eq!(request.limits.move_time_ms, Some(500));
        assert!(session.view(now).computer_thinking);
        assert_eq!(session.play("e7e5", now), Err(SessionError::NotYourTurn));

        session
            .play_engine(request.token, "e7e5", now)
            .expect("mossa del computer");
        assert!(session.view(now).can_move);
    }

    #[test]
    fn the_computer_starts_when_the_human_plays_black() {
        let now = Instant::now();
        let session = GameSession::start(
            &against_computer(ColorChoice::Random, None),
            Side::Black,
            now,
        );

        let view = session.view(Instant::now());
        assert_eq!(view.computer.expect("computer").side, Side::White);
        assert!(session.engine_request(Instant::now()).is_some());
        assert!(!view.can_move);
        assert!(!view.can_undo);
    }

    #[test]
    fn an_old_computer_move_is_discarded() {
        let now = Instant::now();
        let mut session = GameSession::start(
            &against_computer(ColorChoice::White, None),
            Side::White,
            now,
        );
        session.play("e2e4", now).expect("mossa legale");
        let request = session.engine_request(now).expect("tocca al computer");

        session.undo(now).expect("si può ritirare");

        assert_eq!(
            session.play_engine(request.token, "e7e5", now),
            Err(SessionError::StaleEngineMove)
        );
        assert_eq!(session.view(now).game.moves.len(), 0);
    }

    #[test]
    fn undo_against_the_computer_takes_back_both_moves() {
        let now = Instant::now();
        let mut session = GameSession::start(
            &against_computer(ColorChoice::White, None),
            Side::White,
            now,
        );
        session.play("e2e4", now).expect("mossa legale");
        let request = session.engine_request(now).expect("tocca al computer");
        session
            .play_engine(request.token, "e7e5", now)
            .expect("mossa del computer");

        session.undo(now).expect("si può ritirare");

        assert_eq!(session.view(now).game.moves.len(), 0);
        assert!(session.view(now).can_move);
    }

    #[test]
    fn against_the_computer_moves_are_played_only_from_the_last_position() {
        let now = Instant::now();
        let mut session = GameSession::start(
            &against_computer(ColorChoice::White, None),
            Side::White,
            now,
        );
        session.play("e2e4", now).expect("mossa legale");
        let request = session.engine_request(now).expect("tocca al computer");
        session
            .play_engine(request.token, "e7e5", now)
            .expect("mossa del computer");

        session.go_to(0);

        assert_eq!(session.play("d2d4", now), Err(SessionError::NotAtEnd));
    }

    #[test]
    fn the_clock_runs_and_sends_the_times_to_the_engine() {
        let start = Instant::now();
        let mut session = GameSession::start(
            &against_computer(ColorChoice::White, Some(BLITZ)),
            Side::White,
            start,
        );
        session.play("e2e4", start).expect("mossa legale");

        let later = start + Duration::from_secs(10);
        let request = session.engine_request(later).expect("tocca al computer");

        assert_eq!(request.limits.white_time_ms, Some(180_000));
        assert_eq!(request.limits.black_time_ms, Some(170_000));
        assert_eq!(request.limits.white_increment_ms, Some(2_000));
        assert_eq!(request.limits.move_time_ms, None);
        assert_eq!(session.undo(later), Err(SessionError::UndoNotAllowed));
        assert!(!session.view(later).can_undo);
    }

    #[test]
    fn running_out_of_time_ends_the_game() {
        let start = Instant::now();
        let setup = GameSetup {
            opponent: OpponentKind::TwoPlayers,
            time_control: Some(BLITZ),
            ..GameSetup::default()
        };
        let mut session = GameSession::start(&setup, Side::White, start);
        session.play("e2e4", start).expect("mossa legale");
        assert_eq!(
            session.time_until_flag(start),
            Some(Duration::from_secs(180))
        );

        let too_late = start + Duration::from_secs(181);
        assert_eq!(
            session.play("e7e5", too_late),
            Err(SessionError::Game(chess_core::GameError::GameOver))
        );

        let result = session.view(too_late).game.result.expect("partita finita");
        assert_eq!(result.reason, GameEndReason::Timeout);
        assert_eq!(result.winner, Some(Side::White));
        assert_eq!(session.time_until_flag(too_late), None);
    }

    #[test]
    fn resigning_against_the_computer_makes_the_computer_win() {
        let now = Instant::now();
        let mut session = GameSession::start(
            &against_computer(ColorChoice::Black, None),
            Side::White,
            now,
        );

        session.resign(now).expect("la partita non è finita");

        let view = session.view(now);
        let result = view.game.result.expect("partita finita");
        assert_eq!(result.winner, Some(Side::White));
        assert!(!view.can_resign);
        assert_eq!(session.engine_request(now), None);
    }

    #[test]
    fn analyses_the_position_shown() {
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        session.play("e7e5", now).expect("mossa legale");
        session.go_to(1);

        let position = session
            .analysis_position()
            .expect("posizione da analizzare");

        assert_eq!(position.moves, ["e2e4"]);
    }

    #[test]
    fn in_a_two_player_game_a_move_from_the_past_becomes_a_variation() {
        let now = Instant::now();
        let mut session = GameSession::default();
        for uci in ["e2e4", "e7e5", "g1f3"] {
            session.play(uci, now).expect("mossa legale");
        }
        let revision = session.revision();

        session.go_to(1);
        session.play("c7c5", now).expect("variante");

        let view = session.view(now);
        assert_ne!(session.revision(), revision);
        assert!(view.can_edit_variations);
        assert_eq!(view.game.tree.nodes.len(), 4);
        // La partita salvata resta quella della linea principale.
        assert_eq!(session.snapshot(now).moves.len(), 3);
    }

    #[test]
    fn a_variation_can_become_the_mainline_in_a_two_player_game() {
        let now = Instant::now();
        let mut session = GameSession::default();
        for uci in ["e2e4", "e7e5"] {
            session.play(uci, now).expect("mossa legale");
        }
        session.go_to(1);
        session.play("c7c5", now).expect("variante");
        let c5 = session.view(now).game.tree.current.expect("mossa mostrata");

        session
            .promote_variation(usize::try_from(c5).expect("numero"), now)
            .expect("promozione");

        let snapshot = session.snapshot(now);
        assert_eq!(snapshot.moves[1].record.san, "c5");
        assert!(snapshot.pgn.contains("1. e4 c5 (1... e5)"));
    }

    #[test]
    fn after_a_finished_game_against_the_computer_variations_are_free() {
        let now = Instant::now();
        let mut session = GameSession::start(
            &against_computer(ColorChoice::White, None),
            Side::White,
            now,
        );
        session.play("e2e4", now).expect("mossa legale");
        session.resign(now).expect("abbandono");

        session.go_to(0);
        // A partita finita si provano altre mosse, e il computer non risponde.
        session.play("d2d4", now).expect("variante dopo la fine");

        let view = session.view(now);
        assert!(!view.computer_thinking);
        assert!(!view.can_edit_variations);
        let node = usize::try_from(view.game.tree.current.expect("mossa")).expect("numero");
        assert_eq!(
            session.promote_variation(node, now),
            Err(SessionError::VariationsLocked)
        );
        assert_eq!(session.snapshot(now).moves.len(), 1);
    }
}
