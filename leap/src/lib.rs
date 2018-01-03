pub fn is_leap_year(year: i32) -> bool {
    match year % 4 {
        0 =>
            match year % 100 {
                0 =>
                    match year % 400 {
                        0 => true,
                        _ => false

                    },
                _ => true
            }
        _ => false
    }
}
