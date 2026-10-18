/**
 * Un finto mixer audio (`AudioContext`) per i test dei suoni.
 *
 * La pagina finta dei test (jsdom) non sa produrre suoni. Questo sostituto
 * offre solo i pezzi usati dai nostri suoni e ricorda quali generatori di
 * suono sono stati avviati, così i test possono verificare cosa sarebbe
 * stato suonato e quando.
 *
 * Uso: `vi.stubGlobal('AudioContext', FakeAudioContext)`, poi si leggono
 * `FakeAudioContext.instances` e `started` di ogni istanza.
 */

/** Un valore regolabile nel tempo (volume, tono): qui accetta e ignora i comandi. */
class FakeAudioParam {
  value = 0;
  setValueAtTime() {
    return this;
  }
  exponentialRampToValueAtTime() {
    return this;
  }
}

/** Un elemento della catena audio: si collega al successivo e lo restituisce. */
class FakeNode {
  connect<T>(next: T): T {
    return next;
  }
}

/** Un generatore di suono (nota o fruscio): ricorda quando viene avviato. */
class FakeSource extends FakeNode {
  constructor(
    private readonly owner: FakeAudioContext,
    readonly kind: 'oscillator' | 'buffer',
  ) {
    super();
  }
  buffer: unknown = null;
  type = '';
  frequency = new FakeAudioParam();
  start(at: number) {
    this.owner.started.push({ kind: this.kind, at });
  }
  stop() {
    return undefined;
  }
}

export class FakeAudioContext {
  /** Tutti i mixer creati durante il test. */
  static instances: FakeAudioContext[] = [];

  /** I generatori avviati: tipo e istante di partenza. */
  readonly started: { kind: 'oscillator' | 'buffer'; at: number }[] = [];
  /** Le riattivazioni richieste. */
  resumeCalls = 0;
  currentTime = 10;
  sampleRate = 8000;
  state: 'running' | 'suspended' = 'running';
  destination = new FakeNode();

  constructor() {
    FakeAudioContext.instances.push(this);
  }

  createGain() {
    return Object.assign(new FakeNode(), { gain: new FakeAudioParam() });
  }
  createBiquadFilter() {
    return Object.assign(new FakeNode(), {
      type: '',
      frequency: new FakeAudioParam(),
      Q: new FakeAudioParam(),
    });
  }
  createOscillator() {
    return new FakeSource(this, 'oscillator');
  }
  createBufferSource() {
    return new FakeSource(this, 'buffer');
  }
  /** Riceve (canali, lunghezza, frequenza di campionamento); conta solo la lunghezza. */
  createBuffer(...[, length]: [number, number, number]) {
    const samples = new Float32Array(length);
    return { getChannelData: () => samples };
  }
  resume() {
    this.resumeCalls += 1;
    this.state = 'running';
    return Promise.resolve();
  }
}
