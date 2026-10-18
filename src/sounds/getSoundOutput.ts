/**
 * L'uscita audio comune a tutti i suoni dell'app.
 *
 * I suoni non sono file registrati: vengono "costruiti" al momento con la
 * Web Audio API del motore web (presente su Windows, macOS e Linux). Serve un
 * solo `AudioContext` (il "mixer" audio) per tutta l'app: lo si crea al primo
 * suono, che parte sempre da un gesto dell'utente (una mossa, un tasto), come
 * richiedono i motori web prima di poter suonare.
 *
 * Tutti i suoni passano da un unico volume generale, così nessuna
 * combinazione di suoni supera il livello massimo e "gracchia".
 */

/** Il volume generale dei suoni (1 = pieno). */
const MASTER_LEVEL = 0.6;

/** Il mixer audio e il punto a cui collegare i suoni. */
export interface SoundOutput {
  /** Il mixer audio: dà l'orologio (`currentTime`) e crea i generatori di suono. */
  context: AudioContext;
  /** Dove collegare ogni suono: passa dal volume generale e arriva agli altoparlanti. */
  destination: AudioNode;
}

/** L'uscita creata al primo suono, riusata in seguito. */
let output: SoundOutput | null = null;

/**
 * @returns l'uscita audio pronta all'uso, oppure `null` se il motore web non
 * sa produrre suoni (ad esempio nei test): in quel caso i suoni non partono,
 * senza errori.
 */
export function getSoundOutput(): SoundOutput | null {
  if (output === null) {
    if (typeof AudioContext === 'undefined') {
      return null;
    }
    const context = new AudioContext();
    const master = context.createGain();
    master.gain.value = MASTER_LEVEL;
    master.connect(context.destination);
    output = { context, destination: master };
  }
  // Il motore web può "sospendere" il mixer (ad esempio quando la finestra non
  // è in primo piano): lo si riattiva prima di ogni suono.
  if (output.context.state === 'suspended') {
    void output.context.resume();
  }
  return output;
}
