//! Exercism Rust Track:  a solution for the scrabble-score exercise

use std::collections::HashMap;

// Function score() takes a word or string and returns what it would score in (English) Scrabble
pub fn score(word: &str) -> i32 {

    let letters = [
        'A', 'E', 'I', 'O', 'U',
        'L', 'N', 'R', 'S', 'T',
        'D', 'G',
        'B', 'C', 'M', 'P',
        'F', 'H', 'V', 'W', 'Y',
        'K',
        'J', 'X',
        'Q', 'Z',
    ];

    let letter_scores = [
        1, 1, 1, 1, 1,
        1, 1, 1, 1, 1,
        2, 2,
        3, 3, 3, 3,
        4, 4, 4, 4, 4,
        5,
        8, 8,
        10, 10,
    ];

    let scores: HashMap<_, _> = letters.iter().zip(letter_scores.iter()).collect();

    let mut sum = 0;

    for cc in word.to_uppercase().chars() {
        match scores.get(&cc) {
            None => (),
            Some(value) => sum = sum + **value,
        }
    }

    sum
}

//
// My solution is different from the majority of other solutions examined.
// Mine uses a 'dict':  all others used a large match statement (and they
// didn't use a macro).
//
// It seems you cannot declare a 'literal' HashMap in Rust.  My fiddling to
// create one had to use .zip and stuff so I should have used .map and stuff
// to finish the job.  Perhaps
//
// .map(| cc | **scores.get(**&cc).sum())
//
// but the return types of the .get routine is ugly and I might have had no end
// of trouble with it.  I expect I would have had to wrap it up in function.
//
// Several of the other solutions used .map and stuff with the syntax I am now
// familiar with but do not yet understand.
//
// In the middle they invoke a routine to score a single letter.  Reminds me of
// good coding practice as I once practised it.
//
// Some other solutions use to_ascii_lowercasee / to_ascii_uppercase.  I wonder
// if that solves the German letters problem.  I tried using the Unicode create
// but was told by cargo that it did not exist.
//
// Finally, I wonder if I could not use the nice idea I saw on the Go track by
// tewe to initialise my hash map.
//
