//! I pesi di una rete Maia, letti dal file nel formato di Lc0.
//!
//! La rete è fatta così (è una rete "convoluzionale", come quelle che
//! riconoscono le immagini, ma l'immagine è la scacchiera 8×8):
//! - un **ingresso**: da 112 "piani" (dove sono i pezzi, arrocchi, turno…) a 64 canali;
//! - **6 blocchi residui**: due convoluzioni ciascuno, più un'unità "SE" che
//!   decide quanto contano i canali in quella posizione;
//! - una **testa delle mosse**: due convoluzioni che danno 73 piani di 8×8 valori,
//!   uno per ogni modo di muovere da ogni casa.
//!
//! Nel file ogni numero è salvato su 2 byte (formato `LINEAR16`): 0 vale il
//! minimo dello strato, 65535 il massimo, in mezzo si va in proporzione.
//!
//! Ogni convoluzione è seguita da una "normalizzazione" (media, varianza,
//! scala e spostamento). Come fa Lc0, qui la si incorpora subito nei pesi e
//! nello spostamento della convoluzione: il risultato è identico e il calcolo
//! della rete diventa più semplice.

use crate::maia_error::MaiaError;
use crate::proto_reader::{Field, find, read_fields};

/// Il numero magico all'inizio di ogni rete di Lc0.
const MAGIC: u32 = 0x1c0;
/// Il piccolo numero che si aggiunge alla varianza per non dividere per zero (come Lc0).
const EPSILON: f32 = 1e-5;
/// Il formato dell'ingresso supportato: i 112 piani "classici".
const INPUT_CLASSICAL_112_PLANE: u64 = 1;
/// La testa delle mosse supportata: quella "a convoluzione".
const POLICY_CONVOLUTION: u64 = 2;
/// La struttura supportata: blocchi residui con unità SE.
const NETWORK_SE_WITH_HEADFORMAT: u64 = 4;
/// La codifica dei numeri supportata (2 byte per numero).
const LINEAR16: u64 = 1;

/// Una convoluzione con la sua normalizzazione già incorporata.
#[derive(Debug, Clone, PartialEq)]
pub struct Conv {
    /// Quanti canali escono.
    pub outputs: usize,
    /// Quanti canali entrano.
    pub inputs: usize,
    /// I pesi: per ogni uscita, per ogni ingresso, i 9 numeri del filtro 3×3
    /// (riga per riga), oppure 1 numero per un filtro 1×1.
    pub weights: Vec<f32>,
    /// Lo spostamento di ogni uscita.
    pub biases: Vec<f32>,
}

/// L'unità SE ("squeeze and excitation") di un blocco residuo.
#[derive(Debug, Clone, PartialEq)]
pub struct SeUnit {
    /// I canali del blocco.
    pub channels: usize,
    /// I neuroni intermedi.
    pub hidden: usize,
    /// Primo strato: `hidden` righe di `channels` pesi, e gli spostamenti.
    pub w1: Vec<f32>,
    /// Vedi `w1`.
    pub b1: Vec<f32>,
    /// Secondo strato: `2 × channels` righe di `hidden` pesi (scala, poi spostamento).
    pub w2: Vec<f32>,
    /// Vedi `w2`.
    pub b2: Vec<f32>,
}

/// Un blocco residuo.
#[derive(Debug, Clone, PartialEq)]
pub struct Residual {
    /// La prima convoluzione.
    pub conv1: Conv,
    /// La seconda convoluzione.
    pub conv2: Conv,
    /// L'unità SE.
    pub se: SeUnit,
}

/// Tutti i pesi che servono per la testa delle mosse.
#[derive(Debug, Clone, PartialEq)]
pub struct NetWeights {
    /// La convoluzione d'ingresso.
    pub input: Conv,
    /// I blocchi residui.
    pub residuals: Vec<Residual>,
    /// La prima convoluzione della testa delle mosse.
    pub policy1: Conv,
    /// La seconda convoluzione della testa delle mosse (73 piani usati).
    pub policy: Conv,
}

/// Un errore di formato con il suo motivo.
fn bad(reason: &'static str) -> MaiaError {
    MaiaError::BadNetwork(reason)
}

/// Il messaggio annidato nel campo `number`, letto nei suoi campi.
fn message<'a>(fields: &[Field<'a>], number: u32) -> Result<Vec<Field<'a>>, MaiaError> {
    find(fields, number)
        .and_then(|field| field.bytes())
        .map_or(Ok(Vec::new()), read_fields)
}

/// Uno strato di numeri: dai 2 byte di ciascuno al valore vero.
fn layer(fields: &[Field<'_>], number: u32) -> Result<Vec<f32>, MaiaError> {
    let layer = message(fields, number)?;
    let Some(params) = find(&layer, 3).and_then(|field| field.bytes()) else {
        return Ok(Vec::new());
    };
    if find(&layer, 4)
        .and_then(|field| field.varint())
        .is_some_and(|code| code != LINEAR16)
    {
        return Err(bad("codifica dei numeri non supportata"));
    }
    let min = find(&layer, 1)
        .and_then(|field| field.float())
        .unwrap_or(0.0);
    let max = find(&layer, 2)
        .and_then(|field| field.float())
        .unwrap_or(0.0);
    let range = max - min;
    Ok(params
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| f32::from(u16::from_le_bytes(*pair)) / 65535.0 * range + min)
        .collect())
}

