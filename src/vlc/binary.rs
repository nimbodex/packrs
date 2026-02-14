use crate::vlc::hex::{HexChunk, HexChunks};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChunks(Vec<BinaryChunk>);

impl BinaryChunks {
    pub fn new(chunks: Vec<BinaryChunk>) -> Self {
        BinaryChunks(chunks)
    }

    pub fn to_hex(&self) -> HexChunks {
        HexChunks::new(
            self.0.iter()
                .map(|chunk| chunk.to_hex())
                .collect()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChunk(String);

impl BinaryChunk {
    pub fn new(s: String) -> Self {
        BinaryChunk(s)
    }

    pub fn to_hex(&self) -> HexChunk {
        match u8::from_str_radix(&self.0, 2) {
            Ok(num) => HexChunk::new(format!("{:02X}", num)),
            Err(e) => panic!("can't parse binary chunk: {}", e),
        }
    }
}