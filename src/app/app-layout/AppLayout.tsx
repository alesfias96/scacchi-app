/**
 * AppLayout — la struttura della finestra.
 *
 * Cosa fa: l'area di lavoro occupa tutta la finestra; il menu principale
 * (le sezioni dell'app) sta in una barra a comparsa sul bordo sinistro, che
 * si apre portando il mouse sul bordo, con la sua linguetta o con il tasto M,
 * e si sovrappone alla pagina senza spostarla.
 *
 * Riceve:
 * - `menu`: il contenuto del menu principale;
 * - `children`: il contenuto dell'area di lavoro (la sezione aperta).
 * Restituisce: la struttura principale dell'app.
 */
import type { ReactNode } from 'react';
import { SideDrawer } from '../../components/side-drawer/SideDrawer';
import styles from './AppLayout.module.css';

export interface AppLayoutProps {
  /** Il menu principale. */
  menu: ReactNode;
  /** La sezione aperta, mostrata nell'area di lavoro. */
  children: ReactNode;
}

export function AppLayout({ menu, children }: AppLayoutProps) {
  return (
    <div className={styles.layout}>
      <SideDrawer side="left" title="Menu" shortcut="m" padded={false}>
        {menu}
      </SideDrawer>
      <main className={styles.main}>{children}</main>
    </div>
  );
}
