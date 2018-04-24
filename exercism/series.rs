//! Exercism Rust Track:  a solution for the series exercise

/// Function series() takes a string and a length and returns all substrings of the given length
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let limit = if digits.len() >= len {digits.len() - len + 1} else {0};

    let mut rv: Vec<String> = Vec::with_capacity(limit);

    let mut ii = 0;
    while ii < limit {
        rv.push(digits[ii..ii+len].to_string());
        ii = ii + 1;
    }

    rv
}

//
// Of the 8 other solutions examined, 5 used map and friends.
//
// Of the three, none were as simple as mine (which is a direct translation
// from the Go).  Mine adds slices to the result.  One did the same but took
// the scenic route.  The other two looped through the digits one at a time.
// One mapped a digit at a time to each (pertinent) string in the result, the
// other did a for each string, for each character double loop.
//
// I do not pretend to understand the map and friends answers, which is a
// shame because I would have to think hard about how to solve this kind of
// problem using so call functional programming constructs.
//
// It occurs to me that perhaps half the friends are about making the other
// half look safe to the compiler and the other half that do real work may
// be a wacky as the loops in the 3 solutions.
//
