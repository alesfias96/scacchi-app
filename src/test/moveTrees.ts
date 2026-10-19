/**
 * Alberi di mosse di prova per i test della lista mosse (non sono usati dall'app).
 *
 * La partita è 1.e4 e5 2.Cf3, con la variante 1... c5 2.Cf3 (e dentro, 2.c3).
 */
import type { MoveNode } from '../bindings/MoveNode';
import type { MoveTree } from '../bindings/MoveTree';
import type { Side } from '../bindings/Side';

/** Una mossa dell'albero. */
function node(
  id: number,
  parent: number | null,
  children: number[],
  [moveNumber, side, san]: [number, Side, string],
  mainline: boolean,
): MoveNode {
  return {
    id,
    parent,
    children,
    mainline,
    record: { ply: 0, moveNumber, side, san, uci: san, from: '', to: '', capture: false },
  };
}

/** 1.e4 e5 (1... c5 2.Cf3 (2.c3)) 2.Cf3, con la mossa mostrata indicata. */
export function sampleTree(current: number | null): MoveTree {
  return {
    roots: [0],
    current,
    nodes: [
      node(0, null, [1, 3], [1, 'white', 'e4'], true),
      node(1, 0, [2], [1, 'black', 'e5'], true),
      node(2, 1, [], [2, 'white', 'Nf3'], true),
      node(3, 0, [4, 5], [1, 'black', 'c5'], false),
      node(4, 3, [], [2, 'white', 'Nf3'], false),
      node(5, 3, [], [2, 'white', 'c3'], false),
    ],
  };
}
