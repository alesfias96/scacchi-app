/**
 * NewGameDialog — la finestra "Nuova partita".
 *
 * Cosa fa: chiede come si vuole giocare e avvia la partita.
 * - **Avversario**: il computer (Stockfish), il computer "umano" (Maia, che gioca
 *   come una persona di quel livello, errori tipici compresi) oppure due persone
 *   sulla stessa scacchiera.
 * - Contro il computer: il **colore** (bianco, nero o a sorte) e il **livello**:
 *   per Stockfish da Principiante a Sovrumano, per Maia da 1100 a 1900, con la
 *   forza indicativa in punti Elo. Ogni motore ricorda il suo livello.
 * - Il **tempo**: senza orologio, uno dei tempi più comuni (Bullet, Blitz,
 *   Rapid, Classica) oppure "Personalizzato", con minuti e secondi a scelta.
 * Si apre con le scelte dell'ultima partita. Esc o "Annulla" chiudono senza
 * cambiare nulla.
 *
 * Riceve:
 * - `initialSetup`: le scelte proposte all'apertura (quelle dell'ultima partita);
 * - `onStart`: chiamata con le scelte fatte, quando si preme "Inizia";
 * - `onClose`: chiude la finestra.
 * Restituisce: la finestra.
 */
import { useEffect, useId, useState, type SyntheticEvent } from 'react';
import type { ColorChoice } from '../../../bindings/ColorChoice';
import type { GameSetup } from '../../../bindings/GameSetup';
import type { ComputerEngine } from '../../../bindings/ComputerEngine';
import type { TimeControl } from '../../../bindings/TimeControl';
import { Button } from '../../../components/button/Button';
import { ChoiceGroup, type Choice } from '../../../components/choice-group/ChoiceGroup';
import { useEngineLevels } from '../../../hooks/useEngineLevels';
import styles from './NewGameDialog.module.css';
import { CUSTOM_PRESET_ID, TIME_PRESETS, presetIdFor } from './timePresets';

export interface NewGameDialogProps {
  /** Le scelte proposte all'apertura. */
  initialSetup: GameSetup;
  /** Avvia la partita con le scelte fatte. */
  onStart: (setup: GameSetup) => void;
  /** Chiude la finestra. */
  onClose: () => void;
}

/** L'avversario come lo si sceglie: uno dei due motori, oppure due persone. */
type OpponentChoice = ComputerEngine | 'twoPlayers';

/** Le scelte dell'avversario. */
const OPPONENT_CHOICES: readonly Choice<OpponentChoice>[] = [
  { value: 'stockfish', label: 'Computer', detail: 'Stockfish' },
  { value: 'maia', label: 'Umano', detail: 'Maia: gioca come una persona' },
  { value: 'twoPlayers', label: 'Due giocatori', detail: 'sulla stessa scacchiera' },
];

/** Le scelte del colore contro il computer. */
const COLOR_CHOICES: readonly Choice<ColorChoice>[] = [
  { value: 'white', label: 'Bianco', detail: 'muovi per primo' },
  { value: 'black', label: 'Nero', detail: 'muove il computer' },
  { value: 'random', label: 'A sorte' },
];

/** Le scelte del tempo: quelle proposte più "Personalizzato". */
const TIME_CHOICES: readonly Choice<string>[] = [
  ...TIME_PRESETS.map((preset) => ({
    value: preset.id,
    label: preset.id === 'none' ? 'Senza orologio' : preset.id,
    detail: preset.category,
  })),
  { value: CUSTOM_PRESET_ID, label: 'Personalizzato', detail: 'a tua scelta' },
];

/** Il tempo proposto per "Personalizzato", se l'ultima partita non ne aveva uno. */
const DEFAULT_CUSTOM: TimeControl = { initialSeconds: 15 * 60, incrementSeconds: 0 };
/** I limiti del tempo personalizzato. */
const MAX_MINUTES = 180;
const MAX_INCREMENT = 60;

