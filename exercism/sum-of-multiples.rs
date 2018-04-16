//! Exercism Rust Track:  a solution for the sum-of-multiples exercise

/// Function sum_of_multiples() does what it must by brute force
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;

    for factor in factors {
        let mut multiple: u32 = *factor;

        while multiple < limit {
            sum += multiple;

            for bigger in factors {
                if bigger > factor && multiple % bigger == 0 {
                    sum -= multiple;
                    break;
                }
            }

            multiple += *factor;
        }
    }
    sum
}

//
// Most of the solutions used (1..limit).filter(...) so I guess are O(limit).
//
// Mine is sort of O(N**2) execution wise but requires constant memory.
//
