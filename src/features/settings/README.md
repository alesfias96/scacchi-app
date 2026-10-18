# src/features/settings — La sezione "Impostazioni"

Qui si scelgono il **tema dell'app**, lo **stile della scacchiera**, lo **stile dei pezzi**,
la **notazione delle mosse** e i **suoni** del gioco (riaccendendoli si sente subito una mossa d'esempio).
Ogni scelta si applica subito in tutte le sezioni dell'app, viene salvata sul computer
e ritrovata alla riapertura. Accanto alle scelte, una scacchiera di anteprima mostra il risultato.

## Contenuto

| Elemento                  | Cosa fa                                                                         |
| ------------------------- | ------------------------------------------------------------------------------- |
| `SettingsPage.tsx`        | La pagina: i cinque gruppi di scelte e l'anteprima                              |
| `SettingsPage.module.css` | La disposizione della pagina                                                    |
| `SettingsPage.test.tsx`   | I test: scelte attuali, nuove scelte applicate e salvate, errore di salvataggio |
| `options/`                | Gli elenchi delle scelte con testi e anteprime                                  |
| `previews/`               | I disegni in miniatura mostrati in ogni scelta                                  |
