pub fn nth(n: usize) -> Result<usize, &'static str> {
    let mut count = 0;

    if n < 1 {
        return Err("Must be bigger than 0")
    }

    for i in 2.. {
        if prime(i) {
            count = count + 1;
        }

        if n == count {
            return Ok(i)
        }
    }

    Err("Not found")
}

fn prime(n: usize) -> bool {
    let sqrt = ((n as f64).sqrt() as usize) + 1;

    for i in 2..sqrt {
        if n % (i as usize) == 0 {
            return false
        }
    }

    true
}
