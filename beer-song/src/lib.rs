pub fn verse(n: i32) -> String {
    let mut first_part = match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\n", n, n)
    };

    let second_part = match n - 1 {
        -1 => "".to_string(),
        0  => "Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        1  => "Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _  => format!("Take one down and pass it around, {} bottles of beer on the wall.\n", n-1)
    };

    first_part.push_str(&second_part[..]);

    first_part.to_string()
}

pub fn sing(start: i32, end: i32) -> String {
    (end..start+1).rev().map(verse).collect::<Vec<String>>().join("\n")
}
