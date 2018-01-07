pub fn encode(num: u64) -> String {
    enc_quintillion(num).trim_right().to_string()
}

fn enc_quintillion(num: u64) -> String {
    if num == 0 { return "zero".to_string() }

    match num / 1_000_000_000_000_000_000 {
        0     => enc_quadrillion(num),
        other => [enc_quadrillion(other), "quintillion".to_string(), enc_quadrillion(num % 1_000_000_000_000_000_000)].join(" ")
    }
}

fn enc_quadrillion(num: u64) -> String {
    match num / 1_000_000_000_000_000 {
        0     => enc_trillion(num),
        other => [enc_trillion(other), "quadrillion".to_string(), enc_trillion(num % 1_000_000_000_000_000)].join(" ")
    }
}

fn enc_trillion(num: u64) -> String {
    match num / 1_000_000_000_000 {
        0     => enc_billion(num),
        other => [enc_billion(other), "trillion".to_string(), enc_billion(num % 1_000_000_000_000)].join(" ")
    }
}

fn enc_billion(num: u64) -> String {
    match num / 1_000_000_000 {
        0     => enc_million(num),
        other => [enc_million(other), "billion".to_string(), enc_million(num % 1_000_000_000)].join(" ")
    }
}

fn enc_million(num: u64) -> String {
    match num / 1_000_000 {
        0     => enc_thousand(num),
        other => [enc_thousand(other), "million".to_string(), enc_thousand(num % 1_000_000)].join(" ")
    }
}

fn enc_thousand(num: u64) -> String {
    match num / 1_000 {
        0     => enc_hundred(num),
        other => [enc_hundred(other), "thousand".to_string(), enc_hundred(num % 1000)].join(" ")
    }
}

fn enc_hundred(num: u64) -> String {
    match num / 100 {
        0     => enc(num),
        other => [enc(other), "hundred".to_string(), enc(num % 100)].join(" ")
    }
}

fn enc(num: u64) -> String {
    match num {
        0  => "".to_string(),
        1  => "one".to_string(),
        2  => "two".to_string(),
        3  => "three".to_string(),
        4  => "four".to_string(),
        5  => "five".to_string(),
        6  => "six".to_string(),
        7  => "seven".to_string(),
        8  => "eight".to_string(),
        9  => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _  => {
            let dec = match num / 10 {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => panic!("wrong decimal!")
            };

            match num % 10 {
                0 => dec.to_string(),
                other => [dec.to_string(), enc(other)].join("-")
            }
        }
    }
}
