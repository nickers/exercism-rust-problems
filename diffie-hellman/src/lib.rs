extern crate rand;

use rand::Rng;

// https://en.wikipedia.org/wiki/Modular_exponentiation
// right-to-left binary method
fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    match exponent {
        0 => 1,
        _ => {
            let multiplier = match exponent % 2 {
                0 => 1,
                _ => base,
            };
            (modular_pow((base * base) % modulus, exponent / 2, modulus) * multiplier) % modulus
        }
    }
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}
