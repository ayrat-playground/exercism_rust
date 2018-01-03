pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| factors.iter().any(|y| factor(x, y)))
        .fold(0, |acc, i| acc + i)
}

fn factor(num: &u32, possible_factor: &u32) -> bool {
    num % possible_factor == 0
}
