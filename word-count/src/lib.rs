use std::collections::HashMap;

pub fn word_count(string: &str) -> HashMap<String, u32> {
    let mut hash: HashMap<String, u32> = HashMap::new();

    for word in string.to_lowercase().split_whitespace() {
        let key: String = word.chars().filter(|c| c.is_alphabetic() || c.is_digit(10)).collect();

        if key != "" {
            match hash.get(&key) {
                None       => hash.insert(key, 1),
                Some(&num) => hash.insert(key, num + 1)
            };
        }
    }

    hash
}
