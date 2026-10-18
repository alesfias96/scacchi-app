//! Per quanto tempo (o quanto in profondità) far pensare il motore.

use std::fmt;

/// I limiti di una ricerca (comando UCI `go ...`).
///
/// Si possono combinare: ad esempio tempo sull'orologio e profondità massima
/// (per i livelli bassi, che devono "vedere" poco).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SearchLimits {
    /// Pensa all'infinito, finché non riceve `stop` (usato per l'analisi).
    pub infinite: bool,
    /// Profondità massima, in mezze mosse.
    pub depth: Option<u32>,
    /// Tempo fisso per questa mossa, in millisecondi.
    pub move_time_ms: Option<u64>,
    /// Tempo rimasto sull'orologio del Bianco, in millisecondi.
    pub white_time_ms: Option<u64>,
    /// Tempo rimasto sull'orologio del Nero, in millisecondi.
    pub black_time_ms: Option<u64>,
    /// Incremento per mossa del Bianco, in millisecondi.
    pub white_increment_ms: Option<u64>,
    /// Incremento per mossa del Nero, in millisecondi.
    pub black_increment_ms: Option<u64>,
}

impl SearchLimits {
    /// Una ricerca senza fine, per l'analisi in tempo reale.
    #[must_use]
    pub fn infinite() -> Self {
        Self {
            infinite: true,
            ..Self::default()
        }
    }
}

impl fmt::Display for SearchLimits {
    /// Scrive il comando UCI, ad esempio `go wtime 60000 btime 60000 depth 5`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("go")?;
        if self.infinite {
            formatter.write_str(" infinite")?;
        }
        let numbers = [
            ("wtime", self.white_time_ms),
            ("btime", self.black_time_ms),
            ("winc", self.white_increment_ms),
            ("binc", self.black_increment_ms),
            ("movetime", self.move_time_ms),
            ("depth", self.depth.map(u64::from)),
        ];
        for (name, value) in numbers {
            if let Some(value) = value {
                write!(formatter, " {name} {value}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SearchLimits;

    #[test]
    fn writes_an_infinite_search() {
        assert_eq!(SearchLimits::infinite().to_string(), "go infinite");
    }

    #[test]
    fn writes_clock_times_and_depth() {
        let limits = SearchLimits {
            white_time_ms: Some(60_000),
            black_time_ms: Some(55_000),
            white_increment_ms: Some(2_000),
            black_increment_ms: Some(2_000),
            depth: Some(5),
            ..SearchLimits::default()
        };

        assert_eq!(
            limits.to_string(),
            "go wtime 60000 btime 55000 winc 2000 binc 2000 depth 5"
        );
    }
}
