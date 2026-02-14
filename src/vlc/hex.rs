const DEFAULT_SEPARATOR: char = ' ';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexChunks(Vec<HexChunk>);

impl HexChunks {
    pub fn new(chunks: Vec<HexChunk>) -> Self {
        HexChunks(chunks)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_string(&self) -> String {
        match self.0.len() {
            0 => String::new(),
            1 => self.0[0].0.clone(),
            _ => {
                let capacity = self.0.iter()
                    .map(|hc| hc.0.len())
                    .sum::<usize>()
                    + (self.0.len() - 1) * DEFAULT_SEPARATOR.len_utf8();

                let mut builder = String::with_capacity(capacity);

                builder.push_str(&self.0[0].0);
                for hc in &self.0[1..] {
                    builder.push(DEFAULT_SEPARATOR);
                    builder.push_str(&hc.0);
                }

                builder
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

    pub fn len(&self) -> usize {
        self.0.len()
    }
}