# src — L'interfaccia

Questa cartella contiene tutto ciò che vedi sullo schermo: finestre, scacchiera, pulsanti, pannelli.
È scritta in **React** con **TypeScript**.

L'interfaccia **non decide nulla**: non conosce le regole degli scacchi e non calcola mosse.
Mostra le informazioni che arrivano dal nucleo Rust e gli invia ciò che fai (clic, mosse, scelte).

## Contenuto

| Elemento      | Cosa fa                                                                                       |
| ------------- | --------------------------------------------------------------------------------------------- |
| `main.tsx`    | Il punto di partenza: carica gli stili globali, avvia React e disegna l'app nella finestra    |
| `app/`        | La struttura dell'app: avvio, navigazione, cornice comune, tema                               |
| `styles/`     | Gli stili globali: token comuni, temi (Studio, Sera, Giorno), azzeramento, base               |
| `assets/`     | Le immagini incluse nell'app: scacchiere e set di pezzi                                       |
| `bindings/`   | I tipi di dati generati automaticamente da Rust — **non si modificano a mano**                |
| `services/`   | Le chiamate al nucleo Rust e l'ascolto dei suoi eventi (mossa del computer, analisi)          |
| `state/`      | Lo stato condiviso tra le sezioni: impostazioni, partita in corso, motore                     |
| `hooks/`      | Hook React condivisi (modalità chiara/scura, posizione iniziale, tastiera, suoni)             |
| `sounds/`     | I suoni del gioco (mossa, presa, fine partita), creati al momento dall'app                    |
| `components/` | Componenti riutilizzabili da più sezioni (scacchiera, barra di valutazione, pulsanti, scelte) |
| `features/`   | Le sezioni dell'app (Gioca, Impostazioni)                                                     |
| `lib/`        | Piccole funzioni di supporto condivise                                                        |
| `test/`       | Impostazioni e strumenti comuni a tutti i test                                                |

## Come si collegano

```
main.tsx → app/App.tsx → SettingsGate (legge le impostazioni) → router
                                                                  ├─ "/"             → features/play/PlayPage
                                                                  └─ "/impostazioni" → features/settings/SettingsPage

state/settingsStore ← services/settingsService ← (Rust)     tema → app/theme-controller
state/gameStore     ← services/gameService     ← (Rust: comandi ed evento game-updated)
state/engineStore   ← services/engineService   ← (Rust: analisi, livelli, errori del motore)
components/chessboard legge da state/ lo stile di scacchiera e pezzi
```
