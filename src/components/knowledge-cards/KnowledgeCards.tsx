/**
 * KnowledgeCards — le schede della base di conoscenza che riguardano la posizione.
 *
 * Cosa fa: mostra ogni scheda (apertura, struttura pedonale, tema di
 * mediogioco, finale, principio) chiusa, con il titolo e il perché vale qui.
 * Aprendola si leggono di cosa si tratta, le idee, i piani per ciascun
 * giocatore, gli errori tipici e (per le schede di strategia) quando usarla,
 * quando no e i segnali che sta funzionando.
 *
 * Riceve:
 * - `cards`: le schede;
 * - `openCardId`: la scheda da tenere aperta (es. dopo «Approfondisci»), o `null`;
 * - `onToggle`: chiamata con il nome della scheda aperta, o `null` se chiusa;
 * - `onTrainTheme` (facoltativo): se c'è, le schede con un tema dei puzzle
 *   hanno il pulsante «Allenati su questo tema», che la chiama con il tema.
 * Restituisce: l'elenco delle schede.
 */
import type { CardView } from '../../bindings/CardView';
import type { Topic } from '../../bindings/Topic';
import { Button } from '../button/Button';
import styles from './KnowledgeCards.module.css';

export interface KnowledgeCardsProps {
  cards: readonly CardView[];
  openCardId: string | null;
  onToggle: (cardId: string | null) => void;
  onTrainTheme?: (theme: string) => void;
}

/** Un elenco con il suo titolo (niente, se l'elenco è vuoto). */
function CardList({ title, items }: { title: string; items: readonly string[] }) {
  if (items.length === 0) {
    return null;
  }
  return (
    <>
      <h4 className={styles.heading}>{title}</h4>
      <ul className={styles.list}>
        {items.map((item) => (
          <li key={item}>{item}</li>
        ))}
      </ul>
    </>
  );
}

/** Il nome di ogni argomento, sopra il titolo della scheda. */
const TOPIC_NAMES: Record<Topic, string> = {
  opening: 'Apertura',
  structure: 'Struttura pedonale',
  middlegame: 'Mediogioco',
  endgame: 'Finale',
  principle: 'Principio',
  tactic: 'Tattica',
};

export function KnowledgeCards({ cards, openCardId, onToggle, onTrainTheme }: KnowledgeCardsProps) {
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
            <CardList title="Quando usarlo" items={card.useWhen} />
            <CardList title="Quando no" items={card.avoidWhen} />
            <CardList title="Segnali che funziona" items={card.signs} />
            <CardList title="Errori da evitare" items={card.mistakes} />
            {card.puzzleTheme !== null && onTrainTheme !== undefined && (
              <div>
                <Button
                  onClick={() => {
                    if (card.puzzleTheme !== null) {
                      onTrainTheme(card.puzzleTheme);
                    }
                  }}
                >
                  Allenati su questo tema
                </Button>
              </div>
            )}
          </div>
        </details>
      ))}
    </div>
  );
}
