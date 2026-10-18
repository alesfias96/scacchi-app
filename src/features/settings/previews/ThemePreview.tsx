/**
 * ThemePreview — una finestra in miniatura disegnata con i colori di un tema.
 *
 * Cosa fa: usa l'attributo `data-theme` su un piccolo riquadro; i fogli di
 * stile dei temi (src/styles/themes/) gli danno i colori e il font di quel
 * tema, quindi l'anteprima è sempre identica al tema vero.
 * Con il tema automatico mostra metà Giorno e metà Sera.
 *
 * Riceve:
 * - `theme`: il tema da mostrare.
 * Restituisce: l'anteprima.
 */
import type { AppTheme } from '../../../bindings/AppTheme';
import styles from './ThemePreview.module.css';

export interface ThemePreviewProps {
  /** Il tema da mostrare. */
  theme: AppTheme;
}

/** Una finestra in miniatura con barra laterale, fondo, testo e accento. */
function MiniWindow({ theme }: { theme: Exclude<AppTheme, 'auto'> }) {
  return (
    <span className={styles.window} data-theme={theme}>
      <span className={styles.sidebar} />
      <span className={styles.content}>
        <span className={styles.sample}>Aa</span>
        <span className={styles.accent} />
      </span>
    </span>
  );
}

export function ThemePreview({ theme }: ThemePreviewProps) {
  if (theme === 'auto') {
    return (
      <span className={styles.split}>
        <MiniWindow theme="giorno" />
        <MiniWindow theme="sera" />
      </span>
    );
  }
  return <MiniWindow theme={theme} />;
}
