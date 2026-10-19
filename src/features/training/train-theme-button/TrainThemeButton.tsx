/**
 * TrainThemeButton — il pulsante «Allenati: …» per i puzzle di un motivo tattico.
 *
 * Cosa fa: un clic sceglie il tema nell'allenamento, comincia un puzzle di
 * quel tema e apre la sezione Allenamento (vedi `useTrainTheme`).
 *
 * Riceve:
 * - `theme`: il tema dei puzzle di Lichess (es. `"fork"`);
 * - `name`: il nome del motivo in italiano (es. "Forchetta").
 * Restituisce: il pulsante.
 */
import { Button } from '../../../components/button/Button';
import { useTrainTheme } from '../useTrainTheme';

export interface TrainThemeButtonProps {
  theme: string;
  name: string;
}

export function TrainThemeButton({ theme, name }: TrainThemeButtonProps) {
  const train = useTrainTheme();
  return (
    <Button
      onClick={() => {
        train(theme);
      }}
    >
      Allenati: {name}
    </Button>
  );
}
