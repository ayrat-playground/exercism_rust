use std::ascii::AsciiExt;

pub fn score(string: &str) -> u32 {
    let mut sc = 0;

    println!("{}", string.to_uppercase());
    for c in string.to_ascii_uppercase().chars() {
        let current_val = match c {
            'A'|'E'|'I'|'O'|'U'|'L'|'N'|'R'|'S'|'T' => 1,
            'D'|'G'                                 => 2,
            'B'|'C'|'M'|'P'                         => 3,
            'F'|'H'|'V'|'W'|'Y'                     => 4,
            'K'                                     => 5,
            'J'|'X'                                 => 8,
            'Q'|'Z'                                 => 10,
            _                                       => 0
        };

        sc += current_val;

        println!("{} {} {}", c, current_val, sc);
    }

    sc
}
