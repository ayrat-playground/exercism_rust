pub fn is_valid(string: &str) -> bool {
    let cleaned_input = clean_input(string);
    println!("{:?}", cleaned_input);

    if cleaned_input.is_err() { return false }

    let transformed_digits: Vec<u32> = transform_digits(cleaned_input.unwrap());
    println!("{:?}", transformed_digits);
    println!("{}", transformed_digits.iter().sum::<u32>());

    transformed_digits.iter().sum::<u32>() % 10 == 0
}

fn clean_input(string: &str) -> Result<Vec<u32>, &'static str> {
    let mut cleaned_string: Vec<u32> = vec!();

    for ch in string.chars() {
        if !(ch.is_digit(10) || ch.is_whitespace()) { return Err("Wrong input"); }


        if ch.is_digit(10) { cleaned_string.push(ch.to_digit(10).unwrap()); }
    }

    if cleaned_string.len() < 2 { return Err("Wrong input size") }

    Ok(cleaned_string)
}

fn transform_digits(mut digits: Vec<u32>) -> Vec<u32> {
    digits.reverse();

    digits.iter().enumerate().map(|enumerated_digit| {
        let idx = enumerated_digit.0;
        let value = enumerated_digit.1;

        if (idx + 1) % 2 == 0 {
            let double = value * 2;
            println!("{}", double);
            if double >= 10 {
                double - 9
            } else {
                double
            }
        } else {
            *value
        }
    }).collect()
}
