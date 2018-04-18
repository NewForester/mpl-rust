//! Exercism Rust Track:  a solution for the grains exercise

/// Function grains::square() returns the number of grains on square N
pub fn square(nn: u32) -> u64 {
    if nn >= 1 && nn <= 64 {
        1 << nn - 1
    } else {
        panic!("Square must be between 1 and 64")
    }
}

/// Function grains::total returns the total number of grains on the chessboard
pub fn total() -> u64 {
    0xffffffffffffffff
}

//
// The majority of solutions used the something like:
//
    2_u64.pow(s - 1)
//
// or:
//
    1_u64.wrapping_shl(s-1),
//
// to be clear that the result returned by square() needs to be 64 bits.
// All used something like:
//
    (1..64+1).map(square).sum()
//
// i∩ total().  One went so far as to use in square():
//
    (1..s).map(|_| 2).product()
//
// What I missed (again) was using match instead of if ... else:
//
    match s {
        1 ... 64 => 2u64.pow(s-1),
       _ => panic!("Square must be between 1 and 64")
   }
//
