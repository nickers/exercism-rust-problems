use std::cmp;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut smallest :Vec<u64> = Vec::new();
    for row in input {
        for i in 0..smallest.len() {
            smallest[i] = cmp::min(row[i], smallest[i])
        }
        if smallest.len() < row.len() {
            let rg = smallest.len()..row.len();
            smallest.extend_from_slice(&row.as_slice()[rg]);
        }
    }

    let mut saddles : Vec<(usize, usize)> = Vec::new();

    for (y, row) in input.iter().enumerate() {
        match row.iter().max() {
            None => {},
            Some (m) => {
                for (x, v) in row.iter().enumerate() {
                    if *v == smallest[x] && v == m {
                        saddles.push((y, x))
                    }
                }
            },
        }
    }

    saddles
}
