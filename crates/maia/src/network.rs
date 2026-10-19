//! Il calcolo della rete: dalla posizione codificata ai valori di ogni mossa.
//!
//! Ogni "canale" è una scacchiera di 64 numeri (riga per riga, da a1 a h8).
//! Una **convoluzione 3×3** calcola ogni casa di ogni canale in uscita
//! guardando la stessa casa e le 8 vicine in tutti i canali in entrata, con
//! i pesi imparati (fuori dalla scacchiera vale 0). Dopo quasi ogni passo si
//! applica **`ReLU`**: i numeri negativi diventano 0.
//!
//! Il percorso, uguale a quello di Lc0:
//! 1. ingresso: convoluzione 3×3 + `ReLU`;
//! 2. per ogni blocco residuo: convoluzione + `ReLU`, convoluzione, poi
//!    l'unità SE (media di ogni canale → due piccoli strati → per ogni canale
//!    una scala tra 0 e 1 e uno spostamento), più l'ingresso del blocco, + `ReLU`;
//! 3. testa delle mosse: convoluzione + `ReLU`, convoluzione: 73 piani × 64 case;
//!    la tabella di Lc0 dice a quale delle 1858 mosse corrisponde ogni valore.

use crate::input_planes::{InputPlanes, PLANES};
use crate::net_weights::{Conv, NetWeights, SeUnit};
use crate::policy_tables::CONV_POLICY_MAP;

/// Le case della scacchiera.
const SQUARES: usize = 64;
/// I piani della testa delle mosse che hanno un significato (gli altri sono riempitivo).
pub const POLICY_PLANES: usize = 73;
/// Quante mosse conosce la rete.
pub const POLICY_SIZE: usize = 1858;

/// Una rete pronta per il calcolo.
#[derive(Debug, Clone, PartialEq)]
pub struct Network {
    weights: NetWeights,
}

/// `ReLU`: i numeri negativi diventano 0.
fn relu(values: &mut [f32]) {
    for value in values {
        *value = value.max(0.0);
    }
}

/// La funzione "sigmoide": porta qualsiasi numero tra 0 e 1.
fn sigmoid(value: f32) -> f32 {
    1.0 / (1.0 + (-value).exp())
}

/// Una convoluzione 3×3 con spostamento: `input` ha `conv.inputs` canali di 64 case.
fn convolve(conv: &Conv, input: &[f32]) -> Vec<f32> {
    let mut output = vec![0.0; conv.outputs * SQUARES];
    for (out_channel, out_plane) in output.as_chunks_mut::<SQUARES>().0.iter_mut().enumerate() {
        out_plane.fill(conv.biases[out_channel]);
        for (in_channel, in_plane) in input.as_chunks::<SQUARES>().0.iter().enumerate() {
            let start = (out_channel * conv.inputs + in_channel) * 9;
            let filter = &conv.weights[start..start + 9];
            for row in 0..8 {
                for column in 0..8 {
                    let mut sum = 0.0;
                    for (index, weight) in filter.iter().enumerate() {
                        // Il filtro va dalla riga e colonna precedenti alle successive.
                        let (Some(r), Some(c)) = (
                            (row + index / 3).checked_sub(1),
                            (column + index % 3).checked_sub(1),
                        ) else {
                            continue;
                        };
                        if r < 8 && c < 8 {
                            sum += weight * in_plane[r * 8 + c];
                        }
                    }
                    out_plane[row * 8 + column] += sum;
                }
            }
        }
    }
    output
}

/// Uno strato "denso": ogni uscita è la somma pesata di tutti gli ingressi, più lo spostamento.
fn dense(weights: &[f32], biases: &[f32], input: &[f32]) -> Vec<f32> {
    weights
        .chunks_exact(input.len())
        .zip(biases)
        .map(|(row, bias)| row.iter().zip(input).map(|(w, x)| w * x).sum::<f32>() + bias)
        .collect()
}

/// L'unità SE applicata all'uscita della seconda convoluzione, più l'ingresso del blocco, + `ReLU`.
fn squeeze_excite(se: &SeUnit, convolved: &[f32], residual: &[f32]) -> Vec<f32> {
    #[allow(clippy::cast_precision_loss)] // 64 è rappresentato esattamente.
    let squares = SQUARES as f32;
    let means: Vec<f32> = convolved
        .as_chunks::<SQUARES>()
        .0
        .iter()
        .map(|plane| plane.iter().sum::<f32>() / squares)
        .collect();
    let mut hidden = dense(&se.w1, &se.b1, &means);
    relu(&mut hidden);
    let gates = dense(&se.w2, &se.b2, &hidden);
    let mut output = Vec::with_capacity(convolved.len());
    for (channel, (plane, skip)) in convolved
        .as_chunks::<SQUARES>()
        .0
        .iter()
        .zip(residual.as_chunks::<SQUARES>().0.iter())
        .enumerate()
    {
        let scale = sigmoid(gates[channel]);
        let shift = gates[se.channels + channel];
        output.extend(
            plane
                .iter()
                .zip(skip)
                .map(|(value, skip)| (value * scale + shift + skip).max(0.0)),
        );
    }
    output
}

impl Network {
    /// Prepara la rete con i pesi letti dal file.
    #[must_use]
    pub fn new(weights: NetWeights) -> Self {
        Self { weights }
    }

    /// I valori grezzi ("logit") di tutte le 1858 mosse, nell'ordine di [`MOVES`](crate::policy_tables::MOVES).
    ///
    /// Più alto il valore, più la mossa è probabile; per avere le probabilità
    /// delle sole mosse legali li si passa alla "softmax" (vedi `move_choice`).
    #[must_use]
    pub fn policy(&self, planes: &InputPlanes) -> Vec<f32> {
        debug_assert_eq!(planes.len(), PLANES * SQUARES);
        let mut flow = convolve(&self.weights.input, planes);
        relu(&mut flow);
        for block in &self.weights.residuals {
            let mut middle = convolve(&block.conv1, &flow);
            relu(&mut middle);
            let convolved = convolve(&block.conv2, &middle);
            flow = squeeze_excite(&block.se, &convolved, &flow);
        }
        let mut head = convolve(&self.weights.policy1, &flow);
        relu(&mut head);
        let raw = convolve(&self.weights.policy, &head);

        let mut logits = vec![f32::NEG_INFINITY; POLICY_SIZE];
        for (value, target) in raw.iter().zip(CONV_POLICY_MAP.iter()) {
            if let Ok(index) = usize::try_from(*target) {
                logits[index] = *value;
            }
        }
        logits
    }
}

#[cfg(test)]
mod tests {
    use super::POLICY_SIZE;
    use crate::policy_tables::MOVES;

    #[test]
    fn the_policy_has_one_value_per_known_move() {
        assert_eq!(POLICY_SIZE, MOVES.len());
    }
}
