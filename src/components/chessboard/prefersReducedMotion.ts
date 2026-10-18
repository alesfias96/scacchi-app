/**
 * Dice se l'utente ha chiesto al sistema di ridurre le animazioni.
 *
 * Su Windows, macOS e Linux c'è un'opzione di accessibilità ("riduci
 * movimento", "mostra animazioni"). I fogli di stile la rispettano da soli;
 * chessground invece anima i pezzi con JavaScript, quindi glielo si dice qui.
 *
 * @returns `true` se le animazioni vanno spente.
 */
export function prefersReducedMotion(): boolean {
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}
