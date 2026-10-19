/**
 * RootLayout — la cornice comune a tutte le pagine.
 *
 * Cosa fa: applica il tema, mette il menu principale nella barra a comparsa
 * di sinistra e il tutor in quella di destra e, nell'area di lavoro, la
 * pagina corrispondente all'indirizzo aperto (`Outlet`).
 * È il componente della "radice" del router (vedi src/app/router.tsx).
 *
 * Riceve: niente.
 * Restituisce: la finestra completa.
 */
import { Outlet } from '@tanstack/react-router';
import { TutorPanel } from '../../features/tutor/TutorPanel';
import { useTutorStore } from '../../state/tutorStore';
import { AppLayout } from '../app-layout/AppLayout';
import { Sidebar } from '../sidebar/Sidebar';
import { ThemeController } from '../theme-controller/ThemeController';

export function RootLayout() {
  const setTutorOpen = useTutorStore((state) => state.setOpen);
  return (
    <>
      <ThemeController />
      <AppLayout menu={<Sidebar />} tutor={<TutorPanel />} onTutorOpenChange={setTutorOpen}>
        <Outlet />
      </AppLayout>
    </>
  );
}
