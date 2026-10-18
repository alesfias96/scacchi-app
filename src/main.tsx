/**
 * Punto di ingresso dell'interfaccia.
 *
 * È il primo file che parte quando si apre la finestra dell'app
 * (viene richiamato da index.html). Ha un solo compito: trovare
 * l'elemento "root" nella pagina e disegnarci dentro il componente App.
 *
 * Non contiene logica né stili: tutto il resto vive nelle sottocartelle di src/.
 */
import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './app/App';
// Stili globali (design token, azzeramento, font, base): importati una sola volta, qui.
import './styles/index.css';

/** Contenitore definito in index.html dentro cui React disegna l'interfaccia. */
const rootElement = document.getElementById('root');

// Se il contenitore manca, index.html è stato modificato in modo errato:
// meglio fermarsi subito con un messaggio chiaro che mostrare una finestra vuota.
if (rootElement === null) {
  throw new Error("Elemento #root non trovato in index.html: impossibile avviare l'interfaccia.");
}

// StrictMode attiva controlli extra durante lo sviluppo (nessun effetto in produzione):
// segnala in anticipo errori comuni nei componenti.
createRoot(rootElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
