//! Exercism Rust Track:  a solution for the scrabble-score exercise

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

// Macro SCORES builds a HashMap for static use that maps (English) letters to Scrabble scores
lazy_static! {
    static ref SCORES: HashMap<char,u8> = {

        let mut scrabble_scores: HashMap<char,u8> = HashMap::new();

        let problem_statement = [
            ("A, E, I, O, U",   1),
            ("L, N, R, S, T",   1),
            ("D, G",            2),
            ("B, C, M, P",      3),
            ("F, H, V, W, Y",   4),
            ("K",               5),
            ("J, X",            8),
            ("Q, Z",           10),
        ];

        for pair in problem_statement.iter() {
            let score = pair.1;

            for letter in pair.0.chars() {
                if letter.is_alphabetic() {
                    scrabble_scores.insert(letter, score);
                }
            }
        }

        scrabble_scores
    };
}

// Function letter_score() takes a single letter and returns its (English) Scrabble score
fn letter_score(cc: char) -> i32 {
    match SCORES.get(&cc) {
        None => 0,
        Some(value) => (*value) as i32,
    }
}

// Function score() takes a word and returns its (English) Scrabble score
pub fn score(word: &str) -> i32 {
    word.to_ascii_uppercase().chars().map(letter_score).sum()
}

//
// So, use .to_ascii_uppercase() instead of .to_uppercase() and the German test
// passes.  Do not ask why.
//
// Moving the lookup into a separate routine hides the ugliness of the HashMap
// lookup.  In its final form, it gives us a default value for unrecognised letters.
//
// This allows the simple use of map() and sum().  Nothing fancy but just plain,
// simple to reason about, code.
//
// It seems from the crate documentation that you can declare and initialise a
// literal HashMap but by this time I was committed to finding a way of having
//     a) static data at the file level
//     b) initialising it at run-time
//
// The first is part of the language but discouraged.  The second uses a crate
// that the language designers would, no doubt, frown upon.
//
