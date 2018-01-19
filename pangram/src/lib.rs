use std::ascii::AsciiExt;

pub fn is_pangram(string: &str) -> bool {
    let unique_chars = string.to_lowercase().chars()
        .fold(vec!(), |mut acc, c| {
            if !acc.contains(&c) && !c.is_whitespace() &&
                c.is_alphabetic() && c.is_ascii()  {
                acc.push(c);
            }

            acc
        });

    println!("{:?} {}", unique_chars, unique_chars.len());
    unique_chars.len() == 26
}
