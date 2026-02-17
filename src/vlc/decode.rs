use crate::vlc::hex::{HexChunk, HexChunks, DEFAULT_SEPARATOR};

pub fn decode(str: String) -> HexChunks {
    let parts: Vec<&str> = str.split(DEFAULT_SEPARATOR).collect();
    let mut result = Vec::with_capacity(parts.len());
    
    for part in parts {
        result.push(HexChunk(part.to_string()));
    }
    
    HexChunks(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vlc::hex::{HexChunk, HexChunks};

    #[test]
    fn decode_tests() {
        let cases = vec![(
            "20 30 3C 18".to_string(),
            HexChunks::new(vec![
                HexChunk::new("20".to_string()),
                HexChunk::new("30".to_string()),
                HexChunk::new("3C".to_string()),
                HexChunk::new("18".to_string()),
            ]),
        )];

        for (str, expected) in cases {
            assert_eq!(decode(str), expected, "base")
        }
    }
}