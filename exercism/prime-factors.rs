//! Exercism Rust Track:  a solution for the prime factors exercise

use std::f64;

/// Function factors() takes a natural number and returns its prime factors in ascending order
pub fn factors(mut nn:u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    let mut primes = vec!(2_u64, 3, 5, 7, 11);
    let mut prime = 1_u64;

    while nn > 1 {
        prime = nextprime(prime, &mut primes);

        while nn % prime == 0 {
            factors.push(prime);

            nn = nn / prime;
        }
    }

    factors
}

/// Function nextprime() returns the next prime, extending the list of primes as necessary
fn nextprime(prime: u64, primes: &mut Vec<u64>) -> u64 {
    let mut candidate: u64 = 1;

    for pr in primes.iter() {
        candidate = *pr;

        if candidate > prime {
            return candidate;
        }
    }

    let mut isprime = false;

    while !isprime {
        candidate = candidate + 1;

        let limit = (candidate as f64).sqrt() as u64;

        for pr in primes.iter() {
            if *pr > limit {
                isprime = true;
                break;
            }
            if candidate % pr == 0 {
                break
            }
        }
    }

    primes.push(candidate);

    candidate
}

//
// My solution incorporates code from the nth-prime exercise and involves the
// generation of a list of primes.  I admit that this is pointless and
// inefficient but I fail to see why this Rust implementation takes ~ 3m6s when
// the same nonsense in Go takes only 6s.
//
// Two other solutions involved the generation of a list of some sort.  The
// others used a single loop and are probably much faster.  Two of these used
// recursion but I fail to see how that helps, particularly for large primes.
//
