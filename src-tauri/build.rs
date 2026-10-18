//! Script di preparazione della compilazione.
//!
//! Cargo lo esegue automaticamente prima di compilare l'app.
//! `tauri_build::build()` legge `tauri.conf.json`, verifica le icone,
//! genera i permessi (cartella `capabilities/`) e, su Windows,
//! incorpora l'icona e le informazioni nell'eseguibile.

fn main() {
    tauri_build::build();
}
