pub fn factors(n: u64) -> Vec<u64> {
    let mut res : Vec<u64> = Vec::new();
    let mut potential_factor = 2;
    let mut val = n;
    while val > 1 {
        if val % potential_factor == 0 {
            res.push(potential_factor);
            val /= potential_factor;
        } else {
            potential_factor += 1;
        }
    }
    return res
}
