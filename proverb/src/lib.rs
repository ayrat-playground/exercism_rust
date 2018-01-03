pub fn build_proverb(list: Vec<&str>) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let first = list.first().unwrap();

    let mut result = match list.len() {
        1 => "".to_string(),
        2 => format!("For want of a {} the {} was lost.\n", list[0], list[1]),
        _ => {
            let mut vec = Vec::new();

            let mut prev = first;
            for val in &list[1..] {
                vec.push((prev, val));

                prev = &val;
            }

            vec.iter()
                .map(|&pair| format!("For want of a {} the {} was lost.\n", pair.0, pair.1))
                .collect::<Vec<_>>()
                .join("")
        }
    };

    result.push_str(&format!("And all for the want of a {}.", first));

    result
}
