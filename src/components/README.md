# src/components — Componenti riutilizzabili

Pezzi di interfaccia generici, usati da più sezioni dell'app. Non sanno in quale sezione
si trovano: ricevono i dati e li mostrano.

Ogni componente ha la sua cartella con il codice (`.tsx`), gli stili (`.module.css`),
i test (`.test.tsx`) e un README.

## Contenuto

| Cartella        | Cosa contiene                                                                                         |
| --------------- | ----------------------------------------------------------------------------------------------------- |
| `chessboard/`   | La scacchiera grafica, usata in tutte le sezioni; rispetta sempre gli stili scelti nelle Impostazioni |
| `option-group/` | Un gruppo di scelte a riquadri con anteprima (usato nelle Impostazioni)                               |
| `piece-icon/`   | Un pezzo disegnato fuori dalla scacchiera, nello stile scelto (es. nella scelta della promozione)     |
| `panel-card/`   | La scheda con titolo usata per tutte le informazioni accanto alla scacchiera                          |
| `button/`       | Il pulsante dell'app, in tre aspetti: principale, secondario, solo icona                              |
| `icon/`         | Le piccole icone a tratto (frecce, copia, incolla, …)                                                 |

## Cartelle che verranno aggiunte

| Cartella    | Fase | Scopo                              |
| ----------- | ---- | ---------------------------------- |
| `eval-bar/` | 3    | La barra di valutazione del motore |
