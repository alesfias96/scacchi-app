# components/side-drawer — La barra laterale a comparsa

Una barra che sta nascosta sul bordo della finestra (sinistro o destro) e compare quando serve, così
la pagina resta libera. La usa il **menu principale** dell'app, sul bordo sinistro. Si apre:

- portando il mouse sul bordo della finestra, o sulla linguetta verticale;
- con un clic sulla linguetta;
- con un tasto (per il menu principale: **M**), che la apre e la chiude.

Si richiude da sola poco dopo che il mouse l'ha lasciata, a meno che sia **fissata** (pulsante
"Fissa") o che la si stia usando con la tastiera. **Esc** la chiude. Si sovrappone alla pagina senza
spostarla, e dentro scorre per conto suo.

| File                    | Cosa contiene                                                      |
| ----------------------- | ------------------------------------------------------------------ |
| `SideDrawer.tsx`        | Il componente                                                      |
| `SideDrawer.module.css` | L'aspetto: striscia sul bordo, linguetta, barra che scivola dentro |
| `SideDrawer.test.tsx`   | I test: apertura, chiusura ritardata, "fissa", scorciatoia, Esc    |

Chi la usa può sapere quando si apre e si chiude (`onOpenChange`): il tutor lavora solo a barra aperta.
