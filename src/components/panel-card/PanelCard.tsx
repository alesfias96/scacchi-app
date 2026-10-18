/**
 * PanelCard — una scheda con titolo, usata nei pannelli accanto alla scacchiera.
 *
 * Cosa fa: dà a tutte le schede lo stesso aspetto (fondo, margini, angoli
 * arrotondati, titolo piccolo) e collega il titolo alla scheda, così i
 * lettori di schermo annunciano "Mosse", "Situazione", ecc.
 *
 * Riceve:
 * - `title`: il titolo della scheda;
 * - `children`: il contenuto.
 * Restituisce: la scheda.
 */
import { useId, type ReactNode } from 'react';
import styles from './PanelCard.module.css';

export interface PanelCardProps {
  /** Il titolo della scheda. */
  title: string;
  /** Il contenuto della scheda. */
  children: ReactNode;
}

export function PanelCard({ title, children }: PanelCardProps) {
  /** Un identificativo unico, per collegare il titolo alla scheda. */
  const titleId = useId();

  return (
    <section className={styles.card} aria-labelledby={titleId}>
      <h2 id={titleId} className={styles.title}>
        {title}
      </h2>
      {children}
    </section>
  );
}
