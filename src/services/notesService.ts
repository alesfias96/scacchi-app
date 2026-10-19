/**
 * Servizio degli appunti: le chiamate al nucleo Rust per leggere, cercare,
 * salvare ed eliminare gli appunti personali.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import type { Note } from '../bindings/Note';
import type { NoteInput } from '../bindings/NoteInput';
import type { NoteQuery } from '../bindings/NoteQuery';

/**
 * Chiede a Rust gli appunti che corrispondono alla ricerca.
 *
 * @param query - le parole da cercare e l'etichetta (entrambe facoltative).
 * @returns gli appunti, dal più recente.
 */
export function listNotes(query: NoteQuery): Promise<Note[]> {
  return invoke<Note[]>('list_notes', { query });
}

/**
 * Chiede a Rust tutte le etichette usate.
 *
 * @returns le etichette, in ordine alfabetico.
 */
export function fetchNoteTags(): Promise<string[]> {
  return invoke<string[]>('note_tags');
}

/**
 * Salva un appunto (lo crea se non ha ancora un nome unico).
 *
 * @param note - l'appunto come l'ha scritto la persona.
 * @returns l'appunto salvato, con il suo nome unico e le etichette ripulite.
 */
export function saveNote(note: NoteInput): Promise<Note> {
  return invoke<Note>('save_note', { note });
}

/**
 * Elimina un appunto.
 *
 * @param id - il nome unico dell'appunto.
 */
export async function deleteNote(id: string): Promise<void> {
  await invoke('delete_note', { id });
}
