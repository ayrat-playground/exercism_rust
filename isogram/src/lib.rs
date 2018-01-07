pub fn check(string: &str) -> bool {
    let string: String = string.chars()
        .filter(|c| (*c != '_') && (*c != ' ') && (*c != '-'))
        .collect();

    string.to_lowercase().chars().all(|c| {
        let count = string.to_lowercase().chars().fold(0, |acc, d| if c == d { acc + 1} else { acc });

        count == 1
    })
}
