use std::fmt;

const DEFAULT_SEPARATOR: char = ' ';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexChunks(Vec<HexChunk>);

impl HexChunks {
    pub fn new(chunks: Vec<HexChunk>) -> Self {
        HexChunks(chunks)
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
pub struct HexChunk(String);

impl HexChunk {
    pub fn new(s: String) -> Self {
        HexChunk(s)
    }
}