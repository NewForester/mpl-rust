//! Exercism Rust Track:  a solution for the bob exercise


/// Function reply() takes something someone says to Bob and returns his answer
pub fn reply(message: &str) -> &str {

    let remark = message.trim();

    if remark == "" {
        "Fine. Be that way!"
    } else if remark.to_uppercase() == remark && remark.to_lowercase() != remark {
        if remark.ends_with("?") {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else {
        if remark.ends_with("?") {
            "Sure."
        } else {
            "Whatever."
        }
    }
}

//
// The eight contemporary solutions I looked all follow the same logic but
// appear to lack refactoring, suggesting the authors are either immature
// or not software engineers.
//
