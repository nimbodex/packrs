use crate::vlc::hex::{HexChunk, HexChunks};

pub fn decode(input: &str) -> HexChunks {
    HexChunks::new(
        input
            .split_whitespace()
            .map(|p| HexChunk::new(p.to_string()))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vlc::hex::{HexChunk, HexChunks};

    #[test]
    fn decode_tests() {
        let cases = vec![
            (
                "20 30 3C 18",
                HexChunks::new(vec![
                    HexChunk::new("20".to_string()),
                    HexChunk::new("30".to_string()),
                    HexChunk::new("3C".to_string()),
                    HexChunk::new("18".to_string()),
                ]),
            ),
            (
                "",
                HexChunks::new(vec![]),
            ),
            (
                "20 30",
                HexChunks::new(vec![
                    HexChunk::new("20".to_string()),
                    HexChunk::new("30".to_string()),
                ]),
            ),
            (
                " 20 30 ",
                HexChunks::new(vec![
                    HexChunk::new("20".to_string()),
                    HexChunk::new("30".to_string()),
                ]),
            )
        ];

        for (str, expected) in cases {
            assert_eq!(decode(str), expected, "base")
        }
    }
}