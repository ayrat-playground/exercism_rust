/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut chars: Vec<_> = isbn.chars()
        .filter(|c| (*c != '-'))
        .collect();

    if chars.len() > 10 { return false };

    chars = isbn.chars()
        .filter(|c| (*c != 'X') && (*c != '-'))
        .collect();

    if !chars.iter().all(|c| c.is_numeric()) { return false }
    if !((chars.len() == 9) || (chars.len() == 10)) { return false }
    if (chars.len() == 9) && (isbn.chars().last().unwrap() != 'X') {
        return false
    }

    let mut multiplier = 10;
    let mut result = 0;

    for c in &chars {
        let digit = c.to_digit(10).unwrap();

        result = digit * multiplier + result;
        multiplier -= 1;
    }

    if chars.len() == 9 {
        result = result + 10;
    }

    result % 11 == 0
}
