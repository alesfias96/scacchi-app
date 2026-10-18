/**
 * App — componente radice dell'interfaccia.
 *
 * Cosa fa: aspetta le impostazioni salvate (SettingsGate) e poi avvia il
 * router, che mostra la cornice comune e la pagina dell'indirizzo aperto.
 * Riceve: niente.
 * Restituisce: l'intera interfaccia.
 */
import { RouterProvider } from '@tanstack/react-router';
import { useState } from 'react';
import { createAppRouter } from './router';
import { SettingsGate } from './settings-gate/SettingsGate';

export function App() {
  // Il router viene creato una sola volta, al primo disegno.
  const [router] = useState(() => createAppRouter());

  return (
    <SettingsGate>
      <RouterProvider router={router} />
    </SettingsGate>
  );
}
