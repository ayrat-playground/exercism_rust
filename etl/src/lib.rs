use std::collections::BTreeMap;
use std::ascii::AsciiExt;

pub fn transform(initial_data: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();

    for (key, value) in initial_data.iter() {
        for ch in value {
            *result.entry(ch.to_ascii_lowercase()).or_insert(0) += key;
        }
    }

    result
}
