//! I temi dei puzzle, con i nomi in italiano.

use std::collections::HashMap;

use crate::catalog::all_puzzles;

/// Un tema da mostrare nella scelta dell'allenamento.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct ThemeInfo {
    /// Il nome del tema per Lichess (es. `"fork"`): è quello che si salva e si cerca.
    pub key: String,
    /// Il nome in italiano (es. "Forchetta").
    pub name: String,
    /// Quanti puzzle hanno questo tema.
    pub count: u32,
}

/// Il tema dei puzzle nati dagli errori gravi delle proprie partite.
pub const OWN_GAME_THEME: &str = "ownGame";

/// I temi riconosciuti, con il nome italiano.
///
/// Mancano apposta le etichette di Lichess che non sono temi di gioco: la
/// lunghezza ("short", "long", "veryLong", "oneMove"), il vantaggio finale
/// ("crushing", "advantage", "equality") e la provenienza ("master",
/// "masterVsMaster", "superGM").
const THEME_NAMES: [(&str, &str); 64] = [
    ("opening", "Apertura"),
    ("middlegame", "Mediogioco"),
    ("endgame", "Finale"),
    ("mate", "Scacco matto"),
    ("mateIn1", "Matto in 1"),
    ("mateIn2", "Matto in 2"),
    ("mateIn3", "Matto in 3"),
    ("mateIn4", "Matto in 4"),
    ("mateIn5", "Matto in 5 o più"),
    ("fork", "Forchetta"),
    ("pin", "Inchiodatura"),
    ("skewer", "Infilata"),
    ("discoveredAttack", "Attacco di scoperta"),
    ("discoveredCheck", "Scacco di scoperta"),
    ("doubleCheck", "Scacco doppio"),
    ("xRayAttack", "Attacco ai raggi X"),
    ("hangingPiece", "Pezzo in presa"),
    ("trappedPiece", "Pezzo intrappolato"),
    ("sacrifice", "Sacrificio"),
    ("attraction", "Attrazione"),
    ("deflection", "Deviazione"),
    ("clearance", "Liberazione di casa o linea"),
    ("interference", "Interferenza"),
    ("intermezzo", "Mossa intermedia"),
    ("capturingDefender", "Eliminazione del difensore"),
    ("quietMove", "Mossa tranquilla"),
    ("defensiveMove", "Mossa difensiva"),
    ("zugzwang", "Zugzwang"),
    ("collinearMove", "Mossa sulla stessa linea"),
    ("advancedPawn", "Pedone avanzato"),
    ("promotion", "Promozione"),
    ("underPromotion", "Sottopromozione"),
    ("enPassant", "Presa en passant"),
    ("castling", "Arrocco"),
    ("kingsideAttack", "Attacco sull'ala di re"),
    ("queensideAttack", "Attacco sull'ala di donna"),
    ("exposedKing", "Re esposto"),
    ("attackingF2F7", "Attacco a f2 / f7"),
    ("backRankMate", "Matto sulla traversa"),
    ("smotheredMate", "Matto affogato"),
    ("anastasiaMate", "Matto di Anastasia"),
    ("arabianMate", "Matto arabo"),
    ("bodenMate", "Matto di Boden"),
    ("doubleBishopMate", "Matto dei due alfieri"),
    ("dovetailMate", "Matto di Cozio (a coda di rondine)"),
    ("epauletteMate", "Matto a spalline"),
    ("hookMate", "Matto ad uncino"),
    ("operaMate", "Matto dell'opera"),
    ("pillsburysMate", "Matto di Pillsbury"),
    ("morphysMate", "Matto di Morphy"),
    ("cornerMate", "Matto nell'angolo"),
    ("killBoxMate", "Matto \"kill box\""),
    ("swallowstailMate", "Matto di Guéridon"),
    ("triangleMate", "Matto a triangolo"),
    ("vukovicMate", "Matto di Vuković"),
    ("blindSwineMate", "Matto dei \"maiali ciechi\""),
    ("balestraMate", "Matto della balestra"),
    ("pawnEndgame", "Finale di pedoni"),
    ("rookEndgame", "Finale di torri"),
    ("bishopEndgame", "Finale di alfieri"),
    ("knightEndgame", "Finale di cavalli"),
    ("queenEndgame", "Finale di donne"),
    ("queenRookEndgame", "Finale di donna e torre"),
    // I puzzle nati dagli errori gravi delle proprie partite (Fase 8).
    (OWN_GAME_THEME, "Dalle tue partite"),
];

