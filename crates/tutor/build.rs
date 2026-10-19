//! Raccoglie le schede della base di conoscenza durante la compilazione.
//!
//! Scorre la cartella `knowledge/` e scrive un piccolo file Rust con l'elenco
//! di tutte le schede (`.toml`), ognuna inclusa nell'app con `include_str!`.
//! Così per aggiungere una scheda basta creare il file: non serve toccare il
//! codice. Se la cartella cambia, Cargo ricompila da solo.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// Le schede `.toml` della cartella e delle sue sottocartelle, in ordine alfabetico.
fn collect(folder: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, found);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "toml")
        {
            found.push(path);
        }
    }
}

fn main() {
    let manifest =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("cartella della crate"));
    let knowledge = manifest.join("../../knowledge");
    println!("cargo:rerun-if-changed={}", knowledge.display());

    let mut files = Vec::new();
    collect(&knowledge, &mut files);
    files.sort();

    let mut code = String::from(
        "/// Le schede della base di conoscenza: (nome, contenuto del file).\n\
         pub(crate) const KNOWLEDGE_FILES: &[(&str, &str)] = &[\n",
    );
    for path in &files {
        let relative = path
            .strip_prefix(&knowledge)
            .expect("scheda dentro knowledge/")
            .with_extension("");
        let id = relative.to_string_lossy().replace('\\', "/");
        let absolute = fs::canonicalize(path).expect("percorso della scheda");
        let _ = writeln!(
            code,
            "    ({id:?}, include_str!({:?})),",
            absolute.display().to_string()
        );
    }
    code.push_str("];\n");

    let output = PathBuf::from(std::env::var("OUT_DIR").expect("cartella di uscita"));
    fs::write(output.join("knowledge_files.rs"), code).expect("elenco delle schede scritto");
}
