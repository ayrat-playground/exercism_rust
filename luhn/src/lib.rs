pub fn is_valid(string: &str) -> bool {
    match clean(string) {
        Err(_)     => false,
        Ok(mut digits) => {
            if digits.len() < 2 { return false }

            digits.reverse();
            let mut fin = vec!();

            for (idx, digit) in digits.iter().enumerate() {
                if (idx + 1) % 2 == 0 {
                    let double = digit * 2;
                    if double >= 10 {
                        fin.push(double - 9);
                    } else {
                        fin.push(double);
                    }
                } else {
                    fin.push(*digit);
                }
            }

            let sum = fin.iter().fold(0, |acc, x| x + acc);

            (sum % 10) == 0
        }
    }
}

fn clean(string: &str) -> Result<Vec<u32>, &'static str> {
    let mut cleaned_string: Vec<u32> = vec!();

    for ch in string.chars() {
        if !(ch.is_digit(10) || ch.is_whitespace()) { return Err("Wrong input"); }


        if ch.is_digit(10) { cleaned_string.push(ch.to_digit(10).unwrap()); }
    }

    Ok(cleaned_string)
}
