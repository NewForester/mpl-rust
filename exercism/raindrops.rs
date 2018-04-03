//! Exercism Rust Track:  a solution for the raindrops exercise

/// Function raindrops() takes a natural number and returns the corresponding 'sound' as per the read me
pub fn raindrops(nn: usize) -> String {
    let rules = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let mut result = String::new();

    for &(divisor, sound) in rules.iter() {
        if nn % divisor == 0 {
            result += sound
        }
    }

    if result == "" {
        result = nn.to_string()
    }

    result
}

//
// 6 of 8 unrolled the loop, not all used mm.to_string().  1 went overboard.
//
// The odd one out use pattern matching with guards (go it is possible):
//
pub fn _raindrops(n: usize) -> String {
    match n {
        n if (n % 3 == 0) => String::from("Pling"),
        n if (n % 5 == 0) => String::from("Plang"),
        n if (n % 7 == 0) => String::from("Plong"),
        _ => String::from(n.to_string()),
    }
}
//
// but sadly this does not work for all test caes.
//
