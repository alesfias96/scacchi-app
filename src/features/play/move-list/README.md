# src/features/play/move-list — L'elenco delle mosse

La scheda "Mosse" della sezione Gioca: la partita scritta come nei libri ("1. e4 e5 2. Cf3 …"),
con le **varianti** come su Lichess.

- La **linea principale** sta in righe: numero, mossa del Bianco, mossa del Nero.
- Sotto la riga di una mossa che ha delle alternative compaiono le **varianti**, rientrate e
  scritte in fila (`1… c5 2. Cf3`), con le sotto-varianti tra parentesi.
- Un clic su una mossa (anche di una variante) mostra la posizione dopo quella mossa; la mossa
  mostrata è evidenziata e resta in vista.
- Quando la mossa mostrata è in una variante, sotto la lista compaiono **«Promuovi a linea
  principale»** (solo a due giocatori senza orologio, a partita in corso: altrimenti la linea
  principale è la partita giocata davvero) ed **«Elimina da qui»** (toglie quella mossa e il suo seguito).

Le mosse arrivano da Rust sempre in notazione internazionale; qui vengono scritte nella notazione
scelta nelle Impostazioni (con `formatSan`, in `src/lib/`):

| Notazione      | Esempio          |
| -------------- | ---------------- |
| Figurine       | ♞f3, ♛xd8+, e8=♛ |
| Italiana       | Cf3, Dxd8+, e8=D |
| Internazionale | Nf3, Qxd8+, e8=Q |

## Contenuto

| File                  | Cosa fa                                                                         |
| --------------------- | ------------------------------------------------------------------------------- |
| `MoveList.tsx`        | Il componente                                                                   |
| `MoveList.module.css` | I suoi stili                                                                    |
| `buildMoveRows.ts`    | Prepara l'albero: le righe della linea principale e le varianti scritte in fila |
| `*.test.ts(x)`        | I test di ciascun file                                                          |
