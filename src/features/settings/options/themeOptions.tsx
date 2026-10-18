/**
 * Le scelte del tema dell'app, con nome, descrizione e anteprima.
 *
 * L'elenco dei valori possibili è definito in Rust
 * (crates/app-settings/src/app_theme.rs); qui ci sono solo i testi e le anteprime.
 */
import type { AppTheme } from '../../../bindings/AppTheme';
import type { Option } from '../../../components/option-group/OptionGroup';
import { ThemePreview } from '../previews/ThemePreview';

export const THEME_OPTIONS: readonly Option<AppTheme>[] = [
  {
    value: 'studio',
    label: 'Studio',
    description: 'Barra blu inchiostro, dettagli arancioni.',
    preview: <ThemePreview theme="studio" />,
  },
  {
    value: 'sera',
    label: 'Sera',
    description: 'Scuro e raccolto, dettagli ambra.',
    preview: <ThemePreview theme="sera" />,
  },
  {
    value: 'giorno',
    label: 'Giorno',
    description: 'Chiaro e arioso, dettagli verdi.',
    preview: <ThemePreview theme="giorno" />,
  },
  {
    value: 'auto',
    label: 'Automatico',
    description: 'Giorno o Sera, come il sistema operativo.',
    preview: <ThemePreview theme="auto" />,
  },
];
