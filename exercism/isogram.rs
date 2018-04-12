//! Exercism Rust Track:  a solution for the isogram exercise

/// Function isogram::check() takes a string of letters and returns true if they represent an isogram
pub fn check(phrase: &str) -> bool {
    let mut letters: Vec<char> = Vec::with_capacity(26);

    for cc in phrase.chars() {
        if cc.is_alphabetic() {
            let ll = cc.to_ascii_lowercase();

            if letters.contains(&ll) {
                return false
            }

            letters.push(ll);
        }
    }

    true
}

//
// My solution is a direct translation from Go.  The Go solution was a pleasure
// to write:  the Rust translation was however frustrating.
//
// The other solutions examined were disappointing.  One took the same approach
// as I did but the others used .map and friends.
//
// Such solutions have no early return.  Some mixed these with their own loops,
// some used far too many friends.  Several used sort and some other function
// to generate uniqueness and then a length comparison.  Some were plain wrong.
//
// Bleh.
//
