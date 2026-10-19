/**
 * TutorSections — le parti della spiegazione: situazione, cosa succede, piano, principi.
 *
 * Cosa fa: mostra ogni parte con il suo titolo e le sue frasi. Passando il
 * mouse (o il focus) su una frase, la scacchiera del tutor mostra le case
 * coinvolte. Le frasi che hanno una scheda della base di conoscenza hanno il
 * pulsante «Approfondisci», che la apre.
 *
 * Riceve:
 * - `sections`: le parti della spiegazione;
 * - `notation`: la notazione scelta nelle Impostazioni;
 * - `onHighlight`: chiamata con la frase indicata, o `null`;
 * - `onOpenCard`: facoltativa, chiamata con il nome della scheda da aprire
 *   (senza, il pulsante «Approfondisci» non compare).
 * Restituisce: l'elenco delle parti.
 */
import type { Notation } from '../../../bindings/Notation';
import type { TutorItem } from '../../../bindings/TutorItem';
import type { TutorSection } from '../../../bindings/TutorSection';
import { formatFact } from '../../../lib/formatFact';
import styles from './TutorSections.module.css';

export interface TutorSectionsProps {
  sections: readonly TutorSection[];
  notation: Notation;
  onHighlight: (item: TutorItem | null) => void;
  onOpenCard?: (cardId: string) => void;
}

export function TutorSections({ sections, notation, onHighlight, onOpenCard }: TutorSectionsProps) {
  return (
    <div className={styles.sections}>
      {sections.map((section) => (
        <section key={section.title} className={styles.section} aria-label={section.title}>
          <h3 className={styles.title}>{section.title}</h3>
          <ul className={styles.list}>
            {section.items.map((item) => (
              <li
                key={item.fact.text + item.fact.moves.join(' ')}
                className={styles.item}
                onMouseEnter={() => {
                  onHighlight(item);
                }}
                onMouseLeave={() => {
                  onHighlight(null);
                }}
              >
                <span>{formatFact(item.fact, notation)}</span>
                {item.cardId !== null && onOpenCard !== undefined && (
                  <button
                    type="button"
                    className={styles.more}
                    onClick={() => {
                      if (item.cardId !== null) {
                        onOpenCard(item.cardId);
                      }
                    }}
                    onFocus={() => {
                      onHighlight(item);
                    }}
                    onBlur={() => {
                      onHighlight(null);
                    }}
                  >
                    Approfondisci
                  </button>
                )}
              </li>
            ))}
          </ul>
        </section>
      ))}
    </div>
  );
}
