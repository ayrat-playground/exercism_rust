pub struct Roman {
    num: String
}

impl Roman {
    pub fn from(number: usize) -> String {
        from(number)
    }
}

fn from(number: usize) -> String {
   if number <= 40 {
       encode_number_from_10_to_40(number)
   } else if number <= 90 {
       encode_number_from_40_to_90(number)
   } else if number <=400 {
       encode_number_from_90_to_400(number)
   } else if number <= 900 {
       encode_number_from_400_900(number)
   } else {
       encode_number_from_900_3000(number)
   }
}

fn encode_digit(num: usize) -> String {
    let result = match num {
        1  => "I",
        2  => "II",
        3  => "III",
        4  => "IV",
        5  => "V",
        6  => "VI",
        7  => "VII",
        8  => "VIII",
        9  => "IX",
        _  => ""
    };

    String::from(result)
}


fn encode_number_from_10_to_40(num: usize) -> String {
    let mut result = String::from("");

    result = (1..(num / 10 + 1)).fold(result, |mut acc, _el| {
        acc.push_str("X");
        acc
    });

    result.push_str(&encode_digit(num % 10));

    result
}

fn encode_number_from_40_to_90(num: usize) -> String {
    let mut result = String::from("");

    let dif = ((num as isize) - 50) as isize;

    if dif < 0 {
        result.push_str("XL");
    } else {
        result.push_str("L");

        result = (1..((dif as isize)) / 10 + 1).fold(result, |mut acc, _el| {
            acc.push_str("X");
            acc
        });
    }

    result.push_str(&encode_digit(num % 10));

    result
}

fn encode_number_from_90_to_400(num: usize) -> String {
    let mut result = String::from("");

    let dif = ((num as isize) - 100) as isize;

    if dif < 0 {
        result.push_str("XC");

        result.push_str(&from(num % 10));
    } else {
        result = (1..(num / 100 + 1)).fold(result, |mut acc, _el| {
            acc.push_str("C");
            acc
        });

        result.push_str(&from(num % 100));
    }

    result
}

fn encode_number_from_400_900(num: usize) -> String {
    let mut result = String::from("");

    let dif = ((num as isize) - 500) as isize;

    if dif < 0 {
        result.push_str("CD");
    } else {
        result.push_str("D");

        result = (1..((dif as isize)) / 100 + 1).fold(result, |mut acc, _el| {
            acc.push_str("C");
            acc
        });
    }

    result.push_str(&from(num % 100));

    result
}

fn encode_number_from_900_3000(num: usize) -> String {
    let mut result = String::from("");

    let dif = ((num as isize) - 1000) as isize;

    if dif < 0 {
        result.push_str("CM");

        result.push_str(&from(num % 100));
    } else {
        result = (1..(num / 1000 + 1)).fold(result, |mut acc, _el| {
            acc.push_str("M");
            acc
        });

        result.push_str(&from(num % 1000));
    }

    result
}
