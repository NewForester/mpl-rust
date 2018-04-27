//! Exercism Rust Track:  a solution for the collatz-conjecture exercise

/// Function collatz() takes a natural number and returns the number of steps required to reach 1
pub fn collatz(mut nn: u64) -> Option<u64> {
    let mut steps = 0;

    loop {
        nn = match nn {
            1                   => return Some(steps),
            nn if nn <= 0       => return None,
            nn if nn % 2 == 0   => nn / 2,
            _                   => 3 * nn + 1,
        };
        steps += 1;
    }
}

//
// Of the 8 examined, 1 was a clone and the other 7 showed considerable
// variety, including use of loop, match, recursions and implementing an
// iterator so they could use count().
//
// The solution that was cloned is astonishing:
//
pub fn _collatz(n: u64) -> Option<u64> {
  match n {
    0 => None,
    1 => Some(0),
    n if n % 2 == 0 => collatz(n / 2).map(|x| x + 1),
    n => collatz(3 * n + 1).map(|x| x + 1),
  }
}
//
// I wonder how it performs.
//
