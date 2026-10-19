/**
 * Servizio delle osservazioni: la chiamata al nucleo Rust che fa lavorare i rilevatori.
 *
 * Insieme agli altri file di questa cartella, è l'unico punto dell'interfaccia
 * che conosce i nomi dei comandi Rust.
 */
import { invoke } from '@tauri-apps/api/core';
import type { PositionReport } from '../bindings/PositionReport';

/**
 * Chiede a Rust i fatti trovati nella posizione mostrata: minacce, tattiche,
 * pedoni, re, attività dei pezzi, materiale.
 *
 * @returns il rapporto, oppure `null` se la posizione non fosse valida.
 */
export function fetchPositionReport(): Promise<PositionReport | null> {
  return invoke<PositionReport | null>('get_position_report');
}
