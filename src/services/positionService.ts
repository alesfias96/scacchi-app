/**
 * Servizio della posizione: le chiamate al nucleo Rust che riguardano la scacchiera.
 *
 * È l'unico punto dell'interfaccia che conosce i nomi dei comandi Rust.
 * Il resto dell'app chiama queste funzioni, senza sapere come avviene la comunicazione.
 */
import { invoke } from '@tauri-apps/api/core';
import type { PositionView } from '../bindings/PositionView';

/**
 * Chiede a Rust la posizione iniziale di una partita.
 *
 * @returns la posizione iniziale (FEN, turno, scacco, mosse legali).
 * @throws se la comunicazione con Rust non riesce (ad esempio se l'interfaccia
 *   è aperta nel browser invece che dentro l'app desktop).
 */
export function fetchInitialPosition(): Promise<PositionView> {
  return invoke<PositionView>('initial_position');
}
