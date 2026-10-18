/**
 * Le scelte della notazione delle mosse, con nome, descrizione e anteprima.
 *
 * L'elenco dei valori possibili è definito in Rust
 * (crates/app-settings/src/notation.rs); qui ci sono solo i testi e le anteprime.
 */
import type { Notation } from '../../../bindings/Notation';
import type { Option } from '../../../components/option-group/OptionGroup';
import { NotationPreview } from '../previews/NotationPreview';

export const NOTATION_OPTIONS: readonly Option<Notation>[] = [
  {
    value: 'figurine',
    label: 'Figurine',
    description: 'Il disegno del pezzo: si legge in ogni lingua.',
    preview: <NotationPreview notation="figurine" />,
  },
  {
    value: 'italiana',
    label: 'Italiana',
    description: 'R re, D donna, T torre, A alfiere, C cavallo.',
    preview: <NotationPreview notation="italiana" />,
  },
  {
    value: 'internazionale',
    label: 'Internazionale',
    description: 'Iniziali inglesi, come su libri e siti stranieri.',
    preview: <NotationPreview notation="internazionale" />,
  },
];
