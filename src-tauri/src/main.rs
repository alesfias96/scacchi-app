//! Punto di avvio dell'eseguibile.
//!
//! Ha un solo compito: chiamare `run()` della libreria dell'app.
//! Tutto il resto vive in `lib.rs` e nei moduli che verranno aggiunti.

// Su Windows, nella versione finale (release), evita che si apra una
// finestra del terminale nera accanto all'app. In sviluppo il terminale
// resta visibile per leggere i messaggi di log.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    scacchi_app_lib::run();
}
