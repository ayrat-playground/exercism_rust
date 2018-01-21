use std::collections::HashMap;
use std::ascii::AsciiExt;

const ALPHABET: &'static [char] = &['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

pub fn rotate(string: &str, offset: usize) -> String {
    let cipher = cipher(offset);

    string.chars().fold(String::new(), |mut acc, ch| {
        if ch.is_alphabetic() {
            if ch.is_uppercase() {
                let cipher_char = cipher.get(&ch.to_ascii_lowercase()).unwrap().to_uppercase();
                acc.push_str(&cipher_char.to_string());
            } else {
                let cipher_char = cipher.get(&ch).unwrap();
                acc.push_str(&cipher_char.to_string());
            }
        } else {
            acc.push_str(&ch.to_string());
        };

        acc
    })
}

fn cipher(offset: usize) -> HashMap<char, char> {
    ALPHABET.iter().enumerate().fold(HashMap::new(), |mut acc, tuple| {
        let idx = tuple.0;
        let value = tuple.1;
        let cipher_idx: isize  = ((idx as isize) + (offset as isize)) % 26;

        acc.insert(*value, ALPHABET[cipher_idx as usize]);
        acc
    })
}
