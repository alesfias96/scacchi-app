/**
 * Prepara l'albero delle mosse per la lista, come nei libri e su Lichess.
 *
 * - La **linea principale** va in righe: numero, mossa del Bianco, mossa del Nero.
 * - Le **varianti** vanno sotto la riga della mossa a cui sono alternative,
 *   scritte in fila (`3... Cf6 4. d4`) con le loro sotto-varianti tra parentesi.
 */
import type { MoveNode } from '../../../bindings/MoveNode';
import type { MoveTree } from '../../../bindings/MoveTree';

/** Una riga della linea principale. */
export interface MoveRow {
  /** Il numero della mossa. */
  moveNumber: number;
  /** La mossa del Bianco, oppure `null` (la partita comincia con il Nero). */
  white: MoveNode | null;
  /** La mossa del Nero, oppure `null` (non ancora giocata). */
  black: MoveNode | null;
  /** Le varianti alternative alle mosse di questa riga (le loro prime mosse). */
  variations: MoveNode[];
}

/** Un pezzo di una variante scritta in fila. */
export type VariationToken =
  { kind: 'move'; node: MoveNode; number: string | null } | { kind: 'open' } | { kind: 'close' };

/** Le mosse dell'albero, cercate per numero. */
function index(tree: MoveTree): Map<number, MoveNode> {
  return new Map(tree.nodes.map((node) => [node.id, node]));
}

/** Le alternative a una mossa: le altre mosse giocate dalla stessa posizione. */
function siblings(tree: MoveTree, nodes: Map<number, MoveNode>, node: MoveNode): MoveNode[] {
  const parentChildren =
    node.parent === null ? tree.roots : (nodes.get(node.parent)?.children ?? []);
  return parentChildren
    .slice(1)
    .map((id) => nodes.get(id))
    .filter((found): found is MoveNode => found !== undefined);
}

/** Le righe della linea principale, con le varianti di ogni riga. */
export function buildMoveRows(tree: MoveTree): MoveRow[] {
  const nodes = index(tree);
  const rows: MoveRow[] = [];
  let next = tree.roots[0];
  while (next !== undefined) {
    const node = nodes.get(next);
    if (node === undefined) {
      break;
    }
    const last = rows[rows.length - 1];
    const record = node.record;
    if (record.side === 'white' || last?.moveNumber !== record.moveNumber) {
      rows.push({ moveNumber: record.moveNumber, white: null, black: null, variations: [] });
    }
    const row = rows[rows.length - 1];
    if (row !== undefined) {
      row[record.side] = node;
      row.variations.push(...siblings(tree, nodes, node));
    }
    next = node.children[0];
  }
  return rows;
}

/**
 * Una variante scritta in fila, a partire dalla sua prima mossa.
 *
 * Il numero va davanti alle mosse del Bianco e, per il Nero, alla prima mossa
 * della variante e a quella subito dopo una sotto-variante (`12... e5`).
 */
export function variationTokens(tree: MoveTree, first: MoveNode): VariationToken[] {
  const nodes = index(tree);
  const tokens: VariationToken[] = [];
  const numberFor = (node: MoveNode, forced: boolean) => {
    const { moveNumber, side } = node.record;
    if (side === 'white') {
      return `${String(moveNumber)}.`;
    }
    return forced ? `${String(moveNumber)}…` : null;
  };

  tokens.push({ kind: 'move', node: first, number: numberFor(first, true) });
  let current = first;
  /** `true` subito dopo una sotto-variante: la mossa successiva riprende con il numero. */
  let forced = false;
  for (;;) {
    const mainId = current.children[0];
    const main = mainId === undefined ? undefined : nodes.get(mainId);
    if (main === undefined) {
      break;
    }
    tokens.push({ kind: 'move', node: main, number: numberFor(main, forced) });
    const alternatives = siblings(tree, nodes, main);
    for (const alternative of alternatives) {
      tokens.push({ kind: 'open' }, ...variationTokens(tree, alternative), { kind: 'close' });
    }
    forced = alternatives.length > 0;
    current = main;
  }
  return tokens;
}
