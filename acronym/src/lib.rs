use std::ascii::AsciiExt;

pub fn abbreviate(string: &str) -> String {
    string.split(" ").flat_map(|word| {
        let mut acc : Vec<char> = Vec::new();
        let mut prev = ' ';

        for ch in word.chars() {
            if (ch.is_uppercase() && prev.is_lowercase()) ||
                ((prev == ' ') || (prev == '-'))  {
                acc.push(ch.to_ascii_uppercase());
            }

            prev = ch;
        }

        acc
    })
        .fold(String::new(), |mut acc, el| {
            acc.push_str(&el.to_string());

            acc
        })
}
