/**
 * CriticalMoments — la scheda «Momenti critici»: le mosse che hanno cambiato la partita.
 *
 * Cosa fa: per ogni momento critico (al massimo cinque, in ordine di partita)
 * mostra la mossa giocata con il suo simbolo, il giudizio a parole, la mossa
 * migliore e il perché: cosa concedeva la mossa giocata e cosa otteneva quella
 * migliore, secondo i rilevatori. «Mostra sulla scacchiera» porta alla
 * posizione prima della mossa, con le frecce della mossa giocata e di quella migliore.
 *
 * Riceve:
 * - `moments`: i momenti critici;
 * - `moves`: le mosse (per scrivere il numero di mossa);
 * - `selectedPly`: il momento mostrato sulla scacchiera, o `null`;
 * - `notation`: la notazione scelta nelle Impostazioni;
 * - `onShow`: chiamata con la mezza mossa da mostrare.
 * Restituisce: la scheda.
 */
import type { CriticalMoment } from '../../../bindings/CriticalMoment';
import type { MoveReview } from '../../../bindings/MoveReview';
import type { Notation } from '../../../bindings/Notation';
import { Button } from '../../../components/button/Button';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { classNames } from '../../../lib/classNames';
import { formatFact } from '../../../lib/formatFact';
import { formatSan } from '../../../lib/formatSan';
import { qualitySeverity, qualitySymbol } from '../describeQuality';
import styles from './CriticalMoments.module.css';

export interface CriticalMomentsProps {
  moments: readonly CriticalMoment[];
  moves: readonly MoveReview[];
  selectedPly: number | null;
  notation: Notation;
  onShow: (ply: number) => void;
}

export function CriticalMoments({
  moments,
  moves,
  selectedPly,
  notation,
  onShow,
}: CriticalMomentsProps) {
  return (
    <PanelCard title="Momenti critici">
      {moments.length === 0 ? (
        <p className={styles.empty}>
          Nessun errore importante: nessuna mossa ha cambiato davvero la partita.
        </p>
      ) : (
        <ol className={styles.list}>
          {moments.map((moment) => {
            const move = moves[moment.ply - 1];
            const number =
              move === undefined
                ? ''
                : `${String(move.moveNumber)}${move.side === 'white' ? '.' : '…'} `;
            const severity = qualitySeverity(moment.quality);
            return (
              <li
                key={moment.ply}
                className={classNames(styles.moment, moment.ply === selectedPly && styles.selected)}
              >
                <p className={classNames(styles.move, severity !== null && styles[severity])}>
                  {number}
                  {formatSan(moment.playedSan, notation)}
                  {qualitySymbol(moment.quality)}
                </p>
                <p className={styles.text}>{moment.text}</p>
                {moment.bestSan !== null && (
                  <p className={styles.best}>Era meglio: {formatSan(moment.bestSan, notation)}</p>
                )}
                {moment.conceded.length > 0 && (
                  <>
                    <p className={styles.heading}>Cosa concedeva la mossa giocata</p>
                    <ul className={styles.facts}>
                      {moment.conceded.map((fact) => (
                        <li key={fact.text}>{formatFact(fact, notation)}</li>
                      ))}
                    </ul>
                  </>
                )}
                {moment.bestGains.length > 0 && (
                  <>
                    <p className={styles.heading}>Cosa otteneva la mossa migliore</p>
                    <ul className={styles.facts}>
                      {moment.bestGains.map((fact) => (
                        <li key={fact.text}>{formatFact(fact, notation)}</li>
                      ))}
                    </ul>
                  </>
                )}
                <Button
                  onClick={() => {
                    onShow(moment.ply);
                  }}
                >
                  Mostra sulla scacchiera
                </Button>
              </li>
            );
          })}
        </ol>
      )}
    </PanelCard>
  );
}
