//! Il tempo impiegato per ogni mossa.

use std::time::Instant;

use crate::clock::to_millis;

/// I tempi di una mossa, registrati per le statistiche.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MoveTiming {
    /// Quanto ha pensato chi ha mosso, in millesimi di secondo (`None` se non
    /// si sa, ad esempio per le mosse di una partita incollata).
    pub time_spent_ms: Option<u32>,
    /// Il tempo rimasto sull'orologio di chi ha mosso, subito dopo la mossa
    /// (`None` nelle partite senza orologio).
    pub clock_after_ms: Option<u32>,
}

/// Misura quanto pensa ciascun giocatore prima di ogni mossa.
///
/// Funziona anche senza orologio: misura il tempo passato dalla mossa
/// precedente (o dall'inizio della partita, per la prima). I tempi stanno
/// nello stesso ordine delle mosse della partita.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveTimer {
    /// Da quando sta pensando chi deve muovere.
    turn_started: Instant,
    /// I tempi delle mosse già giocate.
    timings: Vec<MoveTiming>,
}

impl MoveTimer {
    /// Un cronometro che parte adesso, senza mosse registrate.
    #[must_use]
    pub fn new(now: Instant) -> Self {
        Self::with_timings(now, Vec::new())
    }

    /// Un cronometro che parte adesso, con i tempi di mosse già giocate
    /// (ad esempio di una partita ripresa dall'archivio).
    #[must_use]
    pub fn with_timings(now: Instant, timings: Vec<MoveTiming>) -> Self {
        Self {
            turn_started: now,
            timings,
        }
    }

    /// Registra la mossa numero `count` (contando da 1) appena giocata.
    ///
    /// Se prima c'erano più tempi (il seguito della partita è stato
    /// sostituito), quelli in più vengono tolti.
    pub fn record(&mut self, count: usize, now: Instant, clock_after_ms: Option<u32>) {
        self.timings.truncate(count.saturating_sub(1));
        self.timings.push(MoveTiming {
            time_spent_ms: Some(to_millis(now.saturating_duration_since(self.turn_started))),
            clock_after_ms,
        });
        self.turn_started = now;
    }

    /// Tiene solo i tempi delle prime `count` mosse (dopo una mossa ritirata)
    /// e fa ripartire il cronometro da `now`.
    pub fn truncate(&mut self, count: usize, now: Instant) {
        self.timings.truncate(count);
        self.turn_started = now;
    }

    /// I tempi della mossa di indice `index` (0 = la prima), oppure vuoti se non si sanno.
    #[must_use]
    pub fn timing(&self, index: usize) -> MoveTiming {
        self.timings.get(index).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{MoveTimer, MoveTiming};

    #[test]
    fn measures_the_time_since_the_previous_move() {
        let start = Instant::now();
        let mut timer = MoveTimer::new(start);

        timer.record(1, start + Duration::from_secs(3), None);
        timer.record(2, start + Duration::from_secs(10), Some(55_000));

        assert_eq!(timer.timing(0).time_spent_ms, Some(3_000));
        assert_eq!(
            timer.timing(1),
            MoveTiming {
                time_spent_ms: Some(7_000),
                clock_after_ms: Some(55_000),
            }
        );
    }

    #[test]
    fn a_replaced_continuation_drops_the_old_times() {
        let start = Instant::now();
        let mut timer = MoveTimer::new(start);
        timer.record(1, start, None);
        timer.record(2, start, None);

        timer.record(1, start + Duration::from_secs(1), None);

        assert_eq!(timer.timing(0).time_spent_ms, Some(1_000));
        assert_eq!(timer.timing(1), MoveTiming::default());
    }

    #[test]
    fn unknown_times_stay_empty() {
        assert_eq!(
            MoveTimer::new(Instant::now()).timing(5),
            MoveTiming::default()
        );
    }
}
