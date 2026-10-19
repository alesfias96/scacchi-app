/**
 * Test dello stato del tutor. Il servizio verso Rust è simulato.
 */
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { AFTER_E4_SCENARIO, START_EXPLANATION } from '../test/tutorExplanations';
import { useTutorStore, type TutorTarget } from './tutorStore';

const serviceMock = vi.hoisted(() => ({
  explainPosition: vi.fn(),
  tryWhatIf: vi.fn(),
}));

vi.mock('../services/tutorService', () => serviceMock);

const START = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1';

function target(lock: TutorTarget['lock'] = null): TutorTarget {
  return { startFen: START, moves: [], orientation: 'white', lock, lockKey: 'partita-1' };
}

describe('tutorStore', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    serviceMock.explainPosition.mockResolvedValue(START_EXPLANATION);
    serviceMock.tryWhatIf.mockResolvedValue(AFTER_E4_SCENARIO);
    useTutorStore.setState({
      target: null,
      isOpen: false,
      withEngine: true,
      unlockedKey: null,
      explanation: null,
      isLoading: false,
      error: null,
      whatIf: null,
    });
  });

  it('spiega la posizione solo quando il pannello è aperto', async () => {
    useTutorStore.getState().setTarget(target());
    expect(serviceMock.explainPosition).not.toHaveBeenCalled();

    useTutorStore.getState().setOpen(true);

    await vi.waitFor(() => {
      expect(useTutorStore.getState().explanation).toEqual(START_EXPLANATION);
    });
    expect(serviceMock.explainPosition).toHaveBeenCalledWith(START, [], true);
  });

  it('contro il computer resta spento finché non lo si accende comunque', async () => {
    useTutorStore.getState().setOpen(true);
    useTutorStore.getState().setTarget(target('computerGame'));
    expect(serviceMock.explainPosition).not.toHaveBeenCalled();

    useTutorStore.getState().unlock();

    await vi.waitFor(() => {
      expect(serviceMock.explainPosition).toHaveBeenCalledOnce();
    });
  });

  it('non rifà la spiegazione se la posizione non cambia', async () => {
    useTutorStore.getState().setOpen(true);
    useTutorStore.getState().setTarget(target());
    await vi.waitFor(() => {
      expect(useTutorStore.getState().explanation).not.toBeNull();
    });

    useTutorStore.getState().setTarget({ ...target(), orientation: 'black' });

    expect(serviceMock.explainPosition).toHaveBeenCalledOnce();
    expect(useTutorStore.getState().target?.orientation).toBe('black');
  });

  it('prova le mosse dopo quelle già provate e le può ritirare', async () => {
    useTutorStore.getState().setOpen(true);
    useTutorStore.getState().setTarget({ ...target(), moves: ['d2d4'] });
    useTutorStore.getState().startWhatIf();

    await useTutorStore.getState().playWhatIf('e2e4');
    await useTutorStore.getState().playWhatIf('e7e5');

    expect(serviceMock.tryWhatIf).toHaveBeenLastCalledWith(START, ['d2d4', 'e2e4'], 'e7e5', true);
    expect(useTutorStore.getState().whatIf).toHaveLength(2);

    useTutorStore.getState().undoWhatIf();
    expect(useTutorStore.getState().whatIf).toHaveLength(1);

    useTutorStore.getState().stopWhatIf();
    expect(useTutorStore.getState().whatIf).toBeNull();
  });
});
