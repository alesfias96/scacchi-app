//! Dalle uscite della rete alla mossa da giocare.
//!
//! La rete dà un valore a ogni mossa; la **softmax** li trasforma in
//! probabilità (tutte positive, che sommano a 1), considerando solo le mosse
//! legali. Sono le probabilità che una persona di quel livello giochi
//! ciascuna mossa.
//!
//! Per non giocare sempre la stessa partita, Maia non prende sempre la mossa
//! più probabile: estrae a sorte tra le mosse che hanno almeno un decimo della
//! probabilità della migliore, ciascuna con la sua probabilità. Così sceglie
//! tra le mosse "da persona" e lascia fuori quelle che quasi nessuno giocherebbe.

use std::time::{SystemTime, UNIX_EPOCH};

/// Una mossa con la sua probabilità.
#[derive(Debug, Clone, PartialEq)]
pub struct MoveProbability {
    /// La mossa in notazione UCI (es. "e2e4"; l'arrocco come "e1g1").
    pub uci: String,
    /// La probabilità, da 0 a 1.
    pub probability: f32,
}

/// Le mosse considerate rispetto alla migliore: almeno un decimo della sua probabilità.
const MIN_SHARE: f32 = 0.1;

/// Le probabilità delle mosse, dai loro valori grezzi; dalla più probabile.
#[must_use]
pub fn softmax(scored: Vec<(String, f32)>) -> Vec<MoveProbability> {
    let highest = scored
        .iter()
        .map(|(_, logit)| *logit)
        .fold(f32::NEG_INFINITY, f32::max);
    // Si sottrae il valore più alto prima dell'esponenziale: il risultato non
    // cambia, ma si evitano numeri troppo grandi.
    let exponentials: Vec<(String, f32)> = scored
        .into_iter()
        .map(|(uci, logit)| (uci, (logit - highest).exp()))
        .collect();
    let total: f32 = exponentials.iter().map(|(_, value)| value).sum();
    let mut probabilities: Vec<MoveProbability> = exponentials
        .into_iter()
        .map(|(uci, value)| MoveProbability {
            uci,
            probability: value / total,
        })
        .collect();
    probabilities.sort_by(|a, b| b.probability.total_cmp(&a.probability));
    probabilities
}

/// Sceglie la mossa: `roll` è un numero a caso tra 0 (compreso) e 1 (escluso).
///
/// Le probabilità devono essere in ordine, dalla più alta (come le dà [`softmax`]).
/// Restituisce `None` se non ci sono mosse.
#[must_use]
pub fn pick(probabilities: &[MoveProbability], roll: f64) -> Option<&MoveProbability> {
    let best = probabilities.first()?.probability;
    let candidates: Vec<&MoveProbability> = probabilities
        .iter()
        .take_while(|candidate| candidate.probability >= best * MIN_SHARE)
        .collect();
    let total: f64 = candidates
        .iter()
        .map(|candidate| f64::from(candidate.probability))
        .sum();
    let mut target = roll.clamp(0.0, 1.0) * total;
    for candidate in &candidates {
        target -= f64::from(candidate.probability);
        if target < 0.0 {
            return Some(candidate);
        }
    }
    candidates.last().copied()
}

/// Un numero a caso tra 0 e 1, dall'orologio di sistema.
///
/// Per scegliere una mossa basta: non serve un generatore "da crittografia".
#[must_use]
pub fn random_roll() -> f64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.subsec_nanos());
    f64::from(nanos % 1_000_000) / 1_000_000.0
}

#[cfg(test)]
mod tests {
    use super::{pick, softmax};

    #[test]
    fn probabilities_sum_to_one_and_are_sorted() {
        let probabilities = softmax(vec![("a".to_owned(), 0.0), ("b".to_owned(), 2.0_f32.ln())]);

        assert_eq!(probabilities[0].uci, "b");
        assert!((probabilities[0].probability - 2.0 / 3.0).abs() < 1e-6);
        assert!((probabilities[1].probability - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn rare_moves_are_never_chosen() {
        // "c" ha meno di un decimo della probabilità di "a": non si sceglie mai.
        let probabilities = softmax(vec![
            ("a".to_owned(), 3.0),
            ("b".to_owned(), 2.5),
            ("c".to_owned(), -2.0),
        ]);

        assert_eq!(pick(&probabilities, 0.0).map(|m| m.uci.as_str()), Some("a"));
        assert_eq!(
            pick(&probabilities, 0.999).map(|m| m.uci.as_str()),
            Some("b")
        );
        assert_eq!(pick(&[], 0.5), None);
    }
}
