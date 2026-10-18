/**
 * Le scelte dei suoni del gioco, con nome, descrizione e anteprima.
 *
 * L'elenco dei valori possibili è definito in Rust
 * (crates/app-settings/src/sound_setting.rs); qui ci sono solo i testi e le anteprime.
 */
import type { SoundSetting } from '../../../bindings/SoundSetting';
import type { Option } from '../../../components/option-group/OptionGroup';
import { SoundPreview } from '../previews/SoundPreview';

export const SOUND_OPTIONS: readonly Option<SoundSetting>[] = [
  {
    value: 'on',
    label: 'Attivi',
    description:
      'Un colpo di legno a ogni mossa, uno più pieno per le prese, una melodia a fine partita.',
    preview: <SoundPreview sounds="on" />,
  },
  {
    value: 'off',
    label: 'Disattivati',
    description: 'Si gioca in silenzio.',
    preview: <SoundPreview sounds="off" />,
  },
];
