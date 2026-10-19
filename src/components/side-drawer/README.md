# components/side-drawer — La barra laterale a comparsa

Una barra che sta nascosta sul bordo destro della finestra e compare quando serve, così la pagina
principale resta libera (in "Gioca": solo la scacchiera). Si apre:

- portando il mouse sul bordo destro della finestra, o sulla linguetta verticale;
- con un clic sulla linguetta;
- con un tasto (in "Gioca": **I**), che la apre e la chiude.

Si richiude da sola poco dopo che il mouse l'ha lasciata, a meno che sia **fissata** (pulsante
"Fissa") o che la si stia usando con la tastiera. **Esc** la chiude. Si sovrappone alla pagina senza
spostarla, e dentro scorre per conto suo.

| File                    | Cosa contiene                                                      |
| ----------------------- | ------------------------------------------------------------------ |
| `SideDrawer.tsx`        | Il componente                                                      |
| `SideDrawer.module.css` | L'aspetto: striscia sul bordo, linguetta, barra che scivola dentro |
| `SideDrawer.test.tsx`   | I test: apertura, chiusura ritardata, "fissa", scorciatoia, Esc    |
