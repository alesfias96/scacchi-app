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

| File                     | Origine in Rust                              | Cosa descrive                                                                               |
| ------------------------ | -------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `Side.ts`                | `crates/chess-core/src/side.rs`              | Il colore di un giocatore: `"white"` o `"black"`                                            |
| `LegalMove.ts`           | `crates/chess-core/src/legal_move.rs`        | Una mossa legale: casa di partenza, di arrivo e notazione UCI                               |
| `PositionView.ts`        | `crates/chess-core/src/position_view.rs`     | Una posizione completa: FEN, turno, scacco, mosse legali                                    |
| `Settings.ts`            | `crates/app-settings/src/settings.rs`        | Le impostazioni dell'utente: tema, scacchiera, pezzi, notazione, suoni                      |
| `AppTheme.ts`            | `crates/app-settings/src/app_theme.rs`       | I temi dell'app                                                                             |
| `BoardStyle.ts`          | `crates/app-settings/src/board_style.rs`     | Gli stili della scacchiera                                                                  |
| `PieceStyle.ts`          | `crates/app-settings/src/piece_style.rs`     | Gli stili dei pezzi                                                                         |
| `Notation.ts`            | `crates/app-settings/src/notation.rs`        | Le notazioni delle mosse (figurine, italiana, internazionale)                               |
| `SoundSetting.ts`        | `crates/app-settings/src/sound_setting.rs`   | I suoni del gioco: `"on"` (attivi) o `"off"`                                                |
| `GameView.ts`            | `crates/chess-core/src/game_view.rs`         | La partita in corso: posizione mostrata, mosse, risultato                                   |
| `MoveRecord.ts`          | `crates/chess-core/src/move_record.rs`       | Una mossa già giocata, pronta da mostrare nella lista                                       |
| `GameResult.ts`          | `crates/chess-core/src/game_result.rs`       | Il risultato di una partita finita (vincitore, motivo, punteggio)                           |
| `GameEndReason.ts`       | `crates/chess-core/src/game_result.rs`       | Il motivo della fine: matto, stallo, materiale insufficiente, ripetizione, 50 mosse         |
| `LineView.ts`            | `crates/chess-core/src/line_view.rs`         | Una sequenza di mosse in notazione SAN (es. la variante del motore), con il numero di mossa |
| `Evaluation.ts`          | `crates/engine/src/evaluation.rs`            | La valutazione per il Bianco: centipedoni oppure matto in N                                 |
| `AnalysisUpdate.ts`      | `crates/engine/src/analysis_update.rs`       | Un aggiornamento dell'analisi: profondità, valutazione, variante                            |
| `LevelInfo.ts`           | `crates/engine/src/strength.rs`              | Un livello del computer: numero, nome, Elo indicativo                                       |
| `SessionView.ts`         | `crates/game-session/src/session_view.rs`    | La partita in corso con avversario e orologio, e cosa si può fare adesso                    |
| `ComputerView.ts`        | `crates/game-session/src/computer_view.rs`   | Il computer avversario: colore e livello                                                    |
| `ClockView.ts`           | `crates/game-session/src/clock_view.rs`      | I tempi dell'orologio e chi sta consumando il tempo                                         |
| `GameSetup.ts`           | `crates/app-settings/src/game_setup.rs`      | La configurazione di una nuova partita (avversario, colore, motore, livelli, tempo)         |
| `OpponentKind.ts`        | `crates/app-settings/src/opponent_kind.rs`   | L'avversario: computer (Stockfish o Maia) o due giocatori                                   |
| `ComputerEngine.ts`      | `crates/app-settings/src/computer_engine.rs` | Il motore del computer: `"stockfish"` o `"maia"`                                            |
| `ColorChoice.ts`         | `crates/app-settings/src/color_choice.rs`    | Il colore contro il computer: bianco, nero o a sorte                                        |
| `TimeControl.ts`         | `crates/app-settings/src/time_control.rs`    | Il tempo di riflessione: secondi iniziali e incremento                                      |
| `Opening.ts`             | `crates/openings/src/opening.rs`             | Un'apertura: codice ECO e nome                                                              |
| `GameSummary.ts`         | `crates/storage/src/game_summary.rs`         | Il riassunto di una partita salvata, per l'archivio                                         |
| `GameFilter.ts`          | `crates/storage/src/game_filter.rs`          | I filtri dell'archivio                                                                      |
| `OutcomeFilter.ts`       | `crates/storage/src/game_filter.rs`          | Il filtro per esito: vinte, perse, patte, in corso                                          |
| `PositionReport.ts`      | `crates/analysis/src/report.rs`              | Il rapporto dei rilevatori: fase, materiale, fatti                                          |
| `Fact.ts`                | `crates/analysis/src/fact.rs`                | Un fatto trovato dai rilevatori, con case e frecce                                          |
| `FactArrow.ts`           | `crates/analysis/src/fact.rs`                | Una freccia da disegnare sulla scacchiera                                                   |
| `FactCategory.ts`        | `crates/analysis/src/fact.rs`                | Il gruppo di un fatto (minacce, tattiche, pedoni, re, pezzi, materiale)                     |
| `GamePhase.ts`           | `crates/analysis/src/game_phase.rs`          | La fase di gioco: apertura, mediogioco, finale                                              |
| `MaterialBalance.ts`     | `crates/analysis/src/material.rs`            | Il materiale dei due giocatori e chi è avanti                                               |
| `MaterialCount.ts`       | `crates/analysis/src/material.rs`            | I pezzi di un giocatore                                                                     |
| `TrainingSetup.ts`       | `crates/app-settings/src/training_setup.rs`  | Le scelte dell'allenamento: modalità, mosse da trovare, tema, difficoltà, secondi           |
| `TrainingMode.ts`        | `crates/app-settings/src/training_mode.rs`   | Le modalità di allenamento: calcolo, visualizzazione, "cosa minaccia?", ripasso             |
| `Difficulty.ts`          | `crates/app-settings/src/difficulty.rs`      | La difficoltà dei puzzle: più facile, normale, più difficile                                |
| `AttemptView.ts`         | `crates/training/src/attempt_view.rs`        | Il puzzle in corso: posizione, mosse trovate, messaggio, aiuto, rating                      |
| `Feedback.ts`            | `crates/training/src/attempt_view.rs`        | Com'è andata l'ultima azione: pronto, giusto, sbagliato, risolto, soluzione mostrata        |
| `Reveal.ts`              | `crates/training/src/attempt_view.rs`        | Ciò che si scopre a puzzle finito: difficoltà, temi, soluzione, perché funziona             |
| `ThemeInfo.ts`           | `crates/training/src/theme.rs`               | Un tema dei puzzle: nome di Lichess, nome italiano, quanti puzzle                           |
| `TrainingOverview.ts`    | `crates/training/src/training_overview.rs`   | Il riepilogo dell'allenamento: rating, puzzle risolti, ripassi di oggi, temi                |
| `TutorExplanation.ts`    | `crates/tutor/src/explanation_view.rs`       | La spiegazione di una posizione: parti, schede, posizione con le mosse legali               |
| `TutorSection.ts`        | `crates/tutor/src/explanation_view.rs`       | Una parte della spiegazione, con il titolo e le frasi                                       |
| `TutorItem.ts`           | `crates/tutor/src/explanation_view.rs`       | Una frase del tutor (con case da evidenziare) e la scheda che la approfondisce              |
| `CardView.ts`            | `crates/tutor/src/explanation_view.rs`       | Una scheda della base di conoscenza adattata alla posizione                                 |
| `PlanView.ts`            | `crates/tutor/src/explanation_view.rs`       | Un piano di una scheda, con chi deve seguirlo                                               |
| `Topic.ts`               | `crates/tutor/src/topic.rs`                  | L'argomento di una scheda: apertura, struttura, mediogioco, finale, principio               |
| `WhatIfView.ts`          | `crates/tutor/src/what_if.rs`                | Una mossa provata ("e se…"): giudizio, valutazioni, risposta migliore, cosa cambia          |
| `GameReview.ts`          | `crates/review/src/review_view.rs`           | La revisione di una partita: mosse giudicate, grafico, riepiloghi, momenti critici          |
| `MoveReview.ts`          | `crates/review/src/review_view.rs`           | Una mossa con il suo giudizio, la valutazione e la mossa migliore                           |
| `MoveQuality.ts`         | `crates/review/src/move_quality.rs`          | Il giudizio di una mossa: ottima, buona, imprecisione, errore, errore grave                 |
| `PlayerSummary.ts`       | `crates/review/src/review_view.rs`           | Il riepilogo di un giocatore: precisione, errori, fase peggiore                             |
| `CriticalMoment.ts`      | `crates/review/src/review_view.rs`           | Un momento critico spiegato                                                                 |
| `ReviewProgress.ts`      | `crates/review/src/review_progress.rs`       | L'avanzamento dell'analisi in corso                                                         |
| `ReviewFinished.ts`      | `crates/review/src/review_progress.rs`       | La fine dell'analisi (riuscita, fermata o fallita)                                          |
| `ReviewQueueProgress.ts` | `crates/review/src/review_progress.rs`       | A che punto è la coda «Analizza le partite mancanti»                                        |
| `StatsFilter.ts`         | `crates/app-settings/src/stats_filter.rs`    | I filtri di «Progressi»: periodo e partite                                                  |
| `StatsPeriod.ts`         | `crates/app-settings/src/stats_filter.rs`    | Il periodo: 30 giorni, 90 giorni, sempre                                                    |
| `StatsOpponent.ts`       | `crates/app-settings/src/stats_filter.rs`    | Le partite: tutte, contro il computer, a due giocatori                                      |
| `UpdateCheck.ts`         | `crates/app-settings/src/update_check.rs`    | Quando controllare gli aggiornamenti: `"manual"` o `"onStart"`                              |
| `IdeaView.ts`            | `crates/tutor/src/idea_view.rs`              | L'idea di una variante: frase principale, momenti chiave, mosse, posizioni, schede          |
| `IdeaMoment.ts`          | `crates/tutor/src/idea_view.rs`              | Un momento chiave di un'idea, con case e frecce                                             |
| `IdeaFrame.ts`           | `crates/tutor/src/idea_view.rs`              | Una posizione della variante, da mostrare sulla scacchiera                                  |
| `ImbalanceItem.ts`       | `crates/tutor/src/strategy_view.rs`          | Una voce del bilancio «Chi ha cosa»                                                         |
| `StrategyPlanView.ts`    | `crates/tutor/src/strategy_view.rs`          | Un piano: perché, come, quando sì, quando no, segnali                                       |
| `StrategyView.ts`        | `crates/tutor/src/strategy_view.rs`          | La strategia della posizione: centro, bilancio, piani                                       |
| `PositionIdeas.ts`       | `crates/tutor/src/idea_view.rs`              | Le idee di una posizione (le migliori varianti) e il confronto                              |
| `MoveIdea.ts`            | `crates/tutor/src/idea_view.rs`              | L'idea della mossa giocata e, se c'era, quella della migliore                               |
| `Note.ts`                | `crates/notes/src/note.rs`                   | Un appunto salvato: titolo, testo, etichette, collegamento, date                            |
| `NoteInput.ts`           | `crates/notes/src/note.rs`                   | Un appunto come lo manda l'interfaccia quando lo si scrive                                  |
| `NoteLink.ts`            | `crates/notes/src/note.rs`                   | La posizione (e la partita, la mossa) collegata a un appunto                                |
| `NoteQuery.ts`           | `crates/notes/src/search.rs`                 | La ricerca tra gli appunti: parole ed etichetta                                             |
| `StatsReport.ts`         | `crates/stats/src/stats_view.rs`             | Tutte le statistiche di «Progressi», con i conteggi delle partite                           |
| `LevelStats.ts`          | `crates/stats/src/stats_view.rs`             | La scheda «Livello»: Elo stimato, rating dei puzzle, precisione                             |
| `GameStats.ts`           | `crates/stats/src/stats_view.rs`             | La scheda «Partite»: risultati, fasi, poco tempo                                            |
| `OpeningStats.ts`        | `crates/stats/src/stats_view.rs`             | Un'apertura giocata, con colore, risultati e precisione                                     |
| `TacticsStats.ts`        | `crates/stats/src/stats_view.rs`             | La scheda «Calcolo e tattica»                                                               |
| `ChartPoint.ts`          | `crates/stats/src/stats_view.rs`             | Un punto di un grafico nel tempo                                                            |
| `Score.ts`               | `crates/stats/src/stats_view.rs`             | Vinte, patte e perse                                                                        |
| `ColorScore.ts`          | `crates/stats/src/stats_view.rs`             | I risultati con un colore                                                                   |
| `LevelScore.ts`          | `crates/stats/src/stats_view.rs`             | I risultati contro un livello del computer                                                  |
| `PhaseStats.ts`          | `crates/stats/src/stats_view.rs`             | Precisione ed errori in una fase della partita                                              |
| `TimePressure.ts`        | `crates/stats/src/stats_view.rs`             | Gli errori con poco tempo e con più tempo                                                   |
| `DepthStats.ts`          | `crates/stats/src/stats_view.rs`             | I puzzle con un certo numero di mosse da trovare                                            |
| `ThemeStats.ts`          | `crates/stats/src/stats_view.rs`             | I puzzle di un tema                                                                         |
| `MoveTree.ts`            | `crates/chess-core/src/move_tree.rs`         | L'albero delle mosse: linea principale e varianti, e la mossa mostrata                      |
| `MoveNode.ts`            | `crates/chess-core/src/move_tree.rs`         | Una mossa dell'albero, con il suo seguito                                                   |
