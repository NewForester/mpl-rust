//! Exercism Rust Track:  a solution for the armstrong exercise

/// Function is_armstrong_number() takes a natural number and returns true if the number is an Armstrong number
pub fn is_armstrong_number(nn: u32) -> bool {
    let (result, _) = calculate(nn, 1);

    nn == result
}

// calculate returns the Armstrong sum using recursion but lets the boss make the decision
fn calculate(nn: u32, count: u32) -> (u32, u32) {
    let digit = nn % 10;

    let (mut partial, total) =
        if nn == digit {(0, count)} else {calculate(nn / 10, count  + 1)};

    let mut factor = digit;
    for _ in 1 .. total {factor *= digit;}

    partial += factor;

    (partial, total)
}

//
// All the solutions examined used map and friends.  Some even went so far as
// implement their own trait.
//
// The problem has two halves:  how many digits and then use that answer.  It
// seemed all these clever solutions calculated how many digits at least twice.
//
