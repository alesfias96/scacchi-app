/**
 * RootLayout — la cornice comune a tutte le pagine.
 *
 * Cosa fa: applica il tema, mette il menu principale nella barra a comparsa
 * di sinistra e, nell'area di lavoro, la pagina corrispondente
 * all'indirizzo aperto (`Outlet`).
 * È il componente della "radice" del router (vedi src/app/router.tsx).
 *
 * Riceve: niente.
 * Restituisce: la finestra completa.
 */
import { Outlet } from '@tanstack/react-router';
import { AppLayout } from '../app-layout/AppLayout';
import { Sidebar } from '../sidebar/Sidebar';
import { ThemeController } from '../theme-controller/ThemeController';

export function RootLayout() {
  return (
    <>
      <ThemeController />
      <AppLayout menu={<Sidebar />}>
        <Outlet />
      </AppLayout>
    </>
  );
}
