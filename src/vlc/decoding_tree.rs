use super::table::ENCODING_TABLE;

pub struct DecodingTree {
    pub value: Option<char>,
    pub zero: Option<Box<DecodingTree>>,
    pub one: Option<Box<DecodingTree>>,
}

pub fn decoding_tree() -> DecodingTree {
    let mut result = DecodingTree::new();

    for (ch, code) in ENCODING_TABLE.iter() {
        result.insert(*ch, code);
    }

    result
}

impl DecodingTree {
    pub fn new() -> Self {
        Self {
            value: None,
            zero: None,
            one: None
        }
    }

    pub fn insert(&mut self, ch: char, code: &str) {
        let mut curr = self;

        for b in code.bytes() {
           curr = match b {
                b'0' => curr.zero.get_or_insert_with(|| Box::new(DecodingTree::new())),
                b'1' => curr.one.get_or_insert_with(|| Box::new(DecodingTree::new())),
                _ => panic!("invalid bit"),
            }
        }

        curr.value = Some(ch)
    }
}