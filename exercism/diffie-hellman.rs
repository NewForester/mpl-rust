//! Exercism Rust Track:  a solution for the diffie-hellman exercise

extern crate rand;

use rand::Rng;

/// Function private_key() returns a private key - a random number in the range [2, p)
pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

/// Function public_key() returns a public key given a private key (a)
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    key(p, g, a)
}

/// Function secret() returns a secret given a private key (a) and a public key (b_pub)
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    key(p, b_pub, a)
}

/// Function key() does the diffie-hellman bit using floating point
fn key(modulo: u64, number: u64, power: u64) -> u64 {
    (number as f64).powf(power as f64) as u64 % modulo
}

//
// This was simple to write but there are no unlimited precision integers here.
//
// All other solutions examined downsized from u64 to u32 in order to use pow()
// whereas I converted to float and back again.
//
