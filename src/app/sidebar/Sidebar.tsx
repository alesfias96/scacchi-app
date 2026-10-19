/**
 * Sidebar — la barra laterale dell'app.
 *
 * Cosa fa: mostra il nome dell'app, le sezioni e, in fondo, le Impostazioni.
 * Ogni sezione è un collegamento: quella aperta è evidenziata (il router
 * aggiunge da solo `aria-current="page"`).
 *
 * Riceve: niente.
 * Restituisce: l'elemento <nav> con la barra laterale.
 */
import { Link } from '@tanstack/react-router';
import { NAVIGATION_ITEMS, SETTINGS_ITEM } from './navigation';
import styles from './Sidebar.module.css';

/**
 * Attributo aggiunto al collegamento della sezione aperta: dice ai lettori
 * di schermo qual è la pagina corrente e permette al CSS di evidenziarla.
 */
const ACTIVE_LINK_PROPS = { 'aria-current': 'page' } as const;

export function Sidebar() {
  return (
    <nav className={styles.sidebar} aria-label="Sezioni dell'app">
      <p className={styles.brand}>Scacchi</p>

      <ul className={styles.list}>
        {NAVIGATION_ITEMS.map((item) => (
          <li key={item.id}>
            <Link
              to={item.to}
              className={styles.item}
              activeOptions={{ exact: true }}
              activeProps={ACTIVE_LINK_PROPS}
            >
              {item.label}
            </Link>
          </li>
        ))}
      </ul>

      <Link to={SETTINGS_ITEM.to} className={styles.item} activeProps={ACTIVE_LINK_PROPS}>
        {SETTINGS_ITEM.label}
      </Link>
    </nav>
  );
}
