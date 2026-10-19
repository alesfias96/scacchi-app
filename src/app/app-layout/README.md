# src/app/app-layout — La struttura della finestra

L'area di lavoro (la sezione aperta) occupa tutta la finestra e scorre se il contenuto è lungo.
Il **menu principale** (Gioca, Archivio, Impostazioni, …) sta in una barra a comparsa sul bordo
sinistro: si apre portando il mouse sul bordo, con un clic sulla sua linguetta o con il tasto **M**,
si può fissare aperto, Esc la chiude. Si sovrappone alla pagina senza spostarla.

## Contenuto

| File                   | Cosa fa                                                                                |
| ---------------------- | -------------------------------------------------------------------------------------- |
| `AppLayout.tsx`        | Il componente: riceve il menu principale, il tutor e il contenuto della sezione aperta |
| `AppLayout.module.css` | L'area di lavoro a tutta finestra                                                      |
