/**
 * SettingsPage — la sezione "Impostazioni".
 *
 * Cosa fa: permette di scegliere il tema dell'app, lo stile della scacchiera,
 * lo stile dei pezzi, la notazione delle mosse e i suoni. Ogni scelta si applica
 * subito a tutta l'app, viene salvata e ritrovata alla riapertura. Accanto,
 * una scacchiera di anteprima mostra il risultato.
 *
 * Riceve: niente.
 * Restituisce: la pagina della sezione.
 */
import type { Settings } from '../../bindings/Settings';
import { Chessboard } from '../../components/chessboard/Chessboard';
import { OptionGroup } from '../../components/option-group/OptionGroup';
import { useInitialPosition } from '../../hooks/useInitialPosition';
import { useLoadedSettings, useSettingsStore } from '../../state/settingsStore';
import { playMoveSound } from '../../sounds/playMoveSound';
import { BOARD_OPTIONS } from './options/boardOptions';
import { NOTATION_OPTIONS } from './options/notationOptions';
import { PIECE_OPTIONS } from './options/pieceOptions';
import { SOUND_OPTIONS } from './options/soundOptions';
import { THEME_OPTIONS } from './options/themeOptions';
import styles from './SettingsPage.module.css';

export function SettingsPage() {
  const settings = useLoadedSettings();
  const update = useSettingsStore((state) => state.update);
  const saveError = useSettingsStore((state) => state.saveError);
  const position = useInitialPosition();

  /** Applica e salva una scelta. */
  const change = (changes: Partial<Settings>) => {
    void update(changes);
  };

  return (
    <section className={styles.page}>
      <header className={styles.header}>
        <h1 className={styles.title}>Impostazioni</h1>
        <p className={styles.subtitle}>
          Le scelte valgono in tutta l’app e vengono ricordate alla prossima apertura.
        </p>
      </header>

      {saveError !== null && (
        <p className={styles.error} role="alert">
          Non sono riuscito a salvare la scelta: {saveError}
        </p>
      )}

      <div className={styles.layout}>
        <div className={styles.groups}>
          <OptionGroup
            legend="Tema dell’app"
            name="theme"
            options={THEME_OPTIONS}
            value={settings.theme}
            onChange={(theme) => {
              change({ theme });
            }}
          />
          <OptionGroup
            legend="Scacchiera"
            name="board"
            options={BOARD_OPTIONS}
            value={settings.board}
            onChange={(board) => {
              change({ board });
            }}
          />
          <OptionGroup
            legend="Pezzi"
            name="pieces"
            options={PIECE_OPTIONS}
            value={settings.pieces}
            onChange={(pieces) => {
              change({ pieces });
            }}
          />
          <OptionGroup
            legend="Notazione delle mosse"
            name="notation"
            options={NOTATION_OPTIONS}
            value={settings.notation}
            onChange={(notation) => {
              change({ notation });
            }}
          />
          <OptionGroup
            legend="Suoni"
            name="sounds"
            options={SOUND_OPTIONS}
            value={settings.sounds}
            onChange={(sounds) => {
              change({ sounds });
              // Chi riaccende i suoni sente subito come suona una mossa.
              if (sounds === 'on') {
                playMoveSound();
              }
            }}
          />
        </div>

        <aside className={styles.preview} aria-label="Anteprima">
          <p className={styles.previewTitle}>Anteprima</p>
          {position.status === 'ready' && (
            <Chessboard
              boardFen={position.position.boardFen}
              turn={position.position.turn}
              inCheck={position.position.inCheck}
            />
          )}
        </aside>
      </div>
    </section>
  );
}
