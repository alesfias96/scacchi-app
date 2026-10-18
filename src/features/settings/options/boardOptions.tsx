/**
 * Le scelte dello stile della scacchiera, con nome, descrizione e anteprima.
 *
 * L'elenco dei valori possibili è definito in Rust
 * (crates/app-settings/src/board_style.rs); qui ci sono solo i testi e le anteprime.
 */
import ardesia from '../../../assets/boards/ardesia.jpg';
import acero from '../../../assets/boards/acero.jpg';
import marmo from '../../../assets/boards/marmo.jpg';
import noce from '../../../assets/boards/noce.jpg';
import rovere from '../../../assets/boards/rovere.jpg';
import torneo from '../../../assets/boards/torneo.jpg';
import type { BoardStyle } from '../../../bindings/BoardStyle';
import type { Option } from '../../../components/option-group/OptionGroup';
import { BoardPreview } from '../previews/BoardPreview';

export const BOARD_OPTIONS: readonly Option<BoardStyle>[] = [
  {
    value: 'rovere',
    label: 'Rovere',
    description: 'Legno intermedio, il più classico.',
    preview: <BoardPreview board="rovere" image={rovere} />,
  },
  {
    value: 'noce',
    label: 'Noce',
    description: 'Legno scuro e caldo.',
    preview: <BoardPreview board="noce" image={noce} />,
  },
  {
    value: 'acero',
    label: 'Acero',
    description: 'Legno chiaro e luminoso.',
    preview: <BoardPreview board="acero" image={acero} />,
  },
  {
    value: 'marmo',
    label: 'Marmo',
    description: 'Bianco venato e verde salvia.',
    preview: <BoardPreview board="marmo" image={marmo} />,
  },
  {
    value: 'ardesia',
    label: 'Ardesia',
    description: 'Grigio-blu opaco, riposante.',
    preview: <BoardPreview board="ardesia" image={ardesia} />,
  },
  {
    value: 'torneo',
    label: 'Torneo',
    description: 'Verde e crema, come in gara.',
    preview: <BoardPreview board="torneo" image={torneo} />,
  },
];
