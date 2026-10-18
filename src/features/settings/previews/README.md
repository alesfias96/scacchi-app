# src/features/settings/previews — Le anteprime delle scelte

I piccoli disegni mostrati in ogni riquadro delle Impostazioni, per vedere una scelta
prima di farla.

## Contenuto

| File                         | Cosa fa                                                                                               |
| ---------------------------- | ----------------------------------------------------------------------------------------------------- |
| `ThemePreview.tsx`           | Una finestra in miniatura con i colori e il font del tema (con "Automatico": metà chiara, metà scura) |
| `ThemePreview.module.css`    | I suoi stili: prendono i colori direttamente dal tema mostrato                                        |
| `BoardPreview.tsx`           | La scacchiera in miniatura con la sua cornice                                                         |
| `BoardPreview.module.css`    | I suoi stili                                                                                          |
| `PiecePreview.tsx`           | Tre pezzi dello stile (re, donna, cavallo) su case alternate                                          |
| `PiecePreview.module.css`    | I suoi stili                                                                                          |
| `NotationPreview.tsx`        | Due mosse d'esempio scritte nella notazione (♞f3 / Cf3 / Nf3)                                         |
| `NotationPreview.module.css` | I suoi stili                                                                                          |
