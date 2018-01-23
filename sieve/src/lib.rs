pub fn primes_up_to(number: usize) -> Vec<usize> {
    if number < 2 { return vec!() }

    let primes = (2..number+1).collect::<Vec<usize>>();

    let mut result = (2..(number / 2 + 1)).fold(primes, |mut acc, el| {
        if prime(el) {
            let mut count = 2;

            while count * el < number {
                acc[count * el - 2] = 0;

                count += 1;
            }
        }

        acc
    });

    if !prime(result[number - 2]) {
        result[number - 2] = 0;
    }

    result.retain(|x| *x != 0);

    result
}

fn prime(num: usize) -> bool {
    if num == 2 { return true };

    let sqrt = ((num as f64).sqrt() as usize) + 1;

    !(2..sqrt+1).any(|possible_factor| factor(num, possible_factor))
}

fn factor(num: usize, possible_factor: usize) -> bool {
    num % possible_factor == 0
}