/// Una convoluzione, con la normalizzazione incorporata (come `ConvBlock` in Lc0).
fn conv(fields: &[Field<'_>], number: u32, inputs: usize) -> Result<Conv, MaiaError> {
    let block = message(fields, number)?;
    let mut weights = layer(&block, 1)?;
    let mut biases = layer(&block, 2)?;
    let means = layer(&block, 3)?;
    let variances = layer(&block, 4)?;
    let mut gammas = layer(&block, 5)?;
    let betas = layer(&block, 6)?;
    if biases.is_empty() {
        biases = vec![0.0; means.len()];
    }
    let outputs = biases.len();
    if outputs == 0 || inputs == 0 || weights.len() % (outputs * inputs) != 0 {
        return Err(bad("convoluzione con dimensioni sbagliate"));
    }
    if !means.is_empty() {
        if gammas.is_empty() {
            gammas = vec![1.0; outputs];
        }
        if [variances.len(), gammas.len(), betas.len()] != [outputs; 3] {
            return Err(bad("normalizzazione incompleta"));
        }
        let per_output = weights.len() / outputs;
        for output in 0..outputs {
            // La normalizzazione è: (x − media) / √(varianza + ε) × scala + spostamento.
            let scale = gammas[output] / (variances[output] + EPSILON).sqrt();
            for weight in &mut weights[output * per_output..(output + 1) * per_output] {
                *weight *= scale;
            }
            biases[output] = (biases[output] - means[output]) * scale + betas[output];
        }
    }
    Ok(Conv {
        outputs,
        inputs,
        weights,
        biases,
    })
}

/// L'unità SE di un blocco.
fn se_unit(fields: &[Field<'_>], channels: usize) -> Result<SeUnit, MaiaError> {
    let se = message(fields, 3)?;
    let w1 = layer(&se, 1)?;
    let b1 = layer(&se, 2)?;
    let w2 = layer(&se, 3)?;
    let b2 = layer(&se, 4)?;
    let hidden = b1.len();
    if hidden == 0
        || w1.len() != hidden * channels
        || w2.len() != 2 * channels * hidden
        || b2.len() != 2 * channels
    {
        return Err(bad("unità SE con dimensioni sbagliate"));
    }
    Ok(SeUnit {
        channels,
        hidden,
        w1,
        b1,
        w2,
        b2,
    })
}

/// Controlla che la rete sia di un tipo che questa crate sa calcolare.
fn check_format(net: &[Field<'_>]) -> Result<(), MaiaError> {
    if find(net, 1).and_then(|field| field.fixed32()) != Some(MAGIC) {
        return Err(bad("non è una rete di Lc0"));
    }
    let format = message(net, 4)?;
    if find(&format, 1).and_then(|field| field.varint()) != Some(LINEAR16) {
        return Err(bad("codifica dei numeri non supportata"));
    }
    let network = message(&format, 2)?;
    let value = |number| find(&network, number).and_then(|field| field.varint());
    if value(1) != Some(INPUT_CLASSICAL_112_PLANE)
        || value(3) != Some(NETWORK_SE_WITH_HEADFORMAT)
        || value(4) != Some(POLICY_CONVOLUTION)
    {
        return Err(bad("tipo di rete non supportato"));
    }
    Ok(())
}

/// Legge i pesi di una rete dal contenuto del file (già decompresso).
///
/// # Errors
///
/// [`MaiaError::BadNetwork`] se il file è rovinato o la rete è di un tipo diverso
/// (serve: ingresso a 112 piani, blocchi residui con SE, testa delle mosse a convoluzione).
pub fn read_net(data: &[u8]) -> Result<NetWeights, MaiaError> {
    let net = read_fields(data)?;
    check_format(&net)?;
    let weights = message(&net, 10)?;
    let input = conv(&weights, 1, crate::input_planes::PLANES)?;
    let channels = input.outputs;
    let residuals = weights
        .iter()
        .filter(|field| field.number == 2)
        .map(|field| {
            let block = read_fields(field.bytes().ok_or(bad("blocco residuo"))?)?;
            Ok(Residual {
                conv1: conv(&block, 1, channels)?,
                conv2: conv(&block, 2, channels)?,
                se: se_unit(&block, channels)?,
            })
        })
        .collect::<Result<Vec<_>, MaiaError>>()?;
    let policy1 = conv(&weights, 11, channels)?;
    let policy = conv(&weights, 3, channels)?;
    if residuals.is_empty() || policy.outputs < crate::network::POLICY_PLANES {
        return Err(bad("rete incompleta"));
    }
    Ok(NetWeights {
        input,
        residuals,
        policy1,
        policy,
    })
}
