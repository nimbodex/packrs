use std::collections::HashMap;
use std::sync::LazyLock;

pub static ENCODING_TABLE: &[(char, &str)] = &[
    (' ', "11"),
    ('t', "1001"),
    ('n', "10000"),
    ('s', "0101"),
    ('r', "01000"),
    ('d', "00101"),
    ('!', "001000"),
    ('c', "000101"),
    ('m', "000011"),
    ('g', "0000100"),
    ('b', "0000010"),
    ('v', "00000001"),
    ('k', "0000000001"),
    ('q', "000000000001"),
    ('e', "101"),
    ('o', "10001"),
    ('a', "011"),
    ('i', "01001"),
    ('h', "0011"),
    ('l', "001001"),
    ('u', "00011"),
    ('f', "000100"),
    ('p', "0000101"),
    ('w', "0000011"),
    ('y', "0000001"),
    ('j', "000000001"),
    ('x', "00000000001"),
    ('z', "000000000000"),
];

pub static ENCODE_MAP: LazyLock<HashMap<char, &'static str>> = LazyLock::new(|| {
    ENCODING_TABLE.iter().copied().collect()
});

pub fn encode_char(ch: char) -> Option<&'static str> {
    ENCODE_MAP.get(&ch).copied()
}
