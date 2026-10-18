/**
 * Trasforma un errore di qualsiasi tipo in un messaggio leggibile.
 *
 * Rust invia gli errori come testo; altri errori arrivano come oggetti `Error`.
 *
 * @param error - l'errore ricevuto, di tipo sconosciuto.
 * @returns un messaggio da mostrare all'utente.
 */
export function describeError(error: unknown): string {
  if (typeof error === 'string') {
    return error;
  }
  if (error instanceof Error) {
    return error.message;
  }
  return 'Errore sconosciuto nella comunicazione con il nucleo Rust.';
}
