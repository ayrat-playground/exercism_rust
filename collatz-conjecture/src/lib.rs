// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n < 1 { return Err("oops") }

    let mut temp = n;
    let mut count = 0;

    while temp != 1 {
        if even(temp) {
            temp = temp / 2;
        } else {
            temp = 3 * temp + 1;
        }

        count += 1;
    }

    Ok(count)
}

fn even(n: u64) -> bool {
    n % 2 == 0
}
