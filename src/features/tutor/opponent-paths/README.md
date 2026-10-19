# src/features/tutor/opponent-paths — Le strade dell'avversario

Cosa potrebbe fare l'avversario **se toccasse a lui**. Il tutor non dice «vuole fare questo e
basta»: mostra le strade possibili, dalla più forte, ognuna spiegata al condizionale («Una strada
possibile: … — si vedrebbe tra 3 mosse») con i suoi motivi tattici e la variante da guardare sulla
scacchiera. In cima, una frase dice se qualcuna è pericolosa. Rust "passa la mano" all'avversario e
chiede a Stockfish le sue varianti migliori (crate `tutor`, file `opponent_paths.rs`).

| File                           | Cosa fa       |
| ------------------------------ | ------------- |
| `OpponentPathsView.tsx`        | Il componente |
| `OpponentPathsView.module.css` | I suoi stili  |
