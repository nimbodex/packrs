pub fn encode(_str: String) -> String {
    todo!("encode not implemented yet")
}

fn prepare_text(str: &str) -> String {
    let mut buf = String::with_capacity(str.len());

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

fn encode_binary(_str: &str) -> String {
    todo!("binary encoding not implemented yet")
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
