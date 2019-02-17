pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::with_capacity(n as usize);

    primes.push(2u32);

    let mut i = 3;

    while primes.len() <= n as usize {
        match primes.iter().find(|&&x| i % x == 0) {
            None => primes.push(i),
            Some(_) => { },
        }
        i = i + 2;
    }

    primes.pop().unwrap()
}
