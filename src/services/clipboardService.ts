/**
 * Servizio degli appunti: copia un testo negli appunti del sistema.
 *
 * La copia la fa Rust (con il plugin ufficiale di Tauri): dal motore web
 * dell'app la scrittura negli appunti non sempre è permessa.
 */
import { invoke } from '@tauri-apps/api/core';

/**
 * Copia un testo negli appunti del sistema.
 *
 * @param text - il testo da copiare (es. una FEN o un PGN).
 * @throws se il sistema non permette di scrivere negli appunti.
 */
export async function copyText(text: string): Promise<void> {
  await invoke('copy_text', { text });
}
