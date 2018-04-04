//! Exercism Rust Track:  a solution for the acronym exercise

/// Function abbreviate() takes a technical term and returns an acronym for that term
pub fn abbreviate(term: &str) -> String {

    let mut result = String::new();
    let mut capitalise_next = true;
    let mut previous_lower = false;

    let chars: Vec<char> = term.chars().collect();
    for cc in chars {
        if capitalise_next || previous_lower && cc.is_uppercase() {
            result.push(cc);
        }

        capitalise_next = cc == ' ' || cc == '-';
        previous_lower = cc.is_lowercase();
    }

    result.to_uppercase()
}

//
// All but 2 of the 8 recent solutions started by splitting the term into to words
// and all appeared to have code to handle 'HyperText Markup Language'.
//
// However, only one managed to cope with the 'extras' without the code becoming
// a total shambles.  It looks like they are incapable of refactoring.
//
// Three of them use regular expressions, filters, maps and collect.
//
