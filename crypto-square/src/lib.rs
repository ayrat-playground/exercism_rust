pub fn encrypt(string: &str) -> String {
    if string == "" { return String::from("") }

    let chars = string.to_lowercase().chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    let c = (chars.len() as f64).sqrt() as usize;
    let r = if c * c == chars.len() { c } else { c + 1 };


    let result = (0..c+1).map(|c_idx| chars.chunks(r)
               .fold(String::from(""), |mut acc, row|
                     {
                         if row.len() > c_idx {
                             acc.push_str(&row[c_idx].to_string())
                         } else {
                             acc.push_str(" ")
                         };

                         String::from(acc.trim())
                     })
    ).collect::<Vec<String>>().join(" ");

    String::from(result.trim())
}
