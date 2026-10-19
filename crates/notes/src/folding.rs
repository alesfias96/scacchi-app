//! Il testo "piegato" per la ricerca: minuscolo e senza accenti.
//!
//! Così "Perché", "perche" e "PERCHÉ" diventano tutti "perche" e la ricerca
//! li trova uno con l'altro. Si tolgono gli accenti delle lettere latine più
//! comuni (italiano, francese, spagnolo, tedesco…); gli altri caratteri
//! restano come sono.

/// La lettera senza accento, se `letter` (già minuscola) ne ha uno.
fn plain(letter: char) -> Option<char> {
    let plain = match letter {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
        'è' | 'é' | 'ê' | 'ë' => 'e',
        'ì' | 'í' | 'î' | 'ï' => 'i',
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
        'ù' | 'ú' | 'û' | 'ü' => 'u',
        'ç' => 'c',
        'ñ' => 'n',
        'ý' | 'ÿ' => 'y',
        _ => return None,
    };
    Some(plain)
}

/// Il testo in minuscolo, senza accenti e con gli spazi ridotti a uno.
#[must_use]
pub fn fold(text: &str) -> String {
    let lowered: String = text
        .chars()
        .flat_map(char::to_lowercase)
        .map(|letter| plain(letter).unwrap_or(letter))
        .collect();
    lowered.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::fold;

    #[test]
    fn ignores_case_accents_and_extra_spaces() {
        assert_eq!(fold("  PERCHÉ   la Città "), "perche la citta");
        assert_eq!(fold("Nimzo-Indiana"), "nimzo-indiana");
    }
}
