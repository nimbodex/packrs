use super::table::encode_char;

const CHUNK_SIZE: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChunk(String);

impl BinaryChunk {
    pub fn new(s: String) -> Self {
        BinaryChunk(s)
    }
}

pub type BinaryChunks = Vec<BinaryChunk>;

pub fn encode(str: String) -> String {
    let str = prepare_text(&str);
    let bin_str = encode_binary(&str);
    let chunks = split_by_chunks(&bin_str, CHUNK_SIZE);

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
        buf.push (ch);

        if i+1 == chunk_size {
            chunks.push(BinaryChunk::new(buf));
            buf.clear();
        }
    }

    if buf.len() != 0 {
        let mut last_chunk = &buf;
        last_chunk.push_str(&"0".repeat(chunk_size - last_chunk.len()));
        chunks.push(BinaryChunk::new(last_chunk));
    }
    
    chunks
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
}
