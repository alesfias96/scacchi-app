//! L'orologio della partita.

use std::time::{Duration, Instant};

use app_settings::TimeControl;
use chess_core::Side;

use crate::clock_view::ClockView;

/// L'orologio da scacchi: due tempi, di cui ne scorre uno alla volta.
///
/// Il conteggio è fatto qui, in Rust, con l'orologio preciso del sistema:
/// l'interfaccia mostra soltanto i valori ricevuti. Ogni funzione riceve
/// l'istante "adesso" (`now`) invece di leggerlo da sola: così i test possono
/// simulare il passare del tempo senza aspettare davvero.
///
/// Regole:
/// - il tempo comincia a scorrere solo dopo la prima mossa (come nei siti di scacchi);
/// - dopo ogni mossa si aggiunge l'incremento a chi ha mosso e parte il tempo dell'altro;
/// - a partita finita l'orologio si ferma.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    /// Il tempo scelto per la partita.
    control: TimeControl,
    /// Il tempo rimasto al Bianco e al Nero, all'ultimo "scatto" dell'orologio.
    remaining: [Duration; 2],
    /// Chi sta consumando il proprio tempo, e da quando; `None` se è fermo.
    running: Option<(Side, Instant)>,
    /// `true` a partita finita: l'orologio non riparte più.
    stopped: bool,
}

/// La posizione di un colore negli elenchi a due elementi (Bianco, Nero).
fn index(side: Side) -> usize {
    match side {
        Side::White => 0,
        Side::Black => 1,
    }
}

impl Clock {
    /// Un orologio fermo, con il tempo iniziale per entrambi.
    #[must_use]
    pub fn new(control: TimeControl) -> Self {
        let initial = Duration::from_secs(u64::from(control.initial_seconds));
        Self {
            control,
            remaining: [initial; 2],
            running: None,
            stopped: false,
        }
    }

    /// Il tempo scelto per la partita.
    #[must_use]
    pub fn control(&self) -> TimeControl {
        self.control
    }

    /// Chi sta consumando il proprio tempo, se l'orologio è in moto.
    #[must_use]
    pub fn running_side(&self) -> Option<Side> {
        self.running.map(|(side, _)| side)
    }

    /// Il tempo rimasto a `side` nell'istante `now` (mai sotto zero).
    #[must_use]
    pub fn remaining(&self, side: Side, now: Instant) -> Duration {
        let stored = self.remaining[index(side)];
        match self.running {
            Some((running, since)) if running == side => {
                stored.saturating_sub(now.saturating_duration_since(since))
            }
            _ => stored,
        }
    }

    /// "Preme il pulsante" dopo la mossa di `mover`: gli toglie il tempo
    /// usato, gli aggiunge l'incremento e fa partire il tempo dell'avversario.
    pub fn press(&mut self, mover: Side, now: Instant) {
        if self.stopped {
            return;
        }
        if let Some((running, since)) = self.running
            && running == mover
        {
            let used = now.saturating_duration_since(since);
            let increment = Duration::from_secs(u64::from(self.control.increment_seconds));
            let slot = &mut self.remaining[index(mover)];
            *slot = slot.saturating_sub(used) + increment;
        }
        self.running = Some((mover.opponent(), now));
    }

    /// Ferma l'orologio per sempre (partita finita), registrando il tempo usato.
    pub fn stop(&mut self, now: Instant) {
        if let Some((running, _)) = self.running {
            self.remaining[index(running)] = self.remaining(running, now);
        }
        self.running = None;
        self.stopped = true;
    }

    /// Chi ha finito il tempo nell'istante `now`, se qualcuno l'ha finito.
    #[must_use]
    pub fn flagged(&self, now: Instant) -> Option<Side> {
        self.running_side()
            .filter(|side| self.remaining(*side, now).is_zero())
    }

    /// I tempi da mostrare nell'istante `now`.
    #[must_use]
    pub fn view(&self, now: Instant) -> ClockView {
        ClockView {
            white_ms: to_millis(self.remaining(Side::White, now)),
            black_ms: to_millis(self.remaining(Side::Black, now)),
            running: self.running_side(),
        }
    }
}

/// Una durata in millesimi di secondo, limitata al massimo di un `u32` (circa 49 giorni).
#[must_use]
pub fn to_millis(duration: Duration) -> u32 {
    u32::try_from(duration.as_millis()).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use app_settings::TimeControl;
    use chess_core::Side;

    use super::Clock;

    /// Tre minuti più due secondi a mossa.
    fn blitz() -> Clock {
        Clock::new(TimeControl {
            initial_seconds: 180,
            increment_seconds: 2,
        })
    }

    #[test]
    fn does_not_run_before_the_first_move() {
        let clock = blitz();
        let later = Instant::now() + Duration::from_secs(30);

        assert_eq!(
            clock.remaining(Side::White, later),
            Duration::from_secs(180)
        );
        assert_eq!(clock.running_side(), None);
    }

    #[test]
    fn the_first_move_starts_the_opponents_time() {
        let start = Instant::now();
        let mut clock = blitz();

        clock.press(Side::White, start);

        assert_eq!(clock.running_side(), Some(Side::Black));
        let later = start + Duration::from_secs(10);
        assert_eq!(
            clock.remaining(Side::Black, later),
            Duration::from_secs(170)
        );
        assert_eq!(
            clock.remaining(Side::White, later),
            Duration::from_secs(180)
        );
    }

    #[test]
    fn a_move_takes_the_time_used_and_adds_the_increment() {
        let start = Instant::now();
        let mut clock = blitz();
        clock.press(Side::White, start);

        clock.press(Side::Black, start + Duration::from_secs(10));

        assert_eq!(
            clock.remaining(Side::Black, start + Duration::from_secs(60)),
            Duration::from_secs(172)
        );
        assert_eq!(clock.running_side(), Some(Side::White));
    }

    #[test]
    fn reports_who_ran_out_of_time() {
        let start = Instant::now();
        let mut clock = blitz();
        clock.press(Side::White, start);

        assert_eq!(clock.flagged(start + Duration::from_secs(179)), None);
        assert_eq!(
            clock.flagged(start + Duration::from_secs(181)),
            Some(Side::Black)
        );
    }

    #[test]
    fn once_stopped_it_never_runs_again() {
        let start = Instant::now();
        let mut clock = blitz();
        clock.press(Side::White, start);
        clock.stop(start + Duration::from_secs(5));

        clock.press(Side::Black, start + Duration::from_secs(6));

        assert_eq!(clock.running_side(), None);
        assert_eq!(
            clock.view(start + Duration::from_secs(100)).black_ms,
            175_000
        );
    }
}
