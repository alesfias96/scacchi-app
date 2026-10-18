/**
 * Un breve "fruscio" (rumore bianco), materia prima del colpo dei pezzi.
 *
 * Il rumore bianco contiene tutte le frequenze; filtrandolo si ottiene il
 * "clic" secco del legno che tocca il legno. Si calcola una volta sola per
 * ogni mixer e poi si riusa.
 */

/** La durata del fruscio, in secondi: basta per il colpo più lungo. */
const NOISE_SECONDS = 0.1;

/** Il fruscio già calcolato per ogni mixer. */
const cache = new WeakMap<BaseAudioContext, AudioBuffer>();

/**
 * @param context - il mixer audio che userà il fruscio.
 * @returns un frammento audio di rumore bianco, lungo `NOISE_SECONDS`.
 */
export function getNoiseBuffer(context: BaseAudioContext): AudioBuffer {
  const cached = cache.get(context);
  if (cached !== undefined) {
    return cached;
  }
  const buffer = context.createBuffer(
    1,
    Math.ceil(context.sampleRate * NOISE_SECONDS),
    context.sampleRate,
  );
  const samples = buffer.getChannelData(0);
  for (let index = 0; index < samples.length; index += 1) {
    samples[index] = Math.random() * 2 - 1;
  }
  cache.set(context, buffer);
  return buffer;
}
