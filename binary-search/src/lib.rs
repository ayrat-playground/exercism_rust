pub fn find(slice: &[u32], num: u32) -> Option<u32> {
    let mut middle = if slice.len() > 0 && slice.len() % 2 == 0 {
        (slice.len() - 1) / 2
    } else {
        slice.len() / 2
    };

    if slice.len() == 0 {
        return None;
    }

    let mut prev_middles = Vec::new();

    loop {
        if num == slice[middle] {
            return Some(middle as u32);
        } else if num > slice[middle] {
            middle = (slice.len() - middle) / 2  + middle;
        } else {
            middle = middle / 2;
        }

        if prev_middles.contains(&middle) {
            break;
        }

        prev_middles.push(middle);
    }

    if num == slice[middle] {
        return Some(middle as u32);
    }

    None
}
