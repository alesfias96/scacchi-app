# src/features/play/move-list — L'elenco delle mosse

La scheda "Mosse" della sezione Gioca: la partita scritta come nei libri
("1. e4 e5 2. Cf3 …"). Un clic su una mossa mostra la posizione dopo quella
mossa; la mossa della posizione mostrata è evidenziata.

Le mosse arrivano da Rust sempre in notazione internazionale; qui vengono
scritte nella notazione scelta nelle Impostazioni (con `formatSan`, in `src/lib/`):

| Notazione      | Esempio          |
| -------------- | ---------------- |
| Figurine       | ♞f3, ♛xd8+, e8=♛ |
| Italiana       | Cf3, Dxd8+, e8=D |
| Internazionale | Nf3, Qxd8+, e8=Q |

## Contenuto

| File                    | Cosa fa                                            |
| ----------------------- | -------------------------------------------------- |
| `MoveList.tsx`          | Il componente                                      |
| `MoveList.module.css`   | I suoi stili                                       |
| `groupMovesIntoRows.ts` | Raggruppa le mosse in righe (numero, bianco, nero) |
| `*.test.ts(x)`          | I test di ciascun file                             |
