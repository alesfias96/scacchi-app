/**
 * Test della scheda di un'idea: momenti chiave e variante sulla scacchiera.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it } from 'vitest';
import { useIdeaReplayStore } from '../../../state/ideaReplayStore';
import { KNIGHT_IDEA } from '../../../test/ideas';
import { IdeaCard } from './IdeaCard';

describe('IdeaCard', () => {
  beforeEach(() => {
    useIdeaReplayStore.setState({ idea: null, frame: 0 });
  });

  it('mostra l’idea, la mossa da cui si vede e i momenti chiave con la loro mossa', () => {
    render(<IdeaCard idea={KNIGHT_IDEA} title="Idea 1" notation="internazionale" />);

    expect(screen.getByText(KNIGHT_IDEA.headline)).toBeDefined();
    expect(screen.getByText(/Si vede dopo 3\. Ng5/)).toBeDefined();
    expect(screen.getByText('3. Ng5')).toBeDefined();
    expect(screen.getByText('2. Nf3 Nc6 3. Ng5')).toBeDefined();
  });

  it('fa scorrere la variante sulla scacchiera e torna alla partita', () => {
    render(<IdeaCard idea={KNIGHT_IDEA} title="Idea 1" notation="internazionale" />);

    fireEvent.click(screen.getByRole('button', { name: 'Guarda sulla scacchiera' }));
    expect(useIdeaReplayStore.getState().idea).toBe(KNIGHT_IDEA);
    expect(screen.getByText('Posizione di partenza')).toBeDefined();

    fireEvent.click(screen.getByRole('button', { name: 'Mossa successiva della variante' }));
    fireEvent.click(screen.getByRole('button', { name: 'Mossa successiva della variante' }));
    fireEvent.click(screen.getByRole('button', { name: 'Mossa successiva della variante' }));
    expect(useIdeaReplayStore.getState().frame).toBe(3);
    // Alla terza mossa c'è il momento chiave: il suo commento compare.
    expect(screen.getAllByText(/si avvicina al re avversario/)).toHaveLength(2);

    fireEvent.click(screen.getByRole('button', { name: 'Torna alla partita' }));
    expect(useIdeaReplayStore.getState().idea).toBeNull();
  });

  it('un clic su un momento chiave lo mostra subito sulla scacchiera', () => {
    render(<IdeaCard idea={KNIGHT_IDEA} title="Idea 1" notation="internazionale" />);

    fireEvent.click(screen.getByRole('button', { name: /3\. Ng5/ }));

    expect(useIdeaReplayStore.getState().frame).toBe(3);
  });
});
