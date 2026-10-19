/**
 * renderWithRouter — disegna un componente dentro un router di prova.
 *
 * I componenti che contengono collegamenti (`Link`) funzionano solo dentro
 * un router. Questa funzione crea un router in memoria con gli stessi
 * indirizzi dell'app, apre l'indirizzo richiesto e disegna il componente.
 *
 * @param element - il componente da disegnare.
 * @param path - l'indirizzo aperto all'inizio (predefinito: "/").
 */
import {
  RouterProvider,
  createMemoryHistory,
  createRootRoute,
  createRoute,
  createRouter,
} from '@tanstack/react-router';
import { render } from '@testing-library/react';
import type { ReactElement } from 'react';

export function renderWithRouter(element: ReactElement, path = '/') {
  const rootRoute = createRootRoute({ component: () => element });
  const pages = [
    '/',
    '/archivio',
    '/allenamento',
    '/tutor',
    '/analisi',
    '/progressi',
    '/impostazioni',
  ].map((pagePath) =>
    createRoute({ getParentRoute: () => rootRoute, path: pagePath, component: () => null }),
  );
  const router = createRouter({
    routeTree: rootRoute.addChildren(pages),
    history: createMemoryHistory({ initialEntries: [path] }),
  });
  return render(<RouterProvider router={router} />);
}
