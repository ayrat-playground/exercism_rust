pub fn factors(mut num: u64) -> Vec<u32> {
    if num == 1 { return vec![] }

    let mut factors = vec!();

    while let Some(prime) = find_prime_factor(num) {
        factors.push(prime as u32);

        num = num / prime;
    }

    factors
}

fn find_prime_factor(num: u64) -> Option<u64> {
    (2..num+1).find(|possible_factor| prime_factor(num, *possible_factor))
}

fn prime_factor(num: u64, possible_factor: u64) -> bool {
    factor(num, possible_factor) && prime(possible_factor)
}

fn prime(num: u64) -> bool {
    if num == 2 { return true };

    let sqrt = ((num as f64).sqrt() as u64) + 1;

    !(2..sqrt+1).all(|possible_factor| factor(num, possible_factor))
}

fn factor(num: u64, possible_factor: u64) -> bool {
    num % possible_factor == 0
}
