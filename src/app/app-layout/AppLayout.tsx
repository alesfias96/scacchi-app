/**
 * AppLayout — la struttura della finestra.
 *
 * Cosa fa: divide la finestra in due colonne, la barra laterale a sinistra
 * e l'area di lavoro a destra.
 *
 * Riceve:
 * - `sidebar`: il contenuto della colonna sinistra;
 * - `children`: il contenuto dell'area di lavoro (la sezione aperta).
 * Restituisce: la griglia principale dell'app.
 */
import type { ReactNode } from 'react';
import styles from './AppLayout.module.css';

export interface AppLayoutProps {
  /** La barra laterale. */
  sidebar: ReactNode;
  /** La sezione aperta, mostrata nell'area di lavoro. */
  children: ReactNode;
}

export function AppLayout({ sidebar, children }: AppLayoutProps) {
  return (
    <div className={styles.layout}>
      {sidebar}
      <main className={styles.main}>{children}</main>
    </div>
  );
}
