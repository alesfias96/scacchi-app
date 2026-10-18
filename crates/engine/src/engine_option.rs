//! Un'impostazione offerta dal motore.

/// Un'impostazione che il motore dichiara all'avvio (riga `option ...`),
/// ad esempio `UCI_Elo` con il suo valore minimo e massimo.
///
/// Serve a non chiedere mai al motore un valore fuori dai suoi limiti: ogni
/// versione di Stockfish può avere limiti diversi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineOption {
    /// Il nome dell'impostazione (può contenere spazi, es. `"Skill Level"`).
    pub name: String,
    /// Il valore minimo, per le impostazioni numeriche.
    pub min: Option<i64>,
    /// Il valore massimo, per le impostazioni numeriche.
    pub max: Option<i64>,
}

impl EngineOption {
    /// Riporta un valore dentro i limiti dell'impostazione.
    ///
    /// - Riceve: il valore desiderato, come testo.
    /// - Restituisce: lo stesso valore se va bene o non è un numero; altrimenti
    ///   il minimo o il massimo più vicino.
    #[must_use]
    pub fn clamp(&self, value: &str) -> String {
        let Ok(number) = value.parse::<i64>() else {
            return value.to_owned();
        };
        let low = self.min.unwrap_or(i64::MIN);
        let high = self.max.unwrap_or(i64::MAX);
        number.clamp(low, high).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::EngineOption;

    fn elo() -> EngineOption {
        EngineOption {
            name: "UCI_Elo".to_owned(),
            min: Some(1320),
            max: Some(3190),
        }
    }

    #[test]
    fn keeps_a_value_inside_the_limits() {
        assert_eq!(elo().clamp("1600"), "1600");
    }

    #[test]
    fn brings_a_value_back_inside_the_limits() {
        assert_eq!(elo().clamp("800"), "1320");
        assert_eq!(elo().clamp("4000"), "3190");
    }

    #[test]
    fn leaves_text_values_alone() {
        assert_eq!(elo().clamp("true"), "true");
    }
}
