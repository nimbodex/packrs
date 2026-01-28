use super::table::encode_char;

const CHUNK_SIZE: usize = 8;
const DEFAULT_SEPARATOR: char = ' ';

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChunks(Vec<BinaryChunk>);

impl BinaryChunks {
    fn new(chunks: Vec<BinaryChunk>) -> Self {
        BinaryChunks(chunks)
    }

    fn to_hex(&self) -> HexChunks {
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
    fn new(s: String) -> Self {
        BinaryChunk(s)
    }

    fn to_hex(&self) -> HexChunk {
        match u8::from_str_radix(&self.0, 2) {
            Ok(num) => HexChunk::new(format!("{:02X}", num)),
            Err(e) => panic!("can't parse binary chunk: {}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexChunks(Vec<HexChunk>);

impl HexChunks {
    fn new(chunks: Vec<HexChunk>) -> Self {
        HexChunks(chunks)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn to_string(&self) -> String {
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
    fn new(s: String) -> Self {
        HexChunk(s)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn encode(str: String) -> String {
    let str = prepare_text(&str);
    let chunks = split_by_chunks(&encode_binary(&str), CHUNK_SIZE);

    chunks.to_hex().to_string()
}

fn prepare_text(str: &str) -> String {
    let mut buf = String::new();

    for ch in str.chars() {
        if ch.is_uppercase() {
            buf.push_str("!");
            for lch in ch.to_lowercase() {
                buf.push(lch);
            }
        } else {
            buf.push(ch);
        }
    }

    buf
}

fn encode_binary(str: &str) -> String {
    let mut buf = String::new();

    for ch in str.chars() {
        let bin = encode_char(ch);

        match bin {
            Some(b) => buf.push_str(b),
            None => panic!("unknown character: {}", ch)
        }
    }

    buf
}

fn split_by_chunks(str: &str, chunk_size: usize) -> BinaryChunks {
    let str_len = str.chars().count();
    let mut chunks_count = str_len / chunk_size;
    
    if str_len % chunk_size != 0 {
        chunks_count += 1;
    }

    let mut chunks = Vec::with_capacity(chunks_count);
    let mut buf = String::new();

    for (i, ch) in str.chars().enumerate() {
        buf.push(ch);

          if (i + 1) % chunk_size == 0 {
            chunks.push(BinaryChunk::new(buf.clone()));
            buf.clear();
        }
    }

    if !buf.is_empty() {
        let buf_len = buf.len();
        buf.push_str(&"0".repeat(chunk_size - buf_len));
        chunks.push(BinaryChunk::new(buf));
    }
    
    BinaryChunks::new(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_text_tests() {
        let cases = vec![("NASA", "!n!a!s!a"), ("mix3D", "mix3!d")];

        for (str, expected) in cases {
            assert_eq!(prepare_text(str), expected, "base");
        }
    }

    #[test]
    fn encode_binary_tests() {
        let cases = vec![("!n!a!s!a", "001000100000010000110010000101001000011")];

        for (str, expected) in cases {
            assert_eq!(encode_binary(str), expected, "base");
        }
    }

    #[test]
    fn split_by_chunks_tests() {
        let cases = vec![(
            "001000100000010000110010000101001000011",
            BinaryChunks::new(vec![
                BinaryChunk::new("00100010".to_string()),
                BinaryChunk::new("00000100".to_string()),
                BinaryChunk::new("00110010".to_string()),
                BinaryChunk::new("00010100".to_string()),
                BinaryChunk::new("10000110".to_string()),
            ]),
        )];

        for (str, expected) in cases {
            assert_eq!(split_by_chunks(str, CHUNK_SIZE), expected, "base")
        }
    }

    #[test]
    fn binary_chunk_to_hex_tests() {
        // TODO: add more test cases
        let cases = vec![(
            BinaryChunks::new(vec![
                BinaryChunk::new("0101111".to_string()),
                BinaryChunk::new("10000000".to_string())
            ]),
            HexChunks::new(vec![
                HexChunk::new("2F".to_string()),
                HexChunk::new("80".to_string())
            ])
        )];

        for (bcs, expected) in cases {
            assert_eq!(bcs.to_hex(), expected, "base")
        }
    }

    #[test]
    fn encode_tests() {
        let cases = vec![
            (String::from("My name is Max"), String::from("20 30 3C 18 77 4A E4 06 C0 08")),
            (String::from("NASA"), String::from("22 04 32 14 86")),
            (String::from(""), String::from("")),
            (String::from(" "), String::from("C0")),
            (String::from("!"), String::from("20")),
            (String::from("Hello!"), String::from("20 E9 24 C4 80")),
            (String::from("hi"), String::from("34 80")),
            (String::from("Hi"), String::from("20 D2")),
            (String::from("Go"), String::from("20 24 40")),
            (String::from("GO"), String::from("20 21 11")),
            (String::from("AAA"), String::from("21 90 C8 60")),
            (String::from("aA a"), String::from("64 3D 80")),
        ];

        for (str, expected) in cases {
            assert_eq!(encode(str), expected, "base")
        }
    }
}
