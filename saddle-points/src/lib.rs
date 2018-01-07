pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut max_in_row_ids = vec!();

    for (i, row) in input.iter().enumerate() {
        let mut max_ids = vec!();

        let mut max = match row.first() {
            None => return max_ids,
            Some(el) => el,
        };


        for (j, el) in row.iter().enumerate() {
            if el == max {
                max_ids.push((i, j));
            } else if el > max {
                max = el;
                max_ids = vec!();
                max_ids.push((i,j));
            }
        }

        max_in_row_ids.push(max_ids);
    }

    let mut saddle_points = vec!();

    for max_in_row_id in max_in_row_ids {
        for (i,j) in max_in_row_id {
            let max_in_row = input[i][j];

            if (0..input.len()).all(|k| input[k][j] >= max_in_row) {
                saddle_points.push((i,j));
            }
        }
    }

    saddle_points
}
