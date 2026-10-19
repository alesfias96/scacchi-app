//! Una scelta "a caso" semplice e ripetibile.

use std::time::{SystemTime, UNIX_EPOCH};

/// Un numero di partenza diverso a ogni chiamata, preso dall'orologio di sistema.
#[must_use]
pub fn seed_from_clock() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            u64::try_from(elapsed.as_nanos() % u128::from(u64::MAX)).unwrap_or(0)
        })
}

/// Mescola i bit del numero di partenza (algoritmo "SplitMix64"): numeri di
/// partenza vicini danno risultati molto diversi.
fn mix(seed: u64) -> u64 {
    let mut value = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

/// Sceglie un elemento "a caso" (lo stesso, a parità di numero di partenza:
/// così i test sono ripetibili).
///
/// Restituisce `None` se l'elenco è vuoto.
#[must_use]
pub fn pick<T>(items: &[T], seed: u64) -> Option<&T> {
    if items.is_empty() {
        return None;
    }
    let length = u64::try_from(items.len()).unwrap_or(u64::MAX);
    let index = usize::try_from(mix(seed) % length).unwrap_or(0);
    items.get(index)
}

#[cfg(test)]
mod tests {
    use super::pick;

    #[test]
    fn the_same_seed_gives_the_same_choice() {
        let items = [1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(pick(&items, 42), pick(&items, 42));
        assert!(pick(&items, 42).is_some());
    }

    #[test]
    fn different_seeds_spread_over_the_list() {
        let items: Vec<u32> = (0..10).collect();
        let chosen: std::collections::HashSet<u32> = (0..100)
            .filter_map(|seed| pick(&items, seed).copied())
            .collect();

        assert!(chosen.len() > 5);
    }

    #[test]
    fn an_empty_list_gives_nothing() {
        assert_eq!(pick::<u8>(&[], 7), None);
    }
}
