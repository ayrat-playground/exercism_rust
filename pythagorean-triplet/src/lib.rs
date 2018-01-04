pub fn find() -> Option<u32> {
    for x in 1.. {
        'inner:
        for y in 1.. {
            let z = 1000 - x - y;
            if z < 1 {
                break 'inner;
            }

            if ((x as i32).pow(2) + (y as i32).pow(2)) == (z as i32).pow(2) {
                println!("{} {} {}", x, y, z);

                return Some((x * y * z));
            }
        }
    }

    None
}
