pub fn series(digits: &str, len: usize) -> Vec<String> {
    (0..digits.len() + 1)
        .filter(|start| (digits.len() - start) >= len)
        .map(|start| digits[start..len+start].to_string())
        .collect()
}
