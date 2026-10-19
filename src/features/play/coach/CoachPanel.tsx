/**
 * CoachPanel — la scheda «Tutor» nella colonna di destra (modalità Tutor).
 *
 * Cosa fa: l'interruttore «Giudizio di Stockfish» per i commenti, l'avviso
 * mentre il tutor guarda la posizione, il pulsante per rivedere i commenti
 * chiusi e, dopo «Approfondisci», la scheda della base di conoscenza aperta.
 *
 * Riceve:
 * - `withEngine`, `onWithEngineChange`: il giudizio di Stockfish acceso o spento;
 * - `isLoading`: `true` mentre il tutor prepara i commenti;
 * - `canReopen`, `onReopen`: per riaprire i commenti chiusi;
 * - `card`, `onCloseCard`: la scheda aperta (o `null`) e come chiuderla.
 * Restituisce: la scheda.
 */
import type { CardView } from '../../../bindings/CardView';
import { Button } from '../../../components/button/Button';
import { KnowledgeCards } from '../../../components/knowledge-cards/KnowledgeCards';
import { PanelCard } from '../../../components/panel-card/PanelCard';
import { Switch } from '../../../components/switch/Switch';
import styles from './CoachPanel.module.css';

export interface CoachPanelProps {
  withEngine: boolean;
  onWithEngineChange: (withEngine: boolean) => void;
  isLoading: boolean;
  canReopen: boolean;
  onReopen: () => void;
  card: CardView | null;
  onCloseCard: () => void;
}

export function CoachPanel({
  withEngine,
  onWithEngineChange,
  isLoading,
  canReopen,
  onReopen,
  card,
  onCloseCard,
}: CoachPanelProps) {
  return (
    <PanelCard title="Tutor">
      <p className={styles.message}>
        Dopo ogni mossa il tutor la commenta sulla scacchiera, un messaggio alla volta (Invio o
        Spazio per andare avanti, Esc per chiudere).
      </p>
      <Switch label="Giudizio di Stockfish" checked={withEngine} onChange={onWithEngineChange} />
      {isLoading && <p className={styles.message}>Il tutor sta guardando la posizione…</p>}
      {canReopen && <Button onClick={onReopen}>Rivedi i commenti</Button>}
      {card !== null && (
        <KnowledgeCards
          cards={[card]}
          openCardId={card.id}
          onToggle={(cardId) => {
            if (cardId === null) {
              onCloseCard();
            }
          }}
        />
      )}
    </PanelCard>
  );
}
