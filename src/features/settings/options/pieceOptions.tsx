/**
 * Le scelte dello stile dei pezzi, con nome, descrizione e anteprima.
 *
 * L'elenco dei valori possibili è definito in Rust
 * (crates/app-settings/src/piece_style.rs); qui ci sono solo i testi e le anteprime.
 * Ogni anteprima mostra re bianco, donna nera e cavallo bianco.
 */
import type { PieceStyle } from '../../../bindings/PieceStyle';
import type { Option } from '../../../components/option-group/OptionGroup';
import { pieceImageUrl } from '../../../components/piece-icon/pieceImageUrl';
import { PiecePreview } from '../previews/PiecePreview';

/**
 * L'anteprima di uno stile: re bianco, donna nera e cavallo bianco.
 *
 * @param style - lo stile da mostrare.
 * @returns l'anteprima da mettere nel riquadro della scelta.
 */
function previewOf(style: PieceStyle) {
  return (
    <PiecePreview
      images={[pieceImageUrl(style, 'wK'), pieceImageUrl(style, 'bQ'), pieceImageUrl(style, 'wN')]}
    />
  );
}

export const PIECE_OPTIONS: readonly Option<PieceStyle>[] = [
  {
    value: 'icona',
    label: 'Icona',
    description: 'Forme geometriche, contorno netto.',
    preview: previewOf('icona'),
  },
  {
    value: 'morbido',
    label: 'Morbido',
    description: 'Senza contorno, con volume.',
    preview: previewOf('morbido'),
  },
  {
    value: 'legno',
    label: 'Legno',
    description: 'Bosso chiaro ed ebano.',
    preview: previewOf('legno'),
  },
  {
    value: 'metallo',
    label: 'Metallo',
    description: 'Argento e ottone.',
    preview: previewOf('metallo'),
  },
  {
    value: 'gettoni',
    label: 'Gettoni',
    description: 'Dischi con l’iniziale italiana.',
    preview: previewOf('gettoni'),
  },
  {
    value: 'classico',
    label: 'Classico',
    description: 'Il set tradizionale dei libri.',
    preview: previewOf('classico'),
  },
];
