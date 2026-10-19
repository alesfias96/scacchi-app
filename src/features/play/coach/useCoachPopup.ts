/**
 * useCoachPopup — quale messaggio del tutor è aperto e se il popup è chiuso.
 *
 * Cosa fa: a ogni nuova serie di messaggi (dopo una mossa) il popup si
 * riapre dal primo messaggio. Si può scorrere, chiudere e riaprire.
 *
 * @param messages - i messaggi della posizione mostrata.
 * @returns se il popup è aperto, il messaggio mostrato e le azioni.
 */
import { useState } from 'react';
import type { CoachMessage } from './buildCoachMessages';

/** Lo stato del popup per una serie di messaggi. */
interface PopupState {
  messages: readonly CoachMessage[];
  index: number;
  isClosed: boolean;
}

export function useCoachPopup(messages: readonly CoachMessage[]) {
  const [state, setState] = useState<PopupState>({ messages, index: 0, isClosed: false });
  // Una serie nuova riparte dal primo messaggio, aperta.
  const current = state.messages === messages ? state : { messages, index: 0, isClosed: false };

  return {
    isOpen: !current.isClosed && messages.length > 0,
    index: current.index,
    setIndex: (index: number) => {
      setState({ messages, index, isClosed: false });
    },
    close: () => {
      setState({ messages, index: current.index, isClosed: true });
    },
    reopen: () => {
      setState({ messages, index: 0, isClosed: false });
    },
  };
}
