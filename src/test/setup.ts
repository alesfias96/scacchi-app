/**
 * Preparazione comune a tutti i test dell'interfaccia (caricata da vitest.config.ts).
 *
 * - Dopo ogni test smonta i componenti disegnati, così ogni test parte
 *   da una pagina vuota e non vede gli elementi lasciati dal test precedente.
 * - Simula `window.matchMedia`, che la pagina finta dei test (jsdom) non ha:
 *   serve al tema "Automatico" per sapere se il sistema è in modalità scura.
 *   Di base risponde "modalità chiara"; i singoli test possono cambiarlo.
 * - Simula `scrollIntoView` (che jsdom non ha): l'elenco delle mosse lo usa
 *   per tenere in vista la mossa evidenziata. Nei test non fa nulla.
 * - Simula `window.scrollTo` (che jsdom non ha): il router lo chiama a ogni
 *   cambio di pagina per riportarla in cima. Nei test non fa nulla.
 */
import { cleanup } from '@testing-library/react';
import { afterEach } from 'vitest';

window.matchMedia = (query: string) =>
  ({
    matches: false,
    media: query,
    addEventListener: () => undefined,
    removeEventListener: () => undefined,
  }) as unknown as MediaQueryList;

Element.prototype.scrollIntoView = () => undefined;

window.scrollTo = () => undefined;

afterEach(() => {
  cleanup();
});
