/**
 * Test del rapporto dei rilevatori. Il servizio verso Rust è simulato.
 */
import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { PositionReport } from '../bindings/PositionReport';
import { usePositionReport } from './usePositionReport';

const insightMock = vi.hoisted(() => ({ fetchPositionReport: vi.fn() }));

vi.mock('../services/insightService', () => insightMock);

const FEN = 'rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1';

/** Un rapporto di prova con un solo fatto. */
const REPORT: PositionReport = {
  fen: FEN,
  turn: 'black',
  phase: 'opening',
  material: {
    white: { pawns: 8, knights: 2, bishops: 2, rooks: 2, queens: 1, points: 39 },
    black: { pawns: 8, knights: 2, bishops: 2, rooks: 2, queens: 1, points: 39 },
    advantage: 0,
  },
  facts: [
    {
      category: 'pawns',
      side: 'white',
      text: 'Un fatto di prova',
      squares: [],
      arrows: [],
      moves: [],
    },
  ],
};

/** Un componente di prova che scrive quanti fatti ha ricevuto. */
function ReportProbe({ fen, enabled }: { fen: string; enabled: boolean }) {
  const report = usePositionReport(fen, enabled);
  return <p>{report === null ? 'nessuno' : String(report.facts.length)}</p>;
}

describe('usePositionReport', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    insightMock.fetchPositionReport.mockResolvedValue(REPORT);
  });

  it('chiede il rapporto della posizione mostrata', async () => {
    render(<ReportProbe fen={FEN} enabled />);

    expect(await screen.findByText('1')).toBeDefined();
  });

  it('spento non chiede nulla', () => {
    render(<ReportProbe fen={FEN} enabled={false} />);

    expect(insightMock.fetchPositionReport).not.toHaveBeenCalled();
    expect(screen.getByText('nessuno')).toBeDefined();
  });

  it('ignora un rapporto di un’altra posizione', async () => {
    insightMock.fetchPositionReport.mockResolvedValue({ ...REPORT, fen: 'altra' });

    render(<ReportProbe fen={FEN} enabled />);
    await Promise.resolve();

    expect(screen.getByText('nessuno')).toBeDefined();
  });
});
