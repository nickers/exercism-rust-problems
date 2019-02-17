pub fn collatz(mut n: u64) -> Option<u64> {
    if n <= 0 {
        return None
    }

    let mut step_nr = 0;
    while n > 1 {
        step_nr += 1;
        n = match n % 2 {
            1 => 3 * n + 1,
            _ => n / 2,
        }
    }
    Some(step_nr)
}
