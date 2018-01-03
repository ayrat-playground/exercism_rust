pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    (1..s).fold(1, |acc, _i| acc * 2)
}

pub fn total() -> u64 {
    (1..65).map(|x| square(x)).sum()
}
