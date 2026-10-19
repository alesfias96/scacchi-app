/**
 * useEngineLevels — i livelli di gioco di un motore del computer, chiesti a Rust.
 *
 * Cosa fa: la prima volta che servono chiede a Rust l'elenco dei livelli del
 * motore (nome e forza indicativa): i dieci di Stockfish oppure i nove di Maia.
 * Li ricorda per tutto il tempo in cui l'app resta aperta: i livelli non cambiano.
 *
 * Riceve: `engine`, il motore (Stockfish o Maia).
 * Restituisce: l'elenco dei livelli, vuoto finché la risposta non è arrivata.
 */
import { useEffect, useState } from 'react';
import type { ComputerEngine } from '../bindings/ComputerEngine';
import type { LevelInfo } from '../bindings/LevelInfo';
import { fetchEngineLevels, fetchMaiaLevels } from '../services/engineService';

/** I livelli già ricevuti, per motore, condivisi da tutte le finestre. */
const cachedLevels: Partial<Record<ComputerEngine, LevelInfo[]>> = {};

/** Come chiedere a Rust i livelli di ciascun motore. */
const FETCHERS: Record<ComputerEngine, () => Promise<LevelInfo[]>> = {
  stockfish: fetchEngineLevels,
  maia: fetchMaiaLevels,
};

export function useEngineLevels(engine: ComputerEngine): LevelInfo[] {
  // Cambiando valore si ridisegna il componente quando arriva una risposta.
  const [, setReceived] = useState(0);

  useEffect(() => {
    if (cachedLevels[engine] !== undefined) {
      return;
    }
    let isMounted = true;
    FETCHERS[engine]()
      .then((received) => {
        cachedLevels[engine] = received;
        if (isMounted) {
          setReceived((count) => count + 1);
        }
      })
      .catch(() => {
        // Senza l'elenco la finestra mostra solo le scelte che non riguardano il computer.
      });
    return () => {
      isMounted = false;
    };
  }, [engine]);

  return cachedLevels[engine] ?? [];
}
