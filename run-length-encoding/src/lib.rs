pub fn encode(string: &str) -> String {
    if string == "" { return "".to_string() }

    let mut prev = string.chars().nth(0).unwrap();
    let mut count = 0;
    let mut result = "".to_string();

    for c in string.chars() {
        if c == prev {
            count += 1;
        } else {
            let encoding = encode_char(prev, count);
            result.push_str(&encoding);

            count = 1;
        }

        prev = c;
    }
    let encoding = encode_char(prev, count);
    result.push_str(&encoding);

    result.to_string()
}

pub fn decode(string: &str) -> String {
    if string == "" { return "".to_string() }

    let mut result = "".to_string();
    let mut chars = string.chars().fuse();
    let mut count = 0;

    while let Some(c) = chars.next() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as usize;

            if count == 0 {
                count = digit;
            } else {
                count = count * 10 + digit;
            }
        } else {
            result.push_str(&decode_char(c, count));

            count = 0;
        }
    }

    result
}

fn encode_char(c: char, count: usize) -> String {
     if count > 1 {
        format!("{}{}", count, c)
    } else {
        format!("{}", c)
    }
}

fn decode_char(c: char, count: usize) -> String {
    if count == 0 { return format!("{}", c); }

    (0..count).fold("".to_string(), |mut acc, _| {
        acc.push_str(&format!("{}", c));
        acc
    })
}
