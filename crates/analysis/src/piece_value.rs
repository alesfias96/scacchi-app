//! Quanto vale ogni pezzo.

use shakmaty::Role;

/// Il valore di un pezzo in pedoni, secondo la scala classica:
/// pedone 1, cavallo e alfiere 3, torre 5, donna 9.
///
/// Il re non si può catturare: gli si dà un valore enorme, così nei calcoli
/// degli scambi "perdere il re" è sempre la cosa peggiore.
#[must_use]
pub fn piece_value(role: Role) -> i32 {
    match role {
        Role::Pawn => 1,
        Role::Knight | Role::Bishop => 3,
        Role::Rook => 5,
        Role::Queen => 9,
        Role::King => 1000,
    }
}

#[cfg(test)]
mod tests {
    use super::piece_value;
    use shakmaty::Role;

    #[test]
    fn follows_the_classical_scale() {
        assert_eq!(piece_value(Role::Pawn), 1);
        assert_eq!(piece_value(Role::Knight), piece_value(Role::Bishop));
        assert!(piece_value(Role::Queen) > piece_value(Role::Rook));
        assert!(piece_value(Role::King) > 100);
    }
}
