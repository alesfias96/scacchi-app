/**
 * I disegni delle icone dell'app.
 *
 * Ogni icona è un elenco di tratti (percorsi SVG) su una griglia di 24×24
 * punti, disegnati come linee di spessore uniforme e senza riempimento: così
 * tutte le icone hanno lo stesso stile e prendono il colore del testo.
 */

/** I nomi delle icone disponibili. */
export type IconName =
  | 'first'
  | 'previous'
  | 'next'
  | 'last'
  | 'flip'
  | 'undo'
  | 'plus'
  | 'copy'
  | 'paste'
  | 'soundOn'
  | 'soundOff'
  | 'flag';

export const ICON_PATHS: Record<IconName, readonly string[]> = {
  /** Vai all'inizio: freccia a sinistra con barra. */
  first: ['M18 6l-6 6 6 6', 'M7 6v12'],
  /** Mossa precedente: freccia a sinistra. */
  previous: ['M15 6l-6 6 6 6'],
  /** Mossa successiva: freccia a destra. */
  next: ['M9 6l6 6-6 6'],
  /** Vai alla fine: freccia a destra con barra. */
  last: ['M6 6l6 6-6 6', 'M17 6v12'],
  /** Gira la scacchiera: una freccia in su e una in giù. */
  flip: ['M7 20V4', 'M3 8l4-4 4 4', 'M17 4v16', 'M13 16l4 4 4-4'],
  /** Annulla: freccia che torna indietro. */
  undo: ['M9 14L4 9l5-5', 'M4 9h10a6 6 0 0 1 0 12h-3'],
  /** Nuovo: il segno più. */
  plus: ['M12 5v14', 'M5 12h14'],
  /** Copia: due fogli sovrapposti. */
  copy: ['M9 9h11v11H9z', 'M5 15H4V4h11v1'],
  /** Incolla: un portablocchi. */
  paste: ['M9 3h6v4H9z', 'M9 5H5v16h14V5h-4'],
  /** Suoni attivi: altoparlante con due onde. */
  soundOn: ['M4 9h4l5-4v14l-5-4H4z', 'M16.5 8.5a5 5 0 0 1 0 7', 'M19 6a8.5 8.5 0 0 1 0 12'],
  /** Suoni disattivati: altoparlante barrato da una croce. */
  /** Abbandona: una bandierina sull'asta. */
  flag: ['M5 21V4', 'M5 4h11l-2.5 4 2.5 4H5'],
  soundOff: ['M4 9h4l5-4v14l-5-4H4z', 'M16 9l5 6', 'M21 9l-5 6'],
};
