# crates/notes — Gli appunti personali

Le regole degli appunti: uno spazio per scrivere e ritrovare idee, pensieri e note di studio.
Ogni appunto ha un **titolo** (può mancare), un **testo**, delle **etichette** libere (es.
"Siciliana", "Finali") e, se nasce da una posizione, il **collegamento** a quella posizione (FEN) e,
se c'è, alla partita dell'archivio e alla mossa.

Questa crate non legge il database (lo fa `storage`) e non sa nulla dell'interfaccia.

## Cosa fa

- **Prepara l'appunto da salvare**: toglie gli spazi in più dal titolo, ripulisce le etichette
  (niente vuote, niente doppioni anche se scritti con maiuscole o accenti diversi, al massimo 12 da
  40 caratteri), dà un nome unico agli appunti nuovi e tiene la data di creazione.
- **Cerca**: un appunto corrisponde se **ogni parola** cercata compare nel titolo, nel testo o
  nelle etichette, senza badare a maiuscole e accenti ("perche" trova "Perché"). Il filtro per
  etichetta tiene solo gli appunti con quell'etichetta. I risultati vanno dal più recente.
- **Elenca le etichette** usate, una volta sola, in ordine alfabetico.

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |

## L'opzione `bindings`

Con l'opzione `bindings` (attivata da `src-tauri`) i test generano i tipi TypeScript in
`src/bindings/`: `Note`, `NoteInput`, `NoteLink`, `NoteQuery`.

## Test

`npm run test:rust` esegue anche i test di questa crate: testo senza accenti, etichette ripulite,
date di creazione e modifica, ricerca con più parole, filtro per etichetta, elenco delle etichette.
