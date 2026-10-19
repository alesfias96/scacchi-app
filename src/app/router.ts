/**
 * Il router: l'elenco degli indirizzi dell'app e della pagina che mostra ciascuno.
 *
 * Usa TanStack Router. Ogni sezione utilizzabile ha un indirizzo:
 * - "/"              → Gioca
 * - "/archivio"      → Archivio delle partite
 * - "/allenamento"   → Allenamento con i puzzle
 * - "/tutor"         → Tutor (la partita di «Gioca» con i commenti dopo ogni mossa)
 * - "/impostazioni"  → Impostazioni
 *
 * Per aggiungere una sezione: si crea qui la sua rotta, la si aggiunge a
 * `routeTree` e si inserisce la voce in src/app/sidebar/navigation.ts.
 */
import {
  createHashHistory,
  createRootRoute,
  createRoute,
  createRouter,
} from '@tanstack/react-router';
import { ArchivePage } from '../features/archive/ArchivePage';
import { CoachedPlayPage, PlayPage } from '../features/play/PlayPage';
import { SettingsPage } from '../features/settings/SettingsPage';
import { TrainingPage } from '../features/training/TrainingPage';
import { RootLayout } from './root-layout/RootLayout';

/** Il tipo di cronologia accettato dal router (a hash nell'app, in memoria nei test). */
type AppHistory = ReturnType<typeof createHashHistory>;

const rootRoute = createRootRoute({ component: RootLayout });

const playRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: PlayPage,
});

const archiveRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/archivio',
  component: ArchivePage,
});

const trainingRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/allenamento',
  component: TrainingPage,
});

const tutorRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/tutor',
  component: CoachedPlayPage,
});

const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/impostazioni',
  component: SettingsPage,
});

const routeTree = rootRoute.addChildren([
  playRoute,
  archiveRoute,
  trainingRoute,
  tutorRoute,
  settingsRoute,
]);

/**
 * Crea il router dell'app.
 *
 * @param history - dove il router tiene traccia dell'indirizzo. Nell'app è la
 *   parte dopo "#" dell'indirizzo (funziona allo stesso modo su ogni sistema);
 *   nei test si può passare una cronologia in memoria.
 * @returns il router pronto da usare.
 */
export function createAppRouter(history: AppHistory = createHashHistory()) {
  return createRouter({ routeTree, history });
}

// Registra il tipo del router: così TypeScript controlla che ogni
// collegamento (`Link`) punti a un indirizzo che esiste davvero.
declare module '@tanstack/react-router' {
  interface Register {
    router: ReturnType<typeof createAppRouter>;
  }
}
