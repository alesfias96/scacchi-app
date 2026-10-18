# src/bindings — Tipi generati automaticamente da Rust

**Non modificare questi file a mano.**

Qui ci sono le "forme" dei dati che arrivano dal nucleo Rust, tradotte in TypeScript.
Vengono scritte automaticamente dallo strumento `ts-rs` ogni volta che si esegue
`npm run test:rust` (o `npm run check`). Così Rust e interfaccia usano sempre
esattamente gli stessi tipi: se un tipo cambia in Rust, cambia anche qui e
TypeScript segnala ogni punto dell'interfaccia da aggiornare.

Il controllo automatico su GitHub verifica che questi file siano aggiornati:
dopo aver modificato un tipo Rust, esegui `npm run check` e salva i file rigenerati.

## Contenuto

| File                | Origine in Rust                            | Cosa descrive                                                                               |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------- |
| `Side.ts`           | `crates/chess-core/src/side.rs`            | Il colore di un giocatore: `"white"` o `"black"`                                            |
| `LegalMove.ts`      | `crates/chess-core/src/legal_move.rs`      | Una mossa legale: casa di partenza, di arrivo e notazione UCI                               |
| `PositionView.ts`   | `crates/chess-core/src/position_view.rs`   | Una posizione completa: FEN, turno, scacco, mosse legali                                    |
| `Settings.ts`       | `crates/app-settings/src/settings.rs`      | Le impostazioni dell'utente: tema, scacchiera, pezzi, notazione, suoni                      |
| `AppTheme.ts`       | `crates/app-settings/src/app_theme.rs`     | I temi dell'app                                                                             |
| `BoardStyle.ts`     | `crates/app-settings/src/board_style.rs`   | Gli stili della scacchiera                                                                  |
| `PieceStyle.ts`     | `crates/app-settings/src/piece_style.rs`   | Gli stili dei pezzi                                                                         |
| `Notation.ts`       | `crates/app-settings/src/notation.rs`      | Le notazioni delle mosse (figurine, italiana, internazionale)                               |
| `SoundSetting.ts`   | `crates/app-settings/src/sound_setting.rs` | I suoni del gioco: `"on"` (attivi) o `"off"`                                                |
| `GameView.ts`       | `crates/chess-core/src/game_view.rs`       | La partita in corso: posizione mostrata, mosse, risultato                                   |
| `MoveRecord.ts`     | `crates/chess-core/src/move_record.rs`     | Una mossa già giocata, pronta da mostrare nella lista                                       |
| `GameResult.ts`     | `crates/chess-core/src/game_result.rs`     | Il risultato di una partita finita (vincitore, motivo, punteggio)                           |
| `GameEndReason.ts`  | `crates/chess-core/src/game_result.rs`     | Il motivo della fine: matto, stallo, materiale insufficiente, ripetizione, 50 mosse         |
| `LineView.ts`       | `crates/chess-core/src/line_view.rs`       | Una sequenza di mosse in notazione SAN (es. la variante del motore), con il numero di mossa |
| `Evaluation.ts`     | `crates/engine/src/evaluation.rs`          | La valutazione per il Bianco: centipedoni oppure matto in N                                 |
| `AnalysisUpdate.ts` | `crates/engine/src/analysis_update.rs`     | Un aggiornamento dell'analisi: profondità, valutazione, variante                            |
| `LevelInfo.ts`      | `crates/engine/src/strength.rs`            | Un livello del computer: numero, nome, Elo indicativo                                       |
| `SessionView.ts`    | `crates/game-session/src/session_view.rs`  | La partita in corso con avversario e orologio, e cosa si può fare adesso                    |
| `ComputerView.ts`   | `crates/game-session/src/computer_view.rs` | Il computer avversario: colore e livello                                                    |
| `ClockView.ts`      | `crates/game-session/src/clock_view.rs`    | I tempi dell'orologio e chi sta consumando il tempo                                         |
| `GameSetup.ts`      | `crates/app-settings/src/game_setup.rs`    | La configurazione di una nuova partita (avversario, colore, livello, tempo)                 |
| `OpponentKind.ts`   | `crates/app-settings/src/opponent_kind.rs` | L'avversario: computer o due giocatori                                                      |
| `ColorChoice.ts`    | `crates/app-settings/src/color_choice.rs`  | Il colore contro il computer: bianco, nero o a sorte                                        |
| `TimeControl.ts`    | `crates/app-settings/src/time_control.rs`  | Il tempo di riflessione: secondi iniziali e incremento                                      |
