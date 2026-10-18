# src/features/play/clock-face — Il quadrante dell'orologio

Il tempo rimasto di un giocatore: `4:07`, oppure `1:05:09` sopra l'ora, e con i decimi negli
ultimi 10 secondi (`9.4`). L'orologio di chi deve muovere è pieno nel colore d'accento e scorre;
negli ultimi 10 secondi diventa rosso.

Il conto vero del tempo lo fa Rust (che decide anche quando il tempo è scaduto); qui i secondi
scorrono sullo schermo partendo dall'ultimo valore ricevuto (`src/hooks/useCountdown.ts`).

| File                   | Cosa fa       |
| ---------------------- | ------------- |
| `ClockFace.tsx`        | Il componente |
| `ClockFace.module.css` | I suoi stili  |
| `ClockFace.test.tsx`   | I suoi test   |
