use std::fmt;
use crate::vlc::binary::{BinaryChunk, BinaryChunks};
use crate::vlc::encode::CHUNK_SIZE;

pub const DEFAULT_SEPARATOR: char = ' ';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexChunks(pub Vec<HexChunk>); // TODO: Make internal data to private

impl HexChunks {
    pub fn new(chunks: Vec<HexChunk>) -> Self {
        HexChunks(chunks)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_binary(&self) -> BinaryChunks {
        let mut result = BinaryChunks::with_capacity(self.len());

        for hex_chunk in self.0.iter() {
            result.push(hex_chunk.to_binary())
        }

        result
    }
}

impl fmt::Display for HexChunks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.len() {
            0 => Ok(()),
            1 => write!(f, "{}", self.0[0].0),
            _ => {
                write!(f, "{}", self.0[0].0)?;
                for hc in &self.0[1..] {
                    write!(f, "{DEFAULT_SEPARATOR}{}", hc.0)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexChunk(pub String); // TODO: Make internal data to private

impl HexChunk {
    pub fn new(s: String) -> Self {
        HexChunk(s)
    }

    pub fn to_binary(&self) -> BinaryChunk {
        match self.parse_uint_chunk(self.0.as_str(), CHUNK_SIZE) {
            Ok(n) => {
                let s = format!("{:08b}", n);
                BinaryChunk::new(s)
            },
            Err(e) => panic!("unable to parse hex chunk: {e}")
        }
    }

    fn parse_uint_chunk(&self, s: &str, chunk_size: usize) -> Result<u64, &'static str> {
        let n = u64::from_str_radix(s, 16).map_err(|_| "invalid syntax")?;
        if chunk_size < 64 && n >= (1u64 << chunk_size) {
            return Err("parse uint chunk: value out of range");
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::vlc::binary::{BinaryChunk, BinaryChunks};
    use crate::vlc::hex::{HexChunk, HexChunks};

    #[test]
    fn hex_chunk_to_binary_tests() {
        // TODO: add more test cases
        let cases = vec![
            (
                HexChunk::new(String::from("2F")),
                BinaryChunk::new(String::from("00101111"))
            ),
            (
                HexChunk::new(String::from("80")),
                BinaryChunk::new(String::from("10000000"))
            )
        ];

        for (hc, expected) in cases {
            assert_eq!(hc.to_binary(), expected, "base");
        }
    }

    #[test]
    fn hex_chunks_to_binary_tests() {
        // TODO: add more test cases
        let cases = vec![
            (
                HexChunks::new(vec![
                    HexChunk::new(String::from("2F")),
                    HexChunk::new(String::from("80"))
                ]),
                BinaryChunks::new(vec![
                    BinaryChunk::new(String::from("00101111")),
                    BinaryChunk::new(String::from("10000000"))
                ])
            ),
            (
                HexChunks::new(vec![
                    HexChunk::new(String::from("00")),
                    HexChunk::new(String::from("20")),
                    HexChunk::new(String::from("40")),
                    HexChunk::new(String::from("00"))
                ]),
                BinaryChunks::new(vec![
                    BinaryChunk::new(String::from("00000000")),
                    BinaryChunk::new(String::from("00100000")),
                    BinaryChunk::new(String::from("01000000")),
                    BinaryChunk::new(String::from("00000000"))
                ]),
            )
        ];

        for (hcs, expected) in cases {
            assert_eq!(hcs.to_binary(), expected, "base");
        }
    }
}