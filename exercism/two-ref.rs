//! Exercism Rust Track:  a solution for the two-fer exercise

/// Function twofer() takes a name and divides the spoils between us
pub fn twofer(name: &str)-> String {
    if name == "" {
        format!("One for {}, one for me.", "you")
    }
    else {
        format!("One for {}, one for me.", name)
    }
}

//
// As you might expect, lots of variation here.  Several
// used match and one of those even used recursion.
//
// Several convered the literal string to a String proper and
// even here there was variation.
//
// The absolute simplest is probably:
//
pub fn _twofer(name: &str)-> String {
    format!(
        "One for {}, one for me.",
        if name.is_empty() { "you" } else { name }
    )
}
//
// but ?: is so ugly in Rust.
//
