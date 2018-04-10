//! Exercism Rust Track:  a solution for the beer-song exercise

/// Function verse() takes a natural number and returns the verse for that number of bottles of beer
pub fn verse(nn: i32) -> String {
    match nn {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),

        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),

        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),

        _ => {
            format!(
                "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",
                nn,
                nn - 1
            )
        }
    }
}

/// Function sing() takes two natural numbers and returns the verses so delimited
pub fn sing(start: i32, end: i32) -> String {
    let mut result = String::new();

    for ii in (end..start).rev() {
        result += &verse(ii + 1);
        result += "\n";
    }
    result += &verse(end);

    result
}

//
// My solution followed the same pattern as all the others.  My addition of
// the extra line feed between verses is as neat as anyone's.
//
// What I was looking for was how to break the long lines in verse() and this
// one solution did.  It just uses a trailing \ (as you would expect) and then
// indented the new line to fit and this passed the tests so it looks like the
// leading white space is ignored.
//
