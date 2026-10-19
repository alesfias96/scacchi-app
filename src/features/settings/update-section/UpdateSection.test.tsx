/**
 * Test della scheda «Aggiornamenti». Il servizio verso Rust è simulato.
 */
import { fireEvent, render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { useUpdateStore } from '../../../state/updateStore';
import { UpdateSection } from './UpdateSection';

const updateMock = vi.hoisted(() => ({
  fetchAppVersion: vi.fn(),
  checkForUpdate: vi.fn(),
  installUpdate: vi.fn(),
}));

vi.mock('../../../services/updateService', () => updateMock);

describe('UpdateSection', () => {
  beforeEach(() => {
    vi.resetAllMocks();
    updateMock.fetchAppVersion.mockResolvedValue('0.10.0');
    updateMock.installUpdate.mockResolvedValue(undefined);
    useUpdateStore.setState({ installedVersion: null, status: { kind: 'idle' } });
  });

  it('mostra la versione e propone quella nuova', async () => {
    updateMock.checkForUpdate.mockResolvedValue('0.11.0');
    render(<UpdateSection checkOnStart={false} onCheckOnStartChange={vi.fn()} />);

    expect(await screen.findByText('0.10.0')).toBeDefined();
    fireEvent.click(screen.getByRole('button', { name: 'Controlla aggiornamenti' }));

    expect(await screen.findByText('È disponibile la versione 0.11.0.')).toBeDefined();
    fireEvent.click(screen.getByRole('button', { name: 'Installa e riavvia' }));
    expect(updateMock.installUpdate).toHaveBeenCalled();
  });

  it('dice se si ha già l’ultima versione o se il controllo non riesce', async () => {
    updateMock.checkForUpdate.mockResolvedValueOnce(null).mockRejectedValueOnce('niente rete');
    render(<UpdateSection checkOnStart={false} onCheckOnStartChange={vi.fn()} />);
    const button = screen.getByRole('button', { name: 'Controlla aggiornamenti' });

    fireEvent.click(button);
    expect(await screen.findByText('Hai già l’ultima versione.')).toBeDefined();

    fireEvent.click(button);
    expect((await screen.findByRole('alert')).textContent).toContain('niente rete');
  });

  it('accende il controllo a ogni avvio', () => {
    const onChange = vi.fn();
    render(<UpdateSection checkOnStart={false} onCheckOnStartChange={onChange} />);

    fireEvent.click(screen.getByRole('switch', { name: 'Controlla a ogni avvio' }));

    expect(onChange).toHaveBeenCalledWith(true);
  });
});
