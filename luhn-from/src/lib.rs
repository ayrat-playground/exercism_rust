pub struct Luhn {
    digits: Vec<u32>,
    is_valid: bool,
}

impl<'a> From<&'a str> for Luhn {
    fn from(string: &'a str) -> Self {
        match clean_input(string) {
            Ok(result) => Luhn { digits: result, is_valid: true },
            Err(_)     => Luhn { digits: Vec::new(), is_valid: false },
        }
    }
}

impl From<String> for Luhn {
    fn from(string: String) -> Self {
        let digits = match clean_input(&string) {
            Ok(result) => result,
            Err(_)     => Vec::new()
        };

        Luhn { digits: digits, is_valid: true }
    }
}

impl From<usize> for Luhn {
    fn from(num: usize) -> Self {
        Luhn::from(num.to_string())
    }
}

impl From<u64> for Luhn {
    fn from(num: u64) -> Self {
        Luhn::from(num.to_string())
    }
}

impl From<u32> for Luhn {
    fn from(num: u32) -> Self {
        Luhn::from(num.to_string())
    }
}

impl From<u16> for Luhn {
    fn from(num: u16) -> Self {
        Luhn::from(num.to_string())
    }
}

impl From<u8> for Luhn {
    fn from(num: u8) -> Self {
        Luhn::from(num.to_string())
    }
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        if !self.is_valid { return false }

        let transformed_digits: Vec<u32> = transform_digits(self.digits.to_owned());

        transformed_digits.iter().sum::<u32>() % 10 == 0
    }
}

fn clean_input(string: &str) -> Result<Vec<u32>, &'static str> {
    let mut cleaned_string: Vec<u32> = vec!();

    for ch in string.chars() {
        if !(ch.is_digit(10) || ch.is_whitespace()) { return Err("Wrong input"); }


        if ch.is_digit(10) { cleaned_string.push(ch.to_digit(10).unwrap()); }
    }

    if cleaned_string.len() <= 1 { return Err("Wrong input size") }

    Ok(cleaned_string)
}

fn transform_digits(mut digits: Vec<u32>) -> Vec<u32> {
    digits.reverse();

    digits.iter().enumerate().map(|enumerated_digit| {
        let idx = enumerated_digit.0;
        let value = enumerated_digit.1;

        if (idx + 1) % 2 == 0 {
            let double = value * 2;

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
