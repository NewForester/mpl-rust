//! Exercism Rust Track:  a solution for the difference-of-squares exercise

/// Function square_of_sum() takes a natural number, nn, and returns the square of the sum of the natural numbers [1..nn]
pub fn square_of_sum(nn: usize) -> usize {
    let sum = nn * (nn + 1) / 2;
    sum * sum
}

/// Function sum_of_squares() takes a natural number, nn, and returns the sum of the squares of the natural numbers [1..nn]
pub fn sum_of_squares(nn: usize) -> usize {
    nn*(nn+1)*(2*nn+1) / 6
}

/// Function difference() returns square_of_sum(nn) - sum_of_squares(nn)
pub fn difference(nn: usize) -> usize {
    square_of_sum(nn) - sum_of_squares(nn)
}

//
// Of the others examined, four used the formula and 4 map and friends
// to emulate the loop.
//
// Here is a friendly solution worth studying:
//
fn _square_of_sum(n: usize) -> usize {
    (1 .. n+1).fold(0, |acc, x| { acc + x }).pow(2)
}

fn _sum_of_squares(n: usize) -> usize {
    (1 .. n+1).fold(0, |acc, x| { acc + x.pow(2) })
}
//
// It is interesting there is no expliict casting between integers and floats.
// I also suspect that first of these might be clearer as:
//
fn __square_of_sum(n: usize) -> usize {
    (1 .. n+1).sum::<usize>().pow(2)
}
//
