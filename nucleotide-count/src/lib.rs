use std::collections::HashMap;

const NUCLEOTIDES: &'static [char] = &['A', 'C', 'G', 'T'];

pub fn count(c: char, string: &str) -> Result<u32, &'static str> {
    if validate_nucleotide(c) { return Err("Wrong nucleotide"); }

    let mut count = 0;

    for ch in string.chars() {
        if validate_nucleotide(ch) { return Err("Wrong nucleotide in dna"); }

        if ch == c { count += 1; }
    }

    Ok(count)
}

pub fn nucleotide_counts(string: &str) -> Result<HashMap<char, usize>, &'static str> {
    let mut map: HashMap<char, usize> = HashMap::new();

    NUCLEOTIDES.iter()
        .fold(&mut map, |acc, ch| {
            acc.insert(*ch, 0);
            acc
        });

    for ch in string.chars() {
        if validate_nucleotide(ch) { return Err("Wrong nucleotide in dna"); }

        match map.get(&ch) {
            Some(&x) => map.insert(ch, x + 1),
            None     => map.insert(ch, 1)
        };
    }

    Ok(map)
}

fn validate_nucleotide(c: char) -> bool {
    !NUCLEOTIDES.iter().any(|nuc| *nuc == c)
}
