/**
 * Il router: l'elenco degli indirizzi dell'app e della pagina che mostra ciascuno.
 *
 * Usa TanStack Router. Ogni sezione utilizzabile ha un indirizzo:
 * - "/"              → Gioca
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
import { PlayPage } from '../features/play/PlayPage';
import { SettingsPage } from '../features/settings/SettingsPage';
import { RootLayout } from './root-layout/RootLayout';

/** Il tipo di cronologia accettato dal router (a hash nell'app, in memoria nei test). */
type AppHistory = ReturnType<typeof createHashHistory>;

const rootRoute = createRootRoute({ component: RootLayout });

const playRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: PlayPage,
});

const settingsRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/impostazioni',
  component: SettingsPage,
});

const routeTree = rootRoute.addChildren([playRoute, settingsRoute]);

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
