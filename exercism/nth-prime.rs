//! Exercism Rust Track:  a solution for the nth-prime exercise

use std::f64;

/// Function nth return the nth prime
pub fn nth(nn: u32) -> Option<u32> {
    if nn < 1 {
        return None
    }

    let mut primes: Vec<u32> = Vec::with_capacity(nn as usize);
    let mut candidate = 2;
    primes.push(candidate);

    while primes.len() < nn as usize {
        candidate = candidate + 1;

        let limit = (candidate as f64).sqrt() as u32;

        let mut isprime = true;
        for prime in &primes {
            if prime > &limit {
                break;
            }
            if candidate % prime == 0 {
                isprime = false;
                break
            }
        }

        if isprime {
            primes.push(candidate);
        }
    }

    Some(candidate)
}

//
// The same length as in Go but then it is a direct translation.
//
// Adding 'the upper bound is the square root' added 7 lines to 30 but the
// 'consider only odd candidates' that I forgot would have been done without
// adding extra lines.
//
// The other 8 examined had one no-show and one sieve.  In general the
// implementations were inefficient and less readable than the Go solutions.
//
