//! Exercism Rust Track:  a solution for the reverse-string exercise

/// Function reverse() takes a string and returns another with the characters reversed with respect to the first
pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    let mut chars: Vec<char> = input.chars().collect();

    chars.reverse();

    for cc in chars {
        result.push(cc);
    }

    result
}

//
// Again I struggled to write anything that would compile.  My solution is the
// clumsiest of those I examined.  It could have been express simply as:
//
pub fn _reverse(input: &str) -> String {
    input.chars().rev().collect()
}
//
// However, this does not work with arbitary Unicode strings.  For these one
// needs an exernal create &c:
//
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn __reverse(input: &str) -> String {
   UnicodeSegmentation::graphemes(input, true).rev().collect()
}
