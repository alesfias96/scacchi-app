//! Prova con il vero Stockfish, se è stato scaricato (`npm run engines`).
//!
//! Verifica che il dialogo UCI funzioni con il programma vero, non solo con
//! il finto motore dei test interni. Se Stockfish non c'è, la prova non fa nulla.

use std::path::PathBuf;
use std::time::Duration;

use engine::{SearchLimits, SearchPosition, launch, strength};
use tokio::sync::mpsc;

/// Il percorso di Stockfish per il sistema in uso, se presente in `src-tauri/binaries/`.
fn stockfish_path() -> Option<PathBuf> {
    let binaries = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../src-tauri/binaries");
    std::fs::read_dir(binaries)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            // L'eseguibile: senza estensione (macOS, Linux) o con .exe (Windows).
            let executable = path
                .extension()
                .is_none_or(|extension| extension.eq_ignore_ascii_case("exe"));
            name.starts_with("stockfish-") && executable
        })
}

#[tokio::test]
async fn the_real_engine_analyzes_and_finds_the_mate() {
    let Some(path) = stockfish_path() else {
        eprintln!("Stockfish non scaricato: prova saltata.");
        return;
    };
    let (updates, mut received) = mpsc::unbounded_channel();
    let (handle, work) = launch(&path, &[], updates)
        .await
        .expect("Stockfish si avvia");
    tokio::spawn(work);

    // Il Bianco dà matto in una mossa (Dg7#).
    let position = SearchPosition {
        start_fen: "7k/8/6QK/8/8/8/8/8 w - - 0 1".to_owned(),
        moves: Vec::new(),
    };
    handle.analyze(position.clone());
    let update = tokio::time::timeout(Duration::from_secs(10), received.recv())
        .await
        .expect("l'analisi risponde in tempo")
        .expect("almeno un aggiornamento");
    assert_eq!(update.line.fen, position.start_fen);

    let full = strength(engine::MAX_LEVEL);
    let limits = SearchLimits {
        move_time_ms: Some(200),
        ..SearchLimits::default()
    };
    let found = handle
        .find_move(position, limits, full.options, Duration::ZERO)
        .await
        .expect("Stockfish trova una mossa");
    assert_eq!(found, "g6g7");
}
