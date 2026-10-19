/**
 * RootLayout — la cornice comune a tutte le pagine.
 *
 * Cosa fa: applica il tema, mette il menu principale nella barra a comparsa
 * di sinistra e, nell'area di lavoro, la pagina corrispondente
 * all'indirizzo aperto (`Outlet`). Se c'è una versione nuova (e si è scelto
 * di controllare a ogni avvio), mostra l'avviso in basso a destra.
 * È il componente della "radice" del router (vedi src/app/router.tsx).
 *
 * Riceve: niente.
 * Restituisce: la finestra completa.
 */
import { Outlet } from '@tanstack/react-router';
import { AppLayout } from '../app-layout/AppLayout';
import { Sidebar } from '../sidebar/Sidebar';
import { ThemeController } from '../theme-controller/ThemeController';
import { UpdateNotice } from '../update-notice/UpdateNotice';

export function RootLayout() {
  return (
    <>
      <ThemeController />
      <AppLayout menu={<Sidebar />}>
        <Outlet />
      </AppLayout>
      <UpdateNotice />
    </>
  );
}
