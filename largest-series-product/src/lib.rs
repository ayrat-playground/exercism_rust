pub fn lsp(string: &str, len: usize) -> Result<u32, &'static str> {
    if !string.chars().all(|c| c.is_digit(10)) || string.len() < len {
        return Err("Oops");
    }

    if len == 0 {
        return Ok(1)
    }

    let digits: Vec<u32> = string.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let result = digits.windows(len)
        .fold(0, |acc, window| {
            let product = window.iter().product();

            if product > acc { product } else { acc }
        });

    Ok(result)
}