/// Il primo aiuto di un puzzle: il tema detto a parole, senza svelare la mossa.
/// I temi più precisi vengono prima: un puzzle con "deflection" e "mateIn2"
/// riceve l'aiuto della deviazione.
const HINTS: &[(&str, &str)] = &[
    (
        "smotheredMate",
        "Il re avversario è chiuso dai suoi pezzi: un cavallo può dargli matto.",
    ),
    (
        "backRankMate",
        "Guarda l'ultima traversa: il re avversario non ha case di fuga.",
    ),
    (
        "doubleCheck",
        "C'è uno scacco doppio: due pezzi possono dare scacco insieme.",
    ),
    (
        "discoveredCheck",
        "Spostando un tuo pezzo puoi scoprire uno scacco di un altro.",
    ),
    (
        "discoveredAttack",
        "Spostando un tuo pezzo puoi scoprire l'attacco di un altro.",
    ),
    (
        "attraction",
        "Un sacrificio può attirare il re o la donna su una casa sfortunata.",
    ),
    (
        "deflection",
        "Un pezzo avversario difende qualcosa di importante: costringilo a spostarsi.",
    ),
    (
        "capturingDefender",
        "Cattura il pezzo che difende, poi prendi ciò che difendeva.",
    ),
    (
        "intermezzo",
        "Prima di riprendere, cerca una mossa più forte: uno scacco o una minaccia.",
    ),
    (
        "clearance",
        "Un tuo pezzo occupa una casa o una linea che serve a un altro: liberala.",
    ),
    (
        "interference",
        "Metti un pezzo in mezzo a una linea con cui l'avversario si difende.",
    ),
    (
        "xRayAttack",
        "Guarda attraverso i pezzi: una linea continua anche dietro un pezzo.",
    ),
    (
        "skewer",
        "Il re o la donna avversari sono sulla stessa linea di un altro pezzo.",
    ),
    ("pin", "Un pezzo avversario è inchiodato: attaccalo ancora."),
    ("fork", "Cerca una mossa che attacchi due pezzi insieme."),
    (
        "trappedPiece",
        "Un pezzo avversario non ha case dove scappare.",
    ),
    ("hangingPiece", "C'è un pezzo avversario senza difesa."),
    (
        "underPromotion",
        "Il pedone promuove, ma forse non a donna.",
    ),
    (
        "promotion",
        "Un pedone può arrivare a promozione: aprigli la strada.",
    ),
    ("sacrifice", "La soluzione comincia con un sacrificio."),
    ("mateIn1", "C'è un matto in una mossa."),
    (
        "mateIn2",
        "C'è un matto in due mosse: comincia con una mossa forzante.",
    ),
    (
        "mate",
        "C'è un matto: cerca le mosse forzanti, prima di tutto gli scacchi.",
    ),
    (
        "quietMove",
        "La mossa giusta è tranquilla: non è uno scacco né una cattura.",
    ),
    (
        "defensiveMove",
        "Prima difenditi: c'è una minaccia avversaria da fermare.",
    ),
    (
        "zugzwang",
        "L'avversario è in zugzwang: qualunque mossa faccia, peggiora.",
    ),
    ("exposedKing", "Il re avversario è scoperto: attaccalo."),
    ("advancedPawn", "Un pedone avanzato è la chiave."),
];

/// Il primo aiuto per un puzzle con questi temi, se c'è (vedi [`HINTS`]).
#[must_use]
pub fn theme_hint(themes: &[String]) -> Option<&'static str> {
    HINTS
        .iter()
        .find(|(key, _)| {
            themes
                .iter()
                .any(|theme| theme == key || (*key == "mate" && theme.starts_with("mateIn")))
        })
        .map(|(_, hint)| *hint)
}

/// Il nome italiano di un tema, se è un tema riconosciuto.
#[must_use]
pub fn theme_name(key: &str) -> Option<&'static str> {
    THEME_NAMES
        .iter()
        .find(|(own, _)| *own == key)
        .map(|(_, name)| *name)
}

/// I nomi italiani dei temi di un puzzle (le etichette che non sono temi si saltano).
#[must_use]
pub fn theme_names(themes: &[String]) -> Vec<String> {
    themes
        .iter()
        .filter_map(|key| theme_name(key))
        .map(str::to_owned)
        .collect()
}

/// I temi che hanno almeno un puzzle, in ordine alfabetico del nome italiano.
#[must_use]
pub fn available_themes() -> Vec<ThemeInfo> {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for puzzle in all_puzzles() {
        for theme in &puzzle.themes {
            *counts.entry(theme.as_str()).or_default() += 1;
        }
    }
    let mut themes: Vec<ThemeInfo> = THEME_NAMES
        .iter()
        .filter_map(|(key, name)| {
            counts.get(key).map(|count| ThemeInfo {
                key: (*key).to_owned(),
                name: (*name).to_owned(),
                count: *count,
            })
        })
        .collect();
    themes.sort_by(|a, b| a.name.cmp(&b.name));
    themes
}

#[cfg(test)]
mod tests {
    use super::{available_themes, theme_name, theme_names};

    #[test]
    fn translates_the_themes() {
        assert_eq!(theme_name("fork"), Some("Forchetta"));
        assert_eq!(theme_name("knightEndgame"), Some("Finale di cavalli"));
        assert_eq!(theme_name("crushing"), None);
    }

    #[test]
    fn skips_labels_that_are_not_themes() {
        let themes = [
            "fork".to_owned(),
            "short".to_owned(),
            "middlegame".to_owned(),
        ];

        assert_eq!(theme_names(&themes), ["Forchetta", "Mediogioco"]);
    }

    #[test]
    fn lists_the_themes_with_their_puzzles_in_alphabetical_order() {
        let themes = available_themes();

        let fork = themes
            .iter()
            .find(|theme| theme.key == "fork")
            .expect("forchette presenti");
        assert!(fork.count > 1_000);
        assert!(themes.windows(2).all(|pair| pair[0].name <= pair[1].name));
    }
}
