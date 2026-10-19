/**
 * I collegamenti degli appunti: da dove nasce un appunto e come si mostra la sua posizione.
 */
import type { NoteLink } from '../../bindings/NoteLink';
import type { SessionView } from '../../bindings/SessionView';
import type { Side } from '../../bindings/Side';

/**
 * Il collegamento per un appunto nato in «Gioca»: sempre la posizione
 * mostrata; la partita e la mossa solo se la partita ha già delle mosse (è
 * salvata nell'archivio) e la posizione è sulla linea principale (le
 * varianti non si salvano mossa per mossa, quindi non si potrebbero riaprire).
 *
 * @param session - la partita in corso.
 * @returns il collegamento.
 */
export function playLink(session: SessionView): NoteLink {
  const { game } = session;
  const currentId = game.tree.current;
  const onMainline =
    currentId === null || game.tree.nodes.some((node) => node.id === currentId && node.mainline);
  const linkGame = game.moves.length > 0 && onMainline;
  return {
    fen: game.position.fen,
    gameId: linkGame ? session.id : null,
    ply: linkGame ? game.cursor : null,
  };
}

/**
 * La posizione da disegnare, dal testo della FEN: il primo campo è la
 * disposizione dei pezzi, il secondo dice chi muove ("w" Bianco, "b" Nero).
 * Si leggono solo i campi del testo: le regole degli scacchi restano in Rust.
 *
 * @param fen - la posizione in notazione FEN.
 * @returns la disposizione dei pezzi e chi deve muovere.
 */
export function fenBoard(fen: string): { boardFen: string; turn: Side } {
  const [board = '', turn = 'w'] = fen.split(' ');
  return { boardFen: board, turn: turn === 'b' ? 'black' : 'white' };
}

/**
 * Il nome di un appunto nell'elenco: il titolo, oppure l'inizio del testo.
 *
 * @param title - il titolo (può essere vuoto).
 * @param body - il testo.
 * @returns ad esempio "Idea per la Siciliana" oppure "Appunto senza titolo".
 */
export function noteLabel(title: string, body: string): string {
  if (title.trim() !== '') {
    return title;
  }
  const firstLine = body
    .split('\n')
    .map((line) => line.trim())
    .find((line) => line !== '');
  return firstLine ?? 'Appunto senza titolo';
}
