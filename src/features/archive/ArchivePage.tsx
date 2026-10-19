/**
 * ArchivePage — la sezione "Archivio": tutte le partite salvate.
 *
 * Cosa fa: le partite si salvano da sole a ogni mossa; qui si ritrovano,
 * dalla più recente, con i filtri (avversario, esito, apertura). Da ogni
 * partita si può:
 * - **Rivedere** (finita) o **Continuare** (non finita): si apre in "Gioca",
 *   dove la si scorre mossa per mossa e la si può analizzare;
 * - **Eliminare**, dopo una conferma.
 *
 * Riceve: niente.
 * Restituisce: la pagina della sezione.
 */
import { useNavigate } from '@tanstack/react-router';
import { useEffect, useState } from 'react';
import type { GameFilter } from '../../bindings/GameFilter';
import { useEngineLevels } from '../../hooks/useEngineLevels';
import { describeError } from '../../lib/describeError';
import { deleteSavedGame } from '../../services/archiveService';
import { useGameStore } from '../../state/gameStore';
import { useReviewStore } from '../../state/reviewStore';
import { ArchiveFilters } from './archive-filters/ArchiveFilters';
import styles from './ArchivePage.module.css';
import { GameRow } from './game-row/GameRow';
import { useArchiveGames } from './useArchiveGames';

/** All'apertura non c'è nessun filtro. */
const NO_FILTER: GameFilter = { opponent: null, outcome: null, text: null };

export function ArchivePage() {
  const navigate = useNavigate();
  const openSaved = useGameStore((state) => state.openSaved);
  const startReview = useReviewStore((state) => state.start);
  const lastFinished = useReviewStore((state) => state.lastFinished);
  const stockfishLevels = useEngineLevels('stockfish');
  const maiaLevels = useEngineLevels('maia');
  const [filter, setFilter] = useState<GameFilter>(NO_FILTER);
  const { games, isLoading, error, reload } = useArchiveGames(filter);
  // Finita un'analisi, l'elenco si rilegge: il pulsante diventa «Vedi l'analisi».
  useEffect(() => {
    if (lastFinished !== null) {
      reload();
    }
  }, [lastFinished, reload]);

  /** Il motivo dell'ultima apertura o eliminazione non riuscita, oppure `null`. */
  const [actionError, setActionError] = useState<string | null>(null);

  /** Apre la partita in "Gioca". */
  const openGame = async (id: string) => {
    try {
      await openSaved(id);
      await navigate({ to: '/' });
    } catch (openError) {
      setActionError(describeError(openError));
    }
  };

  /** Apre l'analisi della partita; se non è ancora analizzata, la fa cominciare. */
  const reviewGame = async (id: string, analyzed: boolean) => {
    if (!analyzed) {
      await startReview(id);
    }
    await navigate({ to: '/analisi', search: { partita: id } });
  };

  /** Elimina la partita e aggiorna l'elenco. */
  const deleteGame = async (id: string) => {
    try {
      await deleteSavedGame(id);
      setActionError(null);
      reload();
    } catch (deleteError) {
      setActionError(describeError(deleteError));
    }
  };

  return (
    <section className={styles.page}>
      <header className={styles.header}>
        <h1 className={styles.title}>Archivio</h1>
        <p className={styles.subtitle}>
          Ogni partita si salva da sola, mossa dopo mossa: qui le ritrovi tutte.
        </p>
      </header>

      <ArchiveFilters filter={filter} onChange={setFilter} />

      {(error ?? actionError) !== null && (
        <p className={styles.error} role="alert">
          {error ?? actionError}
        </p>
      )}

      {!isLoading && error === null && games.length === 0 && (
        <p className={styles.empty}>
          {filter === NO_FILTER
            ? 'Non hai ancora giocato nessuna partita. Vai in "Gioca" e comincia!'
            : 'Nessuna partita corrisponde ai filtri scelti.'}
        </p>
      )}

      {games.length > 0 && (
        <>
          <p className={styles.count} aria-live="polite">
            {games.length === 1 ? '1 partita' : `${String(games.length)} partite`}
          </p>
          <ul className={styles.list} aria-label="Partite salvate">
            {games.map((game) => (
              <GameRow
                key={game.id}
                game={game}
                levelName={
                  (game.engine === 'maia' ? maiaLevels : stockfishLevels).find(
                    (info) => info.level === game.level,
                  )?.name
                }
                onOpen={(id) => {
                  void openGame(id);
                }}
                onReview={(id) => {
                  void reviewGame(id, game.analyzed);
                }}
                onDelete={(id) => {
                  void deleteGame(id);
                }}
              />
            ))}
          </ul>
        </>
      )}
    </section>
  );
}
