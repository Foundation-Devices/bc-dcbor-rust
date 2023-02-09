use crate::{cbor_encodable::CBOREncodable, CBORDecodable, decode_error::DecodeError, CBORCodable};

use super::{cbor::CBOR, varint::{EncodeVarInt, MajorType}, hex::{hex_to_data, data_to_hex}};


/// A CBOR byte string.
#[derive(Clone)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    /// Creates a new CBOR byte string from the provided data.
    pub fn from_data<T>(data: T) -> Bytes where T: AsRef<[u8]> {
        Bytes(data.as_ref().to_owned())
    }

    /// Creates a new CBOR byte string from the provided hexadecimal string.
    ///
    /// Panics if the string is not well-formed hexadecimal with no spaces or
    /// other characters.
    pub fn from_hex<T>(hex: T) -> Bytes where T: AsRef<str> {
        Bytes(hex_to_data(hex))
    }

    /// The wrapped data.
    pub fn data(&self) -> &Vec<u8> {
        &self.0
    }

    /// The wrapped data as a hexadecimal string.
    pub fn hex(&self) -> String {
        data_to_hex(self.data())
    }

    /// The length of the wrapped data in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the wrapped data is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl CBOREncodable for Bytes {
    fn cbor(&self) -> CBOR {
        CBOR::Bytes(self.to_owned())
    }

    fn cbor_data(&self) -> Vec<u8> {
        let a = &self.0;
        let mut buf = a.len().encode_varint(MajorType::Bytes);
        for b in a {
            buf.push(*b);
        }
        buf
    }
}

impl CBORDecodable for Bytes {
    fn from_cbor(cbor: &CBOR) -> Result<Box<Self>, crate::decode_error::DecodeError> {
        match cbor {
            CBOR::Bytes(data) => Ok(Box::new(data.clone())),
            _ => Err(DecodeError::WrongType),
        }
    }
}

impl CBORCodable for Bytes { }

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&data_to_hex(&self.0))
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("h'")?;
        f.write_str(&data_to_hex(&self.0))?;
        f.write_str("'")
    }
}
