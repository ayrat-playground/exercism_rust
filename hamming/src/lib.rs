pub fn hamming_distance(string1: &str, string2: &str) -> Result<usize, & 'static str> {
    if string1.len() != string2.len() {
        return Err("Different sizes")
    }

    let mut iter1 = string1.chars().fuse();
    let mut iter2 = string2.chars().fuse();

    let mut count = 0;
    while let Some(s1) = iter1.next() {
        let s2 = iter2.next().unwrap();

        if s2 != s1 {
            count += 1;
        }
    }

    Ok(count)
}
