use std::collections::HashMap;
use std::ascii::AsciiExt;

pub fn encode(string: &str) -> String {
    let cipher: HashMap<char, char> = [
        ('a','z'), ('b','y'), ('c','x'), ('d','w'), ('e','v'),
        ('f','u'), ('g','t'), ('h','s'), ('i','r'), ('j','q'),
        ('k','p'), ('l','o'), ('m','n'), ('n','m'), ('o','l'),
        ('p','k'), ('q','j'), ('r','i'), ('s','h'), ('t','g'),
        ('u','f'), ('v','e'), ('w','d'), ('x','c'), ('y','b'),
        ('z','a')
    ].iter().cloned().collect();

    let result = string.to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace() &&
                (c.is_digit(10) || (c.is_alphabetic() && c.is_ascii())))
        .map(|c| {
            match cipher.get(&c) {
                None     => c,
                Some(&d) => d
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .fold(String::from(""), |mut acc, word|
              {
                  acc.push_str(&word.iter().collect::<String>());
                  acc.push_str(" ");
                  acc
              }
        );

    String::from(result.as_str().trim())

}

pub fn decode(string: &str) -> String {
    let cipher: HashMap<char, char> = [
        ('a','z'), ('b','y'), ('c','x'), ('d','w'), ('e','v'),
        ('f','u'), ('g','t'), ('h','s'), ('i','r'), ('j','q'),
        ('k','p'), ('l','o'), ('m','n'), ('n','m'), ('o','l'),
        ('p','k'), ('q','j'), ('r','i'), ('s','h'), ('t','g'),
        ('u','f'), ('v','e'), ('w','d'), ('x','c'), ('y','b'),
        ('z','a')
    ].iter().map(|tuple| (tuple.1, tuple.0)).collect::<Vec<(char, char)>>().iter().cloned().collect();

    string.chars().map(|c| {
        match cipher.get(&c) {
            None      => c,
            Some(&d)  => d
        }
    })
        .filter(|c| !c.is_whitespace())
        .collect()
}
