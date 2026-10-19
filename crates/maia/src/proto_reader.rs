//! La lettura del formato in cui sono salvate le reti di Lc0 ("protobuf").
//!
//! Un file protobuf è una fila di **campi**. Ogni campo comincia con un
//! numero (quale campo è, e come è scritto) seguito dal valore. Qui servono
//! solo tre modi di scrittura:
//! - **numero intero** di lunghezza variabile ("varint": 7 bit per byte, il
//!   bit più alto dice se il numero continua nel byte successivo);
//! - **4 byte** (un numero decimale `f32`);
//! - **blocco di byte** preceduto dalla sua lunghezza (un testo, un elenco di
//!   pesi, oppure un altro messaggio con i suoi campi).
//!
//! Basta questo piccolo lettore: così non serve una libreria in più.

use crate::maia_error::MaiaError;

/// Il valore di un campo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldValue<'a> {
    /// Un numero intero.
    Varint(u64),
    /// Quattro byte (un `f32` o un intero a 32 bit).
    Fixed32([u8; 4]),
    /// Otto byte.
    Fixed64([u8; 8]),
    /// Un blocco di byte (testo, pesi o messaggio annidato).
    Bytes(&'a [u8]),
}

/// Un campo: il suo numero e il valore.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field<'a> {
    /// Il numero del campo, come nella descrizione del formato (`net.proto` di Lc0).
    pub number: u32,
    /// Il valore.
    pub value: FieldValue<'a>,
}

impl<'a> Field<'a> {
    /// Il blocco di byte del campo, se è scritto così.
    #[must_use]
    pub fn bytes(&self) -> Option<&'a [u8]> {
        match self.value {
            FieldValue::Bytes(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Il numero intero del campo, se è scritto così.
    #[must_use]
    pub fn varint(&self) -> Option<u64> {
        match self.value {
            FieldValue::Varint(value) => Some(value),
            _ => None,
        }
    }

    /// Il numero intero a 32 bit del campo, se è scritto in 4 byte.
    #[must_use]
    pub fn fixed32(&self) -> Option<u32> {
        match self.value {
            FieldValue::Fixed32(bytes) => Some(u32::from_le_bytes(bytes)),
            _ => None,
        }
    }

    /// Il numero decimale (`f32`) del campo, se è scritto in 4 byte.
    #[must_use]
    pub fn float(&self) -> Option<f32> {
        match self.value {
            FieldValue::Fixed32(bytes) => Some(f32::from_le_bytes(bytes)),
            _ => None,
        }
    }
}

/// Legge un numero intero di lunghezza variabile a partire da `position`.
fn read_varint(data: &[u8], position: &mut usize) -> Result<u64, MaiaError> {
    let mut value = 0_u64;
    for shift in (0..64).step_by(7) {
        let byte = *data
            .get(*position)
            .ok_or(MaiaError::BadNetwork("numero troncato"))?;
        *position += 1;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(MaiaError::BadNetwork("numero troppo lungo"))
}

/// Prende `length` byte a partire da `position`.
fn take<'a>(data: &'a [u8], position: &mut usize, length: usize) -> Result<&'a [u8], MaiaError> {
    let end = position
        .checked_add(length)
        .filter(|end| *end <= data.len())
        .ok_or(MaiaError::BadNetwork("blocco troncato"))?;
    let bytes = &data[*position..end];
    *position = end;
    Ok(bytes)
}

/// Legge tutti i campi di un messaggio, nell'ordine in cui sono scritti.
///
/// # Errors
///
/// [`MaiaError::BadNetwork`] se il messaggio è troncato o usa un modo di
/// scrittura sconosciuto.
pub fn read_fields(data: &[u8]) -> Result<Vec<Field<'_>>, MaiaError> {
    let mut fields = Vec::new();
    let mut position = 0;
    while position < data.len() {
        let key = read_varint(data, &mut position)?;
        let number = u32::try_from(key >> 3).map_err(|_| MaiaError::BadNetwork("campo"))?;
        let value = match key & 7 {
            0 => FieldValue::Varint(read_varint(data, &mut position)?),
            1 => {
                let bytes = take(data, &mut position, 8)?;
                FieldValue::Fixed64(
                    bytes
                        .try_into()
                        .map_err(|_| MaiaError::BadNetwork("8 byte"))?,
                )
            }
            2 => {
                let length = usize::try_from(read_varint(data, &mut position)?)
                    .map_err(|_| MaiaError::BadNetwork("lunghezza"))?;
                FieldValue::Bytes(take(data, &mut position, length)?)
            }
            5 => {
                let bytes = take(data, &mut position, 4)?;
                FieldValue::Fixed32(
                    bytes
                        .try_into()
                        .map_err(|_| MaiaError::BadNetwork("4 byte"))?,
                )
            }
            _ => return Err(MaiaError::BadNetwork("modo di scrittura sconosciuto")),
        };
        fields.push(Field { number, value });
    }
    Ok(fields)
}

/// Il primo campo con quel numero, se c'è.
#[must_use]
pub fn find<'a>(fields: &[Field<'a>], number: u32) -> Option<Field<'a>> {
    fields.iter().copied().find(|field| field.number == number)
}

#[cfg(test)]
mod tests {
    use super::{FieldValue, find, read_fields};

    #[test]
    fn reads_numbers_floats_and_nested_blocks() {
        // Campo 1 = 300 (varint su due byte), campo 2 = 1.5 (f32), campo 3 = blocco [7, 8].
        let data = [
            0x08, 0xac, 0x02, 0x15, 0x00, 0x00, 0xc0, 0x3f, 0x1a, 0x02, 7, 8,
        ];

        let fields = read_fields(&data).expect("messaggio valido");

        assert_eq!(fields.len(), 3);
        assert_eq!(find(&fields, 1).and_then(|field| field.varint()), Some(300));
        assert_eq!(find(&fields, 2).and_then(|field| field.float()), Some(1.5));
        assert_eq!(
            find(&fields, 3).map(|field| field.value),
            Some(FieldValue::Bytes(&[7, 8]))
        );
    }

    #[test]
    fn a_truncated_message_is_an_error() {
        assert!(read_fields(&[0x1a, 0x05, 1]).is_err());
    }
}
