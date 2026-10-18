# src/app — La struttura dell'app

Qui vive tutto ciò che tiene insieme l'app: l'avvio, la navigazione tra le sezioni,
la cornice comune a tutte le pagine e l'applicazione del tema.

## Contenuto

| Elemento            | Cosa fa                                                                                              |
| ------------------- | ---------------------------------------------------------------------------------------------------- |
| `App.tsx`           | Il componente radice: aspetta le impostazioni e avvia la navigazione                                 |
| `App.test.tsx`      | Il suo test: avvio completo con barra laterale e sezione "Gioca"                                     |
| `router.ts`         | L'elenco degli indirizzi dell'app e della pagina che mostra ciascuno (Gioca, Archivio, Impostazioni) |
| `settings-gate/`    | Legge le impostazioni all'avvio e mostra l'app solo quando sono pronte                               |
| `root-layout/`      | La cornice comune a tutte le pagine: tema, barra laterale, pagina aperta                             |
| `theme-controller/` | Applica a tutta l'app il tema scelto                                                                 |
| `app-layout/`       | La griglia della finestra: barra laterale a sinistra, area di lavoro a destra                        |
| `sidebar/`          | La barra laterale con il nome dell'app, le sezioni e le Impostazioni                                 |
