# crates/app-settings — Le impostazioni dell'utente

Descrive le scelte che valgono in tutta l'app e sa salvarle e rileggerle da un file:

- **tema dell'app**: Studio (predefinito), Sera, Giorno, Automatico;
- **scacchiera**: Rovere (predefinita), Noce, Acero, Marmo, Ardesia, Torneo;
- **pezzi**: Icona (predefinito), Morbido, Legno, Metallo, Gettoni, Classico;
- **notazione delle mosse**: Figurine (predefinita), Italiana, Internazionale;
- **suoni** di mosse, prese e fine partita: Attivi (predefinito), Disattivati.

Ricorda anche l'**ultima configurazione di partita** scelta (avversario, colore, livello del
computer, tempo), così la finestra "Nuova partita" la ripropone, e le **ultime scelte
dell'allenamento** (modalità, mosse da trovare, tema, difficoltà, secondi della visualizzazione).

Le impostazioni vengono salvate in un piccolo file `settings.json`, nella cartella di
configurazione dell'app (su Windows in `AppData\Roaming`). Il percorso lo decide il ponte
`src-tauri`: questa crate riceve solo il percorso, quindi funziona uguale su ogni sistema.

Un file mancante (primo avvio) o rovinato non blocca mai l'app: vengono usate le impostazioni predefinite.

## Contenuto

| Elemento     | Cosa contiene                                                         |
| ------------ | --------------------------------------------------------------------- |
| `Cargo.toml` | Descrizione della crate e librerie usate                              |
| `src/`       | Il codice, un file per responsabilità (vedi il README della cartella) |

## L'opzione `bindings`

Con l'opzione `bindings` (attivata da `src-tauri`) i test generano automaticamente i tipi
TypeScript corrispondenti in `src/bindings/` (`AppTheme`, `BoardStyle`, `PieceStyle`, `Notation`, `SoundSetting`, `Settings`, `GameSetup`, `ComputerEngine`,
`TrainingSetup`, `TrainingMode`, `Difficulty`, `StatsFilter`, `StatsPeriod`, `StatsOpponent`, …).

## Test

`npm run test:rust` esegue anche i test di questa crate. Verificano tra l'altro che:

- le impostazioni predefinite siano Studio + Rovere + Icona + Figurine, con i suoni attivi;
- il file JSON sia leggibile e che i valori mancanti prendano quelli predefiniti;
- quanto salvato venga riletto identico;
- un file rovinato venga segnalato come errore, ma non impedisca all'app di partire.
