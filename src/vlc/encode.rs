use super::table::encode_char;

const CHUNK_SIZE: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChunk(String);

impl BinaryChunk {
    pub fn new(s: String) -> Self {
        BinaryChunk(s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChunks(Vec<BinaryChunk>);

impl BinaryChunks {
    pub fn new(chunks: Vec<BinaryChunk>) -> Self {
        BinaryChunks(chunks)
    }
}

pub fn encode(str: String) -> String {
    let str = prepare_text(&str);
    let bin_str = encode_binary(&str);
    let _ = split_by_chunks(&bin_str, CHUNK_SIZE);

    bin_str
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
}