export function NewGameDialog({ initialSetup, onStart, onClose }: NewGameDialogProps) {
  const [opponent, setOpponent] = useState<OpponentChoice>(
    initialSetup.opponent === 'computer' ? initialSetup.engine : 'twoPlayers',
  );
  const [color, setColor] = useState(initialSetup.color);
  const [level, setLevel] = useState(initialSetup.level);
  const [maiaLevel, setMaiaLevel] = useState(initialSetup.maiaLevel);
  /** Il motore scelto (se si gioca a due conta l'ultimo usato, per la prossima volta). */
  const engine: ComputerEngine = opponent === 'twoPlayers' ? initialSetup.engine : opponent;
  const levels = useEngineLevels(engine);
  const isMaia = engine === 'maia';
  const [presetId, setPresetId] = useState(() => presetIdFor(initialSetup.timeControl));
  const [custom, setCustom] = useState(initialSetup.timeControl ?? DEFAULT_CUSTOM);
  const titleId = useId();

  // Esc chiude la finestra.
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [onClose]);

  /** Il tempo scelto, come lo vuole Rust. */
  const timeControl =
    presetId === CUSTOM_PRESET_ID
      ? custom
      : (TIME_PRESETS.find((preset) => preset.id === presetId)?.timeControl ?? null);

  const handleSubmit = (event: SyntheticEvent<HTMLFormElement>) => {
    event.preventDefault();
    onStart({
      opponent: opponent === 'twoPlayers' ? 'twoPlayers' : 'computer',
      color,
      engine,
      level,
      maiaLevel,
      timeControl,
    });
  };

  return (
    <div className={styles.backdrop}>
      <form
        className={styles.dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        onSubmit={handleSubmit}
      >
        <h2 id={titleId} className={styles.title}>
          Nuova partita
        </h2>

        <div className={styles.sections}>
          <ChoiceGroup
            legend="Avversario"
            name="opponent"
            options={OPPONENT_CHOICES}
            value={opponent}
            onChange={setOpponent}
          />

          {opponent !== 'twoPlayers' && (
            <>
              <ChoiceGroup
                legend="Il tuo colore"
                name="color"
                options={COLOR_CHOICES}
                value={color}
                onChange={setColor}
              />
              {levels.length > 0 && (
                <ChoiceGroup
                  legend={isMaia ? 'Livello di Maia' : 'Livello del computer'}
                  name="level"
                  options={levels.map((info) => ({
                    value: String(info.level),
                    // Per Maia il nome è già l'Elo ("Maia 1500"): basta il numero.
                    label: isMaia ? String(info.elo) : info.name,
                    detail: isMaia ? 'Elo, come una persona' : `~${String(info.elo)} Elo`,
                  }))}
                  value={String(isMaia ? maiaLevel : level)}
                  onChange={(value) => {
                    if (isMaia) {
                      setMaiaLevel(Number(value));
                    } else {
                      setLevel(Number(value));
                    }
                  }}
                />
              )}
            </>
          )}

          <ChoiceGroup
            legend="Tempo"
            name="time"
            options={TIME_CHOICES}
            value={presetId}
            onChange={setPresetId}
          />

          {presetId === CUSTOM_PRESET_ID && (
            <div className={styles.custom}>
              <label className={styles.field}>
                <span>Minuti a testa</span>
                <input
                  className={styles.number}
                  type="number"
                  min={1}
                  max={MAX_MINUTES}
                  value={Math.round(custom.initialSeconds / 60)}
                  onChange={(event) => {
                    const value = Math.min(MAX_MINUTES, Math.max(1, Number(event.target.value)));
                    setCustom({ ...custom, initialSeconds: value * 60 });
                  }}
                />
              </label>
              <label className={styles.field}>
                <span>Secondi in più a mossa</span>
                <input
                  className={styles.number}
                  type="number"
                  min={0}
                  max={MAX_INCREMENT}
                  value={custom.incrementSeconds}
                  onChange={(event) => {
                    const value = Math.min(MAX_INCREMENT, Math.max(0, Number(event.target.value)));
                    setCustom({ ...custom, incrementSeconds: value });
                  }}
                />
              </label>
            </div>
          )}
        </div>

        <div className={styles.actions}>
          <Button onClick={onClose}>Annulla</Button>
          <Button type="submit" variant="primary">
            Inizia la partita
          </Button>
        </div>
      </form>
    </div>
  );
}
