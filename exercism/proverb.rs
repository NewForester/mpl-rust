//! Exercism Rust Track:  a solution for the proverb exercise

/// Function build_proverb() takes a list of words and uses them to generate a proverb about loss
pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();

    if list.len() == 0 {
        return proverb;
    }

    let mut previous = list[0];

    for current in list[1..].iter() {
        proverb.push_str(&format!("For want of a {} the {} was lost.\n", previous, current));

        previous = current;
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}

//
// Of the solution examined only two had the same structure as mine.  These
// suggested a couple of minor changes to make my codd look more mature.
//
// The other half dozen solutions used map() and friends.  The worst case had:
//
//      .iter().zip().iter().skip().map().chain().collect().join()
//
// I suspect they are part of the 'team' that has agreed to do things this
// way for the hell of it as I can't see the benefit.
//
