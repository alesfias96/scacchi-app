/**
 * Elenco delle sezioni dell'app mostrate nella barra laterale.
 *
 * È un semplice elenco di dati, separato dal componente che lo disegna:
 * per aggiungere o rinominare una sezione si modifica solo questo file
 * (e, se è utilizzabile, si aggiunge la sua pagina in src/app/router.tsx).
 */

/** Una sezione già utilizzabile: ha un indirizzo a cui portare. */
export interface AvailableSection {
  /** Identificativo stabile della sezione (usato come chiave da React). */
  id: string;
  /** Il nome mostrato all'utente. */
  label: string;
  /** L'indirizzo della pagina, come definito in src/app/router.tsx. */
  to: '/' | '/archivio' | '/allenamento' | '/tutor' | '/impostazioni';
}

/** Una sezione non ancora costruita: visibile ma disattivata. */
export interface UpcomingSection {
  /** Identificativo stabile della sezione. */
  id: string;
  /** Il nome mostrato all'utente. */
  label: string;
  /** La fase della roadmap in cui arriverà (vedi PROGETTO.md). */
  availableFromPhase: number;
}

/** Una voce della barra laterale. */
export type NavigationItem = AvailableSection | UpcomingSection;

/** Le sezioni principali, nell'ordine in cui compaiono. */
export const NAVIGATION_ITEMS: readonly NavigationItem[] = [
  { id: 'play', label: 'Gioca', to: '/' },
  { id: 'archive', label: 'Archivio', to: '/archivio' },
  { id: 'training', label: 'Allenamento', to: '/allenamento' },
  { id: 'tutor', label: 'Tutor', to: '/tutor' },
  { id: 'review', label: 'Analisi partite', availableFromPhase: 8 },
  { id: 'progress', label: 'Progressi', availableFromPhase: 9 },
];

/** La voce delle impostazioni, mostrata in fondo alla barra laterale. */
export const SETTINGS_ITEM: AvailableSection = {
  id: 'settings',
  label: 'Impostazioni',
  to: '/impostazioni',
};

/**
 * Distingue le sezioni utilizzabili da quelle in arrivo.
 *
 * @returns `true` se la voce ha un indirizzo a cui portare.
 */
export function isAvailable(item: NavigationItem): item is AvailableSection {
  return 'to' in item;
}
