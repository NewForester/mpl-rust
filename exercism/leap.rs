//! Exercism Rust Track:  a solution for the leap (year) exercise


/// Function is_leap_year() takes an integer representing a year and returns true if that year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}

//
// This is a simple exercise that is crying out for pattern matching.  Alas,
// Rist has pattern matching instead of a case statement but has no guards.
//
// Most solution used a single expression that is less easy to reason about.
// The couple that did were not elegant.
//
// I miss pattern matching with guards.
//
