/**
 * useEngineLevels — i livelli di gioco del computer, chiesti a Rust.
 *
 * Cosa fa: alla prima apertura della finestra "Nuova partita" chiede a Rust
 * l'elenco dei dieci livelli (nome e forza indicativa) e lo ricorda per tutto
 * il tempo in cui l'app resta aperta: i livelli non cambiano.
 *
 * Riceve: niente.
 * Restituisce: l'elenco dei livelli, vuoto finché la risposta non è arrivata.
 */
import { useEffect, useState } from 'react';
import type { LevelInfo } from '../bindings/LevelInfo';
import { fetchEngineLevels } from '../services/engineService';

/** I livelli già ricevuti, condivisi da tutte le finestre. */
let cachedLevels: LevelInfo[] | null = null;

export function useEngineLevels(): LevelInfo[] {
  const [levels, setLevels] = useState<LevelInfo[]>(cachedLevels ?? []);

  useEffect(() => {
    if (cachedLevels !== null) {
      return;
    }
    let isMounted = true;
    fetchEngineLevels()
      .then((received) => {
        cachedLevels = received;
        if (isMounted) {
          setLevels(received);
        }
      })
      .catch(() => {
        // Senza l'elenco la finestra mostra solo le scelte che non riguardano il computer.
      });
    return () => {
      isMounted = false;
    };
  }, []);

  return levels;
}
