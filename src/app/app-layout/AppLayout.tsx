/**
 * AppLayout — la struttura della finestra.
 *
 * Cosa fa: l'area di lavoro occupa tutta la finestra. Due barre a comparsa
 * si sovrappongono alla pagina senza spostarla:
 * - a sinistra il menu principale (le sezioni dell'app): si apre portando il
 *   mouse sul bordo, con la sua linguetta o con il tasto M;
 * - a destra il tutor, uguale in ogni sezione: stessi modi, con il tasto T.
 *
 * Riceve:
 * - `menu`: il contenuto del menu principale;
 * - `tutor`: il contenuto della barra del tutor;
 * - `onTutorOpenChange`: chiamata quando la barra del tutor si apre o si chiude;
 * - `children`: il contenuto dell'area di lavoro (la sezione aperta).
 * Restituisce: la struttura principale dell'app.
 */
import type { ReactNode } from 'react';
import { SideDrawer } from '../../components/side-drawer/SideDrawer';
import styles from './AppLayout.module.css';

export interface AppLayoutProps {
  /** Il menu principale. */
  menu: ReactNode;
  /** Il contenuto della barra del tutor. */
  tutor: ReactNode;
  /** Chiamata con `true` quando la barra del tutor si apre, `false` quando si chiude. */
  onTutorOpenChange: (isOpen: boolean) => void;
  /** La sezione aperta, mostrata nell'area di lavoro. */
  children: ReactNode;
}

export function AppLayout({ menu, tutor, onTutorOpenChange, children }: AppLayoutProps) {
  return (
    <div className={styles.layout}>
      <SideDrawer side="left" title="Menu" shortcut="m" padded={false}>
        {menu}
      </SideDrawer>
      <main className={styles.main}>{children}</main>
      <SideDrawer side="right" title="Tutor" shortcut="t" onOpenChange={onTutorOpenChange}>
        {tutor}
      </SideDrawer>
    </div>
  );
}
