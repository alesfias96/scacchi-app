/**
 * Elenco delle sezioni dell'app mostrate nella barra laterale.
 *
 * È un semplice elenco di dati, separato dal componente che lo disegna:
 * per aggiungere o rinominare una sezione si modifica solo questo file
 * (e si aggiunge la sua pagina in src/app/router.ts).
 */

/** Una voce della barra laterale: una sezione con l'indirizzo a cui portare. */
export interface NavigationItem {
  /** Identificativo stabile della sezione (usato come chiave da React). */
  id: string;
  /** Il nome mostrato all'utente. */
  label: string;
  /** L'indirizzo della pagina, come definito in src/app/router.ts. */
  to:
    | '/'
    | '/archivio'
    | '/allenamento'
    | '/tutor'
    | '/analisi'
    | '/progressi'
    | '/appunti'
    | '/impostazioni';
}

/** Le sezioni principali, nell'ordine in cui compaiono. */
export const NAVIGATION_ITEMS: readonly NavigationItem[] = [
  { id: 'play', label: 'Gioca', to: '/' },
  { id: 'archive', label: 'Archivio', to: '/archivio' },
  { id: 'training', label: 'Allenamento', to: '/allenamento' },
  { id: 'tutor', label: 'Tutor', to: '/tutor' },
  { id: 'review', label: 'Analisi partite', to: '/analisi' },
  { id: 'progress', label: 'Progressi', to: '/progressi' },
  { id: 'notes', label: 'Appunti', to: '/appunti' },
];

/** La voce delle impostazioni, mostrata in fondo alla barra laterale. */
export const SETTINGS_ITEM: NavigationItem = {
  id: 'settings',
  label: 'Impostazioni',
  to: '/impostazioni',
};
