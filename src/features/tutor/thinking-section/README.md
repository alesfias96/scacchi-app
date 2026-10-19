# src/features/tutor/thinking-section — Prima di muovere

Il metodo di pensiero da usare prima di ogni mossa, riempito con la posizione: **i tuoi scacchi**,
**le tue catture**, **cosa minaccia l'avversario**, **il controllo finale**. Per i primi due passi
compaiono le mosse (nella notazione scelta), per il terzo le minacce trovate dai rilevatori. Sotto
c'è la scheda del metodo (`knowledge/tactics/thinking-method.toml`). Lo prepara Rust (crate `tutor`,
file `thinking.rs`).

| File                         | Cosa fa       |
| ---------------------------- | ------------- |
| `ThinkingSection.tsx`        | Il componente |
| `ThinkingSection.module.css` | I suoi stili  |
| `ThinkingSection.test.tsx`   | I test        |
