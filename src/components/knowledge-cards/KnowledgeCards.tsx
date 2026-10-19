/**
 * KnowledgeCards — le schede della base di conoscenza che riguardano la posizione.
 *
 * Cosa fa: mostra ogni scheda (apertura, struttura pedonale, tema di
 * mediogioco, finale, principio) chiusa, con il titolo e il perché vale qui.
 * Aprendola si leggono di cosa si tratta, le idee, i piani per ciascun
 * giocatore e gli errori tipici.
 *
 * Riceve:
 * - `cards`: le schede;
 * - `openCardId`: la scheda da tenere aperta (es. dopo «Approfondisci»), o `null`;
 * - `onToggle`: chiamata con il nome della scheda aperta, o `null` se chiusa.
 * Restituisce: l'elenco delle schede.
 */
import type { CardView } from '../../bindings/CardView';
import type { Topic } from '../../bindings/Topic';
import styles from './KnowledgeCards.module.css';

export interface KnowledgeCardsProps {
  cards: readonly CardView[];
  openCardId: string | null;
  onToggle: (cardId: string | null) => void;
}

/** Il nome di ogni argomento, sopra il titolo della scheda. */
const TOPIC_NAMES: Record<Topic, string> = {
  opening: 'Apertura',
  structure: 'Struttura pedonale',
  middlegame: 'Mediogioco',
  endgame: 'Finale',
  principle: 'Principio',
};

export function KnowledgeCards({ cards, openCardId, onToggle }: KnowledgeCardsProps) {
  return (
    <div className={styles.cards}>
      {cards.map((card) => (
        <details
          key={card.id}
          id={`tutor-card-${card.id}`}
          className={styles.card}
          open={card.id === openCardId}
          onToggle={(event) => {
            const isOpen = event.currentTarget.open;
            if (isOpen !== (card.id === openCardId)) {
              onToggle(isOpen ? card.id : null);
            }
          }}
        >
          <summary className={styles.summary}>
            <span className={styles.topic}>{TOPIC_NAMES[card.topic]}</span>
            <span className={styles.title}>{card.title}</span>
            <span className={styles.context}>{card.context}</span>
          </summary>
          <div className={styles.body}>
            <p>{card.summary}</p>
            <h4 className={styles.heading}>Le idee</h4>
            <ul className={styles.list}>
              {card.ideas.map((idea) => (
                <li key={idea}>{idea}</li>
              ))}
            </ul>
            {card.plans.map((plan) => (
              <div key={plan.label}>
                <h4 className={styles.heading}>Piano: {plan.label}</h4>
                <ol className={styles.list}>
                  {plan.steps.map((step) => (
                    <li key={step}>{step}</li>
                  ))}
                </ol>
              </div>
            ))}
            {card.mistakes.length > 0 && (
              <>
                <h4 className={styles.heading}>Errori da evitare</h4>
                <ul className={styles.list}>
                  {card.mistakes.map((mistake) => (
                    <li key={mistake}>{mistake}</li>
                  ))}
                </ul>
              </>
            )}
          </div>
        </details>
      ))}
    </div>
  );
}
