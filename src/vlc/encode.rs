use super::table::encode_char;

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
    let mut chunks = Vec::new();
    
    if str_len % chunk_size != 0 {
        chunks_count += 1;
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
