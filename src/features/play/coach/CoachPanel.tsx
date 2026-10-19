/**
 * CoachPanel — la scheda «Tutor» nella colonna di destra (modalità Tutor).
 *
 * Cosa fa: spiega come scorrere i commenti, ha l'interruttore «Giudizio di
 * Stockfish» e, dopo «Approfondisci», mostra la scheda della base di
 * conoscenza aperta. I commenti stessi stanno nella fascia sotto la scacchiera.
 *
 * Riceve:
 * - `withEngine`, `onWithEngineChange`: il giudizio di Stockfish acceso o spento;
 * - `card`, `onCloseCard`: la scheda aperta (o `null`) e come chiuderla.
 * Restituisce: la scheda.
 */
import type { CardView } from '../../../bindings/CardView';
import { KnowledgeCards } from '../../../components/knowledge-cards/KnowledgeCards';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { Switch } from '../../../components/switch/Switch';
import { useTrainTheme } from '../../training/useTrainTheme';
import styles from './CoachPanel.module.css';

export interface CoachPanelProps {
  withEngine: boolean;
  onWithEngineChange: (withEngine: boolean) => void;
  card: CardView | null;
  onCloseCard: () => void;
}

export function CoachPanel({ withEngine, onWithEngineChange, card, onCloseCard }: CoachPanelProps) {
  const trainTheme = useTrainTheme();
  return (
    <PanelCard title="Tutor">
      <p className={styles.message}>
        Dopo ogni mossa il tutor la commenta sotto la scacchiera, un messaggio alla volta (Invio o
        Spazio per andare avanti, Esc per chiudere).
      </p>
      <Switch label="Giudizio di Stockfish" checked={withEngine} onChange={onWithEngineChange} />
      {card !== null && (
        <KnowledgeCards
          cards={[card]}
          openCardId={card.id}
          onToggle={(cardId) => {
            if (cardId === null) {
              onCloseCard();
            }
          }}
          onTrainTheme={trainTheme}
        />
      )}
    </PanelCard>
  );
}
