//! Exercism Rust Track:  a solution for the prime factors exercise

/// Function factors() takes a natural number and returns its prime factors in ascending order
pub fn factors(mut nn:u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    while nn & 1 == 0 {
        factors.push(2);
        nn >>= 1;
    }

    let mut candy = 3_u64;
    while nn > 1 {
        while nn % candy == 0 {
            factors.push(candy);
            nn /= candy;
        }
        candy += 2
    }
    factors
}

//
// This second solution does not incorporate any code from the nth-prime
// exercise and does not involve the generation of a list of primes.
//
// It does as almost all the other solutions did and uses a brute force
// method of calculating factors.
//
// It is a lot (orders of magitude) faster than my first attempt and this
// appears to be because it cuts out the generation of the list of primes
// and the memory managment / garbage collection that involves.  Beware.
//
// I was unable to run benchmarks - seems I have the wrong compiler.
//

