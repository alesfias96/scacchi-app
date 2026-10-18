# src/features/play/promotion-picker — La scelta della promozione

Quando un pedone arriva sull'ultima traversa, sopra la scacchiera compare una
finestra con quattro pezzi: **donna, torre, alfiere, cavallo**, del colore del
pedone e nello stile di pezzi scelto nelle Impostazioni.

- Clic su un pezzo → la mossa viene giocata con quel pezzo.
- Invio → sceglie la donna (ha già il focus).
- Esc, oppure clic fuori dai pezzi → la mossa viene annullata e il pedone torna al suo posto.

| File                         | Cosa fa       |
| ---------------------------- | ------------- |
| `PromotionPicker.tsx`        | Il componente |
| `PromotionPicker.module.css` | I suoi stili  |
| `PromotionPicker.test.tsx`   | I suoi test   |
