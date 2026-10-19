/**
 * ThinkingSection — «Prima di muovere»: il metodo di pensiero, con la posizione.
 *
 * Cosa fa: mostra i quattro passi del metodo, sempre nello stesso ordine (i
 * tuoi scacchi, le tue catture, cosa minaccia l'avversario, il controllo
 * finale), ognuno con cosa dice in questa posizione e le sue mosse nella
 * notazione scelta. Sotto, la scheda del metodo da aprire.
 *
 * Riceve:
 * - `thinking`: il metodo riempito da Rust;
 * - `notation`: la notazione scelta nelle Impostazioni.
 * Restituisce: la sezione.
 */
import { useState } from 'react';
import type { Notation } from '../../../bindings/Notation';
import type { ThinkingView } from '../../../bindings/ThinkingView';
import { KnowledgeCards } from '../../../components/knowledge-cards/KnowledgeCards';
import { formatSan } from '../../../lib/formatSan';
import styles from './ThinkingSection.module.css';

export interface ThinkingSectionProps {
  thinking: ThinkingView;
  notation: Notation;
}

export function ThinkingSection({ thinking, notation }: ThinkingSectionProps) {
  /** La scheda del metodo aperta. */
  const [openCardId, setOpenCardId] = useState<string | null>(null);

  return (
    <section className={styles.section} aria-label="Prima di muovere">
      <h3 className={styles.title}>Prima di muovere</h3>
      <ol className={styles.steps}>
        {thinking.steps.map((step) => (
          <li key={step.title} className={styles.step}>
            <span className={styles.name}>{step.title}</span>
            <span className={styles.text}>{step.text}</span>
            {step.moves.length > 0 && (
              <span className={styles.moves}>
                {step.moves.map((san) => formatSan(san, notation)).join(', ')}
              </span>
            )}
          </li>
        ))}
      </ol>
      {thinking.card !== null && (
        <KnowledgeCards cards={[thinking.card]} openCardId={openCardId} onToggle={setOpenCardId} />
      )}
    </section>
  );
}
